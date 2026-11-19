"""Combine the pinned parser and Spec42 conformance reports for triage."""

import argparse
from collections import Counter
import json
from pathlib import Path
import re
import tomllib


FOCUS = re.compile(r"definition|usage|speciali[sz]", re.IGNORECASE)
OPEN_STATES = {"blocked", "failed", "stale", "unclassified"}


def parser_pin(manifest):
    with manifest.open("rb") as source:
        pin = tomllib.load(source)["workspace"]["dependencies"]["sysml-v2-parser"]
    repository = pin["git"].removeprefix("https://github.com/").removesuffix(".git")
    revision = pin["rev"]
    if not pin["git"].startswith("https://github.com/") or not re.fullmatch(
        r"[\w.-]+/[\w.-]+", repository
    ) or not re.fullmatch(r"[0-9a-f]{40}", revision):
        raise ValueError("Expected a GitHub repository and an exact parser commit in Cargo.toml")
    return repository, revision


def grammar_candidates(path):
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        entry = line.partition("#")[0].split()
        if not entry:
            continue
        grammar, pattern, status = entry
        if status != "implemented" and FOCUS.search(pattern):
            rows.append((grammar, pattern, status))
    return sorted(rows)


def summarize(directory, coverage_map, spec42_revision, parser_revision):
    scorecard = json.loads((directory / "conformance-scorecard.json").read_text(encoding="utf-8"))
    semantic = json.loads((directory / "snapshot-report.json").read_text(encoding="utf-8"))
    report_exit = int((directory / "snapshot-report.exit").read_text(encoding="utf-8"))
    if semantic["schema_version"] != 2:
        raise ValueError(f"Unsupported snapshot report schema {semantic['schema_version']}")
    manifest = semantic["manifest_audit"]
    audit = manifest["audit"] if manifest["state"] == "complete" else {}
    missing = audit.get("missing_rule_ids", [])
    fixtures = [fixture for fixture in semantic["fixtures"] if fixture["state"] in OPEN_STATES]
    focused = [
        fixture for fixture in fixtures
        if FOCUS.search(" ".join([fixture["path"], *fixture["rule_ids"]]))
    ]
    lines = [
        "# Conformance gap audit",
        "",
        f"- Spec42 revision: `{spec42_revision}`",
        f"- Parser revision: `{parser_revision}`",
        f"- Parser OMG release: `{scorecard['target']['release_tag']}`",
        "",
        "Focus: definitions, usages, and specialization. The full reports retain every result.",
        "Missing evidence and oracle disagreements are leads. Confirm deviations against an OMG clause.",
        "",
        "## Parser evidence",
        "",
    ]
    for key in ("L1_syntax_acceptance", "L2_structured_ast"):
        lines.append(f"- {key}: {scorecard['layers'][key]['status']}")
    productions = scorecard["layers"]["L1_syntax_acceptance"]["bnf_productions"]
    lines.append(f"- Classified implemented: {productions['implemented']} / {productions['expected']}")
    lines.extend(["", "Coverage-map patterns for review (a pattern can cover multiple productions):", ""])
    candidates = grammar_candidates(coverage_map)
    lines.extend(
        [f"- {grammar} `{pattern}` — {status}" for grammar, pattern, status in candidates]
        or ["No matching patterns are classified as incomplete."]
    )
    lines.extend([
        "",
        "Review [official corpus disagreements](parser-oracle-release.md) and",
        "[OpenSysML disagreements](parser-oracle-opensysml.md) with the pinned grammar.",
        "",
        "## Spec42 evidence",
        "",
    ])
    counts = Counter(fixture["state"] for fixture in fixtures)
    lines.extend(f"- {state}: {counts[state]}" for state in sorted(OPEN_STATES))
    lines.extend([f"- Manifest rules without fixture evidence: {len(missing)}", "", "### Focused fixtures", ""])
    for fixture in sorted(focused, key=lambda item: item["path"]):
        blocker = fixture["blocked_by"]
        detail = f" ({blocker['owner']}, {blocker['id']})" if blocker else ""
        lines.append(f"- **{fixture['state']}** `{fixture['path']}`{detail}")
        if fixture["rule_ids"]:
            lines.append(f"  - {', '.join(fixture['rule_ids'])}")
        if blocker:
            lines.append(f"  - {blocker['summary']}")
    lines.extend(["", "### Focused rules without evidence", ""])
    lines.extend(
        [f"- `{rule}`" for rule in sorted(missing) if FOCUS.search(rule)]
        or ["No matching rules lack fixture evidence."]
    )
    errors = []
    if not scorecard["release_stamp"]["matches_pin"]:
        errors.append("Parser scorecard did not verify the pinned OMG release")
    if report_exit not in (0, 1):
        errors.append(f"Unexpected Spec42 report exit code {report_exit}")
    if manifest["state"] != "complete":
        errors.append(f"Manifest audit failed: {manifest['error']}")
    for key, value in audit.items():
        if value and (key.endswith("_mismatches") or key in {
            "duplicate_primary_rule_ids", "orphan_secondary_rule_ids", "unknown_rule_ids"
        }):
            errors.append(f"{key}: {len(value)}")
    if semantic["processing_failures"]:
        errors.append(f"{len(semantic['processing_failures'])} fixture processing failures")
    if counts["failed"] or counts["stale"]:
        errors.append(f"{counts['failed']} failed expectations, {counts['stale']} stale blockers")
    if semantic["stale_generated_snapshots"]:
        errors.append(f"{len(semantic['stale_generated_snapshots'])} stale generated snapshots")
    if report_exit == 1 and not missing and not errors:
        errors.append("Spec42 report failed without missing evidence; inspect its manifest audit")
    if errors:
        lines.extend(["", "## Audit errors", "", *(f"- {error}" for error in errors)])
    (directory / "gap-audit.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    return not errors


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    pin = commands.add_parser("parser-pin")
    pin.add_argument("--manifest", type=Path, default=Path("Cargo.toml"))
    report = commands.add_parser("summarize")
    report.add_argument("directory", type=Path)
    report.add_argument("coverage_map", type=Path)
    report.add_argument("--spec42-revision", required=True)
    report.add_argument("--parser-revision", required=True)
    args = parser.parse_args()
    try:
        if args.command == "parser-pin":
            repository, revision = parser_pin(args.manifest)
            print(f"repository={repository}\nrevision={revision}")
        elif not summarize(args.directory, args.coverage_map, args.spec42_revision, args.parser_revision):
            parser.exit(1, "Conformance audit reported errors. See gap-audit.md.\n")
    except (OSError, ValueError, KeyError, TypeError) as error:
        parser.exit(1, f"Conformance audit could not read its inputs: {error}\n")


if __name__ == "__main__":
    main()
