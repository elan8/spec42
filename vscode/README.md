<p align="center">
  <img src="https://raw.githubusercontent.com/elan8/spec42/HEAD/vscode/media/screenshot.png" alt="Spec42 SysML v2 Editor Screenshot" />
</p>

# Spec42 for VS Code

Spec42 is open, local-first tooling for [SysML v2](https://www.omg.org/sysml/sysmlv2/) and KerML: edit textual models with live feedback, understand structure and behavior beyond the source, and keep the same analysis engine in the editor, CI, and assistant workflows.

This extension is the primary modeling surface. Marketplace builds include the matching language server, standard library, and Elan8 domain/method libraries — install, open a `.sysml` or `.kerml` file, and start modeling.

## Why Spec42 in the editor

Textual SysML v2 is only as useful as the feedback around it. Spec42 focuses on three things that matter while you work:

- **Confidence while editing** — diagnostics, navigation, and completion that stay with you as the model grows across files.
- **Understanding beyond the source** — Feature Inspector shows resolved semantics, not just tokens.
- **Continuity outside the IDE** — the same engine powers CLI checks, GitHub Actions, and assistant tools, so editor results and automation stay aligned.

Everything runs locally. Models stay in your workspace; Spec42 does not require a cloud modeling runtime.

## What you get

- Live diagnostics, semantic highlighting, completion, hover, snippets, and formatting.
- Definitions, references, rename, document symbols, and hierarchy navigation.
- Workspace-aware indexing for multi-file models and configured libraries.
- **Feature Inspector** for resolved typing, inheritance, relationships, values, and keyword help.
- Bundled SysML standard library plus searchable Elan8 domain and method libraries.
- A local **Open Diagram** command backed by the packaged Rust WASM generator and D3/ELK renderer.
  Its picker is filtered by the standard view usages authored in the active file. State-transition
  projections are implemented; the other declared views currently identify their missing typed
  query explicitly in the diagram panel.
- Built-in Copilot Language Model Tools for check, doctor, model summary, and diagnostic explanation.

## Views

| View | What it is for |
| --- | --- |
| General View | Structure and relationships across the exposed model. |
| Interconnection View | Parts, ports, connectors, and architecture wiring. |
| Action Flow View | Control and data flow through actions. |
| State Transition View | States and transitions for lifecycle behavior. |
| Sequence View | Lifelines and messages for interaction-oriented models. |
| Browser View | Hierarchical membership browsing. |
| Grid View | Tabular arrangement of exposed elements and relationships. |
| Geometry View | Spatial items (partial while authored geometry and 3D catch up). |

## Get started

1. Install **SysML v2 Editor** from the Visual Studio Marketplace.
2. Open a `.sysml` or `.kerml` file — or pick the starred **timer** example from the Spec42 sidebar.
3. Use navigation, diagnostics, **Feature Inspector**, and **Open Diagram** as you explore.

Full walkthrough: **[Getting Started](https://elan8.github.io/spec42/guide/getting-started)**.

For CLI checks, CI, environment diagnostics, and other AI hosts, see the [main Spec42 README](https://github.com/elan8/spec42#readme) and the docs linked below. Editor and automation share the same analysis engine.

## Learn more

- [User documentation](https://elan8.github.io/spec42/)
- [Feature Inspector guide](https://elan8.github.io/spec42/guide/feature-inspector)
- [Libraries](https://elan8.github.io/spec42/guide/libraries)
- [AI assistants](https://github.com/elan8/spec42/blob/HEAD/docs/user/AI-ASSISTANTS.md)
- [Troubleshooting](https://github.com/elan8/spec42/blob/HEAD/docs/user/TROUBLESHOOTING.md)
- [Source and releases](https://github.com/elan8/spec42)
- [Issues](https://github.com/elan8/spec42/issues)

Settings, advanced configuration, and contributor notes live in the docs site and repository development guide rather than this Marketplace overview.
