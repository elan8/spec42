# SysML notation inventory (generated)

Generated: 2026-08-18

Source: `/Users/luke/Documents/GitHub/spec42/third_party/sysml-v2-release/bnf/images` (34 entries)

Shipped product views: **general-view**, **interconnection-view** (+ behavior views).

| SVG | Inferred views | Status | Code pointer |
|-----|----------------|--------|--------------|
| part-def.svg | general-view | shared | `node-notation.ts` `resolveNodeChrome` |
| part.svg | general-view, interconnection-view | shared | `node-notation.ts`, `sysml-node-builder.ts` (general); `renderer.ts` `renderIbdNode` (IBD) |
| part-ref.svg | general-view, interconnection-view | shared | `node-notation.ts` `isReferenceKind` |
| port-def.svg | general-view | shared | `sysml-node-builder.ts` compartments (general) |
| port.svg | general-view, interconnection-view | shared | `sysml-node-builder.ts` (general); `renderer.ts` `drawIbdPorts` (IBD) |
| port-usage.svg | interconnection-view | shared | `renderer.ts` `drawIbdPorts` |
| port-l-1.svg | interconnection-view | shared | `renderer.ts` (IBD) |
| port-r-1.svg | interconnection-view | shared | `renderer.ts` (IBD) |
| action-def.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` |
| action.svg | general-view, action-flow-view | shared | `views/action-flow.ts` |
| state-def.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` |
| state.svg | general-view, state-transition-view | shared | `views/state-transition.ts` |
| requirement-def.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` |
| requirement.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` |
| connection.svg | interconnection-view | shared | `renderer.ts` `applyEdgeMarker` (IBD) |
| binding-connection.svg | interconnection-view | shared | `renderer.ts` `applyEdgeMarker` bind branch |
| flow.svg | interconnection-view | shared | `renderer.ts` `applyEdgeMarker` flow branch |
| interface.svg | interconnection-view | shared | `renderer.ts` `applyEdgeMarker` interface branch |
| interface-connection.svg | interconnection-view | shared | `renderer.ts` `applyEdgeMarker` interface branch |
| binary-dependency.svg | general-view | shared | `renderer.ts` `applyEdgeMarker` dependency |
| definition.svg | general-view | shared | `node-notation.ts` definition chrome |
| extended-usage.svg | general-view | shared | `node-notation.ts` usage chrome |
| specializes.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` |
| typing.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` |
| hierarchy.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` |
| composition.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` |
| allocate.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` |
| satisfy.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` |
| verify.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` |
| bind.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` |
| dependency.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` |
| usage.svg | general-view | shared | `node-notation.ts` / `sysml-node-builder.ts` |
| redefinition.svg | general-view | shared | `renderer.ts` `applyEdgeMarker` (general) |
| package-with-name-inside.svg | general-view | shared | `renderer.ts` `drawGeneralPackageContainers` |

Regenerate:

```powershell
$env:SYSML_V2_RELEASE_DIR = 'C:\path\to\SysML-v2-Release'
node scripts/generate-notation-inventory.mjs
```
