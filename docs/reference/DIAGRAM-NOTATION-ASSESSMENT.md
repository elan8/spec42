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

How each row was established: the rows on node shapes, keywords, the composite, typing,
subclassification and subsetting lines, ports, proxies, the ellipsis, compartments, states,
transitions and the view frame were checked in rendered diagrams against the notation images.

The rows on the action flow and state transition views were checked on models written to use
each element (`tests/snapshots/generation/diagram_action_flow_then_successions.md`,
`diagram_state_actions.md`, `diagram_composite_state.md`), since the example models use few of
them.

## General view (8.2.3.6, 8.2.3.11)

| Notation | Specification | Drawn | Status |
| --- | --- | --- | --- |
| Definition node | Rectangle with square corners | Square corners | Conforms |
| Usage node | Rectangle with rounded corners | Rounded corners | Conforms |
| Keyword | `«part def»`, `«part»` above the name | As specified | Conforms |
| Name compartment | The declaration, `name : Type` | Name, then `: Type` on its own line | Deviates, layout only |
| Composite membership | Line with a filled diamond at the owner | Filled diamond at the owner | Conforms |
| Definition (feature typing) | Solid line, hollow triangle with two dots at the definition | As specified since this change; it was dashed with an open arrowhead. Drawn thinner than a membership line | Conforms |
| Subclassification | Solid line, hollow triangle | Solid line, hollow triangle | Conforms |
| Subsetting | Solid line, hollow triangle at the subsetted feature | As specified since this change; it was dashed with an open arrowhead | Conforms |
| Compartment label | Centred, italic, lower case (`parts`) | Centred, italic, lower case. A compartment that can be opened or closed keeps its expand control at the left | Conforms |
| Left-out content | `…` after the listed members, or `...` in the lower left corner | `…` after the listed members. A closed compartment shows its label and expand control only | Conforms for listed members |
| Reference usage | Dashed outline | Dashed outline | Conforms |

## Interconnection view (8.2.3.11 to 8.2.3.13)

| Notation | Specification | Drawn | Status |
| --- | --- | --- | --- |
| Part usage | Rounded rectangle, solid | Rounded rectangle, solid; the shaded name compartment holds the keyword, name and type | Conforms |
| Part that shows its parts | The same solid part symbol with the nested view inside | Solid since this change; it was dashed, the notation of a reference | Conforms |
| Port | Small square on the boundary | Small square on the boundary | Conforms |
| Port label | Beside the port, outside the part | Outside the part, below the connector, since this change; it was inside | Conforms |
| Port direction | Arrow inside the port square for `in`, `out`, `inout` | Drawn for a port usage with an authored direction. A port whose direction follows only from the features of its definition gets none | Conforms for authored directions |
| Conjugated port | `~` before the type in the port's declaration | The port is labelled `name : ~Type`; other ports by name only | Conforms |
| Connection | Solid line between ports or parts, optional name | Solid line, name or `: Type` where there is room | Conforms |
| Connection colour | Not defined | One colour for each port type | Extension, harmless |
| Left-out nested parts | `...` in the lower left corner | `...` in the lower left corner | Conforms |
| Connector end on a nested feature of a closed part | Proxy: filled dot on the boundary, labelled with the feature | Filled dot on the boundary, labelled with the feature's path | Conforms |
| Names | Written in full | Written in full; the part and the gap beside it are sized for them | Conforms |
| Flow on a connection | Flow node (arrow) on the line with the item | Not drawn on the connection | Missing |

## State transition view (8.2.3.18)

| Notation | Specification | Drawn | Status |
| --- | --- | --- | --- |
| State | Rounded rectangle | Rounded rectangle | Conforms |
| Name compartment | `«state»`, then the name | `«state»`, then the name | Conforms |
| Entry, do and exit actions | A state actions compartment listing `entry x`, `do x`, `exit x` | A labelled state actions compartment with those lines. An action without a name is listed by the action it is typed by | Conforms |
| Start node | Filled circle | Filled circle | Conforms |
| Done node | Circle around a filled circle | Drawn for a final state | Conforms |
| Terminate node | Circle with a cross | Circle with a cross | Conforms |
| Transition | Solid line, open arrowhead | Solid line, open arrowhead since this change; it was filled | Conforms |
| Transition label | Plain text: trigger, guard, `/` effect | Plain text. Trigger, guard and effect are joined as published. Three or more transitions with one label into one state share a line and one label at that state | Conforms; the shared line is an extension |
| Composite state | The state symbol with its state transition view inside | The state with its states, their transitions and its initial node inside it. A transition between a nested state and a state outside is drawn straight, without a route | Conforms |
| View frame | Frame with the view's name compartment | A frame with the name in a compartment in its upper left corner, also for the action flow view. The sequence view still has a title | Conforms |

## Action flow view (8.2.3.17)

| Notation | Specification | Drawn | Status |
| --- | --- | --- | --- |
| Action | Rounded rectangle, `«action»` and name | Rounded rectangle with `«action»` and the name | Conforms |
| Action definition | Rectangle with square corners, `«action def»` | Drawn as the frame of the flow | Conforms |
| Start node | Filled circle | Filled circle, where a succession starts at `start` | Conforms |
| Done node | Circle around a filled circle | Circle around a filled circle, where a succession ends at `done` | Conforms |
| Succession | Dashed line, open arrowhead, optional guard | Dashed line, open arrowhead, for `first a then b;` and for the `then` shorthand, from the nearest occurrence before it (7.17.4). No guard is drawn | Conforms without guards |
| Conditional succession | A succession with its guard in brackets | `first a if g then b;` is drawn as a succession without its guard, which is not published. `if g then b; else c;` after a `decide` is read as an if action and drawn as a node, with no line from the decision | Deviates |
| Decision and merge | Diamond | Diamond | Conforms |
| Fork and join | Bar | Bar | Conforms |
| Parameters | Small squares on the action's boundary, with direction | Small squares on the boundary, inputs on the left and outputs on the right, with the name beside them. Inherited parameters are included | Conforms; the side stands for the direction |
| Flow | Solid line, filled arrowhead, optional `«flow»` and item | Solid line with a filled arrowhead between the actions that own the two parameters; the item is not named | Conforms without the item |
| Accept and send actions | The action symbol with a notched or a pointed flag in its upper left corner | As specified for `action a accept ...;` and `then send ...;`. `then accept ...;` is not lowered as an action of its own and is not drawn | Conforms, except `then accept` |
| Swimlanes | Supported by the notation | Not drawn | Missing |

## What to change next, by effect on a reader

1. The guards of conditional successions in an action flow, `if ... then` after a `decide`,
   and `then accept`.
2. The direction a port takes from the features of its definition.
3. Flows on connections in an interconnection view.
4. Routes for transitions that cross the boundary of a containing state.

