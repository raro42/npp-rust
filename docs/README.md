# Documentation

Project docs live here. Root keeps only `README.md`, `LICENSE`, and `CONTRIBUTING.md`.

| Doc | Topic |
|-----|--------|
| [scope.md](scope.md) | Feature boundary |
| [testing.md](testing.md) | How tests run |
| [changelog.md](changelog.md) | Release history |
| [release.md](release.md) | Version, CI, daily release |
| [../agents/AGENTS.md](../agents/AGENTS.md) | Short agent entry |
| [agent-loop.md](agent-loop.md) | GitHub issue agent loop |
| [security-public-repo.md](security-public-repo.md) | No private data on public GitHub |
| [tail-log.md](tail-log.md) | Log tail / Monitoring |
| [issue-5-log-tail.md](issue-5-log-tail.md) | Issue #5 status and failure mode |
| [help-debug.md](help-debug.md) | Help → Debug Info and Open Logs |
| [close-unsaved.md](close-unsaved.md) | Prompt before closing dirty tabs |
| [unattended-20h.md](unattended-20h.md) | How to run the agent loop for hours |
| [update-check.md](update-check.md) | How Notepad++ updates; our plan |
| [menu-todo.md](menu-todo.md) | Menu stub inventory |
| [whats-missing.md](whats-missing.md) | Coming Soon vs honest partials |
| [status-bar-version.md](status-bar-version.md) | Commit hash link in the status bar |
| [execute-dont-ask.md](execute-dont-ask.md) | Do not ask permission to proceed |
| [compare.md](compare.md) | Built-in 2-way file compare |
| [tab-drag.md](tab-drag.md) | Mouse drag-reorder on the tab bar |
| [bump-version.md](bump-version.md) | Bump semver when shipping features |
| [overnight-gaps.md](overnight-gaps.md) | Overnight gap loop (issue #6) |
| [dual-view.md](dual-view.md) | Writable dual view + pane focus |
| [parallel-gap-sweep.md](parallel-gap-sweep.md) | Parallel agents on remaining gaps |
| [pin-tab-file.md](pin-tab-file.md) | Pin via `IDM_PINTAB` + Close All but Pinned |
| [project-taste.md](project-taste.md) | Taste / quality bar |
| [notepad-plus-plus-github.md](notepad-plus-plus-github.md) | Upstream GitHub notes |
| [cursor-skills.md](cursor-skills.md) | Where to put Rust vs project skills |

## Local Notepad++ reference

Clone upstream locally if needed (not in git):

```bash
mkdir -p reference
git clone --depth 1 https://github.com/notepad-plus-plus/notepad-plus-plus.git reference/notepad-plus-plus
```

`reference/` is gitignored.

| [agent-parallel.md](agent-parallel.md) | File ownership for parallel agents |
