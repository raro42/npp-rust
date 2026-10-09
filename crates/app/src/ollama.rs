//! Local Ollama helper (loopback HTTP only).
//!
//! Talks to a user-run Ollama on `127.0.0.1` / `localhost` / `::1`.
//! No cloud API keys. Remote hosts are rejected on purpose.

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

/// Default Ollama base URL (loopback).
pub const DEFAULT_OLLAMA_HOST: &str = "http://127.0.0.1:11434";
/// Default model name when Preferences leave the field empty-ish.
pub const DEFAULT_OLLAMA_MODEL: &str = "llama3.2";
/// Soft cap on prompt body chars sent to the model.
pub const MAX_PROMPT_CHARS: usize = 12_000;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
const READ_TIMEOUT: Duration = Duration::from_secs(90);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OllamaEndpoint {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OllamaError {
    BadHost(String),
    Connect(String),
    Http(String),
    Parse(String),
    EmptyModel,
    EmptyPrompt,
}

impl std::fmt::Display for OllamaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadHost(m) => write!(f, "{m}"),
            Self::Connect(m) => write!(f, "Ollama connect: {m}"),
            Self::Http(m) => write!(f, "Ollama HTTP: {m}"),
            Self::Parse(m) => write!(f, "Ollama parse: {m}"),
            Self::EmptyModel => write!(f, "Set ollama_model in Preferences (e.g. llama3.2)"),
            Self::EmptyPrompt => write!(f, "Nothing to send (empty selection / buffer)"),
        }
    }
}

/// Parse and validate a loopback-only base URL (`http://127.0.0.1:11434`).
pub fn parse_loopback_host(raw: &str) -> Result<OllamaEndpoint, OllamaError> {
    let s = raw.trim();
    if s.is_empty() {
        return parse_loopback_host(DEFAULT_OLLAMA_HOST);
    }
    let without_scheme = s
        .strip_prefix("http://")
        .or_else(|| s.strip_prefix("https://"))
        .unwrap_or(s);
    if s.starts_with("https://") {
        return Err(OllamaError::BadHost(
            "Ollama helper uses plain HTTP on loopback only (no https://)".into(),
        ));
    }
    let without_path = without_scheme.split('/').next().unwrap_or(without_scheme);
    let (host, port) = parse_host_port(without_path)?;
    let host_l = host.to_ascii_lowercase();
    if !matches!(host_l.as_str(), "127.0.0.1" | "localhost" | "::1") {
        return Err(OllamaError::BadHost(
            "Ollama host must be loopback (127.0.0.1, localhost, or ::1)".into(),
        ));
    }
    Ok(OllamaEndpoint { host: host_l, port })
}

fn parse_host_port(authority: &str) -> Result<(String, u16), OllamaError> {
    if let Some(rest) = authority.strip_prefix('[') {
        // [::<ipv6>]:port or [::1]
        let (host_inner, after) = rest.split_once(']').ok_or_else(|| {
            OllamaError::BadHost(format!("invalid IPv6 Ollama host `{authority}`"))
        })?;
        let port = if let Some(p) = after.strip_prefix(':') {
            if p.is_empty() {
                11434
            } else {
                p.parse().map_err(|_| {
                    OllamaError::BadHost(format!("invalid Ollama port in `{authority}`"))
                })?
            }
        } else if after.is_empty() {
            11434
        } else {
            return Err(OllamaError::BadHost(format!(
                "invalid Ollama host `{authority}`"
            )));
        };
        return Ok((host_inner.to_string(), port));
    }
    if let Some((h, p)) = authority.rsplit_once(':') {
        // Avoid treating bare IPv6 without brackets as host:port.
        if h.contains(':') {
            return Err(OllamaError::BadHost(
                "IPv6 Ollama host must use brackets, e.g. http://[::1]:11434".into(),
            ));
        }
        if p.chars().all(|c| c.is_ascii_digit()) {
            let port: u16 = p.parse().map_err(|_| {
                OllamaError::BadHost(format!("invalid Ollama port in `{authority}`"))
            })?;
            return Ok((h.to_string(), port));
        }
    }
    Ok((authority.to_string(), 11434))
}

pub fn effective_model(raw: &str) -> Result<String, OllamaError> {
    let m = raw.trim();
    if m.is_empty() {
        return Err(OllamaError::EmptyModel);
    }
    Ok(m.to_string())
}

/// Truncate on a char boundary for the prompt body.
pub fn truncate_prompt(text: &str, max_chars: usize) -> (String, bool) {
    let count = text.chars().count();
    if count <= max_chars {
        return (text.to_string(), false);
    }
    let truncated: String = text.chars().take(max_chars).collect();
    (truncated, true)
}

#[derive(Debug, Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    system: &'a str,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TagsResponse {
    models: Option<Vec<TagModel>>,
}

#[derive(Debug, Deserialize)]
struct TagModel {
    name: Option<String>,
}

/// Build the user-facing prompt for Ask Ollama.
pub fn build_ask_prompt(lang: &str, body: &str, truncated: bool) -> String {
    let mut p = String::with_capacity(body.len() + 160);
    p.push_str("You are a concise local coding assistant inside a text editor.\n");
    p.push_str("Explain, review, or suggest improvements for the snippet below.\n");
    p.push_str("Prefer short answers. Use markdown when helpful.\n");
    if !lang.is_empty() && lang != "plain" {
        p.push_str("Language hint: ");
        p.push_str(lang);
        p.push('\n');
    }
    if truncated {
        p.push_str("(Note: input was truncated for size.)\n");
    }
    p.push_str("\n```\n");
    p.push_str(body);
    p.push_str("\n```\n");
    p
}

fn resolve_addr(ep: &OllamaEndpoint) -> Result<SocketAddr, OllamaError> {
    let target = format!("{}:{}", ep.host, ep.port);
    target
        .to_socket_addrs()
        .map_err(|e| OllamaError::Connect(e.to_string()))?
        .next()
        .ok_or_else(|| OllamaError::Connect(format!("no address for {target}")))
}

fn http_post_json(ep: &OllamaEndpoint, path: &str, body: &str) -> Result<String, OllamaError> {
    let addr = resolve_addr(ep)?;
    let mut stream = TcpStream::connect_timeout(&addr, CONNECT_TIMEOUT)
        .map_err(|e| OllamaError::Connect(e.to_string()))?;
    stream
        .set_read_timeout(Some(READ_TIMEOUT))
        .map_err(|e| OllamaError::Connect(e.to_string()))?;
    stream
        .set_write_timeout(Some(CONNECT_TIMEOUT))
        .map_err(|e| OllamaError::Connect(e.to_string()))?;

    let host_header = format!("{}:{}", ep.host, ep.port);
    let req = format!(
        "POST {path} HTTP/1.1\r\nHost: {host_header}\r\nContent-Type: application/json\r\nContent-Length: {len}\r\nConnection: close\r\n\r\n{body}",
        len = body.len(),
    );
    stream
        .write_all(req.as_bytes())
        .map_err(|e| OllamaError::Http(e.to_string()))?;

    let mut buf = Vec::new();
    stream
        .read_to_end(&mut buf)
        .map_err(|e| OllamaError::Http(e.to_string()))?;
    let raw = String::from_utf8_lossy(&buf);
    split_http_body(&raw)
}

fn http_get(ep: &OllamaEndpoint, path: &str) -> Result<String, OllamaError> {
    let addr = resolve_addr(ep)?;
    let mut stream = TcpStream::connect_timeout(&addr, CONNECT_TIMEOUT)
        .map_err(|e| OllamaError::Connect(e.to_string()))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .map_err(|e| OllamaError::Connect(e.to_string()))?;
    stream
        .set_write_timeout(Some(CONNECT_TIMEOUT))
        .map_err(|e| OllamaError::Connect(e.to_string()))?;

    let host_header = format!("{}:{}", ep.host, ep.port);
    let req = format!("GET {path} HTTP/1.1\r\nHost: {host_header}\r\nConnection: close\r\n\r\n");
    stream
        .write_all(req.as_bytes())
        .map_err(|e| OllamaError::Http(e.to_string()))?;

    let mut buf = Vec::new();
    stream
        .read_to_end(&mut buf)
        .map_err(|e| OllamaError::Http(e.to_string()))?;
    let raw = String::from_utf8_lossy(&buf);
    split_http_body(&raw)
}

fn split_http_body(raw: &str) -> Result<String, OllamaError> {
    let (head, body) = raw
        .split_once("\r\n\r\n")
        .or_else(|| raw.split_once("\n\n"))
        .ok_or_else(|| OllamaError::Http("missing HTTP header/body separator".into()))?;
    let status_line = head.lines().next().unwrap_or("");
    if !status_line.contains(" 200") && !status_line.contains(" 201") {
        let snippet: String = body.chars().take(200).collect();
        return Err(OllamaError::Http(format!("{status_line} — {snippet}")));
    }
    // Drop Trailer / chunked framing if Ollama ever sends it; normal JSON is fine.
    if head
        .to_ascii_lowercase()
        .contains("transfer-encoding: chunked")
    {
        return decode_chunked(body);
    }
    Ok(body.trim().to_string())
}

fn decode_chunked(body: &str) -> Result<String, OllamaError> {
    let mut out = String::new();
    let mut rest = body;
    while let Some((size_line, after)) = rest.split_once('\n') {
        let size_hex = size_line.trim().trim_end_matches('\r');
        let size = usize::from_str_radix(size_hex, 16)
            .map_err(|_| OllamaError::Http(format!("bad chunk size `{size_hex}`")))?;
        if size == 0 {
            break;
        }
        let bytes: Vec<u8> = after.as_bytes().iter().copied().take(size).collect();
        out.push_str(&String::from_utf8_lossy(&bytes));
        let skip = size
            + if after.as_bytes().get(size) == Some(&b'\r') {
                2
            } else {
                1
            };
        if after.len() < skip {
            break;
        }
        rest = &after[skip..];
        if let Some(stripped) = rest.strip_prefix('\n') {
            rest = stripped;
        }
    }
    Ok(out)
}

/// POST `/api/generate` (non-streaming) and return the model text.
pub fn generate(host_raw: &str, model_raw: &str, prompt: &str) -> Result<String, OllamaError> {
    if prompt.trim().is_empty() {
        return Err(OllamaError::EmptyPrompt);
    }
    let ep = parse_loopback_host(host_raw)?;
    let model = effective_model(model_raw)?;
    let req = GenerateRequest {
        model: &model,
        prompt,
        stream: false,
        system: "You help inside a local text editor. Stay concise. Do not invent file paths or secrets.",
    };
    let body = serde_json::to_string(&req).map_err(|e| OllamaError::Parse(e.to_string()))?;
    let resp_body = http_post_json(&ep, "/api/generate", &body)?;
    let parsed: GenerateResponse = serde_json::from_str(&resp_body)
        .map_err(|e| OllamaError::Parse(format!("{e}; body={}", trunc_err(&resp_body))))?;
    if let Some(err) = parsed.error.filter(|e| !e.is_empty()) {
        return Err(OllamaError::Http(err));
    }
    let text = parsed.response.unwrap_or_default();
    if text.trim().is_empty() {
        return Err(OllamaError::Http("empty model response".into()));
    }
    Ok(text)
}

/// GET `/api/tags` — list installed model names (empty vec if none).
pub fn list_models(host_raw: &str) -> Result<Vec<String>, OllamaError> {
    let ep = parse_loopback_host(host_raw)?;
    let resp_body = http_get(&ep, "/api/tags")?;
    let parsed: TagsResponse = serde_json::from_str(&resp_body)
        .map_err(|e| OllamaError::Parse(format!("{e}; body={}", trunc_err(&resp_body))))?;
    let mut names: Vec<String> = parsed
        .models
        .unwrap_or_default()
        .into_iter()
        .filter_map(|m| m.name.filter(|n| !n.is_empty()))
        .collect();
    names.sort();
    Ok(names)
}

fn trunc_err(s: &str) -> String {
    s.chars().take(120).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loopback_hosts_ok() {
        let a = parse_loopback_host("http://127.0.0.1:11434").unwrap();
        assert_eq!(a.host, "127.0.0.1");
        assert_eq!(a.port, 11434);
        let b = parse_loopback_host("localhost").unwrap();
        assert_eq!(b.host, "localhost");
        assert_eq!(b.port, 11434);
        let c = parse_loopback_host("http://[::1]:11434").unwrap();
        assert_eq!(c.host, "::1");
    }

    #[test]
    fn remote_host_rejected() {
        let err = parse_loopback_host("http://example.com:11434").unwrap_err();
        assert!(matches!(err, OllamaError::BadHost(_)));
        let err2 = parse_loopback_host("https://127.0.0.1:11434").unwrap_err();
        assert!(matches!(err2, OllamaError::BadHost(_)));
    }

    #[test]
    fn truncate_marks_overflow() {
        let (t, cut) = truncate_prompt("abcdef", 3);
        assert_eq!(t, "abc");
        assert!(cut);
        let (t2, cut2) = truncate_prompt("hi", 10);
        assert_eq!(t2, "hi");
        assert!(!cut2);
    }

    #[test]
    fn ask_prompt_includes_lang_and_body() {
        let p = build_ask_prompt("rust", "fn main() {}", false);
        assert!(p.contains("Language hint: rust"));
        assert!(p.contains("fn main() {}"));
        assert!(!p.contains("truncated"));
    }

    #[test]
    fn empty_model_errors() {
        assert!(matches!(
            effective_model("  "),
            Err(OllamaError::EmptyModel)
        ));
        assert_eq!(effective_model("llama3.2").unwrap(), "llama3.2");
    }

    #[test]
    fn split_http_ok() {
        let raw = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"response\":\"hi\"}";
        let body = split_http_body(raw).unwrap();
        assert_eq!(body, "{\"response\":\"hi\"}");
    }
}
