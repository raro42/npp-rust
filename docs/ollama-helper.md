# Local Ollama helper (vs a Cursor / VS Code plugin)

Date: 2026-10-09  
Issue: https://github.com/raro42/npp-rust/issues/18  
Follow-up: https://github.com/raro42/npp-rust/issues/21

## Product take

**npp-rs should not become a VS Code / Cursor extension.** The editor is its own Rust / egui app. A “Cursor plugin” for VS Code would help VS Code users, not npp-rs.

What *does* fit:

1. **Local LLM help inside npp-rs** via [Ollama](https://ollama.com) on loopback.
2. Keep AI optional, offline-first, and free of cloud API keys in the core app.
3. Stay honest: this is a helper (explain / review snippet), not a full agent with tools, multi-file edits, or repo indexing.

Cursor’s value is agent + codebase context + apply-patch loops. Porting that whole stack is a large product. A thin Ollama bridge is a useful first step and matches npp-rs privacy goals.

## What shipped (v0.3.165 + v0.3.242 + v0.3.407)

| Piece | Behavior |
|-------|----------|
| **Plugins → Ollama Chat** | Chat panel: transcript, exact prompt text, Thinking… while the worker runs, follow-ups. |
| **Plugins → Ask Ollama** | Opens the chat panel, sends selection (or whole buffer) via `/api/chat`, shows the prompt before the reply. |
| **Editor right-click → Ask Ollama** | Same as the Plugins item. |
| **Plugins → Ollama Status** | Pings `/api/tags` and lists installed models in a tab. |
| **Open reply in tab** | Optional: copy the last assistant reply into a read-only markdown tab. |
| **Background HTTP** | Ask / Status / chat Send run on a worker thread (one request at a time). |
| **Preferences** | `ollama_host`, `ollama_model` (also `npp-rs/settings.json`). |
| **Host rule** | **Loopback only** (`127.0.0.1`, `localhost`, `::1`). Remote URLs are rejected. |

Defaults: host `http://127.0.0.1:11434`, model `llama3.2`. Prompt / transcript soft-capped (~12k chars).

## How to try it

1. Install and run Ollama on the same machine.
2. `ollama pull llama3.2` (or set Preferences → Model to a model you have).
3. Open a file, select a snippet (optional).
4. **Plugins → Ask Ollama**, or right-click the editor → **Ask Ollama**.
5. Watch the **Ollama Chat** panel: your prompt is listed under **You**, then **Thinking…**, then **Ollama**.

If Ollama is down, the chat shows an error line and the status bar shows a connect error. Use **Ollama Status** to confirm models. While a call is in flight you can keep editing.

## Why not a Cursor plugin?

| Approach | Pros | Cons for npp-rs |
|----------|------|-----------------|
| VS Code / Cursor extension | Reuse Cursor UX | Wrong host app; splits the product |
| Cloud LLM API in-core | Easy setup | Keys, privacy, network dependency |
| **Local Ollama in-core** | Offline, no keys, fits FOSS | Needs a local model |

## Non-goals (for now)

- Tool-using agents, apply-patch, or multi-tab refactors
- Sending buffers to non-loopback hosts from the core app
- Drop-in Notepad++ plugin ABI for AI DLLs
- Shipping or bundling model weights
- Token streaming into the chat bubble (Thinking… covers “working”)

## Later (optional)

- Cancellable in-flight requests
- Streamed assistant tokens
- Insert-at-caret / replace-selection apply modes
- Remappable Ask Ollama / Ollama Chat shortcut
- Optional OpenAI-compatible local servers still bound to loopback
