# Library & Dependencies

## Standard Library

The SysML v2 standard library is bundled with Spec42 — no separate installation needed. It provides base definitions like `Part`, `AttributeValue`, `ScalarValue`, SI units, and more.

You can browse the library using the **Lookup** search box in the **Library** section of the Spec42 sidebar.

## Domain Libraries

Elan8 domain libraries extend the standard library with domain-specific definitions (e.g., electrical, mechanical). These are also bundled and available automatically.

## Custom Libraries

You can add your own library paths in the **Library** section:

1. Click the **Manage custom library paths** button (gear icon) next to **Custom Libraries**.
2. Add the folder path containing your `.sysml` or `.kerml` library files.
3. The language server restarts and indexes the new library.

## Sysand Dependencies

[Sysand](https://github.com/sensmetry/sysand) is an optional package manager for SysML v2 dependencies. If your workspace has a `sysand.toml` manifest, Spec42 will detect it and show the **Sysand** section in the Library panel.

- If Sysand is not installed, a link to the installation instructions is shown.
- Click **Refresh** to re-scan dependencies after installing or updating packages.
