# File encoding

Date: 2026-08-29

## Memory

The editor always keeps text as UTF-8 in the buffer.

## Load

| Disk bytes | Result |
|------------|--------|
| Valid UTF-8 with BOM | UTF-8-BOM (keeps U+FEFF in memory) |
| Valid UTF-8, no BOM | UTF-8 |
| Not valid UTF-8 | Windows-1252 decode (lossy stand-in for ANSI) |

The tab stores the chosen encoding for the next save.

## Save (Format menu)

| Menu | Save bytes |
|------|------------|
| UTF-8 | UTF-8, no BOM |
| UTF-8-BOM | UTF-8 with `EF BB BF` |
| ANSI | Windows-1252 (lossy; chars outside the code page become `?`) |

## Code

- `fs::write_file_with_encoding` — `crates/fs` (atomic: temp sibling + `sync_all` + rename; no auto parent dirs)
- `Document.encoding` / `FileEncoding` — `crates/doc`
- Format handlers — `crates/app/src/commands/format.rs`
