# Behavior and Requirements

Use actions to describe work, states to describe lifecycle, and requirements to capture obligations. These elements can be connected, but they answer different questions: what happens, what condition the system is in, and what must be true.

## Build behavior from actions

An action definition names a reusable behavior. Inside another action, action usages and successions make an ordered flow explicit.

```sysml
package ControllerBehavior {
    action def ReadSensor;
    action def UpdateDisplay;

    action def RefreshDisplay {
        action read : ReadSensor;
        action update : UpdateDisplay;
        first read then update;
    }
}
```

A part can declare an action it performs:

```sysml
part def DisplayController {
    perform action refresh : RefreshDisplay;
}
```

Actions can also declare parameters, control nodes, sends, accepts, and additional successions. The available editor and view support is intentionally narrower than the full language specification; [Supported Workflows](https://github.com/elan8/spec42/blob/main/docs/user/SUPPORTED-WORKFLOWS.md) describes the current boundary.

## Model a state lifecycle

A state definition can own named state usages and transitions. This small machine waits for an event before changing state.

```sysml
package DoorBehavior {
    item def OpenCommand;
    state def Closed;
    state def Open;
    state def Locked;

    state def DoorStates {
        entry;
        then closed;
        state closed : Closed;
        state open : Open;
        state locked : Locked;
        transition open_door first closed accept OpenCommand then open;
        transition close_door first open then locked;
        final locked;
    }
}
```

The [Diagram Visualizer](./visualizer) can render supported state-transition views. Treat its diagram as a presentation of the resolved model, not a second place to define lifecycle semantics.

## Capture and satisfy a requirement

Use `requirement def` for a reusable requirement form and `requirement` for a requirement usage in a model context. A `satisfy` relationship links that requirement usage to the implementing usage.

```sysml
package AccessControl {
    part def Lock;

    requirement def LocksWhenClosed {
        doc /* The system shall lock when the door is closed. */
    }

    part lock : Lock;
    requirement lockWhenClosed : LocksWhenClosed;
    satisfy lockWhenClosed by lock;
}
```

Satisfaction records traceability; it does not by itself prove the requirement. Keep the requirement text, its constrained subject or condition, and any verification evidence explicit in the model and its engineering workflow.

Next: [Validation and diagnostics](./validation-and-diagnostics).
