# Action flow and state transition view — BNF sign-off checklist

Generated: 2026-06-09  
Updated: 2026-06-15

Normative figures: `SysML-v2-Release/bnf/images/`. Implementation: [`shared/diagram-renderer`](../../shared/diagram-renderer). Server projection: [`state_views`](../../crates/semantic_core/src/semantic/state_views/) and [`extracted_model`](../../crates/semantic_core/src/semantic/extracted_model.rs).

## State transition view

| BNF SVG | Status | Implementation | Tests |
|---------|--------|----------------|-------|
| state.svg | shared | `views/state-transition.ts` | `shared/diagram-renderer/src/renderer.test.ts`, `crates/semantic_core/tests/state_machine_semantics.rs`, `crates/kernel/tests/integration/model.rs` |
| state-transition-compartment.svg | shared | `views/state-transition.ts` entry/do/exit compartments | `shared/diagram-renderer/src/prepare.test.ts`, `renderer.test.ts` |
| transition.svg | shared | `StateMachineDto` transitions with guard/effect/accept/send | `state_views/graph_extractor.rs`, `state_machine_semantics.rs` |

**Graph-first path:** `build_workspace_state_machines` → `SysmlVisualizationResultDto.stateMachines` → `dtoAdapter.buildSharedRendererInput` → shared `prepareState` in `prepare/behavior.ts`.

**Covered:** composite regions via `RegionDto` on `StateMachineDto`; terminate (cross) vs final (double circle); transition labels `[guard] / effect / accept / send`.

## Action flow view

| BNF SVG | Status | Implementation | Tests |
|---------|--------|----------------|-------|
| action.svg | shared | `views/action-flow.ts` | `renderer.test.ts`, `activity_graph_semantics.rs` |
| action-flow-compartment.svg | shared | perform/parameters in `drawActionNode` | `renderer.test.ts` |
| succession.svg | shared | `aflow-succession` + `aflow-conditional` edge classes | `prepare/behavior.ts`, `action-flow.ts`, `renderer.test.ts` |
| decision.svg / merge.svg | shared | `ActivityStateDto` + `DecisionNodeDto`; graph enrichment in `activity_graph.rs` | `extracted_model` tests, `activity_graph_semantics.rs` |
| fork.svg / join.svg | WONTFIX (inventory) | — | — |

**Graph-first path:** `extract_activity_diagrams` + `enrich_activity_diagrams_from_graph` (then-action, bind, perform, assign, for-loop, decision/merge control nodes, in/out parameters).

**Covered:** `Assign` / `ForLoop` topology; decision via typed action usage (`: Decision`); conditional succession labels on edges; swim lanes only when >1 lane.

## Conformance metadata

`docs/reference/conformance-metadata.json` marks both views **complete** (fork/join remain WONTFIX inventory).
