# META
~~~ini
description=SysML Training 13 (Flows): Flow Usage Example
type=file
~~~
# SOURCE
~~~sysml
package 'Flow Usage Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	part vehicle : Vehicle {
		part tankAssy : FuelTankAssembly;
		part eng : Engine;
		
		flow of Fuel
		  from tankAssy.fuelTankPort.fuelSupply
			to eng.engineFuelPort.fuelSupply;
			
		flow of Fuel
		  from eng.engineFuelPort.fuelReturn
			to tankAssy.fuelTankPort.fuelReturn;
	} 
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwFlow,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,
KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFlow,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,
KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Flow Usage Example''
    (import_decl private ''Port Example'::*')
    (part_def 'Vehicle')
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'tankAssy' : 'FuelTankAssembly')
      (part_usage 'eng' : 'Engine')
      (flow_usage 'of')
      (flow_usage 'of'))))
~~~
# FORMAT
~~~sysml
package 'Flow Usage Example' {
    private import 'Port Example'::*;

    part def Vehicle;

    part vehicle : Vehicle {
        part tankAssy : FuelTankAssembly;
        part eng : Engine;

        flow of Fuel
        from tankAssy.fuelTankPort.fuelSupply
        to eng.engineFuelPort.fuelSupply;

        flow of Fuel
        from eng.engineFuelPort.fuelReturn
        to tankAssy.fuelTankPort.fuelReturn;
    }
}

~~~
# EXPECTED
~~~
semantic.duplicate_name 'of'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'Engine'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'of'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'Engine'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Flow Usage Example"))) (name "Flow Usage Example") (declared-name "Flow Usage Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Flow Usage Example::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Flow Usage Example::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Flow Usage Example::vehicle::eng"))) (name "eng") (declared-name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Flow Usage Example::Vehicle")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Flow Usage Example::vehicle::tankAssy"))) (name "tankAssy") (declared-name "tankAssy") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Flow Usage Example::Vehicle")))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (to (node (document "d0") (qualified-name "Flow Usage Example::Vehicle"))))
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
  (document "sysml/training/13_flow_usage_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 18) (end 6 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 13) (end 7 19))
      )
    )
  )
)
~~~
