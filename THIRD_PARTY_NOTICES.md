# Third-party notices

## SysML v2 standard library (`sysml.library`)

Spec42 embeds the textual normative SysML/KerML standard library from the
[SysML v2 Release](https://github.com/Systems-Modeling/SysML-v2-Release) repository
(pinned release tag in `config/standard-library.json`), under the terms described in that repository’s
`LICENSE` (GNU Lesser General Public License v3.0 / related notices).

The embedded artifact contains only the `sysml.library/` tree from that release, repacked at build time.

## elkrs (native diagram layout)

The `crates/diagram_layout` native layout boundary, headless export through
`crates/diagram_draw`, and `crates/lsp_server`'s `spec42/layout` and `spec42/draw`
requests (#119, #176)
depend on the public `elan8/elkrs` repository at revision
`84f95ae55688fe1e0d269d3a5080edf61b218c87`.

As of #119, `diagram_layout` is a plain (non-feature-gated) `lsp_server` dependency. `crates/server`
(the single `spec42` binary that is both the CLI/MCP host and, via its own plain, non-optional
dependency on `lsp_server`, the language server the VS Code extension bundles) normally depends on
`lsp_server` to launch it, so `elkrs` is linked into every default build of `spec42` -- confirmed
via `cargo tree -p server -i elkrs`, which shows `elkrs -> diagram_layout -> lsp_server -> server`
with no feature gate or dev-dependency edge on that path.

`elkrs` 0.1.1 is distributed under the Apache License 2.0. The public repository records that its
history was recovered from the crates.io package with SHA-256
`a0aa6d17007599c4bb42b342b55148832289bc8c7e41d83f01b19af1ef363de4`; its ELK 0.11.0 oracle,
tools, and golden corpus were subsequently rebuilt independently. Apache-2.0 is suitable for use by
this MIT-licensed project provided its license and attribution notices accompany any distributed
binary that incorporates it. This notice is that attribution for the repository/source
distribution; no automated check yet verifies a packaged VSIX itself carries it -- that is a
follow-up, not something #119 adds speculatively.
