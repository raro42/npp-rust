# How Notepad++ checks for updates

Date: 2026-08-28  
Sources: [WinGUp how](https://wingup.org/how/), [WinGUp usage](https://wingup.org/usage/), Notepad++ community / GUP notes

## Notepad++ approach (do not reinvent)

Notepad++ does **not** poll GitHub Releases from the editor for updates.

It uses a separate updater: **WinGUp / GUP** (`gup.exe`).

### Flow

1. Notepad++ launches GUP with the **current version** (and arch param, e.g. `x64` / `arm64`).
2. GUP reads local `gup.xml` (version + **info URL**).
3. GUP calls the server, e.g.  
   `https://notepad-plus-plus.org/update/getDownloadUrl.php?version=…&param=x64`
4. Server returns **XML**: need-update flag, latest version, download **Location**.
5. Optional but recommended: **XML signature** check, then download installer.
6. Optional: verify **code signature** on the downloaded EXE/MSI, then run it.

So: **version compare + download URL come from N++’s own update endpoint**, not from inventing a custom protocol. The wheel is WinGUp + a small server response.

### What npp-rust has today

**? → Releases** only opens the GitHub Releases page in a browser. No version compare, no download.

### Sensible mirror for npp-rust (public GitHub project)

We do not need WinGUp or a PHP endpoint.

Closest equivalent wheel:

1. Call GitHub Releases API:  
   `GET https://api.github.com/repos/raro42/npp-rust/releases/latest`
2. Compare `tag_name` (e.g. `v0.1.2`) to `CARGO_PKG_VERSION`.
3. If newer: show dialog + link/open the release page (or asset URL for OS).
4. Do **not** auto-run an unsigned binary without user consent (N++ uses signed installers + GUP).

Optional later: ship a small updater helper; for now, “check + open Releases” matches user intent safely.

### Menu mapping

| Notepad++ | npp-rust today | Target |
|-----------|----------------|--------|
| Update Notepad++ / GUP | Opens Releases URL | Check API → prompt if newer |
| Set Updater Proxy | Stub / removed | Skip unless needed |
