#!/usr/bin/env python3
"""Validate commit subjects against the repository's Conventional Commit format."""

from __future__ import annotations

import re
import sys
from pathlib import Path

CONVENTIONAL_SUBJECT = re.compile(
    r"^(?:feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)"
    r"(?:\([a-z0-9][a-z0-9._/-]*\))?: .+\S$"
)
MERGE_SUBJECT = re.compile(r"^Merge .+")
MAX_SUBJECT_LENGTH = 100


def main() -> int:
    if len(sys.argv) != 2:
        print(f"usage: {Path(sys.argv[0]).name} <commit-message-file>", file=sys.stderr)
        return 2

    message_path = Path(sys.argv[1])
    try:
        first_line = message_path.read_text(encoding="utf-8").splitlines()[0]
    except (OSError, IndexError) as error:
        print(f"Unable to read commit message: {error}", file=sys.stderr)
        return 2

    if MERGE_SUBJECT.fullmatch(first_line):
        return 0

    if len(first_line) > MAX_SUBJECT_LENGTH:
        print(
            f"Commit subject is {len(first_line)} characters; maximum is "
            f"{MAX_SUBJECT_LENGTH}.",
            file=sys.stderr,
        )
        return 1

    if not CONVENTIONAL_SUBJECT.fullmatch(first_line):
        print(
            "Invalid commit subject. Use '<type>(<scope>): <description>', "
            "for example 'ci(hooks): strengthen local quality hooks'.",
            file=sys.stderr,
        )
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
