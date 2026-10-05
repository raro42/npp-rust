#!/usr/bin/env python3
"""Tests for scripts/handoff_gate.py — run: python3 scripts/test_handoff_gate.py"""

from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from handoff_gate import (  # noqa: E402
    awaiting_handoff,
    issue_number_from_name,
    locally_pending,
    needs_github_query,
    parse_text,
    stamp_github_closed,
)


def test_parse_complete_variants() -> None:
    assert parse_text("Handoff: complete\n")["complete"]
    assert parse_text("- Handoff: complete\n")["complete"]
    assert not parse_text("Handoff: deferred\n")["complete"]
    assert parse_text("- Handoff: deferred\n")["deferred"]


def test_issue_number() -> None:
    assert issue_number_from_name(
        "DONE-14-20260831-queued-lexer-aware-folding-fold-margin.md"
    ) == 14
    assert issue_number_from_name("DONE-ci-20260830-0626-fix-github-ci.md") is None


def test_complete_without_github_closed_is_pending() -> None:
    text = "Handoff: complete\n"
    assert locally_pending(text, has_issue=True)
    assert needs_github_query(text, has_issue=True)
    assert awaiting_handoff(text, has_issue=True, issue_open=True)
    assert not awaiting_handoff(text, has_issue=True, issue_open=False)


def test_github_closed_is_done() -> None:
    text = "Handoff: complete\nGitHub: closed\n"
    assert not locally_pending(text, has_issue=True)
    assert not needs_github_query(text, has_issue=True)
    assert not awaiting_handoff(text, has_issue=True, issue_open=True)


def test_deferred_never_pending() -> None:
    text = "Handoff: deferred\n"
    assert not locally_pending(text, has_issue=True)
    assert not awaiting_handoff(text, has_issue=True, issue_open=True)


def test_no_issue_until_complete() -> None:
    assert locally_pending("notes\n", has_issue=False)
    assert not locally_pending("Handoff: complete\n", has_issue=False)
    assert not needs_github_query("notes\n", has_issue=False)


def test_stamp(tmp_path: Path | None = None) -> None:
    root = tmp_path or Path("/tmp")
    p = root / "DONE-14-test.md"
    p.write_text("goal met\n", encoding="utf-8")
    stamp_github_closed(p)
    out = p.read_text(encoding="utf-8")
    assert "Handoff: complete" in out
    assert "GitHub: closed" in out
    stamp_github_closed(p)
    assert out.count("GitHub: closed") == 1


def main() -> int:
    test_parse_complete_variants()
    test_issue_number()
    test_complete_without_github_closed_is_pending()
    test_github_closed_is_done()
    test_deferred_never_pending()
    test_no_issue_until_complete()
    import tempfile

    with tempfile.TemporaryDirectory() as d:
        test_stamp(Path(d))
    print("ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
