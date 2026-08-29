# Unattended 20-hour run

Date: 2026-08-29

## What was wrong (why you babysat)

1. Loop process dies if the shell session ends — must use `nohup` + keep Mac awake.
2. Coder was told **not to push** — work stayed local or incomplete.
3. Agents marked menu IDs teal (`is_implemented`) without real UI — fake “stubs 0”.
4. Task queue blocked: open `FEAT-` / stale `WIP-` files steal cycles from real menu work.
5. One serial `cursor-agent` per cycle — not parallel domain workers.

## What you do once (human)

1. **Leave the Mac on** (lid open or power adapter + prevent sleep).
2. **Auth stays valid**
   - `gh auth status` works
   - `cursor-agent` on `PATH` (`~/.local/bin`)
   - Git can `push` to `origin` (`dev`)
3. **Clear the task queue** so the loop works the right goal:

```bash
cd /Users/raro42/projects/notepad-plus-plus
mkdir -p agents/tasks/done
# Park anything that is not “fill real placeholders”:
mv agents/tasks/FEAT-*.md agents/tasks/WIP-*.md agents/tasks/done/ 2>/dev/null || true
```

4. **Open (or keep) one GitHub issue** whose title is the goal, e.g.  
   “Implement real behaviour for placeholder menus in docs/menu-todo.md”  
   Then run pickup once, or drop a task file:

```bash
cat > agents/tasks/FEAT-1-menu-placeholders.md <<'EOF'
# Real menu placeholders

## GitHub Issues
- **Issue:** https://github.com/raro42/npp-rust/issues/1

## Problem / goal
Implement real behaviour for items listed under Placeholder in docs/menu-todo.md.
Do not mark is_implemented teal for status-only fakes.
Commit and push to origin/dev each batch.
Prefer Settings/Preferences depth, Search styles that already have partial state, then View fold.

## Privacy
- Do not paste private paths into GitHub.
EOF
```

5. **Start the loop and walk away:**

```bash
pkill -f 'npp-cursor-loop.sh loop' 2>/dev/null || true
export PATH="$HOME/.local/bin:$PATH"
export AGENT_USE_CURSOR=1
export AGENT_LOOP_SLEEP_MINUTES=5
cd /Users/raro42/projects/notepad-plus-plus
nohup ./agents/npp-cursor-loop.sh loop >>/tmp/npp-agent-loop.log 2>&1 &
echo "pid=$!"
tail -f /tmp/npp-agent-loop.log
```

6. **Optional — keep awake (macOS):**

```bash
caffeinate -dimsu -t 72000 &   # 20 hours
```

## What you do not need to do

- Sit in Cursor chat for each menu item
- Ask “should I commit?”
- Restart after every cycle (only if the log stops)

## Check progress later

```bash
pgrep -fl npp-cursor-loop
tail -50 /tmp/npp-agent-loop.log
git log origin/dev -5 --oneline
head -40 docs/menu-todo.md
gh issue list --repo raro42/npp-rust
```

## Expectation

In 20 hours you get many small commits on `dev`, fewer true placeholders in `docs/menu-todo.md`, and `cargo check` still green. You will **not** get a full Notepad++ clone. Hard items (true dual view, document map, full Preferences) may stay partial on purpose.
