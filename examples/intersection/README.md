# Traffic Light Intersection

This example is centered on controller behavior. It is useful for learners who want to study a phase-oriented state machine in a domain that is easy to reason about at system level.

## Purpose

Model a signalized intersection with a compact controller, signal heads, detector input, and an operator command path.

## What This Example Demonstrates

- a controller-focused state machine with phase sequencing
- a clear relationship between controller logic, signal heads, detectors, and operator input
- simple requirements and use cases attached to an operational scenario
- a compact example of mode-oriented behavior using `Normal` and `Flash`

## What Is Intentionally Simplified

- the example uses a reduced intersection model rather than a full real-world traffic installation
- pedestrian timing, detector behavior, all-red handling, and preemption are discussed more fully in the README than in the model
- requirements and safety rules are illustrative and not a substitute for traffic-control standards

## What To Inspect In The Model

- `TrafficLightIntersection` for the top-level structure
- `Normal` and `ControllerStateMachine` for the key behavior
- the signal, detector, and operator ports for simple interface patterns
- the requirement definitions and `satisfy` usages

## Known Limitations

- this example keeps timing logic and safety constraints lightweight
- several operational concepts are intentionally sketched rather than modeled in full detail

## Validation And Tooling Notes

- expected use: a behavior-focused example to inspect in a SysML v2-capable editor
- validation stance: syntax and package loadability should be reviewed in your chosen tool, but this is not claimed as a fully validated traffic-control reference
- if a tool reports differences in state or requirement notation, treat the model as an illustrative pattern rather than a standards-complete deployment model

## File

- [TrafficLightIntersection.sysml](TrafficLightIntersection.sysml) - complete SysML v2 model for this example
