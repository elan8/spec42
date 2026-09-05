# Getting Started

## Installation

Install the **Spec42** extension from the VS Code Marketplace.

Once installed, open a folder containing `.sysml` or `.kerml` files and the language server starts automatically.

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

After opening a SysML workspace, the **Spec42** sidebar gives you three tools:

- **Examples** for opening bundled sample workspaces.
- **Help** for quick reference links and the quick-reference panel.
- **Library** for searching the bundled standard/domain libraries and any custom libraries you add.

## Opening the Diagram view

The **Diagram** view lives in the **secondary side bar** (right side of VS Code).

- Run **Spec42: Open Diagram** from the Command Palette, or
- Open **View → Secondary Side Bar** and select **Diagram**.

Once open, pick an authored view from the toolbar dropdown to render your model as a diagram. It
regenerates on its own as the model changes. See the [Diagram view guide](DIAGRAM-VIEW.md) for the
toolbar and export options.

## Inspecting elements

The **Feature Inspector** also lives in the secondary side bar alongside the Diagram view.

- Place the cursor on a SysML/KerML keyword, element, reference, value, or unit.
- The inspector follows the current selection and shows resolved semantics, relationships, source location, and language help.
- Clicking a node in the Diagram view pins the inspector to that element until you resume following the cursor.

## Next steps

- [Browse the examples](EXAMPLES.md) to see what Spec42 can do.
- [Learn the Diagram view](DIAGRAM-VIEW.md) for rendering authored views.
- [Learn the Feature Inspector](FEATURE-INSPECTOR.md) for resolved semantic details.
- [Manage libraries and dependencies](LIBRARIES.md) for standard, domain, custom, and Sysand-backed content.
- Learn SysML v2 language patterns from the [OMG SysML v2 specification](https://www.omg.org/spec/SysML/2.0/) or the in-editor **SysML v2 Quick Reference** (Spec42 sidebar → Help).
- [What's included](../reference/WHATS-INCLUDED.md) for Spec42 and bundled library versions.
