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

After opening a SysML workspace, the **Spec42** sidebar gives you four main tools:

- **Model Explorer** for browsing packages and elements.
- **Examples** for opening bundled sample workspaces.
- **Help** for quick reference links and the quick-reference panel.
- **Library** for searching the bundled standard/domain libraries and any custom libraries you add.

The **Model Explorer** can switch between a semantic model view and a by-file view. Use it to reveal source locations, copy qualified names, and open package-level visualizations.

## Opening the visualizer

The diagram visualizer lives in the **secondary sidebar** (right side of VS Code).

- Open it via **View → Secondary Side Bar**, or
- Use the **Help** section in the Spec42 sidebar and click **Open diagram visualizer**.

Once open, select a view from the dropdown in the toolbar to render your model as a diagram.

## Inspecting elements

The **Feature Inspector** also lives in the secondary sidebar alongside the visualizer.

- Place the cursor on a SysML/KerML keyword, element, reference, value, or unit.
- The inspector follows the current selection and shows resolved semantics, relationships, source location, and language help.
- Clicking a node in the visualizer pins the inspector to that element until you resume following the cursor.

## Next steps

- [Browse the examples](./examples) to see what Spec42 can do.
- [Learn the Model Explorer](./model-explorer) for navigating larger workspaces.
- [Learn the visualizer](./visualizer) to get the most out of the diagrams.
- [Learn the Feature Inspector](./feature-inspector) for resolved semantic details.
- [Manage libraries and dependencies](./libraries) for standard, domain, custom, and Sysand-backed content.
- [Learn the SysML language patterns](./language-basics) for definitions, usages, packages, structure, behavior, and validation.
- [What's included](../reference/whats-included) for Spec42 and bundled library versions.
- [SysML v2 Quick Reference](../reference/sysml-quick-reference) for language syntax.
