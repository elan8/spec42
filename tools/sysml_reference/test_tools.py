#!/usr/bin/env python3
"""Smoke tests for the portable reference tooling."""
from __future__ import annotations

import json
import subprocess
import sys
import tarfile
import tempfile
import unittest
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def run(*arguments: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run([sys.executable, *arguments], cwd=ROOT, text=True, capture_output=True, check=False)


class ToolsTest(unittest.TestCase):
    def test_conformance_audit_distinguishes_missing_evidence_from_failures(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            scorecard = {
                "target": {"release_tag": "2026-04"},
                "release_stamp": {"matches_pin": True},
                "layers": {
                    "L1_syntax_acceptance": {
                        "status": "incomplete", "bnf_productions": {"implemented": 1, "expected": 2},
                    },
                    "L2_structured_ast": {"status": "pass"},
                },
            }
            semantic = {
                "schema_version": 2,
                "fixtures": [],
                "processing_failures": [],
                "stale_generated_snapshots": [],
                "manifest_audit": {
                    "state": "complete",
                    "audit": {"missing_rule_ids": ["sysml-2.0:8.3:deriveUsageDefinition"]},
                },
            }
            (directory / "conformance-scorecard.json").write_text(json.dumps(scorecard), encoding="utf-8")
            report = directory / "snapshot-report.json"
            report.write_text(json.dumps(semantic), encoding="utf-8")
            (directory / "snapshot-report.exit").write_text("1", encoding="utf-8")
            coverage = directory / "bnf_coverage.map"
            coverage.write_text("SysML PartUsage partial\nSysML PartDefinition implemented\n", encoding="utf-8")
            command = (
                "tools/sysml_reference/conformance_audit.py", "summarize", temporary, str(coverage),
                "--spec42-revision", "spec42-test", "--parser-revision", "parser-test",
            )
            result = run(*command)
            self.assertEqual(result.returncode, 0, result.stderr)
            summary = directory / "gap-audit.md"
            text = summary.read_text(encoding="utf-8")
            self.assertIn("PartUsage", text)
            self.assertNotIn("PartDefinition", text)
            self.assertIn("deriveUsageDefinition", text)

            semantic["processing_failures"] = [{"path": "probe.md", "error": "failed to evaluate"}]
            report.write_text(json.dumps(semantic), encoding="utf-8")
            result = run(*command)
            self.assertEqual(result.returncode, 1, result.stderr)
            self.assertIn("fixture processing failures", summary.read_text(encoding="utf-8"))

    def test_grammar_lookup_finds_part_usage(self) -> None:
        result = run("tools/sysml_reference/query_grammar.py", "show", "PartUsage")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("PartUsage", result.stdout)

    def test_specification_query_lists_sections_and_handles_missing_input(self) -> None:
        listing = run(
            "tools/sysml_reference/query_specification.py",
            "--spec",
            "tools/sysml_reference/fixtures/minimal-specification.md",
            "list",
        )
        self.assertEqual(listing.returncode, 0, listing.stderr)
        self.assertIn("Element rules", listing.stdout)
        self.assertNotIn("Not a section", listing.stdout)
        self.assertNotIn("Also not a section", listing.stdout)
        missing = run("tools/sysml_reference/query_specification.py", "search", "rule")
        self.assertEqual(missing.returncode, 0, missing.stderr)
        self.assertIn("SKIP: no specification Markdown supplied", missing.stdout)
        strict = run("tools/sysml_reference/query_specification.py", "--strict-input", "search", "rule")
        self.assertEqual(strict.returncode, 2, strict.stdout + strict.stderr)
        self.assertIn("ERROR: no specification Markdown supplied", strict.stdout)

    def test_specification_query_shows_unambiguous_heading(self) -> None:
        result = run(
            "tools/sysml_reference/query_specification.py",
            "--spec",
            "tools/sysml_reference/fixtures/minimal-specification.md",
            "show",
            "Element rules",
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("An example element", result.stdout)
        self.assertNotIn("# Example Reference", result.stdout)

    def test_library_inspection_searches_directory_and_archives_without_extraction(self) -> None:
        directory = "tools/sysml_reference/fixtures/standard-library"
        listed = run("tools/sysml_reference/inspect_standard_library.py", "--library", directory, "list")
        self.assertEqual(listed.returncode, 0, listed.stderr)
        self.assertEqual(listed.stdout, "Example.sysml\n")
        found = run("tools/sysml_reference/inspect_standard_library.py", "--library", directory, "search", "examplepart")
        self.assertEqual(found.returncode, 0, found.stderr)
        self.assertEqual(found.stdout, "Example.sysml\n")
        source = run(
            "tools/sysml_reference/inspect_standard_library.py",
            "--library",
            f"{directory}/Example.sysml",
            "list",
        )
        self.assertEqual(source.returncode, 0, source.stderr)
        self.assertEqual(source.stdout, "Example.sysml\n")
        with tempfile.TemporaryDirectory() as temporary:
            archive_path = Path(temporary) / "library.zip"
            with zipfile.ZipFile(archive_path, "w") as archive:
                archive.writestr("nested/Archive.sysml", "package ArchiveLibrary { part def ArchivePart; }")
            archive = run("tools/sysml_reference/inspect_standard_library.py", "--library", str(archive_path), "search", "archivepart")
            self.assertEqual(archive.returncode, 0, archive.stderr)
            self.assertEqual(archive.stdout, "nested/Archive.sysml\n")

    def test_library_inspection_accepts_tar_and_skips_missing_inputs(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            source = Path(temporary) / "Source.sysml"
            source.write_text("package SourceLibrary {}", encoding="utf-8")
            archive_path = Path(temporary) / "library.tar"
            with tarfile.open(archive_path, "w") as archive:
                archive.add(source, arcname="Source.sysml")
            result = run("tools/sysml_reference/inspect_standard_library.py", "--library", str(archive_path), "list")
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(result.stdout, "Source.sysml\n")
        missing = run("tools/sysml_reference/inspect_standard_library.py", "search", "anything")
        self.assertEqual(missing.returncode, 0, missing.stderr)
        self.assertIn("SKIP: no standard-library input supplied", missing.stdout)
        strict = run("tools/sysml_reference/inspect_standard_library.py", "--strict-input", "search", "anything")
        self.assertEqual(strict.returncode, 2, strict.stdout + strict.stderr)
        self.assertIn("ERROR: no standard-library input supplied", strict.stdout)


if __name__ == "__main__":
    unittest.main()
