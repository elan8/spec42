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
