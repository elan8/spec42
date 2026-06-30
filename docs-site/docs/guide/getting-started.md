# Getting Started

## Installation

Install the **Spec42** extension from the VS Code Marketplace.

Once installed, open a folder containing `.sysml` or `.kerml` files and the language server starts automatically.

## Your first model

The fastest way to get started is to open one of the built-in examples:

1. Open the **Spec42** tab in the left sidebar.
2. Under **Examples**, click **timer** (marked with a star — this is the recommended starting point).
3. The workspace opens with a complete SysML v2 model.

You can also create a new file with a `.sysml` extension and start writing:

```sysml
package MyFirstModel {

    part def Sensor {
        attribute name : String;
        port dataOut;
    }

    part def Controller {
        port dataIn;
    }

    part def System {
        part sensor : Sensor;
        part controller : Controller;
        connect sensor.dataOut to controller.dataIn;
    }
}
```

## Opening the visualizer

The diagram visualizer lives in the **secondary sidebar** (right side of VS Code).

- Open it via **View → Secondary Side Bar**, or
- Use the **Help** section in the Spec42 sidebar and click **Open diagram visualizer**.

Once open, select a view from the dropdown in the toolbar to render your model as a diagram.

## Next steps

- [Browse the examples](./examples) to see what Spec42 can do.
- [Learn the visualizer](./visualizer) to get the most out of the diagrams.
- [SysML v2 Quick Reference](../reference/sysml-quick-reference) for language syntax.
