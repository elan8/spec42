# Office Workstation

This is the smallest example in the repository. It is designed as a quick first read for learners who want to see a compact SysML v2 text model with typed ports, simple structural connections, a small state machine, and basic `satisfy` relationships.

## Purpose

Model a realistic office workstation used for engineering and analysis tasks: laptop, monitor, keyboard, mouse, and wall power.

## What This Example Demonstrates

- typed interfaces and explicit structural connections
- a small operational state machine on the laptop
- simple requirements with `satisfy` traces
- a lightweight use-case layer

## What Is Intentionally Simplified

- the model stays at desk-setup level and does not try to model networking, software deployment, thermal behavior, or ergonomics
- interface typing is kept simple so the example stays readable for first-time learners
- the requirements are illustrative and are not meant to represent a full verification-ready specification

## What To Inspect In The Model

- `OfficeDeskSetup` for the main parts and connections
- the port definitions for the power, HDMI, and USB examples
- `WorkSessionStateMachine` for the smallest behavior example in the repo
- `officeInstance` and the `satisfy` usages for traceability

## Known Limitations

- this example optimizes for readability over strict realism
- it is intentionally smaller and less formal than the timer, intersection, and drone examples

## Validation And Tooling Notes

- expected use: a quick orientation example in any SysML v2-capable editor
- validation stance: reviewed for readability and teaching coherence rather than full formalization
- if your tool uses a narrower supported subset, adapt the example conventions to that tool rather than treating this model as normative

## File

- [office.sysml](office.sysml) - complete SysML v2 model for this example
