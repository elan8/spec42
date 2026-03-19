# `org.eclipse.elk.core` Pragmatic Parity Contract

This document defines the in-scope parity behavior for `org.eclipse.elk.core` in `elk-rust`.

## In Scope

- Preserve ELK option compatibility for headless JSON workflows:
  - Alias and canonical key resolution for known options.
  - Deprecated option key replacement to canonical keys.
  - Case-insensitive option key matching with deterministic canonicalization.
- Validate options against metadata:
  - Unknown key diagnostics.
  - Wrong-type diagnostics.
  - Disallowed-scope diagnostics.
- Keep strict algorithm dispatch behavior:
  - Missing algorithm option is an error.
  - Unknown algorithm id is an error.
- Run a deterministic pre-dispatch option pipeline:
  - Normalize option keys.
  - Validate normalized options.
  - Dispatch using canonicalized values.

## Non-Goals

- Eclipse extension-point mechanics from OSGi plugins.
- UI-specific integrations (`*.ui`, IDE integrations).
- EMF/XMI persistence compatibility.
- Full one-to-one port of every Java utility/helper class.

## Design Rule

Use ELK-compatible external contracts (option ids, aliases, validation semantics), but keep internals Rust-native and explicit (crate APIs, typed helpers, and direct registration).
