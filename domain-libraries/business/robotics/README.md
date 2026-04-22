# Robotics Business Domain Libraries

This directory contains a SysML v2 business-domain library family for robotics, composed from reusable technical and cross-cutting capabilities.

It is organized so it can evolve into:

- reusable library content
- example material
- a future Spec42 analysis fixture

## Structure

- `robotics-core/RoboticsCore.sysml` - small robotics kernel with base system and environment concepts
- `structure/RobotStructure.sysml` - neutral robot composition and hardware structure overlay
- `perception/RobotPerception.sysml` - sensing and state-estimation overlay
- `actuation/RobotActuation.sysml` - actuation and motion-production overlay
- `control/RobotControl.sysml` - control-loop and command/feedback overlay
- `autonomy/RobotAutonomy.sysml` - planning, navigation, localization, and behavior overlay
- `runtime/RobotRuntime.sysml` - runtime execution, communication, parameters, and lifecycle overlay
- `simulation/RobotSimulation.sysml` - simulation and hardware-in-the-loop overlay
- `operations/RobotOperations.sysml` - fleet, telemetry, maintenance, and mission operations overlay
- `safety-assurance/RobotSafetyAssurance.sysml` - hazards, safety functions, fail-safe behavior, and assurance overlay
- `robotics-core/rules/robotics-core-rules.yaml` - core robotics rule catalog
- `perception/rules/robot-perception-rules.yaml` - perception rule catalog
- `control/rules/robot-control-rules.yaml` - control rule catalog
- `runtime/rules/robot-runtime-rules.yaml` - runtime rule catalog
- `safety-assurance/rules/robot-safety-assurance-rules.yaml` - safety and assurance rule catalog

## Notes

- The SysML files define reusable robotics concepts using ordinary package declarations and specialization.
- Naming is normalized around a primary `name` attribute in base definitions; examples override values instead of redefining schemas.
- Rule catalogs can be descriptive or executable and should stay focused on business-domain modeling quality.
- Safety and assurance are explicit overlays rather than being folded into the robotics kernel.
- Future overlays can add ecosystem-specific content without restructuring the core packages.
