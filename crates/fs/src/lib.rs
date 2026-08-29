//! File open/save with optional background loading for large files.

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use thiserror::Error;

/// Files larger than this use a background thread to load.
pub const LARGE_FILE_THRESHOLD: u64 = 2 * 1024 * 1024; // 2 MiB

#[derive(Debug, Error)]
pub enum FsError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("path has no parent")]
    NoParent,
}

pub type Result<T> = std::result::Result<T, FsError>;

/// Result of a background (or sync) open.
#[derive(Debug)]
pub struct OpenResult {
    pub path: PathBuf,
    pub content: String,
    pub bytes: u64,
    pub elapsed_ms: u128,
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

/// Read file synchronously (small files).
pub fn read_file(path: &Path) -> Result<OpenResult> {
    let start = std::time::Instant::now();
    let meta = fs::metadata(path)?;
    let bytes = meta.len();
    let mut file = File::open(path)?;
    let mut buf = Vec::with_capacity(bytes.min(64 * 1024 * 1024) as usize);
    file.read_to_end(&mut buf)?;
    let content = String::from_utf8_lossy(&buf).into_owned();
    Ok(OpenResult {
        path: path.to_path_buf(),
        content,
        bytes,
        elapsed_ms: start.elapsed().as_millis(),
    })
}

/// Write UTF-8 text to path (creates parent dirs if needed).
pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
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
    const MAX_CHUNK: u64 = 1024 * 1024;
    let want = (size - offset).min(MAX_CHUNK);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn roundtrip_temp() {
        let dir = std::env::temp_dir().join("npp-rs-fs-test");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("hello.txt");
        write_file(&path, "hi").unwrap();
        let r = read_file(&path).unwrap();
        assert_eq!(r.content, "hi");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn async_open() {
        let dir = std::env::temp_dir().join("npp-rs-fs-test");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("async.txt");
        write_file(&path, "async-data").unwrap();
        let ch = LoadChannel::new();
        open_async(path.clone(), ch.tx.clone());
        let msg = ch.rx.recv_timeout(Duration::from_secs(5)).unwrap();
        match msg {
            LoadMsg::Done(r) => assert_eq!(r.content, "async-data"),
            LoadMsg::Failed { error, .. } => panic!("{error}"),
        }
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn large_file_uses_async_path() {
        let dir = std::env::temp_dir().join("npp-rs-fs-test");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("large.bin.txt");
        // Just over the threshold.
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
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn tail_appends_and_detects_rotation() {
        let dir = std::env::temp_dir().join("npp-rs-fs-test");
        let _ = fs::create_dir_all(&dir);
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
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn utf8_complete_len_drops_partial_trailing() {
        // "é" is C3 A9; leave only C3.
        assert_eq!(utf8_complete_len(&[0x41, 0xC3]), 1);
        assert_eq!(utf8_complete_len(&[0x41, 0xC3, 0xA9]), 3);
        let (text, n) = decode_utf8_prefix(&[0x41, 0xC3]);
        assert_eq!(text, "A");
        assert_eq!(n, 1);
    }
}
