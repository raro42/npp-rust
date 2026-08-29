//! File open/save with optional background loading for large files.
//!
//! Text load/save uses UTF-8 in memory. On load, a UTF-8 BOM is kept as U+FEFF.
//! Bytes that are not valid UTF-8 decode as Windows-1252 (lossy stand-in for ANSI).

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use thiserror::Error;

/// Files larger than this use a background thread to load.
pub const LARGE_FILE_THRESHOLD: u64 = 2 * 1024 * 1024; // 2 MiB

/// UTF-8 BOM bytes.
const UTF8_BOM_BYTES: &[u8] = &[0xEF, 0xBB, 0xBF];

/// UTF-8 BOM as a Unicode character (save may write it as `EF BB BF`).
pub const UTF8_BOM_CHAR: char = '\u{FEFF}';

#[derive(Debug, Error)]
pub enum FsError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("path has no parent")]
    NoParent,
}

pub type Result<T> = std::result::Result<T, FsError>;

/// Encoding detected on load, or chosen for save.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEncoding {
    /// Valid UTF-8, no BOM.
    Utf8,
    /// Valid UTF-8 with a leading BOM.
    Utf8Bom,
    /// Not valid UTF-8; decoded (or encoded) as Windows-1252.
    Windows1252,
}

impl TextEncoding {
    pub fn label(self) -> &'static str {
        match self {
            Self::Utf8 => "UTF-8",
            Self::Utf8Bom => "UTF-8-BOM",
            Self::Windows1252 => "Windows-1252",
        }
    }
}

/// Result of a background (or sync) open.
#[derive(Debug)]
pub struct OpenResult {
    pub path: PathBuf,
    pub content: String,
    pub bytes: u64,
    pub elapsed_ms: u128,
    /// Encoding used to build `content`.
    pub encoding: TextEncoding,
    /// Short note when load used a fallback (for example Windows-1252).
    pub encoding_note: Option<String>,
}

impl OpenResult {
    /// Build a result with UTF-8 and no note (tests / simple callers).
    pub fn new(path: PathBuf, content: String, bytes: u64, elapsed_ms: u128) -> Self {
        Self {
            path,
            content,
            bytes,
            elapsed_ms,
            encoding: TextEncoding::Utf8,
            encoding_note: None,
        }
    }
}

/// Message from background loader.
#[derive(Debug)]
pub enum LoadMsg {
    Done(OpenResult),
    Failed { path: PathBuf, error: String },
}

/// Channel pair for background opens.
pub struct LoadChannel {
    pub tx: Sender<LoadMsg>,
    pub rx: Receiver<LoadMsg>,
}

impl LoadChannel {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { tx, rx }
    }
}

impl Default for LoadChannel {
    fn default() -> Self {
        Self::new()
    }
}

/// Max bytes read in one tail poll (keeps bursts bounded).
pub const TAIL_MAX_CHUNK: u64 = 1024 * 1024;

/// Outcome of one background tail poll (GUI applies this; no disk I/O on UI thread).
#[derive(Debug)]
pub enum TailOutcome {
    /// No new data (size == offset).
    Unchanged { size: u64 },
    /// Appended UTF-8 text (lossy) and new file offset after a complete decode.
    Appended { text: String, size: u64 },
    /// File shrank; worker already reloaded content for the GUI.
    Rotated {
        content: String,
        bytes: u64,
        encoding: TextEncoding,
        encoding_note: Option<String>,
    },
    /// File shrank but reload failed.
    RotatedReloadFailed { size: u64, error: String },
}

/// Message from a background tail worker.
#[derive(Debug)]
pub struct TailMsg {
    pub path: PathBuf,
    /// Offset the worker was asked to read from (GUI drops stale results).
    pub offset: u64,
    pub outcome: std::result::Result<TailOutcome, String>,
}

/// Channel pair for background tail polls.
pub struct TailChannel {
    pub tx: Sender<TailMsg>,
    pub rx: Receiver<TailMsg>,
}

impl TailChannel {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { tx, rx }
    }
}

impl Default for TailChannel {
    fn default() -> Self {
        Self::new()
    }
}

/// Decode file bytes to a UTF-8 string and an encoding label.
pub fn decode_bytes(buf: &[u8]) -> (String, TextEncoding, Option<String>) {
    let has_bom = buf.starts_with(UTF8_BOM_BYTES);
    let body = if has_bom { &buf[3..] } else { buf };

    if let Ok(body_str) = std::str::from_utf8(body) {
        let mut content = String::with_capacity(body_str.len() + if has_bom { 3 } else { 0 });
        if has_bom {
            content.push(UTF8_BOM_CHAR);
        }
        content.push_str(body_str);
        let encoding = if has_bom {
            TextEncoding::Utf8Bom
        } else {
            TextEncoding::Utf8
        };
        return (content, encoding, None);
    }

    // Not valid UTF-8 (ignore a false BOM prefix — decode the whole buffer).
    let content = decode_windows_1252(buf);
    let note = Some("Not valid UTF-8; decoded as Windows-1252".to_string());
    (content, TextEncoding::Windows1252, note)
}

/// Read file synchronously (small files).
pub fn read_file(path: &Path) -> Result<OpenResult> {
    let start = std::time::Instant::now();
    let meta = fs::metadata(path)?;
    let bytes = meta.len();
    let mut file = File::open(path)?;
    let mut buf = Vec::with_capacity(bytes.min(64 * 1024 * 1024) as usize);
    file.read_to_end(&mut buf)?;
    let (content, encoding, encoding_note) = decode_bytes(&buf);
    Ok(OpenResult {
        path: path.to_path_buf(),
        content,
        bytes,
        elapsed_ms: start.elapsed().as_millis(),
        encoding,
        encoding_note,
    })
}

/// Write text with encoding inferred from a leading U+FEFF (UTF-8-BOM) or plain UTF-8.
pub fn write_file(path: &Path, content: &str) -> Result<()> {
    let encoding = if content.starts_with(UTF8_BOM_CHAR) {
        TextEncoding::Utf8Bom
    } else {
        TextEncoding::Utf8
    };
    write_file_with_encoding(path, content, encoding)
}

/// Write text using an explicit encoding.
///
/// - [`TextEncoding::Utf8`]: UTF-8 bytes, no BOM (strips a leading U+FEFF).
/// - [`TextEncoding::Utf8Bom`]: UTF-8 with `EF BB BF` prefix.
/// - [`TextEncoding::Windows1252`]: lossy Windows-1252 (ANSI stand-in); no BOM.
///
/// Saves write a sibling temp file, flush (`sync_all`), then rename over the target.
/// Missing parent directories are not created.
pub fn write_file_with_encoding(path: &Path, content: &str, encoding: TextEncoding) -> Result<()> {
    write_bytes_atomic(path, &encode_content(content, encoding))
}

/// Encode in-memory text to on-disk bytes for `encoding`.
fn encode_content(content: &str, encoding: TextEncoding) -> Vec<u8> {
    let body = content.strip_prefix(UTF8_BOM_CHAR).unwrap_or(content);
    match encoding {
        TextEncoding::Utf8 => body.as_bytes().to_vec(),
        TextEncoding::Utf8Bom => {
            let mut out = Vec::with_capacity(UTF8_BOM_BYTES.len() + body.len());
            out.extend_from_slice(UTF8_BOM_BYTES);
            out.extend_from_slice(body.as_bytes());
            out
        }
        TextEncoding::Windows1252 => encode_windows_1252_lossy(body),
    }
}

/// Directory that holds `path`. A bare file name uses `.`.
fn parent_dir(path: &Path) -> Result<&Path> {
    match path.parent() {
        None => Err(FsError::NoParent),
        Some(p) if p.as_os_str().is_empty() => Ok(Path::new(".")),
        Some(p) => Ok(p),
    }
}

/// Write `bytes` via temp sibling + `sync_all` + rename. Does not create parents.
fn write_bytes_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let dir = parent_dir(path)?;
    let file_name = path.file_name().ok_or_else(|| {
        FsError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "path has no file name",
        ))
    })?;

    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let temp_name = format!(
        ".{}.{}.{}.tmp",
        file_name.to_string_lossy(),
        std::process::id(),
        nanos
    );
    let temp_path = dir.join(temp_name);

    let result = (|| -> Result<()> {
        {
            let mut file = File::create(&temp_path)?;
            file.write_all(bytes)?;
            // Durable contents before the rename replaces the target.
            file.sync_all()?;
        }

        // Keep mode / readonly bit when the target already exists.
        if let Ok(meta) = fs::metadata(path) {
            let _ = fs::set_permissions(&temp_path, meta.permissions());
        }

        // Unix: atomic replace. Windows: std rename replaces via MoveFileEx / fallback.
        fs::rename(&temp_path, path)?;

        // Best-effort durable directory entry (often a no-op or error on Windows).
        let _ = sync_parent_dir(dir);
        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }
    result
}

fn sync_parent_dir(dir: &Path) -> std::io::Result<()> {
    File::open(dir)?.sync_all()
}

/// File size in bytes, if the path exists.
pub fn file_size(path: &Path) -> Result<u64> {
    Ok(fs::metadata(path)?.len())
}

/// Read new bytes from `offset` to EOF (for log tail). Empty if no growth.
/// If the file shrank (rotation), returns [`TailRead::Rotated`] so the caller can reload.
#[derive(Debug)]
pub enum TailRead {
    /// No new data (size == offset).
    Unchanged { size: u64 },
    /// Appended UTF-8 text (lossy) and new file size.
    Appended { text: String, size: u64 },
    /// File smaller than offset — likely rotated/truncated.
    Rotated { size: u64 },
}

pub fn read_tail_since(path: &Path, offset: u64) -> Result<TailRead> {
    let size = file_size(path)?;
    if size < offset {
        return Ok(TailRead::Rotated { size });
    }
    if size == offset {
        return Ok(TailRead::Unchanged { size });
    }
    use std::io::{Seek, SeekFrom};
    // Cap one poll so a huge burst cannot freeze or OOM the UI.
    let want = (size - offset).min(TAIL_MAX_CHUNK);
    let mut file = File::open(path)?;
    file.seek(SeekFrom::Start(offset))?;
    let mut buf = vec![0u8; want as usize];
    let mut read_total = 0usize;
    while read_total < buf.len() {
        match file.read(&mut buf[read_total..]) {
            Ok(0) => break,
            Ok(n) => read_total += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e.into()),
        }
    }
    buf.truncate(read_total);
    let (text, consumed) = decode_utf8_prefix(&buf);
    let new_offset = offset + consumed;
    Ok(TailRead::Appended {
        text,
        size: new_offset,
    })
}

/// Run [`read_tail_since`] (and rotate reload) on a worker thread; send [`TailMsg`].
pub fn poll_tail_async(path: PathBuf, offset: u64, tx: Sender<TailMsg>) {
    thread::spawn(move || {
        let outcome = match read_tail_since(&path, offset) {
            Ok(TailRead::Unchanged { size }) => Ok(TailOutcome::Unchanged { size }),
            Ok(TailRead::Appended { text, size }) => Ok(TailOutcome::Appended { text, size }),
            Ok(TailRead::Rotated { size }) => match read_file(&path) {
                Ok(r) => Ok(TailOutcome::Rotated {
                    content: r.content,
                    bytes: r.bytes,
                    encoding: r.encoding,
                    encoding_note: r.encoding_note,
                }),
                Err(e) => Ok(TailOutcome::RotatedReloadFailed {
                    size,
                    error: e.to_string(),
                }),
            },
            Err(e) => Err(e.to_string()),
        };
        let _ = tx.send(TailMsg {
            path,
            offset,
            outcome,
        });
    });
}

/// Decode as much valid UTF-8 as possible from `buf`.
/// Incomplete trailing multi-byte sequences are left for the next poll.
fn decode_utf8_prefix(buf: &[u8]) -> (String, u64) {
    let end = utf8_complete_len(buf);
    let text = String::from_utf8_lossy(&buf[..end]).into_owned();
    (text, end as u64)
}

fn utf8_complete_len(buf: &[u8]) -> usize {
    if buf.is_empty() {
        return 0;
    }
    let mut i = buf.len();
    // Walk back over continuation bytes (10xxxxxx).
    while i > 0 && (buf[i - 1] & 0xC0) == 0x80 {
        i -= 1;
    }
    if i == 0 {
        // All continuation — nothing complete.
        return 0;
    }
    let lead = buf[i - 1];
    let need = if lead & 0x80 == 0 {
        1
    } else if lead & 0xE0 == 0xC0 {
        2
    } else if lead & 0xF0 == 0xE0 {
        3
    } else if lead & 0xF8 == 0xF0 {
        4
    } else {
        // Invalid lead; drop it so the next poll can recover.
        return i - 1;
    };
    let have = buf.len() - (i - 1);
    if have < need {
        i - 1
    } else {
        buf.len()
    }
}

/// Open on a background thread and send [`LoadMsg`] when done.
pub fn open_async(path: PathBuf, tx: Sender<LoadMsg>) {
    thread::spawn(move || match read_file(&path) {
        Ok(result) => {
            let _ = tx.send(LoadMsg::Done(result));
        }
        Err(e) => {
            let _ = tx.send(LoadMsg::Failed {
                path,
                error: e.to_string(),
            });
        }
    });
}

/// Choose sync vs async based on size. Returns `Some(OpenResult)` if sync; else starts async.
pub fn open_auto(path: PathBuf, tx: Sender<LoadMsg>) -> Result<Option<OpenResult>> {
    let size = file_size(&path)?;
    if size >= LARGE_FILE_THRESHOLD {
        open_async(path, tx);
        Ok(None)
    } else {
        Ok(Some(read_file(&path)?))
    }
}

/// Windows-1252 byte → Unicode scalar (0x80..=0x9F differ from Latin-1).
fn windows_1252_char(byte: u8) -> char {
    match byte {
        0x80 => '\u{20AC}',
        0x81 => '\u{0081}',
        0x82 => '\u{201A}',
        0x83 => '\u{0192}',
        0x84 => '\u{201E}',
        0x85 => '\u{2026}',
        0x86 => '\u{2020}',
        0x87 => '\u{2021}',
        0x88 => '\u{02C6}',
        0x89 => '\u{2030}',
        0x8A => '\u{0160}',
        0x8B => '\u{2039}',
        0x8C => '\u{0152}',
        0x8D => '\u{008D}',
        0x8E => '\u{017D}',
        0x8F => '\u{008F}',
        0x90 => '\u{0090}',
        0x91 => '\u{2018}',
        0x92 => '\u{2019}',
        0x93 => '\u{201C}',
        0x94 => '\u{201D}',
        0x95 => '\u{2022}',
        0x96 => '\u{2013}',
        0x97 => '\u{2014}',
        0x98 => '\u{02DC}',
        0x99 => '\u{2122}',
        0x9A => '\u{0161}',
        0x9B => '\u{203A}',
        0x9C => '\u{0153}',
        0x9D => '\u{009D}',
        0x9E => '\u{017E}',
        0x9F => '\u{0178}',
        b => b as char,
    }
}

fn decode_windows_1252(buf: &[u8]) -> String {
    let mut out = String::with_capacity(buf.len());
    for &b in buf {
        out.push(windows_1252_char(b));
    }
    out
}

fn windows_1252_byte(c: char) -> Option<u8> {
    let u = c as u32;
    if u <= 0x7F || (0xA0..=0xFF).contains(&u) {
        return Some(u as u8);
    }
    Some(match c {
        '\u{20AC}' => 0x80,
        '\u{0081}' => 0x81,
        '\u{201A}' => 0x82,
        '\u{0192}' => 0x83,
        '\u{201E}' => 0x84,
        '\u{2026}' => 0x85,
        '\u{2020}' => 0x86,
        '\u{2021}' => 0x87,
        '\u{02C6}' => 0x88,
        '\u{2030}' => 0x89,
        '\u{0160}' => 0x8A,
        '\u{2039}' => 0x8B,
        '\u{0152}' => 0x8C,
        '\u{008D}' => 0x8D,
        '\u{017D}' => 0x8E,
        '\u{008F}' => 0x8F,
        '\u{0090}' => 0x90,
        '\u{2018}' => 0x91,
        '\u{2019}' => 0x92,
        '\u{201C}' => 0x93,
        '\u{201D}' => 0x94,
        '\u{2022}' => 0x95,
        '\u{2013}' => 0x96,
        '\u{2014}' => 0x97,
        '\u{02DC}' => 0x98,
        '\u{2122}' => 0x99,
        '\u{0161}' => 0x9A,
        '\u{203A}' => 0x9B,
        '\u{0153}' => 0x9C,
        '\u{009D}' => 0x9D,
        '\u{017E}' => 0x9E,
        '\u{0178}' => 0x9F,
        _ => return None,
    })
}

/// Lossy encode to Windows-1252. Unmapped characters become `?` (0x3F).
pub fn encode_windows_1252_lossy(text: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len());
    for c in text.chars() {
        out.push(windows_1252_byte(c).unwrap_or(b'?'));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// Unique temp directory for one test (cleaned by the OS later).
    fn temp_test_dir(label: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!(
            "npp-rs-fs-{}-{}-{}",
            label,
            std::process::id(),
            nanos
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn roundtrip_temp() {
        let dir = temp_test_dir("roundtrip");
        let path = dir.join("hello.txt");
        write_file(&path, "hi").unwrap();
        let r = read_file(&path).unwrap();
        assert_eq!(r.content, "hi");
        assert_eq!(r.encoding, TextEncoding::Utf8);
        assert!(r.encoding_note.is_none());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_utf8_bom_keeps_bom_char() {
        let dir = temp_test_dir("bom-load");
        let path = dir.join("bom.txt");
        let mut raw = Vec::from(UTF8_BOM_BYTES);
        raw.extend_from_slice(b"hello");
        fs::write(&path, &raw).unwrap();
        let r = read_file(&path).unwrap();
        assert_eq!(r.encoding, TextEncoding::Utf8Bom);
        assert!(r.content.starts_with(UTF8_BOM_CHAR));
        assert_eq!(&r.content[UTF8_BOM_CHAR.len_utf8()..], "hello");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn save_utf8_bom_writes_bom_bytes() {
        let dir = temp_test_dir("bom-save");
        let path = dir.join("save-bom.txt");
        let mut text = String::new();
        text.push(UTF8_BOM_CHAR);
        text.push('x');
        write_file(&path, &text).unwrap();
        let raw = fs::read(&path).unwrap();
        assert!(raw.starts_with(UTF8_BOM_BYTES));
        assert_eq!(&raw[3..], b"x");
        write_file_with_encoding(&path, &text, TextEncoding::Utf8).unwrap();
        let raw2 = fs::read(&path).unwrap();
        assert_eq!(raw2, b"x");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_windows_1252_when_not_utf8() {
        let dir = temp_test_dir("ansi-load");
        let path = dir.join("ansi.txt");
        fs::write(&path, [0x80u8, b' ', b'E']).unwrap();
        let r = read_file(&path).unwrap();
        assert_eq!(r.encoding, TextEncoding::Windows1252);
        assert!(r.encoding_note.is_some());
        assert_eq!(r.content, "\u{20AC} E");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn save_windows_1252_lossy() {
        let dir = temp_test_dir("ansi-save");
        let path = dir.join("ansi-out.txt");
        write_file_with_encoding(&path, "\u{20AC}?\u{1F600}", TextEncoding::Windows1252).unwrap();
        let raw = fs::read(&path).unwrap();
        assert_eq!(raw, [0x80, b'?', b'?']);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn async_open() {
        let dir = temp_test_dir("async");
        let path = dir.join("async.txt");
        write_file(&path, "async-data").unwrap();
        let ch = LoadChannel::new();
        open_async(path.clone(), ch.tx.clone());
        let msg = ch.rx.recv_timeout(Duration::from_secs(5)).unwrap();
        match msg {
            LoadMsg::Done(r) => assert_eq!(r.content, "async-data"),
            LoadMsg::Failed { error, .. } => panic!("{error}"),
        }
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn large_file_uses_async_path() {
        let dir = temp_test_dir("large");
        let path = dir.join("large.bin.txt");
        let chunk = "x".repeat(1024);
        let mut content = String::new();
        let target = LARGE_FILE_THRESHOLD as usize + 1024;
        while content.len() < target {
            content.push_str(&chunk);
        }
        write_file(&path, &content).unwrap();
        assert!(file_size(&path).unwrap() >= LARGE_FILE_THRESHOLD);

        let ch = LoadChannel::new();
        let sync = open_auto(path.clone(), ch.tx.clone()).unwrap();
        assert!(sync.is_none(), "large file should not load synchronously");
        let msg = ch.rx.recv_timeout(Duration::from_secs(30)).unwrap();
        match msg {
            LoadMsg::Done(r) => {
                assert_eq!(r.content.len(), content.len());
                assert!(r.elapsed_ms < 60_000);
            }
            LoadMsg::Failed { error, .. } => panic!("{error}"),
        }
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn tail_appends_and_detects_rotation() {
        let dir = temp_test_dir("tail");
        let path = dir.join("tail.log");
        write_file(&path, "line1\n").unwrap();
        let size = file_size(&path).unwrap();
        match read_tail_since(&path, size).unwrap() {
            TailRead::Unchanged { .. } => {}
            other => panic!("expected unchanged: {other:?}"),
        }
        {
            use std::io::Write;
            let mut f = fs::OpenOptions::new().append(true).open(&path).unwrap();
            f.write_all(b"line2\n").unwrap();
        }
        let after_append = match read_tail_since(&path, size).unwrap() {
            TailRead::Appended { text, size: new } => {
                assert_eq!(text, "line2\n");
                assert!(new > size);
                new
            }
            other => panic!("expected append: {other:?}"),
        };
        write_file(&path, "x\n").unwrap();
        match read_tail_since(&path, after_append).unwrap() {
            TailRead::Rotated { .. } => {}
            other => panic!("expected rotated: {other:?}"),
        }
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn utf8_complete_len_drops_partial_trailing() {
        assert_eq!(utf8_complete_len(&[0x41, 0xC3]), 1);
        assert_eq!(utf8_complete_len(&[0x41, 0xC3, 0xA9]), 3);
        let (text, n) = decode_utf8_prefix(&[0x41, 0xC3]);
        assert_eq!(text, "A");
        assert_eq!(n, 1);
    }

    #[test]
    fn atomic_overwrite_replaces_contents() {
        let dir = temp_test_dir("atomic-ow");
        let path = dir.join("doc.txt");
        write_file(&path, "old-content").unwrap();
        write_file(&path, "new-content").unwrap();
        let r = read_file(&path).unwrap();
        assert_eq!(r.content, "new-content");
        let leftovers: Vec<_> = fs::read_dir(&dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().ends_with(".tmp"))
            .collect();
        assert!(leftovers.is_empty(), "temp siblings left: {leftovers:?}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn atomic_save_does_not_create_missing_parent() {
        let dir = temp_test_dir("atomic-noparent");
        let missing = dir.join("no-such-dir").join("file.txt");
        let err = write_file(&missing, "x").unwrap_err();
        match err {
            FsError::Io(e) => assert_eq!(e.kind(), std::io::ErrorKind::NotFound),
            other => panic!("expected NotFound io error, got {other:?}"),
        }
        assert!(!missing.exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[cfg(unix)]
    #[test]
    fn atomic_overwrite_preserves_unix_mode() {
        use std::os::unix::fs::PermissionsExt;
        let dir = temp_test_dir("atomic-mode");
        let path = dir.join("mode.txt");
        write_file(&path, "a").unwrap();
        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o640);
        fs::set_permissions(&path, perms).unwrap();
        write_file(&path, "b").unwrap();
        let mode = fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o640);
        assert_eq!(read_file(&path).unwrap().content, "b");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn poll_tail_async_sends_appended() {
        let dir = std::env::temp_dir().join(format!(
            "npp-rs-fs-tail-async-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("tail-async.log");
        write_file(&path, "line1\n").unwrap();
        let size = file_size(&path).unwrap();
        {
            use std::io::Write;
            let mut f = fs::OpenOptions::new().append(true).open(&path).unwrap();
            f.write_all(b"line2\n").unwrap();
        }
        let ch = TailChannel::new();
        poll_tail_async(path.clone(), size, ch.tx.clone());
        let msg = ch.rx.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(msg.path, path);
        assert_eq!(msg.offset, size);
        match msg.outcome.unwrap() {
            TailOutcome::Appended { text, size: new } => {
                assert_eq!(text, "line2\n");
                assert!(new > size);
            }
            other => panic!("expected append: {other:?}"),
        }
        let _ = fs::remove_dir_all(&dir);
    }
}
