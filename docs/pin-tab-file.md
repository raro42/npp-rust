# Pin tab (File command path)

Date: 2026-08-29

## Menu JSON (read-only)

`crates/app/data/npp_menu.json` has no File/View “Pin” item.

It has only:

- `IDM_FILE_CLOSEALL_BUT_PINNED` — Close All but Pinned Documents

## Upstream IDM

Notepad++ defines `IDM_PINTAB` (tab context menu; `menuCmdID.h`, under View range).

That ID is not in the export menu JSON.

## npp-rs path (`commands/file.rs`)

| Command | Behaviour |
|---------|-----------|
| `IDM_PINTAB` (any `*PIN*` that is not Close All but Pinned) | `toggle_active_pin` on the active tab |
| `IDM_FILE_CLOSEALL_BUT_PINNED` | Close unpinned tabs; clear status if none pinned |

Status examples:

- `Pinned tab: …` / `Unpinned tab: …`
- `Close all but pinned: no tabs are pinned — pin first (IDM_PINTAB)`
- `Close all but pinned: kept N pinned, closed M`

Tab chrome pin UI stays with the UI agent (`ui.rs`).
