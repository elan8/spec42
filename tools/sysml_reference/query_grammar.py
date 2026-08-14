#!/usr/bin/env python3
"""Small, dependency-free lookup tool for the bundled SysML v2 EBNF.

The source files are intentionally limited to the textual grammars.  They are
useful without a large external reference checkout; use ``--help`` for the
available queries.
"""
from __future__ import annotations

import argparse
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent
SOURCES = {
    "kerml": ROOT / "KerML-textual-bnf.kebnf",
    "sysml": ROOT / "SysML-textual-bnf.kebnf",
}
RULE = re.compile(r"^([A-Z][A-Za-z0-9_]*)\s*=\s*$", re.MULTILINE)


def rules(source: str | None) -> dict[str, tuple[str, str]]:
    selected = SOURCES.items() if source is None else [(source, SOURCES[source])]
    result = {}
    for label, path in selected:
        text = path.read_text(encoding="utf-8")
        matches = list(RULE.finditer(text))
        for index, match in enumerate(matches):
            end = matches[index + 1].start() if index + 1 < len(matches) else len(text)
            result[match.group(1)] = (label, text[match.start():end].strip())
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", choices=sorted(SOURCES))
    sub = parser.add_subparsers(dest="command", required=True)
    show = sub.add_parser("show", help="show one production")
    show.add_argument("name")
    search = sub.add_parser("search", help="search production names and bodies")
    search.add_argument("pattern")
    sub.add_parser("list", help="list production names")
    args = parser.parse_args()
    all_rules = rules(args.source)
    if args.command == "list":
        for name in sorted(all_rules):
            print(f"{name}\t{all_rules[name][0]}")
        return 0
    if args.command == "show":
        item = all_rules.get(args.name)
        if not item:
            print(f"ERROR: no production named {args.name}")
            return 2
        print(f"# {args.name} ({item[0]})\n{item[1]}")
        return 0
    pattern = re.compile(args.pattern, re.IGNORECASE)
    found = [(name, value) for name, value in all_rules.items() if pattern.search(name) or pattern.search(value[1])]
    for name, (source, _) in sorted(found):
        print(f"{name}\t{source}")
    return 0 if found else 1


if __name__ == "__main__":
    raise SystemExit(main())
