# View expose and General View rendering

Implementation notes for SysML v2-conformant view `expose` resolution (§7.6.6, §7.26.2).

## Status (Spec42 0.30.0+)

- **Done:** Semantic expose resolution via [`reference_resolution::resolve_expose_target`](../../crates/semantic_core/src/semantic/reference_resolution.rs) (feature chains, `::**` / `::*`, typing-aware recursive closure).
- **Done:** `evaluate_views` uses `SemanticGraph` + DTO projection (`expand_structural_scope` for inherited definition members).
- **Done:** Parser support for `variation part` usages and `variant` members in part usage bodies (`sysml-v2-parser`).
- **Done:** Diagnostics `view_expose_unresolved`, `view_expose_empty_result` (catalog); `view_expose_empty` unchanged.
- **Regression:** `view_expose_*` integration tests with inline grid fixtures; optional ignored drill-down when `SYSML_POWERSYSTEMS_DIR` is set.

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
```
