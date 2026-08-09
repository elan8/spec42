# Packages and Multi-File Workspaces

Packages give model elements qualified names. A workspace lets those packages live in separate `.sysml` or `.kerml` files while remaining one model for editing and validation.

## Keep packages aligned with model concerns

A useful starting layout groups files by concern, not by a required filename convention:

```text
my-model/
  model/
    architecture/
      powertrain.sysml
    system/
      vehicle.sysml
    requirements/
      vehicle-requirements.sysml
```

Each file may declare one or more packages. Matching a filename to its principal package is a readable convention, not a language requirement.

```sysml
// model/architecture/powertrain.sysml
package Powertrain {
    part def Motor;
}
```

## Import a package member into scope

Use an import when a package's members should be available by their simple names. This consumer can refer to `Motor` after importing the direct members of `Powertrain`.

```sysml
// model/system/vehicle.sysml
package VehicleAssembly {
    private import Powertrain::*;

    part def Vehicle {
        part motor : Motor;
    }
}
```

For a name that is clearer when fully spelled out, use its qualified name instead:

```sysml
part def Vehicle {
    part motor : Powertrain::Motor;
}
```

`::*` selects direct package members. Use imports deliberately: a small explicit import set makes ownership and name resolution easier to understand than a broad namespace pull-in.

## Validate the workspace, not just one file

Pass the directory containing the related files to `spec42 check` so cross-file definitions, imports, and relationships are analyzed together:

```bash
spec42 check model --workspace-root .
```

Directory validation discovers `.sysml` and `.kerml` files recursively. `--workspace-root` makes the model root explicit when the path being checked is a subdirectory or file. In VS Code, open the workspace folder so the language server can index the same model scope.

If an import or qualified name cannot be resolved, check that the defining file is under the workspace path, that the package path is correct, and that any external library root is configured. See [Library & Dependencies](./libraries) for library setup and [Validation and diagnostics](./validation-and-diagnostics) for the command-line workflow.

Next: [Attributes, parts, and connections](./structure-and-values).
