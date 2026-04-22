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
- `ros2-overlay/Ros2Overlay.sysml` - ROS 2 middleware overlay built on the neutral robotics runtime
- `robotics-core/rules/robotics-core-rules.yaml` - core robotics rule catalog
- `perception/rules/robot-perception-rules.yaml` - perception rule catalog
- `control/rules/robot-control-rules.yaml` - control rule catalog
- `runtime/rules/robot-runtime-rules.yaml` - runtime rule catalog
- `safety-assurance/rules/robot-safety-assurance-rules.yaml` - safety and assurance rule catalog
- `ros2-overlay/rules/ros2-overlay-rules.yaml` - ROS 2 executable rule catalog (condition-based)
- `examples/inspection-rover/` - generation-ready ROS 2 inspection rover example (hierarchy, ports, connectors, deploy mapping, traceability)

## Notes

- The SysML files define reusable robotics concepts using ordinary package declarations and specialization.
- Naming is normalized around a primary `name` attribute in base definitions; examples override values instead of redefining schemas.
- Rule catalogs can be descriptive or executable. The ROS 2 catalog is maintained as an executable condition-based schema.
- The ROS 2 overlay now models generation-critical abstractions: runtime component mapping, endpoint ports, connections, deployment units, and traceability entries.
- The inspection rover example encodes explicit part hierarchy (`RoverSystem -> RoverRuntimeArchitecture`), typed ports, and connector wiring for deterministic endpoint bindings.
- Core generation targets are represented directly in-model: interface types (`msg/srv/action` refs), launch/config mapping, package/executable mapping, and artifact traceability.
- Full ROS2 config generation coverage now includes launch mode/include/group modeling, parameter profiles, remaps, TF authority declarations, lifecycle manager sequencing, manifest dependency declarations, and security enclave/policy mappings.
- The inspection rover example models reusable platform package mappings for Nav2, robot_localization, and ros2_control, while keeping mission-specific logic in the bringup package.
- Generation artifacts/traceability are expected for launch files, runtime/nav2/security parameter files, package manifests (`package.xml`, `CMakeLists.txt`), interface outputs, and security policy artifacts.
- ROS 2 value fields are typed where possible; string literals are used mainly for cross-element references and external ROS identifiers.
- ROS 2 rules cover semantic runtime checks and full-config generation-readiness checks for launch, params, manifests, interfaces, TF, lifecycle, remaps, and security completeness.
- Neutral robotics concepts stay separate from ROS 2-specific concepts so robots can be modeled with or without middleware commitments.
- Safety and assurance are explicit overlays rather than being folded into the robotics kernel.
- Future overlays can add ecosystem-specific content such as Nav2, MoveIt, Gazebo, industrial cells, or safety-standard mappings without restructuring the core packages.
