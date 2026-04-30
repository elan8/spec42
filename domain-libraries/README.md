# Domain Libraries Layering

This directory organizes reusable libraries by role so technical capability libraries and business-domain libraries can evolve independently.

These libraries are intended to help `spec42` users move beyond isolated examples and start building shared modeling vocabulary: common software, communication, electronics, robotics, operations, security, and assurance concepts that can be imported into project models.

## How To Use This Directory

- Browse `technical/` when you need reusable engineering concepts that should stay business-domain neutral.
- Browse `cross-cutting/` when you need governance, security, observability, delivery, or assurance overlays.
- Browse `business/` when you need domain-specific libraries composed from technical and cross-cutting capabilities.
- Use these libraries as starting points for project-specific packages rather than as closed standards.

To validate a model that imports these packages, include the relevant directory as a library root:

```bash
spec42 --library-path domain-libraries check path/to/model.sysml
```

## Layers

- `technical/` - engineering capability libraries that are reusable across industries.
- `cross-cutting/` - overlays such as delivery, observability, security, and assurance that apply across technical/business domains.
- `business/` - business-domain libraries built by composing technical and cross-cutting capabilities.

## Initial Mapping

- `technical/software/`
  - `software-core`
  - `distributed-systems`
  - `data`
  - `platform`
  - `interactions`
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

## Dependency Direction

- Technical libraries must not import business libraries.
- Cross-cutting libraries may import technical libraries, but not business libraries.
- Business libraries may import technical and cross-cutting libraries.
- Foundational packages (`SoftwareCore`, `RoboticsCore`) remain stable to minimize import churn.
