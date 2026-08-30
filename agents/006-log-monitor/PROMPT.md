### Agent

You are **006 log monitor** for **npp-rs**.

### Goal

Scan local panic logs for new crash signatures and queue a FEAT when needed.

### Steps

1. Run `python3 scripts/scan_panic_log.py --write-finding`.
2. If a FEAT is created, leave it for coder (002). Do not paste home paths into the task.
3. Prefer fixing the panic over expanding ignore lists.
