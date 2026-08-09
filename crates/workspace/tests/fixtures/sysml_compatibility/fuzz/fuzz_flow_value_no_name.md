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

        flow = FuelFlow of Fuel
        from tank.fuelSupply
        to eng.engineFuelPort;
    }
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "P"))) (name "P") (declared-name "P")
      (contains
        (element (kind "part") (id (node (document "d0") (qualified-name "P::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "P::vehicle::eng"))) (name "eng") (declared-name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
          )
        )
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz/fuzz_flow_value_no_name.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 19) (end 1 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 19) (end 2 25))
      )
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "sysml")
        (range (start 4 8) (end 4 108))
      )
    )
  )
)
~~~
