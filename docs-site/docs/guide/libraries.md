# Library & Dependencies

Spec42 bundles the SysML v2 standard library plus Elan8 domain and method libraries. For exact release pins, see [What's included](/reference/whats-included).

## Standard Library

The SysML v2 standard library is bundled with Spec42 — no separate installation needed. It provides base definitions like `Part`, `AttributeValue`, `ScalarValue`, SI units, and more.

The **Library** view in the Spec42 sidebar shows a dashboard for bundled libraries, configured custom paths, and Sysand dependency status.

You can:

- Search for symbols by name, package, or type.
- Browse indexed library content when you do not know the exact name yet.
- Open a symbol definition directly in the editor.
- Copy a qualified name or import statement from a result.
- Open details for the standard library and bundled domain / method libraries.

## Domain Libraries

Elan8 domain libraries extend the standard library with reusable vocabulary for domain and technical modeling (for example robotics, software, electronics, and communication). They are bundled and available automatically.

See the [Domain libraries](/reference/domain-libraries) reference for the bundled version and library families.

## Method Libraries

Elan8 Method libraries provide SysML packages for requirements metadata, concerns, and viewpoints. They are bundled separately from domain vocabulary.

See the [Method libraries](/reference/method-libraries) reference for the package list and version.

## Custom Libraries

You can add your own library paths in the **Library** section:

1. Click the **Manage custom library paths** button (gear icon) in the **Library** view.
2. Add the folder path containing your `.sysml` or `.kerml` library files.
3. The language server restarts and indexes the new library.

If a configured path is missing, the Library dashboard shows it as a warning so you can fix the path quickly.

## Sysand Dependencies

[Sysand](https://github.com/sensmetry/sysand) is an optional package manager for SysML v2 dependencies. If your workspace has a `sysand.toml` manifest, Spec42 will detect it and show the **Sysand** section in the Library panel.

- If Sysand is not installed, a link to the installation instructions is shown.
- You can copy the install command directly from the panel.
- Click **Refresh** to re-scan dependency roots and restart the language server after installing or updating packages.
