#!/usr/bin/env python3
"""Smoke tests for the portable reference tooling."""
from __future__ import annotations

import subprocess
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def run(*arguments: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run([sys.executable, *arguments], cwd=ROOT, text=True, capture_output=True, check=False)


class ToolsTest(unittest.TestCase):
    def test_grammar_lookup_finds_part_usage(self) -> None:
        result = run("tools/sysml_reference/query_grammar.py", "show", "PartUsage")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("PartUsage", result.stdout)

    def test_audit_is_deterministic_without_external_schema(self) -> None:
        result = run("tools/audit_spec42_metamodel.py")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("PASS: Spec42 mapping consistency", result.stdout)
        self.assertIn("SKIP: OMG schema conformance", result.stdout)

    def test_metamodel_query_uses_explicit_schema_input(self) -> None:
        result = run(
            "tools/sysml_reference/query_metamodel.py",
            "--schema",
            "tools/sysml_reference/fixtures/minimal-schema.json",
            "show",
            "PartUsage",
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('"type": "object"', result.stdout)


if __name__ == "__main__":
    unittest.main()
