# Highlight / format / plugins notes

Date: 2026-08-28

## tree-sitter language crates (with tree-sitter / tree-sitter-highlight 0.24)

| Crate | Version | Role |
|-------|---------|------|
| tree-sitter-python | 0.23 (resolves 0.23.6) | LANGUAGE + HIGHLIGHTS_QUERY; uses tree-sitter-language |
| tree-sitter-sequel | 0.3 (DerekStride SQL) | Published SQL grammar for modern tree-sitter; tree-sitter-sql 0.0.2 needs tree-sitter ^0.19 |
| tree-sitter-md | 0.3.2 | Block LANGUAGE + HIGHLIGHT_QUERY_BLOCK; default features omit optional tree-sitter 0.23 |

Existing: tree-sitter-rust/c/cpp 0.23, tree-sitter-json 0.24.

Sources: crates.io dependency metadata for each crate.
