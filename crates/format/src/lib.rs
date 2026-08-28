//! Light document format helpers by language id.

/// Format `source` for `language`. Unknown languages get a plain trim.
pub fn format_document(language: &str, source: &str) -> String {
    match language {
        "python" => format_python(source),
        "cpp" | "c" => format_cpp(source),
        "sql" => format_sql(source),
        "markdown" => format_markdown(source),
        _ => format_plain(source),
    }
}

fn format_plain(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    for (i, line) in source.split_inclusive('\n').enumerate() {
        let _ = i;
        let (body, nl) = split_line_ending(line);
        out.push_str(body.trim_end());
        out.push_str(nl);
    }
    out
}

fn format_python(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    for line in source.split_inclusive('\n') {
        let (body, nl) = split_line_ending(line);
        let trimmed = body.trim_end();
        let leading = trimmed.len() - trimmed.trim_start().len();
        let spaces = leading;
        // Normalize leading tabs to 4 spaces; keep existing spaces.
        let indent_cols = if trimmed.starts_with('\t') {
            trimmed.chars().take_while(|c| *c == '\t').count() * 4
                + trimmed
                    .chars()
                    .skip_while(|c| *c == '\t')
                    .take_while(|c| *c == ' ')
                    .count()
        } else {
            spaces
        };
        let content = trimmed.trim_start_matches([' ', '\t']);
        if content.is_empty() {
            out.push_str(nl);
            continue;
        }
        // Round indent down to a multiple of 4 spaces when mixing tabs; otherwise keep spaces.
        let indent = if trimmed.contains('\t') {
            (indent_cols / 4) * 4
        } else {
            indent_cols
        };
        out.push_str(&" ".repeat(indent));
        out.push_str(content);
        out.push_str(nl);
    }
    ensure_final_newline(out)
}

fn format_cpp(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    for line in source.split_inclusive('\n') {
        let (body, nl) = split_line_ending(line);
        let trimmed = body.trim_end();
        // Light touch: trim trailing; keep brace on its own line when "} else" style not needed.
        let normalized = trimmed.replace('\t', "    ");
        out.push_str(&normalized);
        out.push_str(nl);
    }
    ensure_final_newline(out)
}

fn format_sql(source: &str) -> String {
    const KEYWORDS: &[&str] = &[
        "select", "from", "where", "and", "or", "not", "insert", "into", "values", "update",
        "set", "delete", "create", "table", "drop", "alter", "join", "left", "right", "inner",
        "outer", "on", "as", "order", "by", "group", "having", "limit", "offset", "union", "all",
        "distinct", "null", "is", "in", "exists", "between", "like", "case", "when", "then",
        "else", "end", "primary", "key", "foreign", "references", "index", "view", "with",
    ];
    let mut out = String::with_capacity(source.len());
    for line in source.split_inclusive('\n') {
        let (body, nl) = split_line_ending(line);
        let trimmed = body.trim_end();
        out.push_str(&uppercase_sql_keywords(trimmed, KEYWORDS));
        out.push_str(nl);
    }
    ensure_final_newline(out)
}

fn uppercase_sql_keywords(line: &str, keywords: &[&str]) -> String {
    let mut result = String::with_capacity(line.len());
    let mut word = String::new();
    for ch in line.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            word.push(ch);
        } else {
            flush_sql_word(&mut result, &word, keywords);
            word.clear();
            result.push(ch);
        }
    }
    flush_sql_word(&mut result, &word, keywords);
    result
}

fn flush_sql_word(out: &mut String, word: &str, keywords: &[&str]) {
    if word.is_empty() {
        return;
    }
    let lower = word.to_ascii_lowercase();
    if keywords.iter().any(|k| *k == lower) {
        out.push_str(&word.to_ascii_uppercase());
    } else {
        out.push_str(word);
    }
}

fn format_markdown(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut blank_run = 0usize;
    for line in source.split_inclusive('\n') {
        let (body, nl) = split_line_ending(line);
        let trimmed = body.trim_end();
        if trimmed.is_empty() {
            blank_run += 1;
            if blank_run <= 2 {
                out.push_str(nl);
            }
            continue;
        }
        blank_run = 0;
        out.push_str(trimmed);
        out.push_str(nl);
    }
    ensure_final_newline(out)
}

fn split_line_ending(line: &str) -> (&str, &str) {
    if let Some(stripped) = line.strip_suffix("\r\n") {
        (stripped, "\r\n")
    } else if let Some(stripped) = line.strip_suffix('\n') {
        (stripped, "\n")
    } else if let Some(stripped) = line.strip_suffix('\r') {
        (stripped, "\r")
    } else {
        (line, "")
    }
}

fn ensure_final_newline(mut s: String) -> String {
    if s.is_empty() || s.ends_with('\n') {
        return s;
    }
    s.push('\n');
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn python_indent_and_trailing() {
        let src = "def f():\n\treturn 1  \n";
        let out = format_document("python", src);
        assert!(out.contains("    return 1\n"));
        assert!(!out.contains("1  "));
        assert!(out.ends_with('\n'));
    }

    #[test]
    fn cpp_trims_trailing_and_tabs() {
        let src = "int main() {\n\treturn 0;  \n}\n";
        let out = format_document("cpp", src);
        assert!(out.contains("    return 0;\n"));
        assert!(!out.contains("0;  "));
        assert!(out.ends_with('\n'));
    }

    #[test]
    fn sql_uppercases_keywords() {
        let src = "select id from users where name = 'a'  \n";
        let out = format_document("sql", src);
        assert!(out.starts_with("SELECT id FROM users WHERE name = 'a'"));
        assert!(out.ends_with('\n'));
    }

    #[test]
    fn markdown_collapses_blank_lines() {
        let src = "# Title  \n\n\n\npara\n";
        let out = format_document("markdown", src);
        assert!(!out.contains("Title  \n"));
        assert!(!out.contains("\n\n\n\n"));
        assert!(out.contains("\n\n"));
        assert!(out.ends_with('\n'));
    }

    #[test]
    fn plain_trims_trailing() {
        let src = "hello  \nworld  ";
        let out = format_document("plain", src);
        assert_eq!(out, "hello\nworld");
    }
}
