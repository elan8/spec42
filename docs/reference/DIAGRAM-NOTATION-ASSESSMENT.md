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
`diagram_action_flow_conditional_successions.md`, `diagram_state_actions.md`,
`diagram_composite_state.md`, `diagram_interconnection_flows.md`,
`diagram_interconnection_feature_direction.md`), since the example models use few of
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
| Port direction | Arrow inside the port square for `in`, `out`, `inout` | Drawn from the direction written on the port usage, or else from the directed features of the port and of its definitions, reversed by a conjugated typing. Features a definition inherits are not followed | Conforms |
| Conjugated port | `~` before the type in the port's declaration | The port is labelled `name : ~Type`; other ports by name only | Conforms |
| Connection | Solid line between ports or parts, optional name | Solid line, name or `: Type` where there is room | Conforms |
| Connection colour | Not defined | One colour for each port type | Extension, harmless |
| Left-out nested parts | `...` in the lower left corner | `...` in the lower left corner | Conforms |
| Connector end on a nested feature of a closed part | Proxy: filled dot on the boundary, labelled with the feature | Filled dot on the boundary, labelled with the feature's path | Conforms |
| Names | Written in full | Written in full; the part and the gap beside it are sized for them | Conforms |
| Flow | A line with a filled arrowhead and the item, or a flow node on a connection | A line with a filled arrowhead between the two ports, labelled with the item (after the name of the flow when it has one). A flow between features of two ports attaches to those ports. A flow is drawn as a line of its own, not as a node on a connection between the same ports | Conforms for flows; flow nodes on connections are missing |

## State transition view (8.2.3.18)

| Notation | Specification | Drawn | Status |
| --- | --- | --- | --- |
| State | Rounded rectangle | Rounded rectangle | Conforms |
| Name compartment | `«state»`, then the name | `«state»`, then the name | Conforms |
| Entry, do and exit actions | A state actions compartment listing `entry x`, `do x`, `exit x` | A labelled state actions compartment with those lines. An action without a name is listed by the action it is typed by | Conforms |
| Start node | Filled circle | Filled circle | Conforms |
| Done node | Circle around a filled circle | Drawn for a transition to `done` | Conforms |
| Terminate node | Circle with a cross | Circle with a cross | Conforms |
| Transition | Solid line, open arrowhead | Solid line, open arrowhead since this change; it was filled | Conforms |
| Transition label | Plain text: trigger, guard, `/` effect | Plain text. Trigger, guard and effect are joined as published. Three or more transitions with one label into one state share a line and one label at that state | Conforms; the shared line is an extension |
| Composite state | The state symbol with its state transition view inside | The state with its states, their transitions and its initial node inside it. A transition between a nested state and a state outside is routed around the states in its way | Conforms |
| View frame | Frame with the view's name compartment | A frame with the name in a compartment in its upper left corner, also for the action flow view. The sequence view still has a title | Conforms |

## Action flow view (8.2.3.17)

| Notation | Specification | Drawn | Status |
| --- | --- | --- | --- |
| Action | Rounded rectangle, `«action»` and name | Rounded rectangle with `«action»` and the name | Conforms |
| Action definition | Rectangle with square corners, `«action def»` | Drawn as the frame of the flow | Conforms |
| Start node | Filled circle | Filled circle, where a succession starts at `start` | Conforms |
| Done node | Circle around a filled circle | Circle around a filled circle, where a succession ends at `done` | Conforms |
| Succession | Dashed line, open arrowhead, optional guard | Dashed line, open arrowhead, for `first a then b;` and for the `then` shorthand, from the nearest occurrence before it (7.17.4) | Conforms |
| Conditional succession | A succession with its guard in brackets | The guard as it is written, in brackets beside the line, for `first a if g then b;` and for `if g then b;` in the `then` shorthand; `[else]` for `else b;` | Conforms |
| Decision and merge | Diamond | Diamond | Conforms |
| Fork and join | Bar | Bar | Conforms |
| Parameters | Small squares on the action's boundary, with direction | Small squares on the boundary, inputs on the left and outputs on the right, with the name beside them. Inherited parameters are included | Conforms; the side stands for the direction |
| Flow | Solid line, filled arrowhead, optional `«flow»` and item | Solid line with a filled arrowhead between the actions that own the two parameters; the item is not named | Conforms without the item |
| Accept and send actions | The action symbol with a notched or a pointed flag in its upper left corner | As specified. An accept or send action without a name is labelled `accept` or `send` | Conforms |
| Swimlanes | Supported by the notation | Not drawn | Missing |

## What to change next, by effect on a reader

1. Flow nodes on a connection, where a flow and a connection join the same two ports.
2. Swimlanes in an action flow.

