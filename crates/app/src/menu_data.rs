//! Notepad++ main-menu tree loaded from the reference RC export.

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum MenuNode {
    #[serde(rename = "popup")]
    Popup {
        label: String,
        #[serde(default)]
        children: Vec<MenuNode>,
    },
    #[serde(rename = "item")]
    Item { label: String, cmd: String },
    #[serde(rename = "separator")]
    Separator,
}

pub fn load_npp_menu() -> Vec<MenuNode> {
    let mut menu: Vec<MenuNode> = serde_json::from_str(include_str!("../data/npp_menu.json"))
        .expect("npp_menu.json must parse");
    // Upstream RC exports two top-level Language menus (flat list + A–Z groups).
    // Keep only the second (grouped) entry.
    let lang_idxs: Vec<usize> = menu
        .iter()
        .enumerate()
        .filter_map(|(i, n)| match n {
            MenuNode::Popup { label, .. } if label == "Language" => Some(i),
            _ => None,
        })
        .collect();
    if lang_idxs.len() >= 2 {
        menu.remove(lang_idxs[0]);
    }
    menu
}

#[allow(dead_code)]
pub fn count_items(nodes: &[MenuNode]) -> usize {
    nodes.iter().map(count_node).sum()
}

fn count_node(n: &MenuNode) -> usize {
    match n {
        MenuNode::Item { .. } => 1,
        MenuNode::Separator => 0,
        MenuNode::Popup { children, .. } => count_items(children),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_matches_notepad_plus_plus_reference_export() {
        let menu = load_npp_menu();
        let tops: Vec<_> = menu
            .iter()
            .filter_map(|n| match n {
                MenuNode::Popup { label, .. } => Some(label.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(
            tops,
            [
                "File", "Edit", "Search", "View", "Encoding", "Language", "Settings", "Tools",
                "Macro", "Run", "Plugins", "Window", "?",
            ]
        );
        // Raw export has 574 items across two Language menus; we drop the flat one.
        assert!(count_items(&menu) < 574);
        assert_eq!(tops.iter().filter(|&&t| t == "Language").count(), 1);
    }
}
