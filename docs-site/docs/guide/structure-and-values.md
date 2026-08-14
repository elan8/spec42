# Attributes, Parts, and Connections

Structural models combine owned parts, their value-like attributes, and ports that describe interaction points. Keep those roles separate: a part composes the system, an attribute records a property, and a port is an endpoint for interaction.

## Model values with attributes

An `attribute def` gives a reusable value type. An `attribute` usage adds a property to a definition or usage; it may be typed and may carry an authored value expression.

```sysml
package VehicleModel {
    attribute def MassValue;

    part def Vehicle {
        attribute mass : MassValue;
        attribute wheelCount = 4;
    }
}
```

Use multiplicity when the number of values matters:

```sysml
part def Vehicle {
    attribute wheelCount [4];
    attribute option [0..*];
}
```

Spec42 keeps declared expressions separate from any resolved or evaluated value. The [Feature Inspector](./feature-inspector) is the right place to inspect both when evaluation is available; the source remains the authority for what was authored.

## Compose a system from parts

A part usage has its own name and is typed by a part definition. Reusing a definition for multiple usages does not make the usages the same element.

```sysml
part def Battery;
part def Controller;

part def System {
    part battery : Battery;
    part controller : Controller;
}
```

Use a nested part path, such as `battery.powerOut`, when referring to a feature owned by a contained part.

## Model interaction endpoints with ports

Ports give explicit interaction endpoints. This example defines a port type, puts port usages on parts, and connects the contained endpoints in the owning structure:

```sysml
package PowerSystem {
    port def ElectricalPort;

    part def Battery {
        port powerOut : ElectricalPort;
    }

    part def Controller {
        port powerIn : ElectricalPort;
    }

    part def System {
        part battery : Battery;
        part controller : Controller;
        connect battery.powerOut to controller.powerIn;
    }
}
```

The connection belongs to `System`, where both endpoint paths are meaningful. A logical `connect` can also relate part usages directly. When a relationship form requires ports, validation can report unresolved, ambiguous, incompatible, or non-port endpoints; when endpoints are ports, Spec42 checks their compatibility. For larger interfaces, introduce explicit interface or connection definitions when the recurring endpoint pattern has a stable meaning.

Next: [Behavior and requirements](./behavior-and-requirements).
