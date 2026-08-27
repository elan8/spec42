# Embedded starter workspace design

`spec42 init <directory>` embeds the focused starter workspace in `init/general/`.
The files are embedded with the CLI so a release can scaffold a workspace without
network access or a separate project-template installation.

The template contains:

- `README.md` with the `spec42 check model --workspace-root .` command;
- a root system definition and baseline configuration;
- basic requirements; and
- a small domain-type library; and
- a root `.project.json` manifest suitable for workspace discovery and bundling.

The generated manifest pins the exact standard-library KPAR resources and versions resolved by the
current Spec42 installation. Bundling preserves those usages; another installation must provide
compatible local libraries when it opens the project.

For a non-empty model directory, `spec42 init` preserves the existing model and adds only a missing
`.project.json`. An existing manifest is authoritative and is never overwritten. The starter remains
separate from the repository's curated examples catalogue, which serves a different tutorial and
demonstration purpose.
