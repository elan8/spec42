# Component implementation context roadmap

Implementation roadmap for exposing resolved usage context from Spec42 so downstream tools can render component implementation details without reconstructing SysML semantics from a flat graph.

## Problem

Component-focused clients such as Babel42 need to support the workflow where a user selects a usage, for example:

```sysml
part cleaningHead : CleaningHead;
```

The selected element is the usage (`cleaningHead`), but the implementation-relevant contract usually lives on the resolved definition (`CleaningHead`). A usable Components tab should therefore keep the usage selected while showing the effective context from the resolved definition: subparts, ports, interfaces, item/data contracts, behavior hooks, constraints, requirements links, and source navigation.

Today, consumers can see view-oriented fields such as `partType`, `portType`, `type`, `typing`, `generalViewDirectParts`, `generalViewDirectPorts`, `generalViewDirectAttributes`, and relationship edges. Those fields are useful for rendering, but they do not form a stable semantic API for implementation context. Clients should not need to infer SysML typing, specialization, inheritance, or redefinition semantics from generic graph shape.

## Relationship to IBD/interconnection

This roadmap is closely related to the IBD/interconnection pipeline because both are usage-centric. The interconnection pipeline answers a rendering question: which concrete part usages, ports, containers, and connector endpoints should appear in an Interconnection View scene. `resolvedUsageContext` answers an inspection question: what effective definition and member contract belongs to the selected usage.

Both contracts should reuse the same backend typing, instance-path, ownership, and relationship-resolution primitives. They should not grow separate name-based or frontend-side resolution paths. The boundary is that `InterconnectionSceneDto` remains a diagram scene contract, while `resolvedUsageContext` is a semantic inspection contract for component implementation workflows.

## Target contract

Spec42 should add a stable `resolvedUsageContext` contract for usage-like nodes. This contract is additive and does not replace existing General View or renderer-oriented fields.

Usage-like elements include at least:

- `part`
- `port`
- `item`
- `attribute`
- `action`
- other occurrence or feature usages where a textual type resolves to a semantic definition node

Every usage-like node with a resolved type should expose canonical `typedBy` metadata:

```json
{
  "typedBy": {
    "id": "...",
    "qualifiedName": "PhysicalArchitecture::CleaningHead",
    "name": "CleaningHead",
    "kind": "part def",
    "source": {
      "uri": "...",
      "line": 42
    }
  }
}
```

The selected usage should also expose `resolvedUsageContext`:

```json
{
  "resolvedUsageContext": {
    "resolvedDefinition": {
      "id": "...",
      "qualifiedName": "PhysicalArchitecture::CleaningHead",
      "name": "CleaningHead",
      "kind": "part def",
      "source": {
        "uri": "...",
        "line": 42
      }
    },
    "parts": [],
    "ports": [],
    "interfaces": [],
    "items": [],
    "attributes": [],
    "behaviors": [],
    "requirements": []
  }
}
```

Each context item should use one common shape:

```json
{
  "id": "...",
  "qualifiedName": "...",
  "name": "...",
  "kind": "...",
  "displayText": "brushMotor : BrushMotor",
  "origin": "direct",
  "source": {
    "uri": "...",
    "line": 123
  }
}
```

Valid `origin` values are:

- `usage`
- `direct`
- `inherited`
- `redefined`
- `relationship`

## Roadmap

### Phase 1: Canonical typing references

- Add canonical `typedBy` metadata for usage-like nodes that currently expose textual type hints through fields such as `partType`, `portType`, `type`, or `typing`.
- Resolve `typedBy` to the actual semantic node where available, not only the textual type name.
- Keep existing textual aliases for rendering and compatibility.
- Add regression coverage proving that `part cleaningHead : CleaningHead;` emits `typedBy.qualifiedName`.

### Phase 2: Direct resolved usage context

- Add `resolvedUsageContext.resolvedDefinition` to selected usage nodes with a resolved definition.
- Populate direct members from the resolved definition into stable buckets: `parts`, `ports`, `interfaces`, `items`, `attributes`, `behaviors`, and `requirements`.
- Preserve source locations for both the usage and the resolved definition so clients can offer source navigation to either declaration.
- Ensure every context item has `id`, `qualifiedName`, `name`, `kind`, `displayText`, `origin`, and `source`.

### Phase 3: Inheritance and redefinition

- Include members inherited through specialization of the resolved definition.
- Distinguish direct, inherited, and redefined members with explicit `origin` values.
- For redefined members, include enough reference data for clients to navigate to both the effective member and the redefined member.
- Avoid name-based merging in consumers; Spec42 should compute the effective member set.

### Phase 4: Relationship-backed implementation context

- Add relationship-derived context for `satisfy`, `subject`, relevant interface ends, connection relationships, and behavior hooks.
- Represent relationship-derived items with `origin: "relationship"`.
- Include enough metadata for clients to render and navigate relationship-backed context without additional graph inference.

### Phase 5: Contract hardening and client adoption

- Document `resolvedUsageContext` as a stable projection contract.
- Keep `generalViewDirect*`, `generalViewInherited*`, `partType`, `portType`, `type`, and `typing` available for existing renderers and clients.
- Update API and client-facing documentation once the shape is stable.
- Update Babel42 to prefer `resolvedUsageContext` for the Components tab and fall back to older fields only for compatibility.

## Key implementation areas

| Area | Files |
|------|-------|
| Usage typing emission | `crates/sysml_model/src/semantic/graph_builder/part_usage.rs`, `crates/sysml_model/src/semantic/graph_builder/occurrence_body.rs`, related usage builders |
| Definition member projection | `crates/sysml_model/src/semantic/model_projection.rs`, `crates/sysml_model/src/semantic/component_view.rs` |
| Reference resolution | `crates/sysml_model/src/semantic/reference_resolution.rs`, `crates/sysml_model/src/semantic/resolution/` |
| Relationships | `crates/sysml_model/src/semantic/relationships.rs` |
| API and LSP model output | `crates/lsp_server/src/views/model.rs`, `crates/lsp_server/tests/integration/model.rs` |
| Robot vacuum fixtures | `tests/fixtures/robot_vacuum_fixture.rs`, `crates/workspace/tests/robot_vacuum_snapshot.rs`, robot-vacuum integration fixtures |

## Acceptance criteria

The robot-vacuum fixture must include a regression where a usage is typed by an implementation-rich definition:

```sysml
part cleaningHead : CleaningHead;

part def CleaningHead {
  // implementation-relevant members
}
```

Minimum checks:

- `cleaningHead` has `typedBy.qualifiedName == "PhysicalArchitecture::CleaningHead"`.
- `cleaningHead.resolvedUsageContext.resolvedDefinition` points to `CleaningHead`.
- Direct subparts, ports, attributes, items, interfaces, behaviors, and requirements from `CleaningHead` appear in `resolvedUsageContext`.
- Inherited members are present and distinguishable from direct members.
- Redefined members are distinguishable and retain references to the effective and redefined declarations.
- Source references identify both the selected usage location and the resolved definition location.
- A client can render the Components tab without name-based lookup or frontend semantic reconstruction.

## Test plan

- Add focused `sysml_model` tests for canonical `typedBy` emission on `part`, `port`, `item`, `attribute`, and action-like usages.
- Add projection tests for `resolvedUsageContext` direct members, inherited members, and redefined members.
- Add relationship context tests for satisfy, subject, interface, and connection-backed context once Phase 4 begins.
- Add LSP/model integration checks to prove the JSON contract is visible through the model view consumed by clients.
- Add or extend a robot-vacuum regression so selecting `cleaningHead` exposes the effective `CleaningHead` implementation context.

## Compatibility notes

- `resolvedUsageContext` is additive.
- Existing `generalView*`, `partType`, `portType`, `type`, and `typing` fields should remain available for renderers and backward-compatible clients.
- Existing diagrams should not change visual output because of this contract.
- Consumers should treat `typedBy` and `resolvedUsageContext` as the preferred semantic contract once available.
