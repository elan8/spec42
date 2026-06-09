# Action flow and state transition view — BNF sign-off checklist

Generated: 2026-06-09

Normative figures: `SysML-v2-Release/bnf/images/`. Implementation: [`shared/diagram-renderer`](../../shared/diagram-renderer). Server projection: [`state_views`](../../crates/semantic_core/src/semantic/state_views/) and [`extracted_model`](../../crates/semantic_core/src/semantic/extracted_model.rs).

## State transition view

| BNF SVG | Status | Implementation | Tests |
|---------|--------|----------------|-------|
| state.svg | partial | `views/state-transition.ts` | `shared/diagram-renderer/src/renderer.test.ts`, `crates/semantic_core/tests/state_machine_semantics.rs`, `crates/kernel/tests/integration/model.rs` |
| state-transition-compartment.svg | partial | `views/state-transition.ts` entry/do/exit compartments | `shared/diagram-renderer/src/prepare.test.ts` |
| transition.svg | partial | `StateMachineDto` transitions with guard/effect/accept | `state_views/graph_extractor.rs`, `state_machine_semantics.rs` |

**Graph-first path:** `build_workspace_state_machines` → `SysmlVisualizationResultDto.stateMachines` → thin `prepareData.ts` selector → `prepareState`.

**Open gaps:** composite region nesting from nested state hierarchy only (no separate region DTO yet); terminate vs final distinction; full accept/send payload notation on edges.

## Action flow view

| BNF SVG | Status | Implementation | Tests |
|---------|--------|----------------|-------|
| action.svg | partial | `views/action-flow.ts` | `renderer.test.ts`, `activity_graph_semantics.rs` |
| action-flow-compartment.svg | partial | perform/parameters in `drawActionNode` | `renderer.test.ts` |
| succession.svg | partial | `aflow-succession` edge class for flow/first guards | `prepare.ts`, `action-flow.ts` |
| decision.svg / merge.svg | partial | `state_type` on `ActivityStateDto` from AST merge | `extracted_model` tests |
| fork.svg / join.svg | WONTFIX (inventory) | — | — |

**Graph-first path:** `extract_activity_diagrams` + `enrich_activity_diagrams_from_graph` (then-action, bind, perform, in/out parameters).

**Open gaps:** fork/join/decision full BNF; `ForLoop` / `Assign` topology; validation 3a fixture end-to-end snapshot.

## Conformance metadata

`docs/reference/conformance-metadata.json` marks both views **partial** until rows above reach **shared** with regression coverage.
