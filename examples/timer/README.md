# Kitchen Timer

This example is the recommended first substantial model in the repository. It stays compact enough for learners to read in one sitting while still combining structure, state behavior, interfaces, requirements, use cases, and a small implementation view.

## Purpose

Model a countdown timer for cooking, workouts, and other timed tasks with a clear link between structure, behavior, requirements, and a compact implementation view.

## Why Start Here

- it is the strongest end-to-end teaching example in the repo
- it stays small enough to read quickly without losing the link between structure, behavior, and traceability
- it is the first example in this repository validated against `spec42`

## What This Example Demonstrates

- a readable state-machine-centered behavioral example
- typed interfaces for buttons, display, buzzer, and power
- a small decomposition from logical timer behavior into implementation-oriented parts
- requirements and `satisfy` traces in a compact setting
- simple use cases that connect the model back to user intent

## What Is Intentionally Simplified

- countdown logic, timing accuracy, and electrical behavior are sketched at example level rather than fully formalized
- the requirements and constraints are illustrative and not intended as a complete product specification
- physical implementation details are selective and only included where they help explain structure or interfaces

## What To Inspect In The Model

- `KitchenTimer` for the top-level system structure
- `TimerStateMachine` for the main behavioral flow
- the port definitions for button, display, buzzer, and battery interactions
- the requirement definitions and `satisfy` usages
- the `TimerPCB` decomposition as a bridge between logical and implementation-oriented views

## Known Limitations

- this is a teaching example, not a validated appliance design
- the requirements remain lightweight and readable rather than fully verification-ready
- the parametric constraints are illustrative support for the timing and runtime requirements, not a full analysis model

## Validation And Tooling Notes

- canonical validation target: [`spec42`](https://github.com/elan8/spec42)
- this is the repository's flagship validated example
- current goal: the file should load cleanly and publish no intended diagnostics in the `spec42` workflow
- if you adapt the model to another SysML v2 tool, keep the teaching story and end-to-end coherence intact even if notation adjustments are required

## File

- [KitchenTimer.sysml](KitchenTimer.sysml) - complete SysML v2 model for this example

## Walkthrough

Read the model in this order:

1. Start at `KitchenTimer` to understand the top-level parts and power fan-out.
2. Inspect the port definitions to see how button input, display output, buzzer control, and battery power are typed.
3. Read `TimerPCB` to see how the logical timer behavior is mapped onto a small implementation-oriented decomposition.
4. Read `TimerStateMachine` to understand the core learner-facing behavior: set, run, pause, expire, and reset.
5. Finish with the requirement definitions, `satisfy` relations, and the two illustrative constraints that support the timing and runtime story.
