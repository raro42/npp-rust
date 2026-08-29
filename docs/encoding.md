# File encoding

Date: 2026-08-29

## Memory

The editor always keeps text as UTF-8 in the buffer.

## Load

| Disk bytes | Result |
|------------|--------|
| Valid UTF-8 with BOM | UTF-8-BOM (keeps U+FEFF in memory) |
| Valid UTF-8, no BOM | UTF-8 |
| Not valid UTF-8 | Windows-1252 decode (no U+FFFD); tab encoding set for save |

Open never uses `String::from_utf8_lossy`. Invalid bytes do not become permanent UTF-8 replacement characters.

The status bar reports the fallback. The tab stores the chosen encoding for the next save.

## Tail follow

Appended chunks keep valid UTF-8. Invalid bytes map via Windows-1252. Status shows a note when that happens.

## Save (Format menu)

| Menu | Save bytes |
|------|------------|
| UTF-8 | UTF-8, no BOM |
| UTF-8-BOM | UTF-8 with `EF BB BF` |
| ANSI | Windows-1252 (lossy; chars outside the code page become `?`) |

## Code

- `fs::decode_bytes` / `fs::count_windows_1252_unmapped` — `crates/fs`
- `Document.encoding` / `FileEncoding` — `crates/doc`
- Format handlers — `crates/app/src/commands/format.rs`
