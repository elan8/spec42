# Surveillance Quadrotor Drone

This is the broadest example in the repository. It is meant as a worked example for learners who already understand the smaller models and want to see how the same patterns scale to a more complex system description.

## Purpose

Model a surveillance quadrotor UAV with flight control, sensors, gimbal, camera payload, communication links, and onboard power distribution.

## What This Example Demonstrates

- decomposition of a larger system into airframe, propulsion, flight control, gimbal, payload, communication, and power
- a wider set of ports, requirements, use cases, mission actions, and flight-mode behavior than the smaller examples
- a learner-friendly bridge from simple examples to a richer system-of-interest narrative

## What Is Intentionally Simplified

- many domain concepts are modeled at sketch level to keep the file readable
- interfaces often use simple value types where a stricter model would introduce richer domain types
- the mission actions are intentionally lightweight and illustrative rather than a detailed executable workflow
- the requirements are not intended to be a complete or authoritative drone architecture

## What To Inspect In The Model

- `SurveillanceQuadrotorDrone` for the system structure and main connections
- `FlightControlAndSensing` for subsystem decomposition
- the requirement set and `satisfy` usages for traceability patterns
- the mission action definitions for a simple high-level behavior layer
- `FlightModeStateMachine` as a larger state-oriented example

## Known Limitations

- this is the broadest and least formalized example in the repo and should be read as a worked teaching model rather than a polished reference architecture
- several behaviors and interfaces favor readability over strict typing or exhaustive completeness
- power, sensing, autonomy, and regulation topics are deliberately compressed into one compact model

## Validation And Tooling Notes

- expected use: an advanced worked example to inspect after the smaller models
- validation stance: treat this model as a readable broad example, not as a fully validated multi-tool reference
- if your tool is sensitive to advanced action or state notation, preserve the top-level teaching structure and simplify locally as needed

## File

- [SurveillanceDrone.sysml](SurveillanceDrone.sysml) - complete SysML v2 model for this example
