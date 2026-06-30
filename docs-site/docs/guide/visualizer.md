# Diagram Visualizer

The Spec42 visualizer renders your SysML v2 model as an interactive diagram. It lives in the **secondary sidebar** (right side of VS Code).

## Opening the visualizer

- Go to **View → Secondary Side Bar**, or
- Click **Open diagram visualizer** in the **Help** section of the Spec42 sidebar.

The visualizer updates automatically as you edit your `.sysml` files.

## Selecting a view

Use the **Select SysML View** dropdown in the visualizer toolbar to pick which defined view to render. Views are defined in your model using `view def` and `expose`.

If no views are defined, the visualizer shows an empty state with a hint.

## Available diagram types

| View | What it shows |
|------|--------------|
| **General View** | Part hierarchy, typing, specialization, containment, binding, and allocation relationships. |
| **Interconnection View** | Port connections between parts — useful for interface and signal flow diagrams. |
| **State Transition View** | State machines with transitions, guards, and actions. |

## Toolbar controls

| Control | Action |
|---------|--------|
| **Home** | Fit the diagram to the window. |
| **LR / TB** | Toggle layout direction (left-to-right or top-to-bottom). |
| **Legend** | Show a legend of diagram line styles and symbols. |
| **Export** | Export the diagram as PNG (1×–4×), SVG, or JSON. |

## Diagram legend

| Symbol | Meaning |
|--------|---------|
| Dashed line + open arrow | **Typing** — a usage is typed by a definition (`: Type`) |
| Solid line + hollow triangle | **Specialization** — a type inherits from another (`:>`) |
| Solid line + filled diamond at source | **Containment** — a part is owned by a container |
| Fine dotted line | **Binding** — two features are equal (`bind … = …`) |
| Long dashed line + filled arrow | **Allocation** — a logical element is allocated to physical |
