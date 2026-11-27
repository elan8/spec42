# Getting started

Spec42 is a language server, VS Code extension and command-line tool for SysML v2 and KerML. The
editor and the command line run the same analysis engine.

## Install the VS Code extension

Install the **Spec42** extension from the VS Code Marketplace.

Once installed, open a folder containing `.sysml` or `.kerml` files and the language server starts automatically.

## Install the command-line tool

The extension includes its own copy of the tool. To use `spec42` in a terminal or in CI:

1. Open [GitHub Releases](https://github.com/elan8/spec42/releases) and download the archive for
   your platform from the release you want: `spec42-<version>-linux-x64.tar.gz`,
   `spec42-<version>-darwin-x64.tar.gz`, `spec42-<version>-darwin-arm64.tar.gz` or
   `spec42-<version>-win32-x64.zip`.
2. Extract it and put the `spec42` executable (`spec42.exe` on Windows) on your `PATH`.
3. Check the installation with `spec42 --version`. It should match the documentation version you
   are reading.

The [CLI reference](CLI-REFERENCE.md) describes every command.

For GitHub Actions, use the [Spec42 GitHub Action](GITHUB-ACTION.md) instead; it installs the
matching release for you.

## Your first model

The fastest way to get started is to open one of the built-in examples:

1. Open the **Spec42** tab in the left sidebar.
2. Under **Examples**, click **timer** (marked with a star — this is the recommended starting point).
3. The workspace opens with a complete SysML v2 model.

You can also create a new file with a `.sysml` extension and start writing:

```sysml
package MyFirstModel {

    part def Sensor {
        attribute name : String;
        port dataOut;
    }

    part def Controller {
        port dataIn;
    }

    part def System {
        part sensor : Sensor;
        part controller : Controller;
        connect sensor.dataOut to controller.dataIn;
    }
}
```

### Start a multi-file workspace from the CLI

If you use the Spec42 CLI, create a validated starter workspace with:

```bash
spec42 init my-model
```

`my-model` must be a new or empty directory. Spec42 never overwrites files during initialization and runs its normal semantic check before reporting success. The starter includes definitions, a baseline configuration, requirements, and a small domain-type library.

## Exploring your model

After opening a SysML workspace, the **Spec42** sidebar includes these sections.

- **Examples** for opening bundled sample workspaces.
- **Help** for quick reference links and the quick-reference panel.
- **Library** for searching the bundled standard/domain libraries and any custom libraries you add.
- **Elan8 Tools** for installing and running tools such as Doc42.

## Generate a PDF with Doc42

1. Open your model project folder in desktop VS Code. In a multi-root workspace, expand the
   project folder you want under Doc42.
2. Open **Spec42 → Elan8 Tools → Doc42** and click **Install Doc42**. Managed installation
   supports local Windows x64 and Linux x64 and reuses a compatible Doc42 on your PATH if
   available. Use **Installation guide** for platform requirements and manual setup.
3. Select a discovered document to generate its PDF, or choose **Generate All** for that
   project. If the list is empty, **Add a Doc42 document…** opens a model example. If Doc42
   needs access, use **Enter Elan8 API token…** with a [token from Elan8](https://www.elan8.com/account?section=tokens).
   The sidebar shows **Doc42 access verified** when Doc42 can use an existing license.
   **Check document…** is available without a token.
4. Review the PDF in the editor tab. It opens at **Fit Page**. Scroll to read successive pages
   and use the zoom controls to change the size. After a batch build, choose which PDF to preview.
   Use **Save As…** to keep a copy or **Open Externally** to open it in your system PDF viewer.
   **Open last build…** lets you reopen any PDF from the project's most recent successful build.

Under **Elan8 Tools → Elan8 API token**, choose **Check Elan8 API token…** to check the
workstation’s token online, or **Enter Elan8 API token…** to verify and save a token for Elan8
tools on this workstation. Both actions work before you install any Elan8 tool. A valid token
and access to Doc42 are checked separately.

Doc42 generates from saved project files. It loads its bundled libraries, project libraries
in `spec42.toml`, and extra roots from `spec42.libraryPaths` by default. If the editor uses a
source checkout that overlaps with Doc42's bundled libraries, set `spec42.doc42.libraryPaths`
to `[]` in the workspace settings to omit the editor's extra roots. Supply a list if Doc42
needs other roots. See the
[Doc42 getting started guide](https://github.com/elan8/doc42-release/blob/main/docs/getting-started.md#create-your-own-document)
for a sample document and the model annotations.

## Opening the Views panel

The **Views** panel lives in the **secondary side bar** (right side of VS Code).

- Run **Spec42: Open Views** from the Command Palette, or
- Open **View → Secondary Side Bar** and select **Views**.

Once open, pick an authored view from the toolbar dropdown to render your model as a diagram. It
regenerates on its own as the model changes. See the [Views panel guide](DIAGRAM-VIEW.md) for the
toolbar and export options.

## Inspecting elements

The **Feature Inspector** also lives in the secondary side bar alongside the Views panel.

- Place the cursor on a SysML/KerML keyword, element, reference, value, or unit.
- The inspector follows the current selection and shows resolved semantics, relationships, source location, and language help.
- Clicking a node in the Views panel pins the inspector to that element until you resume following the cursor.

## Next steps

- [Browse the examples](EXAMPLES.md) to see what Spec42 can do.
- [Learn about diagram views](DIAGRAM-VIEW.md) for rendering authored views.
- [Learn the Feature Inspector](FEATURE-INSPECTOR.md) for resolved semantic details.
- [Set up projects and libraries](LIBRARIES.md) for standard, domain, custom, and Sysand-backed content.
- Learn SysML v2 language patterns from the [OMG SysML v2 specification](https://www.omg.org/spec/SysML/2.0/) or the in-editor **SysML v2 Quick Reference** (Spec42 sidebar → Help).
- [What's included](../reference/WHATS-INCLUDED.md) for Spec42 and bundled library versions.
