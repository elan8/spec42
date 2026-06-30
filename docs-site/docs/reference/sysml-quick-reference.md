# SysML v2 Quick Reference

A concise reference for the most common SysML v2 constructs. For the full language specification, see the [OMG SysML v2 Language Specification](https://www.omg.org/spec/SysML/2.0/Language/).

## Definitions

Definitions declare reusable types. Every usage keyword has a matching `def` form.

| Keyword | Description |
|---------|-------------|
| `part def` | A structural component type. Parts compose a system hierarchy. |
| `item def` | Things that flow between parts (messages, fuel, signals). Supertype of `part def`. |
| `attribute def` | A data value type (scalar, string, quantity). Always referential — no sub-parts. |
| `enum def` | An attribute definition with a fixed set of enumeration literals. |
| `port def` | A typed interaction point through which parts connect or flow. |
| `action def` | A behavioral step or function a part can perform. |
| `state def` | A state machine with entry/do/exit actions and transitions. |
| `connection def` | A typed binary link between two end features (ports or parts). |
| `allocation def` | A binary mapping from a logical element to a physical one. |
| `requirement def` | A formal requirement with a subject, text body, and constraints. |
| `view def` | A specialized `part def` that selects and renders a model subset. |
| `metadata def` | A structured annotation type applied to elements with `@`. |

## Usages

Drop the `def` suffix to get the usage keyword. Usages appear inside definitions and create owned, typed members.

| Keyword | Description |
|---------|-------------|
| `part` | A structural member inside a containing part or package. |
| `item` | Something that flows or is transferred (payload, message, entity). |
| `attribute` | A data property on a definition or usage, always referential. |
| `port` | An interaction point on a part, typed by a `port def`. |
| `action` | A behavioral step: a usage of an `action def`. |
| `state` | A state usage inside a state machine body. |
| `connection` | Instantiates a `connection def` between two end features. |
| `allocate` | Shorthand for an allocation usage: `allocate source to target;` |
| `satisfy requirement` | Asserts that a design element satisfies a named requirement. |
| `expose` | Imports elements into a view (like `import`, but always protected). |

## Relationships & Symbols

| Symbol | Description |
|--------|-------------|
| `:>` | **Specialization / subsetting.** On definitions: inherits and refines a parent type. On usages: subsets an inherited member. |
| `:>>` | **Redefinition.** A usage overrides an inherited member — narrower than subsetting. Also used to set a feature value inline. |
| `::>` | **Reference subsetting.** An end feature references an existing usage without owning it. Used mainly on connection ends. |
| `:` | **Typing.** A usage is classified by a definition. |
| `bind … = …` | **Binding connector.** Declares that two features always have equal values. |
| `= <expr>` | **Feature value.** Assigns a fixed or default value to a feature (not a connector). |

```sysml
part def Truck :> Vehicle { ... }       // specialization
part engine :>> powerUnit;              // redefinition
end part hub ::> mainSwitch;            // reference subsetting
part w : Wheel;                         // typing
bind tank.fuelOut = engine.fuelIn;      // binding connector
attribute mass = 1200 [kg];             // feature value
```

## Annotations & Modifiers

| Keyword | Description |
|---------|-------------|
| `@` | Applies a `metadata def` to an element: `@ Approved { by = "Jane"; }` |
| `doc` | Attaches a documentation string: `doc /* Human-readable description */` |
| `abstract` | Marks a definition or usage as abstract (cannot be instantiated directly). |
| `variation` | Marks a definition whose members are mutually exclusive variant choices. |

## Views & Visualization

| Keyword | Description |
|---------|-------------|
| `view def` | Defines a kind of view with filter conditions and a rendering rule. |
| `view` | A usage of a `view def` that selects concrete elements to expose. |
| `expose` | Selects elements into a view (supports `::*` wildcard and `::**` recursive). |
| `filter` | Boolean condition limiting which elements are included. |
| `render` | Specifies the rendering tool/format for the view. |

## Minimal example

```sysml
package MySystem {

    attribute def Mass { attribute :>> num; attribute :>> mRef = 1 [kg]; }

    part def Engine {
        attribute mass : Mass;
        port powerOut;
    }

    part def Vehicle :> Engine {          // specializes Engine
        part engine : Engine;
        attribute :>> mass = 1200 [kg];   // redefines + sets value
        bind engine.powerOut = powerOut;  // binding connector
    }

    view def Overview {
        expose Vehicle::**;               // all nested elements, recursively
        filter not @SysML::AttributeUsage;
    }
}
```
