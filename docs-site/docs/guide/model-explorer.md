# Model Explorer

The **Model Explorer** is the main navigation view in the Spec42 sidebar. It lets you browse the loaded SysML/KerML workspace as a semantic model instead of just a file tree.

## What it shows

Depending on the current mode, the explorer can show:

- A **semantic model** view grouped by packages and model elements.
- A **by-file** view that keeps the model aligned with the source files it came from.

From the explorer, you can:

- Open an element's source location.
- Copy its qualified name.
- Visualize a package.
- Refresh the current model tree.

## Switching views

Use the toolbar buttons in the Model Explorer title area to switch between:

- **Semantic Model** for model-centric navigation.
- **By File** for source-centric navigation.

This is especially useful in larger workspaces where you may want to alternate between language structure and file layout.

## When it refreshes

The explorer refreshes automatically as the language server indexes and updates the workspace. You can also run **Refresh SysML Model Explorer** from the explorer toolbar or command palette.

## Related tools

- Use the [Feature Inspector](./feature-inspector) when you want resolved semantic detail for the current element.
- Use the [Diagram Visualizer](./visualizer) when you want a graphical view of packages, connections, or state machines.
