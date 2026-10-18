//! Session clipboard ring for Edit → Clipboard History.
//!
//! Newest entry is index 0. The chord walks older entries and wraps.

/// How many distinct copies the session keeps.
pub const CAP: usize = 10;

/// Push `text` as the newest entry. Empty text is ignored.
/// A repeat of the newest entry resets the walk cursor and does not duplicate.
pub fn push(entries: &mut Vec<String>, cursor: &mut usize, text: &str) {
    if text.is_empty() {
        return;
    }
    if entries.first().is_some_and(|head| head == text) {
        *cursor = 0;
        return;
    }
    entries.insert(0, text.to_string());
    if entries.len() > CAP {
        entries.truncate(CAP);
    }
    *cursor = 0;
}

/// Entry at `cursor` (0 = newest), then advance for the next pick.
/// Returns `(index, text)`.
pub fn pick(entries: &[String], cursor: &mut usize) -> Option<(usize, String)> {
    if entries.is_empty() {
        return None;
    }
    let i = *cursor % entries.len();
    let text = entries[i].clone();
    *cursor = (i + 1) % entries.len();
    Some((i, text))
}

pub fn status_line(index: usize, total: usize, text: &str) -> String {
    let preview: String = text.chars().take(40).collect();
    let more = if text.chars().count() > 40 { "…" } else { "" };
    format!(
        "Clipboard history ({}/{}): \"{preview}{more}\" ({} chars)",
        index + 1,
        total,
        text.chars().count()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_dedupes_newest_and_caps() {
        let mut entries = Vec::new();
        let mut cursor = 3;
        push(&mut entries, &mut cursor, "");
        assert!(entries.is_empty());
        assert_eq!(cursor, 3);

        push(&mut entries, &mut cursor, "a");
        push(&mut entries, &mut cursor, "a");
        assert_eq!(entries, vec!["a".to_string()]);
        assert_eq!(cursor, 0);

        for n in 0..12 {
            push(&mut entries, &mut cursor, &n.to_string());
        }
        assert_eq!(entries.len(), CAP);
        assert_eq!(entries[0], "11");
        assert_eq!(entries[CAP - 1], "2");
    }

    #[test]
    fn pick_walks_older_then_wraps() {
        let entries = vec!["c".to_string(), "b".to_string(), "a".to_string()];
        let mut cursor = 0;
        assert_eq!(pick(&entries, &mut cursor), Some((0, "c".into())));
        assert_eq!(pick(&entries, &mut cursor), Some((1, "b".into())));
        assert_eq!(pick(&entries, &mut cursor), Some((2, "a".into())));
        assert_eq!(pick(&entries, &mut cursor), Some((0, "c".into())));
        assert!(pick(&[], &mut cursor).is_none());
    }

    #[test]
    fn status_names_slot() {
        let line = status_line(1, 3, "hello");
        assert_eq!(line, "Clipboard history (2/3): \"hello\" (5 chars)");
    }
}
