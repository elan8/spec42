# Views

The **Views** panel renders views authored in your model as SVG. It lives in the
**secondary side bar** (right side of VS Code), in the **Spec42** container next to the
[Feature Inspector](FEATURE-INSPECTOR.md).

Views are projected from the same immutable analysis publication the editor and CI use,
then laid out and drawn by the native renderer. The diagram is a
presentation of the resolved model — never a second place to define semantics.

## Opening the view

- Run **Spec42: Open Views** from the Command Palette, or
- Open the secondary side bar (**View → Secondary Side Bar**) and select **Views**.

If the secondary side bar is not visible, `Spec42: Open Views` reveals it and focuses the view.

## Choosing a view to render

Authored diagram views come from `view def` / `view` usages typed by a standard view definition
(for example `GeneralView`, `InterconnectionView`) with `expose` clauses that name the model
elements to include.

The toolbar dropdown lists every authored view in the model, grouped by their owning namespace, with namespace groups and view names sorted alphabetically.
The selected view kind appears separately; duplicate names include a kind suffix.
Pick one and the view regenerates in place. If the model authors no views, the panel shows an
empty state with a hint.

## Staying in sync

The view re-generates on its own whenever the model changes — there is no manual refresh button.
The language server emits a notification after each workspace publication rebuild and the view
reconciles against it. A backoff reconcile loop runs whenever the view is visible and its render
does not match the current publication, so a slow index, a missed notification, a language-server
restart, or a stalled webview all recover with no user action.

## Toolbar

| Button | Action |
| --- | --- |
| **Go to definition** | Open the selected view declaration in the editor. |
| **Home** | Fit and center the whole diagram. |
| **JSON** | Copy the generated diagram product (schema-versioned JSON) to the clipboard. |
| **SVG** | Export the current diagram as a standalone SVG file. |
| **PNG** | Export the current view as a PNG. |
| **Transparent export** | Export SVG/PNG without a canvas background; keep element fills. |

The viewer canvas blends into the VS Code editor theme. `SVG` and `PNG` export use a light
palette and a white canvas by default, so files remain readable outside the webview.
Enable **Transparent export** to omit only the canvas background. SysML view frames and
element fills remain visible.

The selected view and viewport are retained on refresh when the view survives in the catalog.
**Home** resets the viewport to fit. Switching to a different view starts with a fitted viewport.

## Working with the Feature Inspector

The Views panel is paired with the [Feature Inspector](FEATURE-INSPECTOR.md) in the same
container. Clicking a diagram node pins the inspector to that element; it then shows resolved
types, specialization / subsetting / redefinition, inherited features, relationships, values, and
source location. Use **Resume following cursor** in the inspector to go back to live editor
selection.

## View kinds

Spec42 projects the standard SysML view kinds — General, Interconnection, Action Flow, State
Transition, Sequence, Browser, Grid, and (partially) Geometry. Where a view's typed projection is
still incomplete the panel reports the typed reason rather than guessing. Coverage is tracked in
the [conformance matrix](../reference/CONFORMANCE-MATRIX.md).
