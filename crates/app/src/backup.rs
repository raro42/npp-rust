//! Backup-on-save: copy the on-disk file into `npp-rs/backup/` before overwrite.

use crate::recent::config_dir;
use std::path::{Component, Path, PathBuf};

/// Repo-relative label for status / Preferences (never a home absolute path).
pub const BACKUP_REL: &str = "npp-rs/backup";

/// Map an absolute or relative source path into a safe relative tree under the backup root.
///
/// Strips drive / root markers so Windows `C:\a\b.txt` and Unix `/a/b.txt` become
/// `C/a/b.txt` and `a/b.txt` respectively.
pub fn mirror_rel_path(source: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in source.components() {
        match c {
            Component::Prefix(prefix) => {
                let raw = prefix.as_os_str().to_string_lossy();
                let cleaned: String = raw.chars().filter(|ch| ch.is_alphanumeric()).collect();
                if !cleaned.is_empty() {
                    out.push(cleaned);
                }
            }
            Component::RootDir | Component::CurDir | Component::ParentDir => {}
            Component::Normal(s) => out.push(s),
        }
    }
    if out.as_os_str().is_empty() {
        out.push("unnamed");
    }
    out
}

/// Full backup destination for `source`, or `None` when the config dir is unavailable.
pub fn backup_dest_for(source: &Path) -> Option<PathBuf> {
    let root = config_dir()?.join("npp-rs").join("backup");
    Some(root.join(mirror_rel_path(source)))
}

/// Copy an existing on-disk file to the config backup tree.
///
/// Returns `Ok(true)` when a copy ran, `Ok(false)` when the source was missing.
pub fn backup_existing_file(source: &Path) -> Result<bool, String> {
    if !source.is_file() {
        return Ok(false);
    }
    let dest = backup_dest_for(source).ok_or_else(|| "backup: no config dir".to_string())?;
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("backup mkdir: {e}"))?;
    }
    std::fs::copy(source, &dest).map_err(|e| format!("backup copy: {e}"))?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn mirror_strips_unix_root() {
        let p = mirror_rel_path(Path::new("/tmp/proj/a.txt"));
        assert_eq!(p, PathBuf::from("tmp/proj/a.txt"));
    }

    #[test]
    fn mirror_relative_unchanged() {
        let p = mirror_rel_path(Path::new("notes/todo.md"));
        assert_eq!(p, PathBuf::from("notes/todo.md"));
    }

    #[test]
    fn mirror_empty_becomes_unnamed() {
        let p = mirror_rel_path(Path::new("/"));
        assert_eq!(p, PathBuf::from("unnamed"));
    }
}
