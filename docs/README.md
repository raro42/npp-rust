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
| [help-debug.md](help-debug.md) | Help → Debug Info and Open Logs |
| [close-unsaved.md](close-unsaved.md) | Prompt before closing dirty tabs |
| [unattended-20h.md](unattended-20h.md) | How to run the agent loop for hours |
| [update-check.md](update-check.md) | How Notepad++ updates; our plan |
| [menu-todo.md](menu-todo.md) | Menu stub inventory |
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
