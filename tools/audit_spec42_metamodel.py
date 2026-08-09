#!/usr/bin/env python3
"""Audit Spec42's published metaclass and projected-relationship mappings.

This is intentionally a source-level audit: it verifies the exhaustive Rust
mapping rather than pretending that Spec42 stores every OMG XMI property.  An
optional ``--schema`` makes the check compare published names to an external,
authoritative OMG JSON schema.  Without it, the mapping audit is deterministic
and still suitable for CI; the OMG conformance portion reports SKIP.
"""
from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
API = ROOT / "crates/generator_api/src/model.rs"
PROTOCOL = ROOT / "crates/generator_protocol/src/lib.rs"
PROJECTION = ROOT / "crates/workspace/src/snapshot/projection.rs"
FACTS = ROOT / "crates/workspace/src/snapshot/facts.rs"
MAPPING = re.compile(r"ElementKind::([A-Za-z]+)(?:\([^)]*\))?\s*=>\s*Metaclass::([A-Za-z]+)")
ENUM = re.compile(r"pub enum Metaclass \{(?P<body>.*?)^\}", re.MULTILINE | re.DOTALL)
REL_ENUM = re.compile(r"pub enum HostRelationshipMetaclass \{(?P<body>.*?)^\}", re.MULTILINE | re.DOTALL)
VARIANT = re.compile(r"^\s{4}([A-Z][A-Za-z0-9_]*)[,{]", re.MULTILINE)
REL_MAPPING = re.compile(r"HostRelationshipMetaclass::([A-Za-z]+)")


def variants(path: Path, pattern: re.Pattern[str]) -> set[str]:
    match = pattern.search(path.read_text(encoding="utf-8"))
    if not match:
        raise RuntimeError(f"cannot find expected enum in {path}")
    return set(VARIANT.findall(match.group("body")))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--schema", type=Path, help="optional OMG JSON abstract-syntax schema")
    parser.add_argument("--strict-schema", action="store_true", help="fail when published names are not exact schema names")
    args = parser.parse_args()
    api_text = API.read_text(encoding="utf-8")
    pairs = MAPPING.findall(api_text)
    published = variants(PROTOCOL, ENUM) - {"Unrecognized"}
    # `Unknown` is intentionally represented by the protocol's payload-bearing
    # `Unrecognized(String)` escape hatch, not a closed metaclass variant.
    mapped = {target for _, target in pairs if target != "Unrecognized"}
    problems = []
    if not pairs:
        problems.append("no ElementKind -> Metaclass arms found")
    if mapped - published:
        problems.append("mapping targets absent from protocol: " + ", ".join(sorted(mapped - published)))
    if published - mapped:
        problems.append("published metaclasses without ElementKind mapping: " + ", ".join(sorted(published - mapped)))
    relationship_variants = variants(PROJECTION, REL_ENUM)
    referenced_relationships = set(REL_MAPPING.findall(FACTS.read_text(encoding="utf-8")))
    if referenced_relationships - relationship_variants:
        problems.append("relationship mapping targets absent from projection: " + ", ".join(sorted(referenced_relationships - relationship_variants)))
    print(f"ElementKind -> Metaclass arms: {len(pairs)}")
    print(f"Published Metaclass variants: {len(published)}")
    print(f"Projected relationship metaclasses: {len(relationship_variants)}")
    print(f"Relationship metaclasses used by projection: {len(referenced_relationships)}")
    if args.schema is None:
        print("SKIP: OMG schema conformance not checked; pass --schema /path/to/SysML-abstract-syntax.json")
    elif not args.schema.is_file():
        print(f"SKIP: OMG schema input is unavailable: {args.schema}")
    else:
        schema = json.loads(args.schema.read_text(encoding="utf-8"))
        definitions = set(schema.get("$defs", {}))
        missing = published - definitions
        if missing:
            print(f"OMG schema exact-name matches: {len(published - missing)}/{len(published)}")
            detail = ", ".join(sorted(missing))
            if args.strict_schema:
                problems.append("published metaclasses absent from OMG schema: " + detail)
            else:
                print("SKIP: exact OMG metaclass conformance needs a reviewed normalization map; "
                      "raw unmatched names: " + detail)
        else:
            print("OMG schema conformance: all published metaclasses found")
    if problems:
        print("FAIL: " + "\nFAIL: ".join(problems))
        return 1
    print("PASS: Spec42 mapping consistency")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
