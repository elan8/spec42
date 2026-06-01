# General and interconnection view — BNF sign-off checklist

Generated: 2026-06-01

Normative figures: `SysML-v2-Release/bnf/images/`. Implementation: [`shared/diagram-renderer`](../shared/diagram-renderer).

**Shipped-element coverage:** 35 / 104 primary notations marked **shared** (34%). Compartment-only rows count as partial notation.

| BNF SVG | View(s) | Status | Implementation | Tests |
|---------|---------|--------|----------------|-------|
| action-def.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` | `shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs` |
| action-flow-compartment.svg | general-view, interconnection-view (compartment text) | shared (action-flow-view) | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| action.svg | general-view, action-flow-view | shared | `views/action-flow.ts` | `shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs` |
| actions-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| actors-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| allocations-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| analyses-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| assert-constraints-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| assume-constraints-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| attributes-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| binary-dependency.svg | general-view | shared | `renderer.ts` `applyEdgeMarker` dependency | `shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs` |
| binding-connection.svg | interconnection-view | shared | `renderer.ts` `applyEdgeMarker` bind branch | `shared/diagram-renderer/src/renderer.test.ts` |
| calcs-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| concerns-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| connection.svg | interconnection-view | shared | `renderer.ts` `applyEdgeMarker` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| connections-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| constraints-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| definition.svg | general-view | shared | `node-notation.ts` definition chrome | `shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs` |
| directed-features-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| documentation-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| element-inside-textual-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| ends-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| enums-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| exhibit-states-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| exposes-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| extended-usage.svg | general-view | shared | `node-notation.ts` usage chrome | `shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs` |
| features-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| filters-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| flow-on-connection.svg | interconnection-view | shared | `renderer.ts` `applyEdgeMarker` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| flow.svg | interconnection-view | shared | `renderer.ts` `applyEdgeMarker` flow branch | `shared/diagram-renderer/src/renderer.test.ts` |
| flows-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| frames-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| general-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| include-use-cases-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| includes-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| individuals-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| interconnection-compartment.svg | interconnection-view | shared (compartment text only) | `renderer.ts` (IBD) | — |
| interface-connection.svg | interconnection-view | shared | `renderer.ts` `applyEdgeMarker` interface branch | `shared/diagram-renderer/src/renderer.test.ts` |
| interface.svg | interconnection-view | shared | `renderer.ts` `applyEdgeMarker` interface branch | `shared/diagram-renderer/src/renderer.test.ts` |
| interfaces-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| items-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| members-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| metadata-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| n-ary-dependency-client-link.svg | general-view | WONTFIX (hub-and-spoke binary edges) | WONTFIX (hub-and-spoke binary edges) | — |
| n-ary-dependency-supplier-link.svg | general-view | WONTFIX (hub-and-spoke binary edges) | WONTFIX (hub-and-spoke binary edges) | — |
| objective-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| occurrences-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| package-with-name-inside.svg | general-view | shared | `renderer.ts` `drawGeneralPackageContainers` | `shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs` |
| packages-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| parameters-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| part-def.svg | general-view | shared | `node-notation.ts` `resolveNodeChrome` | `shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs` |
| part-ref.svg | general-view, interconnection-view | shared | `node-notation.ts` `isReferenceKind` | `shared/diagram-renderer/src/renderer.test.ts` |
| part.svg | general-view, interconnection-view | shared | `node-notation.ts`, `sysml-node-builder.ts` (general); `renderer.ts` `renderIbdNode` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| parts-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| perform-actions-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| performed-by-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| port-b-1.svg | interconnection-view | shared | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| port-b-2.svg | interconnection-view | shared | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| port-b-3.svg | interconnection-view | shared | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| port-def.svg | general-view | shared | `sysml-node-builder.ts` compartments (general) | `shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs` |
| port-l-1.svg | interconnection-view | shared | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| port-l-2.svg | interconnection-view | shared | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| port-l-3.svg | interconnection-view | shared | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| port-r-1.svg | interconnection-view | shared | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| port-r-2.svg | interconnection-view | shared | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| port-r-3.svg | interconnection-view | shared | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| port-t-1.svg | interconnection-view | shared | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| port-t-2.svg | interconnection-view | shared | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| port-t-3.svg | interconnection-view | shared | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| port-usage.svg | interconnection-view | shared | `renderer.ts` `drawIbdPorts` | `shared/diagram-renderer/src/renderer.test.ts` |
| portion-relationship.svg | general-view | shared | `renderer.ts` `applyEdgeMarker` (general) | `shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs` |
| ports-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| redefinition.svg | general-view | shared | `renderer.ts` `applyEdgeMarker` (general) | `shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs` |
| relationships-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| rendering-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| require-constraints-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| requirement-def.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` | `shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs` |
| requirement.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` | `shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs` |
| requirements-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| result-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| satisfies-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| satisfy-requirements-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| sequence-compartment.svg | general-view, interconnection-view (compartment text) | legacy|shared (sequence-view) | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| snapshots-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| stakeholders-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| state-actions-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| state-def.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` | `shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs` |
| state-transition-compartment.svg | general-view, interconnection-view (compartment text) | shared (state-transition-view) | `renderer.ts` (IBD) | `shared/diagram-renderer/src/renderer.test.ts` |
| state.svg | general-view, state-transition-view | shared | `views/state-transition.ts` | `shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs` |
| states-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| subject-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| successions-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| timeslices-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| use-cases-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| variant-elementusages-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| variants-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| verification-methods-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| verifications-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| verifies-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| view-frame-info-compartment-bl.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| view-frame-info-compartment-br.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| view-frame-info-compartment-tr.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| viewpoints-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |
| views-compartment.svg | general-view, interconnection-view (compartment text) | shared (compartment text only) | `renderer.ts` (IBD) | — |

## Automated regression

- `cd shared/diagram-renderer && npm test` — edge markers, def/usage/ref chrome, IBD ports/connectors
- `cargo test -p kernel --test integration model` — visualization payloads

## Manual validation fixtures

- General: `sysml/src/validation/.../01-Parts Tree/1d-Parts Tree with Reference.sysml`
- Interconnection: KitchenTimer workspace fixture; connected-blocks / webshop models
