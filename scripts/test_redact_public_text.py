#!/usr/bin/env python3
"""Tests for scripts/redact_public_text.py — run: python3 scripts/test_redact_public_text.py"""

from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from redact_public_text import find_issues, redact, summarize_for_task


def test_clean_passes() -> None:
    text = "Fix scroll on Mac. See crates/app/src/ui.rs and issue #12."
    assert find_issues(text) == []


def test_blocks_home_path() -> None:
    text = "Log is at /Users/someone/ProtonDrive/secret.txt"
    names = {f.rule for f in find_issues(text)}
    assert "home_users" in names
    assert "proton_drive" in names


def test_blocks_github_token() -> None:
    text = "token ghp_abcdefghijklmnopqrstuvwxyz0123456789"
    assert any(f.rule == "github_pat" for f in find_issues(text))


def test_blocks_private_key() -> None:
    text = "-----BEGIN RSA PRIVATE KEY-----\nMIIE...\n"
    assert any(f.rule == "private_key" for f in find_issues(text))


def test_blocks_email() -> None:
    text = "Contact me at alice.private@example.com please"
    assert any(f.rule == "email" for f in find_issues(text))


def test_redact_replaces() -> None:
    text = "see /Users/bob/secret and ghp_abcdefghijklmnopqrstuvwxyz0123456789"
    out = redact(text)
    assert "/Users/bob" not in out
    assert "ghp_" not in out
    assert "REDACTED" in out


def test_summarize_limits() -> None:
    body = "Fix the bug. " + ("x" * 500)
    s = summarize_for_task(body, limit=100)
    assert len(s) <= 100


def main() -> int:
    tests = [
        test_clean_passes,
        test_blocks_home_path,
        test_blocks_github_token,
        test_blocks_private_key,
        test_blocks_email,
        test_redact_replaces,
        test_summarize_limits,
    ]
    failed = 0
    for t in tests:
        try:
            t()
            print(f"ok  {t.__name__}")
        except AssertionError as e:
            failed += 1
            print(f"FAIL {t.__name__}: {e}")
    print(f"{len(tests) - failed}/{len(tests)} passed")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
