# File encoding

Date: 2026-08-31

## Memory

The editor always keeps text as UTF-8 in the buffer.

## Load

| Disk bytes | Result |
|------------|--------|
| UTF-16 BE BOM (`FE FF`) | UTF-16 BE; decode to Unicode (no BOM char in memory) |
| UTF-16 LE BOM (`FF FE`) | UTF-16 LE; decode to Unicode (no BOM char in memory) |
| Valid UTF-8 with BOM | UTF-8-BOM (keeps U+FEFF in memory) |
| Valid UTF-8, no BOM | UTF-8 |
| Not valid UTF-8 | Windows-1252 decode (no U+FFFD); tab encoding set for save |

Open never uses `String::from_utf8_lossy`. Invalid UTF-8 bytes do not become permanent UTF-8 replacement characters.

UTF-16 unpaired surrogates may become U+FFFD. A trailing odd byte on UTF-16 is dropped (status note).

The status bar reports the fallback. The tab stores the chosen encoding for the next save.

## Tail follow

Appended chunks keep valid UTF-8. Invalid bytes map via Windows-1252. Status shows a note when that happens.

## Save (Format menu)

| Menu | Save bytes |
|------|------------|
| UTF-8 | UTF-8, no BOM |
| UTF-8-BOM | UTF-8 with `EF BB BF` |
| UTF-16 LE BOM | UTF-16 LE with `FF FE` |
| UTF-16 BE BOM | UTF-16 BE with `FE FF` |
| ANSI | Windows-1252 (lossy; chars outside the code page become `?`) |

Convert-to menu items set the same per-tab save encoding.

## Code

- `fs::decode_bytes` / `fs::count_windows_1252_unmapped` — `crates/fs`
- `Document.encoding` / `FileEncoding` — `crates/doc`
- Format handlers — `crates/app/src/commands/format.rs`
