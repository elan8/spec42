# Diagram notation assessment

How the four view kinds that engineering documents use most compare with the graphical notation
of SysML v2 (specification clause 8.2.3, release 2026-04). The reference is the graphical BNF
and its notation images in `Systems-Modeling/SysML-v2-Release`, `bnf/SysML-graphical-bnf.html`.
The generated [notation inventory](SYSML-NOTATION-INVENTORY.md) counts every production; this
document judges what is drawn.

Assessed on the views of `elan8/sysml-examples` and `elan8/sysml-robot-vacuum-cleaner`, by
comparing each drawn symbol with the production and its image. Status: **conforms**, **deviates**
(drawn, but not as the notation defines it), **missing** (the notation has it and nothing is
drawn).

How each row was established: the rows on node shapes, keywords, the composite, typing and
subsetting lines, ports, proxies, the ellipsis, states and transitions were checked in rendered
diagrams against the notation images. The rows on the done and terminate nodes, decision, merge,
fork and join, subclassification, flows, and accept and send actions were read from the drawing
code and not seen in a rendered diagram, because the assessed models do not use them.

## General view (8.2.3.6, 8.2.3.11)

| Notation | Specification | Drawn | Status |
| --- | --- | --- | --- |
| Definition node | Rectangle with square corners | Square corners | Conforms |
| Usage node | Rectangle with rounded corners | Rounded corners | Conforms |
| Keyword | `«part def»`, `«part»` above the name | As specified | Conforms |
| Name compartment | The declaration, `name : Type` | Name, then `: Type` on its own line | Deviates, layout only |
| Composite membership | Line with a filled diamond at the owner | Filled diamond at the owner | Conforms |
| Definition (feature typing) | Solid line, hollow triangle with two dots at the definition | Dashed line, open arrowhead | Deviates |
| Subclassification | Solid line, hollow triangle | Solid line, hollow triangle | Conforms |
| Subsetting | Solid line, hollow triangle (open) at the subsetted feature | Dashed line, open arrowhead | Deviates |
| Compartment label | Centred, italic, lower case (`parts`) | Left, bold, capitalised (`Parts`), with an expand control | Deviates |
| Left-out content | `…` after the listed members, or `...` in the lower left corner | A collapsed compartment with an expand control | Deviates |
| Reference usage | Dashed outline | Dashed outline | Conforms |

## Interconnection view (8.2.3.11 to 8.2.3.13)

| Notation | Specification | Drawn | Status |
| --- | --- | --- | --- |
| Part usage | Rounded rectangle, solid | Rounded rectangle, solid | Conforms |
| Part that shows its parts | The same solid part symbol with the nested view inside | Solid since this change; it was dashed, the notation of a reference | Conforms |
| Port | Small square on the boundary | Small square on the boundary | Conforms |
| Port label | Beside the port, outside the part | Inside the part | Deviates |
| Port direction | Arrow inside the port square for `in`, `out`, `inout` | No arrow | Missing |
| Conjugated port | `~` in the port label | Not shown | Missing |
| Connection | Solid line between ports or parts, optional name | Solid line, name or `: Type` where there is room | Conforms |
| Connection colour | Not defined | One colour for each port type | Extension, harmless |
| Left-out nested parts | `...` in the lower left corner | `...` in the lower left corner | Conforms |
| Connector end on a nested feature of a closed part | Proxy: filled dot on the boundary, labelled with the feature | Filled dot on the boundary, labelled with the feature's path | Conforms |
| Names | Written in full | Cut at 32 characters (40 for a port or proxy label); the part is sized for its labels | Deviates for very long names |
| Flow on a connection | Flow node (arrow) on the line with the item | Not drawn on the connection | Missing |

## State transition view (8.2.3.18)

| Notation | Specification | Drawn | Status |
| --- | --- | --- | --- |
| State | Rounded rectangle | Rounded rectangle | Conforms |
| Name compartment | `«state»`, then the name | `«state»`, then the name | Conforms |
| Entry, do and exit actions | Compartments labelled `entry`, `do`, `exit` | Lines `entry / x`, `do / x`, `exit / x` in one compartment | Deviates |
| Start node | Filled circle | Filled circle | Conforms |
| Done node | Circle around a filled circle | Drawn for a final state | Conforms |
| Terminate node | Circle with a cross | Circle with a cross | Conforms |
| Transition | Solid line, open arrowhead | Solid line, open arrowhead since this change; it was filled | Conforms |
| Transition label | Plain text: trigger, guard, `/` effect | Plain text since this change; it was framed. Trigger, guard and effect are joined as published | Conforms |
| Composite state | The state symbol with its state transition view inside | Named regions, not the nested states | Deviates |
| View frame | Frame with the view's name compartment | The name as a title above the content | Deviates |

## Action flow view (8.2.3.17)

| Notation | Specification | Drawn | Status |
| --- | --- | --- | --- |
| Action | Rounded rectangle, `«action»` and name | Rounded rectangle; the keyword is drawn for the definition, not for each action | Deviates |
| Action definition | Rectangle with square corners, `«action def»` | Drawn as the frame of the flow | Conforms |
| Start node | Filled circle | Filled circle | Conforms |
| Done node | Circle around a filled circle | Drawn | Conforms |
| Succession | Dashed line, open arrowhead, optional guard | Dashed line, open arrowhead | Conforms |
| Flow | Solid line, filled arrowhead, optional `«flow»` and item | Solid line, filled arrowhead | Conforms |
| Decision and merge | Diamond | Diamond | Conforms |
| Fork and join | Bar | Bar | Conforms |
| Parameters | Small squares on the action's boundary, with direction | Not drawn | Missing |
| Accept and send actions | Their own node shapes | Drawn as plain actions | Deviates |
| Swimlanes | Supported by the notation | Not drawn | Missing |

## What to change next, by effect on a reader

1. Port labels outside the part, with direction arrows and `~` for a conjugated port. This is
   the most visible difference from a diagram drawn by hand to the specification.
2. The definition (typing) and subsetting lines in a general view.
3. Compartment labels and the ellipsis for left-out members in a general view.
4. `«action»` on each action, and parameters on the boundary.
5. A composite state that shows its nested states.
6. `entry`, `do` and `exit` as labelled compartments.
