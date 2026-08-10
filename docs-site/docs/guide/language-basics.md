# Definitions, Usages, and Specialization

Most SysML v2 models become easier to read once you separate three ideas: a reusable type, an occurrence of that type, and a more-specific type.

## Define a reusable type

A definition describes a kind of model element that can be reused. A usage places a named occurrence in a model. In this example, `Vehicle` and `ElectricVehicle` are definitions; `vehicle` is a usage owned by `Garage`.

```sysml
package VehicleModel {
    part def Vehicle {
        attribute mass;
    }

    part def ElectricVehicle :> Vehicle {
        attribute batteryCapacity;
    }

    part def Garage {
        part vehicle : ElectricVehicle;
    }
}
```

The same pattern appears throughout the language: `part def` / `part`, `attribute def` / `attribute`, `port def` / `port`, `action def` / `action`, and `requirement def` / `requirement`.

## Typing and specialization are different

The two operators in the example have different jobs:

| Form | Meaning | Example |
| --- | --- | --- |
| `: Type` | A usage is typed by a definition. | `part vehicle : ElectricVehicle;` |
| `:> Base` | A definition specializes a more-general definition. | `part def ElectricVehicle :> Vehicle` |

Typing says what a particular member is. Specialization says that one definition is a kind of another definition and can inherit its features. Do not use `:>` merely to give a part usage a type.

When you need to adjust an inherited feature, use the explicit SysML specialization forms for subsetting or redefinition. The [SysML v2 Quick Reference](../reference/sysml-quick-reference) summarizes those operators; keep the original authored relationship visible rather than treating an inherited effective value as though it were declared locally.

## A practical modeling flow

1. Define stable vocabulary first: parts, values, ports, actions, or requirements.
2. Specialize a definition when a new type truly inherits from a base type.
3. Create usages where those types occur in a particular structure or behavior.
4. Inspect a usage in the [Feature Inspector](./feature-inspector) when you need to distinguish its declared type from inherited or resolved information.

This keeps type design separate from the structure that uses it, which also makes multi-file organization more straightforward.

Next: [Packages and multi-file workspaces](./packages-and-workspaces).
