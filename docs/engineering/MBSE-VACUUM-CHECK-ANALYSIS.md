# MBSE Vacuum-Cleaner Example: `spec42 check` Analysis

**Corpus:** `C:\Git\MBSE_AG_vacuum-cleaner-robot-example`  
**Command:** `spec42 check` (workspace scan, embedded standard library, default diagnostics)  
**Parser:** `sysml-v2-parser` **0.24.0** ([crates.io](https://crates.io/crates/sysml-v2-parser/0.24.0))  
**Date:** June 2026 (after WP1 diagnostic fidelity + legacy parse coverage)  
**Documents checked:** 52  

## Executive summary

| Severity | Original run | Mid run (0.21.0) | Latest run (0.22.0) |
|----------|-------------:|-----------------:|--------------------------:|
| Error | 20 | 13 | **12** |
| Warning | 136 | 310 | **305** |
| Info | 10 | 17 | **17** |
| **Total** | **166** | **340** | **334** |

**WP1 tooling fixes:**

1. **Kind-aware `duplicate_namespace_member`** — counts by `(name, element_kind)`; subject + `then action` name reuse no longer warns (4→0 false positives). One true positive remains: duplicate `then action roboticVacuumCleaner` steps in `VRS_UseCases.sysml`.
2. **Parser 0.22.0** — `inout item` in port def bodies (`directed_item_usage`); `out volume :> …` pin subsetting in `in_out_decl`; comment-aware EOF brace balance (fixes false `missing_closing_brace` on `VacuumingTypes.sysml` line comments).
3. **Container-scoped connection disambiguation** — suffix/import narrowing under `container_prefix` when a unique local match exists; vacuum `ambiguous_connection_endpoint` unchanged (4) — homonymous ports under the same composite scope.

**Remaining 12 errors** are corpus/model issues (connection endpoints, state-entry shorthand, requirement `id` dialect), not Spec42 false positives.

---

## Diagnostic distribution (latest run)

| Code | Count | Spec/BNF verdict | Notes |
|------|------:|------------------|-------|
| `port_type_mismatch` | 78 | **Model / import issue** | Homonymous port defs from different packages |
| `flow_direction_incompatible` | 60 | **Model issue** | Flow ends disagree on direction |
| `unresolved_type_reference` | 41 | **Mixed** | Missing packages, unqualified names, legacy fragments |
| `unresolved_import_target` | 33 | **Model / corpus gap** | Packages not present or not imported |
| `incompatible_subset_redefine_kind` | 26 | **Semantic** | Subset/redefine target kind mismatch |
| `invalid_qualified_name_segment` | 21 | **Mixed** | Often legacy naming (`VRSA::…` segments) |
| `conjugated_port_inconsistent` | 14 | **Model issue** | `~` conjugation vs usage |
| `unresolved_redefines_target` | 10 | **Mixed** | Redefinition target not in scope |
| `missing_final_state` | 10 | **Info** (§7.18.3 guidance) | Conformance hint, not hard syntax |
| `unresolved_pending_expression_relationship` | 7 | **Model issue** | Missing boundary ports, incomplete `Engine` def |
| `untyped_part_usage` | 7 | **Heuristic** | Informational |
| `succession_endpoint_invalid` | 5 | **Model issue** | Action-like endpoint mismatch |
| `invalid_bare_identifier_in_state_body` | 4 | **Invalid syntax** | Not in `ActionBodyItem` / `StateBodyItem` BNF |
| `ambiguous_connection_endpoint` | 4 | **Model issue** | Multiple homonymous ports under composite |
| `incompatible_type_kind` | 0 | **Fixed (was false positive)** | Actor typed by part/item def is valid per §7.11.2 / §7.22.2 (`ActorUsage : PartUsage`) |
| `analysis_evaluation_unresolved` | 4 | **Expected** | Analysis not executed in `check` |
| `duplicate_namespace_member` | 1 | **Valid** | Two `then action roboticVacuumCleaner` in same use case |
| `unresolved_connection_segment` | 3 | **Model issue** | Broken connection paths |
| `invalid_requirement_short_name_syntax` | 1 | **Invalid syntax** | `id 'Req001'` vs `<'Req001'>` (BNF `Identification`) |
| `unresolved_satisfy_target` | 1 | **Model issue** | |

**Eliminated since 0.21.0 run:** `missing_closing_brace` (1→0), `duplicate_namespace_member` false positives for subject/action reuse (4→1 true positive only).

**Eliminated since spec-alignment fix (0.29.x):** `incompatible_type_kind` on `actor … : RoboticVacuumCleaner` (4→0); `view_filter_non_boolean` / `invalid_import_filter` on `@SysML::…` metaclass filters.

**Eliminated since original run:** `visibility_violation` (75→0), `recovered_part_usage_body_element` (7→0), `unresolved_pending_relationship` (4→0), `'def'` duplicate collisions (3→0).

---

## Errors (12) — spec alignment

### Parse / syntax errors (5) — **invalid per SysML v2 BNF**

| Code | File | Assessment |
|------|------|------------|
| `invalid_bare_identifier_in_state_body` (×4) | `Integration.sysml`, `BatteryLevelComputer.sysml` (×2 paths) | **Invalid.** `entry act { batCap; … }` — bare names not in `ActionBodyItem` |
| `invalid_requirement_short_name_syntax` (×1) | `SystemLevel/DriveUnit.sysml` | **Invalid.** Use `requirement def <'Req001'> …` |

### Semantic errors (7) — **model issues (not Spec42 bugs)**

| Code | Count | Assessment |
|------|------:|------------|
| `unresolved_pending_expression_relationship` | 7 | **Model** — unqualified `dirtyAirFlow` / `cleanAirFlow` on `RoboticVacuumCleaner` without boundary ports; incomplete `Engine` def for `pwmInputPort` |

---

## Warnings — signal vs. noise

### Port and flow (138 total) — **mostly real**

`port_type_mismatch` (78) and `flow_direction_incompatible` (60) reflect homonymous port definitions imported from different packages.

### `duplicate_namespace_member` (1) — **true positive**

`VRS_UseCases.sysml` declares `roboticVacuumCleaner` as `then action` twice in `use case def Vacuming`. Subject + `then action` reuse with the same name is **no longer flagged** (kind-aware rule).

### Import and type resolution (74 total)

`unresolved_type_reference` (41) + `unresolved_import_target` (33) — missing `VRSA::…` packages and domain libraries.

---

## Resolved issues (timeline)

| Issue | Original | Latest | Fix |
|-------|----------|--------|-----|
| `visibility_violation` on `private import ::*` | 75 | **0** | Rule removed (valid per §7.2) |
| `duplicate_namespace_member` for `'def'` | 3 | **0** | Opaque-member naming |
| `duplicate_namespace_member` subject vs `then action` | 4 | **0** | Kind-aware `(name, element_kind)` counting |
| `recovered_part_usage_body_element` | 7 | **0** | Parser 0.20.4: `redefines` keyword |
| `unresolved_pending_relationship` (use-case `first start`) | 4 errors | **0** | `CaseSuccessionChain` in `use_case.rs` |
| `out attribute redefines` in port def | parse cascade | **parsed** | Parser 0.21.0 `directed_attribute_usage` |
| `inout item` in port def | parse cascade | **parsed** | Parser 0.22.0 `directed_item_usage` |
| `missing_closing_brace` on `VacuumingTypes.sysml` | 1 | **0** | Comment-aware brace balance + legacy port forms |
| Nested `part def` in `part def` | 2 | **0** | Parser + semantic graph |

---

## Files with remaining errors

| File | Primary code(s) |
|------|-----------------|
| `Integration.sysml` (×2 paths) | `invalid_bare_identifier_in_state_body`, `unresolved_pending_expression_relationship` |
| `BatteryLevelComputer.sysml` (×2 paths) | `invalid_bare_identifier_in_state_body` |
| `NavigationSystem.sysml`, `RobotVacuum.sysml` | `unresolved_pending_expression_relationship` |
| `SystemLevel/DriveUnit.sysml` | `invalid_requirement_short_name_syntax` |

**No longer erroring:** `legacy/VacuumingSystem/VacuumingTypes.sysml` (`missing_closing_brace` cleared).

---

## Spec alignment summary

| Category | Verdict | Reference |
|----------|---------|-----------|
| `redefines` keyword | **Valid** | SysML §7.6; BNF `REDEFINES` |
| `private import X::*` | **Valid** | SysML §7.2 |
| `out attribute redefines …` in port def | **Valid** | BNF directed feature pattern; parser 0.21.0 |
| `inout item …` in port def | **Valid** | Legacy corpus dialect; parser 0.22.0 |
| Use-case `first start` / `then action` | **Valid** | §7.16; graph materializes succession |
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
| Kind-aware `duplicate_namespace_member` | Done |
| Nested `part def` in `part def` body | Done |
| `redefines` keyword parsing (0.20.4) | Done |
| Use-case succession graph (`CaseSuccessionChain`) | Done |
| `directed_attribute_usage` in port def bodies (0.21.0) | Done |
| `directed_item_usage` in port def bodies (0.22.0) | Done |
| Vacuum baseline (`error_count ≤ 12`, `missing_closing_brace == 0`) | Done |

## Remaining follow-ups (corpus / optional tooling)

1. **Corpus fixes** — boundary ports on `RoboticVacuumCleaner`, complete `Engine` def, state entry action syntax.
2. **Connection ambiguity** — vacuum still has 4× `ambiguous_connection_endpoint`; needs model qualification or richer import disambiguation.

---

## References

- SysML v2 spec text: [elan8-monorepo/library/SysML_v2.txt](C:\Git\elan8-monorepo\library\SysML_v2.txt)
- OMG textual BNF: [sysml-v2-release/bnf/SysML-textual-bnf.kebnf](C:\Git\sysml-v2-release\bnf\SysML-textual-bnf.kebnf)
- Diagnostic pipeline: [DIAGNOSTIC-CHECKS-ROADMAP.md](DIAGNOSTIC-CHECKS-ROADMAP.md)
- CLI workflow: [DEVELOPMENT.md](../DEVELOPMENT.md)
- Regression test: `MBSE_VACUUM_EXAMPLE_DIR=… cargo test -p kernel --test mbse_vacuum_baseline -- --ignored`
