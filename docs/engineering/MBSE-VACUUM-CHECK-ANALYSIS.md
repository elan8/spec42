# MBSE Vacuum-Cleaner Example: `spec42 check` Analysis

**Corpus:** `C:\Git\MBSE_AG_vacuum-cleaner-robot-example`  
**Command:** `spec42 check` (workspace scan, embedded standard library, default diagnostics)  
**Parser:** `sysml-v2-parser` **0.21.0** ([crates.io](https://crates.io/crates/sysml-v2-parser))  
**Date:** June 2026 (after use-case succession + directed port-attribute parser fixes)  
**Documents checked:** 52  

## Executive summary

| Severity | Original run | Mid run (0.20.4) | Latest run (0.21.0) |
|----------|-------------:|-----------------:|--------------------:|
| Error | 20 | 17 | **13** |
| Warning | 136 | 291 | **310** |
| Info | 10 | 17 | **17** |
| **Total** | **166** | **325** | **340** |

**Latest tooling fixes (this wave):**

1. **Use-case succession** — `first start` / `then action` / `then done` now materialize graph nodes and Flow edges; **`unresolved_pending_relationship` errors eliminated** (was 4).
2. **Parser 0.21.0** — `out attribute redefines …` in port def bodies parses as directed `AttributeUsage` (no longer mis-read as `InOutDecl` with name `redefines`).
3. **Prior fixes retained** — `visibility_violation` (0), `recovered_part_usage_body_element` (0), `'def'` name collisions (0).

**Remaining 13 errors** are corpus/model issues (connection endpoints, state-entry shorthand, requirement `id` dialect, one legacy parse cascade in `VacuumingTypes.sysml`), not Spec42 false positives.

---

## Diagnostic distribution (latest run)

| Code | Count | Spec/BNF verdict | Notes |
|------|------:|------------------|-------|
| `port_type_mismatch` | 80 | **Model / import issue** | Homonymous port defs from different packages |
| `flow_direction_incompatible` | 60 | **Model issue** | Flow ends disagree on direction |
| `unresolved_type_reference` | 39 | **Mixed** | Missing packages, unqualified names, legacy fragments |
| `unresolved_import_target` | 33 | **Model / corpus gap** | Packages not present or not imported |
| `incompatible_subset_redefine_kind` | 26 | **Semantic** | Subset/redefine target kind mismatch |
| `invalid_qualified_name_segment` | 21 | **Mixed** | Often legacy naming (`VRSA::…` segments) |
| `conjugated_port_inconsistent` | 16 | **Model issue** | `~` conjugation vs usage |
| `unresolved_redefines_target` | 10 | **Mixed** | Redefinition target not in scope |
| `missing_final_state` | 10 | **Info** (§7.18.3 guidance) | Conformance hint, not hard syntax |
| `unresolved_pending_expression_relationship` | 7 | **Model issue** | Missing boundary ports, incomplete `Engine` def |
| `untyped_part_usage` | 7 | **Heuristic** | Informational |
| `succession_endpoint_invalid` | 5 | **Model issue** | Action-like endpoint mismatch |
| `invalid_bare_identifier_in_state_body` | 4 | **Invalid syntax** | Not in `ActionBodyItem` / `StateBodyItem` BNF |
| `duplicate_namespace_member` | 4 | **Mixed** | `roboticVacuumCleaner` reused across subject + `then action` steps |
| `incompatible_type_kind` | 4 | **Valid check** | Actor typed by part def (BNF: `ActorUsage`) |
| `analysis_evaluation_unresolved` | 4 | **Expected** | Analysis not executed in `check` |
| `ambiguous_connection_endpoint` | 4 | **Model issue** | Multiple endpoint matches |
| `unresolved_connection_segment` | 3 | **Model issue** | Broken connection paths |
| `invalid_requirement_short_name_syntax` | 1 | **Invalid syntax** | `id 'Req001'` vs `<'Req001'>` (BNF `Identification`) |
| `missing_closing_brace` | 1 | **Parse cascade** | `VacuumingTypes.sysml` — legacy `inout item` / brace structure |
| `unresolved_satisfy_target` | 1 | **Model issue** | |

**Eliminated since original run:** `visibility_violation` (75→0), `recovered_part_usage_body_element` (7→0), `unresolved_pending_relationship` (4→0), `'def'` duplicate collisions (3→0).

---

## Errors (13) — spec alignment

### Parse / syntax errors (6) — **invalid per SysML v2 BNF**

| Code | File | Assessment |
|------|------|------------|
| `invalid_bare_identifier_in_state_body` (×4) | `Integration.sysml`, `BatteryLevelComputer.sysml` (×2 paths) | **Invalid.** `entry act { batCap; … }` — bare names not in `ActionBodyItem` |
| `invalid_requirement_short_name_syntax` (×1) | `SystemLevel/DriveUnit.sysml` | **Invalid.** Use `requirement def <'Req001'> …` |
| `missing_closing_brace` (×1) | `legacy/VacuumingSystem/VacuumingTypes.sysml` | **Invalid / legacy.** Port-def `out attribute redefines` now parses; remaining error from other body forms (`inout item …`) or brace layout |

### Semantic errors (7) — **model issues (not Spec42 bugs)**

| Code | Count | Assessment |
|------|------:|------------|
| `unresolved_pending_expression_relationship` | 7 | **Model** — unqualified `dirtyAirFlow` / `cleanAirFlow` on `RoboticVacuumCleaner` without boundary ports; incomplete `Engine` def for `pwmInputPort` |

---

## Warnings — signal vs. noise

### Port and flow (140 total) — **mostly real**

`port_type_mismatch` (80) and `flow_direction_incompatible` (60) reflect homonymous port definitions imported from different packages.

### `duplicate_namespace_member` (4) — **new visibility from use-case graph**

Materializing `subject roboticVacuumCleaner` and `then action roboticVacuumCleaner:…` steps surfaces name reuse inside `use case def Vacuming`. The `'def'` opaque-member collision remains **fixed** (baseline asserts zero `'def'` collisions).

### Import and type resolution (72 total)

`unresolved_type_reference` (39) + `unresolved_import_target` (33) — missing `VRSA::…` packages and domain libraries.

---

## Resolved issues (timeline)

| Issue | Original | Latest | Fix |
|-------|----------|--------|-----|
| `visibility_violation` on `private import ::*` | 75 | **0** | Rule removed (valid per §7.2) |
| `duplicate_namespace_member` for `'def'` | 3 | **0** | Opaque-member naming |
| `recovered_part_usage_body_element` | 7 | **0** | Parser 0.20.4: `redefines` keyword |
| `unresolved_pending_relationship` (use-case `first start`) | 4 errors | **0** | `CaseSuccessionChain` in `use_case.rs` |
| `out attribute redefines` in port def | parse cascade | **parsed** | Parser 0.21.0 `directed_attribute_usage` |
| Nested `part def` in `part def` | 2 | **0** | Parser + semantic graph |

---

## Files with remaining errors

| File | Primary code(s) |
|------|-----------------|
| `Integration.sysml` (×2 paths) | `invalid_bare_identifier_in_state_body`, `unresolved_pending_expression_relationship` |
| `BatteryLevelComputer.sysml` (×2 paths) | `invalid_bare_identifier_in_state_body` |
| `NavigationSystem.sysml`, `RobotVacuum.sysml` | `unresolved_pending_expression_relationship` |
| `legacy/VacuumingSystem/VacuumingTypes.sysml` | `missing_closing_brace` |
| `SystemLevel/DriveUnit.sysml` | `invalid_requirement_short_name_syntax` |

**No longer erroring:** `VRS_UseCases.sysml`, `integration.sysml` (use-case succession resolved).

---

## Spec alignment summary

| Category | Verdict | Reference |
|----------|---------|-----------|
| `redefines` keyword | **Valid** | SysML §7.6; BNF `REDEFINES` |
| `private import X::*` | **Valid** | SysML §7.2 |
| `out attribute redefines …` in port def | **Valid** | BNF directed feature pattern; parser 0.21.0 |
| Use-case `first start` / `then action` | **Valid** | §7.16; graph now materializes succession |
| `actor` typed by `part def` | **Invalid** usage kind | BNF `ActorUsage` |
| `entry act { batCap; … }` | **Invalid** syntax | BNF `StateBodyItem` / `ActionBodyItem` |
| Homonymous port defs across imports | **Real** model problems | §7.13–7.14 |
| Robot-level `dirtyAirFlow` connects | **Model gap** | Missing composite boundary ports |

---

## Implementation status (June 2026)

| Item | Status |
|------|--------|
| Remove false-positive `visibility_violation` | Done |
| Fix `duplicate_namespace_member` `'def'` collision | Done |
| Nested `part def` in `part def` body | Done |
| Default `spec42 check`: semantic checks despite parse errors | Done |
| `redefines` keyword parsing (0.20.4) | Done |
| Use-case succession graph (`CaseSuccessionChain`) | Done |
| `directed_attribute_usage` in port def bodies (0.21.0) | Done |
| Vacuum baseline (`error_count ≤ 15`, `unresolved_pending_relationship == 0`) | Done |

## Remaining follow-ups (corpus / optional tooling)

1. **Corpus fixes** — boundary ports on `RoboticVacuumCleaner`, complete `Engine` def, state entry action syntax, `VacuumingTypes.sysml` legacy port bodies.
2. **`duplicate_namespace_member` for subject/action name reuse** — decide whether same local name across feature kinds should warn (corpus uses `roboticVacuumCleaner` repeatedly).
3. **`VacuumingTypes.sysml` `inout item` in port def** — separate parser path if needed beyond directed attributes.

---

## References

- SysML v2 spec text: [elan8-monorepo/library/SysML_v2.txt](C:\Git\elan8-monorepo\library\SysML_v2.txt)
- OMG textual BNF: [sysml-v2-release/bnf/SysML-textual-bnf.kebnf](C:\Git\sysml-v2-release\bnf\SysML-textual-bnf.kebnf)
- Diagnostic pipeline: [DIAGNOSTIC-CHECKS-ROADMAP.md](DIAGNOSTIC-CHECKS-ROADMAP.md)
- CLI workflow: [DEVELOPMENT.md](../DEVELOPMENT.md)
- Regression test: `MBSE_VACUUM_EXAMPLE_DIR=… cargo test -p kernel --test lsp_integration mbse_vacuum -- --ignored`
