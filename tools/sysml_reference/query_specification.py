#!/usr/bin/env python3
"""Query an explicitly supplied specification Markdown document.

The document remains outside this repository. Missing inputs are reported as
SKIP unless --strict-input is selected, making the tool safe for optional CI
checks as well as local reference lookups.
"""
from __future__ import annotations

import argparse
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Section:
    title: str
    level: int
    start: int
    end: int


def unavailable(message: str, strict: bool) -> int:
    prefix = "ERROR" if strict else "SKIP"
    print(f"{prefix}: {message}")
    return 2 if strict else 0


def sections(text: str) -> list[Section]:
    lines = text.splitlines(keepends=True)
    headings: list[tuple[str, int, int]] = []
    for index, line in enumerate(lines):
        stripped = line.rstrip("\r\n")
        if not stripped.startswith("#"):
            continue
        marker, separator, title = stripped.partition(" ")
        if separator and set(marker) == {"#"}:
            headings.append((title.rstrip("#").rstrip(), len(marker), index))
    result = []
    for index, (title, level, start) in enumerate(headings):
        end = len(lines)
        for _, next_level, next_start in headings[index + 1 :]:
            if next_level <= level:
                end = next_start
                break
        result.append(Section(title, level, start, end))
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--spec", type=Path, help="path to a specification Markdown file")
    parser.add_argument("--strict-input", action="store_true", help="fail instead of skipping an unavailable input")
    sub = parser.add_subparsers(dest="command", required=True)
    listing = sub.add_parser("list", help="list Markdown sections")
    listing.add_argument("--max-depth", type=int, default=None, help="omit sections deeper than this heading level")
    show = sub.add_parser("show", help="show a section by its exact heading")
    show.add_argument("heading")
    search = sub.add_parser("search", help="case-insensitively search document text")
    search.add_argument("text")
    args = parser.parse_args()

    if args.spec is None:
        return unavailable("no specification Markdown supplied; pass --spec /path/to/specification.md", args.strict_input)
    if not args.spec.is_file():
        return unavailable(f"specification Markdown input is unavailable: {args.spec}", args.strict_input)
    try:
        text = args.spec.read_text(encoding="utf-8")
    except UnicodeDecodeError as error:
        print(f"ERROR: specification Markdown is not UTF-8: {error}")
        return 2

    found_sections = sections(text)
    lines = text.splitlines()
    if args.command == "list":
        for section in found_sections:
            if args.max_depth is None or section.level <= args.max_depth:
                print(f"{section.start + 1}\t{'  ' * (section.level - 1)}{section.title}")
        return 0
    if args.command == "show":
        matches = [section for section in found_sections if section.title == args.heading]
        if not matches:
            print(f"ERROR: no section headed {args.heading!r}")
            return 2
        if len(matches) > 1:
            print(f"ERROR: heading is ambiguous ({len(matches)} matches): {args.heading!r}")
            return 2
        section = matches[0]
        print("\n".join(lines[section.start : section.end]))
        return 0

    needle = args.text.casefold()
    matches = [(number, line) for number, line in enumerate(lines, start=1) if needle in line.casefold()]
    for number, line in matches:
        print(f"{number}: {line}")
    return 0 if matches else 1


if __name__ == "__main__":
    raise SystemExit(main())
