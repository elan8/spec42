# Feature Inspector

The **Feature Inspector** is a secondary-sidebar panel that shows resolved semantic information for the current SysML/KerML selection.

## How it works

- Place the cursor on a keyword, declaration, reference, value, or unit.
- The inspector follows the current editor selection automatically.
- If you click a node in the [Diagram view](DIAGRAM-VIEW.md), the inspector pins to that element until you choose **Resume following cursor**.

## What it shows

Depending on the selected element, the inspector can show:

- Language help for a SysML/KerML keyword.
- The fully qualified name and declaration text.
- Declared type and effective type.
- Specialization, subsetting, and redefinition targets.
- Inherited features.
- Incoming and outgoing relationships.
- Values, evaluated values, units, and quantity types.
- Metadata, documentation, and source location.

## Typical uses

Use the Feature Inspector when:

- A hover tooltip is too small and you need the full semantic context.
- You want to understand where a reference resolves.
- You need to inspect inherited or effective typing.
- You want to jump back to the defining source location quickly.

## Best workflow

For large models, a good flow is:

1. Open a graphical view in the [Diagram view](DIAGRAM-VIEW.md).
2. Click elements in the diagram and inspect their resolved semantics in the Feature Inspector.
3. Use the source location in the inspector to jump to the declaration in the editor.
