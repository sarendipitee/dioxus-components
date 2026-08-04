#!/usr/bin/env python3
"""Reject CSS module class identifiers converted with .to_string()."""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

RUST_ROOTS = (Path("primitives"), Path("dioxus-components"), Path("preview"))
RUST_SUFFIXES = {".rs"}

STYLE_IDENTIFIER = re.compile(
    r"\b[A-Za-z_][A-Za-z0-9_]*::[A-Za-z_][A-Za-z0-9_]*\.to_string\(\)"
)
CLASS_ATTRIBUTE = re.compile(
    r"\bclass\s*:\s*[^\n]*\b[A-Za-z_][A-Za-z0-9_]*::[A-Za-z_][A-Za-z0-9_]*\.to_string\(\)"
)


def candidate_files(paths: list[str], staged: bool) -> list[Path]:
    if paths:
        return [Path(path) for path in paths]
    if staged:
        result = subprocess.run(
            ["git", "diff", "--cached", "--name-only", "--diff-filter=ACMR"],
            check=True,
            capture_output=True,
            text=True,
        )
        return [Path(path) for path in result.stdout.splitlines()]
    return [
        path
        for root in RUST_ROOTS
        if root.exists()
        for path in root.rglob("*.rs")
    ]


def violations(path: Path) -> list[tuple[int, str]]:
    if path.suffix not in RUST_SUFFIXES or not path.is_file():
        return []
    findings: list[tuple[int, str]] = []
    for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if STYLE_IDENTIFIER.search(line) or CLASS_ATTRIBUTE.search(line):
            findings.append((line_number, line.strip()))
    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("paths", nargs="*", help="Rust files to check")
    parser.add_argument(
        "--staged",
        action="store_true",
        help="check only staged files (used by the pre-commit hook)",
    )
    args = parser.parse_args()

    findings = [
        (path, line_number, line)
        for path in candidate_files(args.paths, args.staged)
        for line_number, line in violations(path)
    ]
    if findings:
        print("Incorrect CSS class conversion detected. Pass CSS module identifiers directly:")
        print("  class: Styles::component_root")
        print("not:")
        print("  class: Styles::component_root.to_string()")
        for path, line_number, line in findings:
            print(f"{path}:{line_number}: {line}")
        return 1

    scope = "staged Rust files" if args.staged else "Rust source files"
    print(f"Class usage check passed ({scope}).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
