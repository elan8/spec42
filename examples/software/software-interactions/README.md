# Software Interaction Sequence Example

This example demonstrates the Spec42 `SoftwareInteractions` library together with `SequenceView`.

It includes:

- a service/API choreography with synchronous calls and return messages
- a creation message for a newly created payment record
- an activation span on the payment service lifeline
- an `alt` fragment for approved vs declined outcomes
- a nested `loop` fragment plus `InteractionRef` for retry handling
