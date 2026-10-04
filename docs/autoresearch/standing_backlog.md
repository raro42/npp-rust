# Overnight standing backlog (npp-rs)

When the issue queue is idle, overnight 009 **must** pull from this list (top first). Strike through items after a keep. Keep this file short. Product gaps live in `docs/whats-missing.md` and `docs/next-gaps.md`.

## P0 — editor correctness

1. **Find / replace edge cases** — ~~wrap toggle~~, ~~regex that does not hang the UI~~, ~~selection-only (Find bar **Sel**, multi-line auto-arm)~~.
2. **Save encoding** — ~~unmapped ANSI chars~~, ~~BOM-less UTF-16 detect~~ (`docs/encoding.md`).
3. **Undo / typing coalesce** — ~~multi-caret / column typing coalesce~~ (`docs/undo-transactions.md`, `docs/column-mode.md`).

## P1 — depth vs Notepad++ (small batches)

4. **Shortcut mapper** — ~~word-wrap remap (`shortcut_word_wrap`)~~. Broader remap / `shortcuts.xml` later.
5. **Compare** — ignore-whitespace exists. ~~next/prev hunk nav (F7)~~. ~~first-hunk jump + ordinal status~~. ~~ignore-case~~. ~~dual-pane hunk align + identical status~~. ~~first/last difference (⌘/Ctrl+F7)~~. ~~keep compare when closing a non-pair tab~~. ~~swap compare sides~~. ~~View ignore toggles + start dual-pane park + status ignore flags~~. ~~click/keyboard change line syncs other pane + L|R nav status~~. ~~next/prev wrap status (`· wrapped`)~~. ~~nav selects hunk on focused pane~~. ~~both-pane hunk select (nav/click/start)~~. ~~copy unified diff~~. ~~open unified diff tab~~. ~~copy current hunk~~. ~~open current hunk tab~~. ~~apply hunk from other view~~. ~~apply hunk to other view~~. ~~char-level replace-hunk wash~~. ~~apply all hunks from other view~~. ~~apply all hunks to other view~~. ~~apply hunk advances to next remaining~~. ~~ignore blank lines~~. ~~compare to saved (disk snapshot)~~. ~~clear closes saved snapshot tab~~. 3-way stays later. More 2-way UX polish (`docs/compare.md`).
6. **Fold margin** — ~~click + keyboard parity with View fold commands~~ (`docs/folding.md`). Nested chrome / session fold state later.
7. **Project panel** — ~~MVP folder list + refresh + reveal in file manager~~. Nested chrome / N++ projects later.

## P2 — reliability

8. **Panic log** — first new signature in `logs/panic.log` that is product-owned.
9. **Preferences persistence** — ~~`recent_max` honored on `recent.txt` load~~. ~~View show-whitespace / EOL / NPC / indent-guide~~. ~~Settings keys round-trip; no silent drop on restart~~.

## P3 — later (do not pick before P0–P2)

10. Popup autocomplete / call tips, then LSP.
11. Macro record + named macros.
12. Drop-in plugins, UDL, hex, UI i18n, full RTL chrome.

## How to pick

1. Lowest open GitHub issue still beats this list.
2. Then the first non-struck item above that fits one overnight tick.
3. Work on `main`. Verify with the ratchet. Push only after `./scripts/ci-local.sh`.
