# Diagnostics editing gaps (v0.58.0, issue #345)

Active defect candidates found by the daily-editing corpus in `tests/snapshots/editing/`. Each
entry names the fixture whose `DIAGNOSTICS` golden shows the gap today. A fix replaces that golden
with the corrected output and adds the focused valid/invalid `EXPECTED DIAGNOSTICS` evidence under
`tests/snapshots/validation/`. Remove an entry when its fix merges; delete this file when it is
empty.

Classifications: **missing check** (the specification rejects the model and Spec42 is silent),
**cascade** (one mistake hides or multiplies others), **location** (right diagnostic, wrong range),
**optional advice** (no normative rule; a hint would help).

## Candidates

| ID | Classification | Gap | Rule | Evidence |
| --- | --- | --- | --- | --- |
| E1 | cascade | An unclosed body drops the whole document: no declarations, no semantic diagnostics, and other parse errors in the file are not reported. | — | `incomplete_source.md` (`c.sysml`) |
| E2 | missing check | A guard on an action-body succession (`first a1 if 42 then a2`) is not checked: lowering mints no guard Expression for it, so the guard rule has no element to judge. Needs the succession guard element from #409. | SysML 8.3.18.8 `validateTransitionFeatureMembershipGuardExpression` | `expression_result_typing.md`, `boolean_guards.md` |
| E4 | missing check | A qualified name reaches private members: `Lib::Hidden` and `Lib::Inner::Deep` resolve although both are private. | KerML 8.2.3.5 name resolution (visible memberships) | `names_qualified_visibility.md` |
| E5 | cascade | An import without a visibility keyword is reported (`missing_import_visibility`) and then discarded, so every name it imports is also reported unresolved. | SysML 2.0 import visibility | `names_imports.md` |
| E6 | missing check | A quantity value for a scalar feature is accepted: `attribute n : Real = 2 [SI::kg]`. A string value for `Real` is reported. | KerML 8.3.4.10.2 `checkFeatureValueBindingConnector` (bound value conformance) | `expression_result_typing.md` |
| E7 | location | `missing_type_reference` and `missing_semicolon` ranges run onto the next line instead of marking the insertion point. | — | `incomplete_source.md` (`a.sysml`, `b.sysml`) |
| E8 | cascade | `--strict-diagnostics` drops unresolved/ambiguous reference diagnostics *before* the first parse error, although `PostprocessPolicy` documents dropping those a parse error earlier in the document explains. | — | `crates/sysml_diagnostics/src/postprocess.rs` |
| E9 | optional advice | `[*..2]` is accepted while `[3..1]` is reported as `invalid_multiplicity`. | — | `expression_result_typing.md` |
| E10 | optional advice | An operator applied to incompatible operands (`1 + "a"`) is accepted. | — | `expression_result_typing.md`, `expressions_units.md` |

Not candidates: connecting incompatible port types, a flow drawn against port directions, and a
subsetting feature with a wider multiplicity than its subsetted feature have no normative rule in
the pinned KerML/SysML; they are optional advice to revisit after the candidates above.

## Open decision

Select the bounded list to fix in v0.58.0 on issue #345.
