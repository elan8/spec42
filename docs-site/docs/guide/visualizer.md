# Diagram Visualizer

The Spec42 visualizer renders your SysML v2 model as an interactive diagram. It lives in the **secondary sidebar** (right side of VS Code).

## Opening the visualizer

- Go to **View → Secondary Side Bar**, or
- Click **Open diagram visualizer** in the **Help** section of the Spec42 sidebar.

The visualizer updates automatically as you edit your `.sysml` files.

## Moving to the editor

For more screen space, or to put the diagram on a second monitor, move the visualizer into the editor area:

- Click the **Move Visualizer to Editor** button in the visualizer title bar, or
- Run **SysML: Move Visualizer to Editor** from the Command Palette.

While it is in the editor:

- The secondary sidebar shows a short placeholder.
- Closing the visualizer editor tab returns it to the secondary sidebar.
- You can also run **SysML: Move Visualizer to Secondary Side Bar**.

From the editor tab, you can use VS Code's normal editor actions such as **Move into New Window**.

## Working with the Feature Inspector

The visualizer is paired with the **Feature Inspector** in the secondary sidebar.

- Clicking a diagram node pins the inspector to that element.
- The inspector then shows resolved types, specialization/subsetting/redefinition, inherited features, relationships, values, and source location.
- Use **Resume following cursor** in the inspector to switch back to live editor selection.

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
| **Export** | Export the diagram as PNG (1×-4×), SVG, or JSON. |

## Diagram legend

| Symbol | Meaning |
|--------|---------|
| Dashed line + open arrow | **Typing** — a usage is typed by a definition (`: Type`) |
| Solid line + hollow triangle | **Specialization** — a type inherits from another (`:>`) |
| Solid line + filled diamond at source | **Containment** — a part is owned by a container |
| Fine dotted line | **Binding** — two features are equal (`bind … = …`) |
| Long dashed line + filled arrow | **Allocation** — a logical element is allocated to physical |
