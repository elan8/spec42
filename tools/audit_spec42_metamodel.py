#!/usr/bin/env python3
"""Audit Spec42's published metaclass and projected-relationship mappings.

This source-level audit verifies that Spec42's Rust projection mappings remain
internally complete and consistent.
"""
from __future__ import annotations

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
    if problems:
        print("FAIL: " + "\nFAIL: ".join(problems))
        return 1
    print("PASS: Spec42 mapping consistency")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
