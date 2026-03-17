# SysML Language Server 1.0 Release Checklist

Use this checklist before publishing a `1.0` release candidate or final release.

## Server Reliability

- [ ] No known crash-on-edit bugs remain.
- [ ] Reproduced crashes have regression tests.
- [ ] Production runtime paths have been audited for panic-prone assumptions.
- [ ] Invalid intermediate edits degrade gracefully with diagnostics or safe fallback behavior.

## VS Code Extension

- [ ] Startup failures are visible and actionable.
- [ ] Repeated crash/restart behavior is capped and user-visible.
- [ ] Status bar states are understandable during startup, indexing, degraded mode, and crash recovery.
- [ ] Workspace indexing limits are visible to users.

## Tests and CI

- [ ] `cargo test -p spec42 -- --nocapture` passes.
- [ ] `npm.cmd run compile` passes.
- [ ] `npm.cmd test` passes.
- [ ] `npm.cmd run test:multi-file` passes.
- [ ] `npm.cmd run test:workspace-smoke` passes.
- [ ] CI runs Rust tests, extension tests, workspace smoke tests, and packaging checks on supported platforms.

## Packaging

- [ ] Bundled server binaries are present for intended release targets.
- [ ] `npm run build:webview-assets` passes.
- [ ] `npm run build:webview` passes.
- [ ] `npm run package` produces a valid `.vsix`.

## Documentation

- [ ] Supported workflows are documented.
- [ ] Experimental areas are documented.
- [ ] Troubleshooting guidance is present and current.
- [ ] Release notes match the actual support level and known limitations.

## Final Sanity Checks

- [ ] Open a real SysML workspace in VS Code and verify core workflows manually:
  hover, definition, rename, diagnostics, formatting, semantic tokens.
- [ ] Verify a large workspace still remains usable when indexing is truncated.
- [ ] Confirm no high-priority open issues remain for the core edit loop.
