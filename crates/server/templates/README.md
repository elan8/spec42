# Embedded starter workspace design

`spec42 init <directory>` embeds the focused starter workspace in `init/general/`.
The files are embedded with the CLI so a release can scaffold a workspace without
network access or a separate project-template installation.

The template contains:

- `README.md` with the `spec42 check model --workspace-root .` command;
- a root system definition and baseline configuration;
- basic requirements; and
- a small domain-type library.

The scaffold deliberately has no project manifest: Spec42's workspace contract is
the model directory itself. It also remains separate from the repository's curated
examples catalogue, which serves a different tutorial and demonstration purpose.
