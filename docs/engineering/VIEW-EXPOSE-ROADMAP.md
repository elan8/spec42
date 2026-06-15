# View expose and General View rendering

Implementation notes for SysML v2-conformant view `expose` resolution (§7.6.6, §7.26.2) and view rendering resolution (§7.26.2, §9.2.19).

## Status (Spec42 0.31.0+)

- **Done:** Semantic expose resolution via [`reference_resolution::resolve_expose_target`](../../crates/semantic_core/src/semantic/reference_resolution.rs) (feature chains, `::**` / `::*`, typing-aware recursive closure).
- **Done:** `evaluate_views` uses `SemanticGraph` + DTO projection (`expand_structural_scope` for inherited definition members).
- **Done:** Parser support for `variation part` usages and `variant` members in part usage bodies (`sysml-v2-parser`).
- **Done:** Diagnostics `view_expose_unresolved`, `view_expose_empty_result` (catalog); `view_expose_empty` unchanged.
- **Done:** View rendering resolution in [`explicit_views.rs`](../../crates/semantic_core/src/semantic/explicit_views.rs): `render asInterconnectionDiagram`, `asTreeDiagram`, `asElementTable`, and `asTextualNotation` map to supported renderers; inherited rendering from `view def`; `GeneralView` fallback for untyped views without `render`.
- **Regression:** `view_expose_*` integration tests with inline grid fixtures; optional ignored drill-down when `SYSML_POWERSYSTEMS_DIR` is set.
- **Regression:** `view_rendering_resolution` integration tests for rendering-only views, explicit-type precedence, and view-def rendering inheritance.

## Rendering vs standard view definitions

Per SysML v2 §7.26.2, a view artifact is produced by **expose → filter → render**. The `Views` library renderings (`asInterconnectionDiagram`, …) specify *how* to draw; `StandardViewDefinitions` (`InterconnectionView`, …) specify *which* element kinds are valid for normative diagram types.

Spec42 resolves effective renderer selection in this order:

1. Explicit standard view type on the usage (`: InterconnectionView`, …) when not typed by a local `view def`
2. `render` on the view usage
3. `render` inherited from the referenced local `view def`
4. `GeneralView` fallback

**Content inference is not spec-conformant:** Spec42 does not infer `ActionFlowView` or `StateTransitionView` from exposed element kinds. For behavior-specific diagrams, type views explicitly (`: ActionFlowView`, `: StateTransitionView`) in the model.

## Reference model

External grid fixture brief (maintained outside spec42): `spec42-view-expose-fixes.md` in the grid fixture repository.

## Key files

| Area | File |
|------|------|
| Expose resolver | `crates/semantic_core/src/semantic/reference_resolution.rs` |
| View evaluation | `crates/semantic_core/src/semantic/explicit_views.rs` |
| View diagnostics | `crates/semantic_core/src/semantic/diagnostics/checks/view_metadata_conformance.rs` |
| Parser (`variation part`) | `sysml-v2-parser/src/parser/part/usage.rs` |

## Acceptance checks

```powershell
# From spec42 repo (CI uses inline fixtures; optional ignored drill-down with SYSML_POWERSYSTEMS_DIR)
cargo test -p semantic_core --test view_expose_powersystems_shaped
cargo test -p semantic_core --test view_expose_powersystems_project_body
cargo test -p semantic_core --test view_expose_inherited_parts
cargo test -p semantic_core --test view_rendering_resolution

# Optional robot-vacuum showcase (requires SYSML_ROBOT_VACUUM_DIR)
cargo test -p kernel --test lsp_integration robot_vacuum_showcase_model_views_are_supported -- --ignored
```
