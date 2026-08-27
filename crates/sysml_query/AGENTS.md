# sysml_query testing rules

- Observable parser, semantic-construction, resolution, navigation, diagnostics, ordering,
  completeness, and sequential/parallel behavior belongs in `tests/snapshots` and must be exercised
  by the standalone `spec42-snapshot` tool.
- Do not construct semantic models in `crates/sysml_query/tests` to duplicate pipeline behavior with
  hand-authored cursor positions or expected candidate lists. Facade tests cover only public API
  shape, opacity, dependency boundaries, and compile-time contracts.
- When a new typed query result is not representable in snapshot output, extend the owner-defined
  snapshot projection and runner first, then add the fixture. Do not substitute an ad hoc facade
  integration test.
