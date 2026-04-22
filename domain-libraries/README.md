# Domain Libraries Layering

This directory organizes reusable libraries by role so technical capability libraries and business-domain libraries can evolve independently.

## Layers

- `technical/` - engineering capability libraries that are reusable across industries.
- `cross-cutting/` - overlays such as delivery, observability, security, and assurance that apply across technical/business domains.
- `business/` - business-domain libraries built by composing technical and cross-cutting capabilities.
- `examples/` - integrated scenarios demonstrating how to compose multiple layers.

## Initial Mapping

- `technical/software/`
  - `software-core`
  - `distributed-systems`
  - `data`
  - `platform`
- `technical/communication/`
  - `core`
  - `http`
  - `grpc`
  - `messaging`
  - `streaming`
  - `transport`
  - `device-bus`
  - `industrial`
- `technical/electronics/`
  - `electronics-core`
  - `power`
  - `compute`
  - `io`
  - `buses`
  - `board`
  - `examples`
- `cross-cutting/software/`
  - `delivery-ops`
  - `security`
- `business/robotics/`
  - `robotics-core`
  - `structure`
  - `perception`
  - `actuation`
  - `control`
  - `autonomy`
  - `runtime`
  - `simulation`
  - `operations`
  - `safety-assurance`
  - `ros2-overlay`
  - `examples`

## Dependency Direction

- Technical libraries must not import business libraries.
- Cross-cutting libraries may import technical libraries, but not business libraries.
- Business libraries may import technical and cross-cutting libraries.
- Foundational packages (`SoftwareCore`, `RoboticsCore`) remain stable to minimize import churn.
