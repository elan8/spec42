# Third-party notices

## SysML v2 standard library (`sysml.library`)

Spec42 embeds the textual normative SysML/KerML standard library from the
[SysML v2 Release](https://github.com/Systems-Modeling/SysML-v2-Release) repository
(pinned release tag in `config/standard-library.json`), under the terms described in that repository’s
`LICENSE` (GNU Lesser General Public License v3.0 / related notices).

The embedded artifact contains only the `sysml.library/` tree from that release, repacked at build time.

## ELK.js (`elkjs`)

Spec42 vendors ELK.js 0.11.1 assets under `crates/server/assets/elkjs/` for
headless Rust-owned diagram export. The exporter embeds QuickJS to execute ELK
layout and routing without requiring Node.js, npm, or VS Code webview assets at
export time.

ELK.js is distributed under the Eclipse Public License 2.0.
See https://github.com/kieler/elkjs and https://www.eclipse.org/legal/epl-2.0/.

## elkrs (feature-gated native diagram layout)

The `crates/diagram_layout` native layout boundary and `tools/elkrs_parity` development tool depend
on the public `elan8/elkrs` repository at revision
`8309be8cf614cfe277c572b28e4f79a1703f8e32`. The server dependency is feature-gated and is not
linked into the default Spec42 product or extension packages.

`elkrs` 0.1.1 is distributed under the Apache License 2.0. The public repository records that its
history was recovered from the crates.io package with SHA-256
`a0aa6d17007599c4bb42b342b55148832289bc8c7e41d83f01b19af1ef363de4`; its ELK 0.11.0 oracle,
tools, and golden corpus were subsequently rebuilt independently. Apache-2.0 is suitable for use by
this MIT-licensed project provided its license and attribution notices accompany any distributed
binary that incorporates it. A production integration must add the license text to packaged notices
and verify the final artifact contents.
