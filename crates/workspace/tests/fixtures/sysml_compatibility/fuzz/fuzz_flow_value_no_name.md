# META
~~~ini
description=Fuzz: flow usage with value and typing but no name preserves value in formatting
type=file
~~~
# SOURCE
~~~sysml
package P {
    part vehicle : Vehicle {
        part eng : Engine;

        flow = FuelFlow of Fuel
            from tank.fuelSupply
                to eng.engineFuelPort;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'Fuel'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'Fuel'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwFlow,Eq,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,
KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'eng' : 'Engine')
      (flow_usage : 'Fuel' value
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package P {
    part vehicle : Vehicle {
        part eng : Engine;

        flow = FuelFlow of Fuel from tank.fuelSupply to eng.engineFuelPort;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'P'
      (part_usage 'vehicle' : 'Vehicle'[unresolved]
        (part_usage composite 'eng' : 'Engine'[unresolved])
        (flow_usage composite : 'Fuel'[unresolved]
          (feature_value (=))
          (connector_end 'tank.fuelSupply')
          (connector_end 'eng.engineFuelPort'))))))
~~~
