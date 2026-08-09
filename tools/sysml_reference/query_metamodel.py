#!/usr/bin/env python3
"""Query an external OMG SysML JSON metamodel schema.

No OMG schema is committed here. Pass ``--schema`` to a local authoritative
JSON schema checkout; absence is an explicit SKIP, so this can run in CI.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path


def skip(message: str) -> int:
    print(f"SKIP: {message}")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--schema", type=Path, help="OMG JSON abstract-syntax schema")
    sub = parser.add_subparsers(dest="command", required=True)
    show = sub.add_parser("show", help="show a metaclass definition")
    show.add_argument("name")
    sub.add_parser("list", help="list metaclass names")
    args = parser.parse_args()
    if args.schema is None:
        return skip("no schema supplied; pass --schema /path/to/SysML-abstract-syntax.json")
    if not args.schema.is_file():
        return skip(f"schema input is unavailable: {args.schema}")
    try:
        definitions = json.loads(args.schema.read_text(encoding="utf-8"))["$defs"]
    except (json.JSONDecodeError, KeyError) as error:
        print(f"ERROR: schema is not an OMG JSON-schema document: {error}")
        return 2
    if args.command == "list":
        print("\n".join(sorted(definitions)))
        return 0
    definition = definitions.get(args.name)
    if definition is None:
        print(f"ERROR: metaclass not found: {args.name}")
        return 2
    print(json.dumps(definition, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
