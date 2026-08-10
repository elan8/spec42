# META
~~~ini
description=SysML Training 13 (Flows): Flow Definition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Flow Definition Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	flow def FuelFlow {
		ref :>> payload : Fuel;
		end port supplierPort : FuelOutPort;
		end port consumerPort : FuelInPort;
	}
	
	part vehicle : Vehicle {
		part tankAssy : FuelTankAssembly;
		part eng : Engine;
		
		flow : FuelFlow of Fuel
		  from tankAssy.fuelTankPort.fuelSupply
			to eng.engineFuelPort.fuelSupply;
			
	} 
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwFlow,KwDef,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwEnd,KwPort,Ident,Colon,Ident,Semicolon,
KwEnd,KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwFlow,Colon,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,
KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Flow Definition Example''
    (import_decl private ''Port Example'::*')
    (part_def 'Vehicle')
    (flow_def 'FuelFlow'
      (ref_usage ref :>> 'payload' : 'Fuel')
      (interface_end end 'supplierPort' : 'FuelOutPort')
      (interface_end end 'consumerPort' : 'FuelInPort'))
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'tankAssy' : 'FuelTankAssembly')
      (part_usage 'eng' : 'Engine')
      (flow_usage 'FuelFlow' : 'Fuel'
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package 'Flow Definition Example' {
    private import 'Port Example'::*;

    part def Vehicle;

    flow def FuelFlow {
        ref :>> payload : Fuel;
        end port supplierPort : FuelOutPort;
        end port consumerPort : FuelInPort;
    }

    part vehicle : Vehicle {
        part tankAssy : FuelTankAssembly;
        part eng : Engine;

        flow : FuelFlow of Fuel
        from tankAssy.fuelTankPort.fuelSupply
        to eng.engineFuelPort.fuelSupply;

    }
}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'payload'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'FuelOutPort'
semantic.unresolved_name 'FuelInPort'
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'Fuel'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'payload'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'FuelOutPort'
semantic.unresolved_name 'FuelInPort'
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'Fuel'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Flow Definition Example"))) (name "Flow Definition Example") (declared-name "Flow Definition Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Flow Definition Example::*"))) (name "*") (declared-name "*"))
        (element (kind "flow def") (id (node (document "d0") (qualified-name "Flow Definition Example::FuelFlow"))) (name "FuelFlow") (declared-name "FuelFlow")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Flow Definition Example::FuelFlow::consumerPort"))) (name "consumerPort") (declared-name "consumerPort") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Flow Definition Example::FuelFlow")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Flow Definition Example::FuelFlow::supplierPort"))) (name "supplierPort") (declared-name "supplierPort") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Flow Definition Example::FuelFlow")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Flow Definition Example::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "Flow Definition Example::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Flow Definition Example::vehicle::eng"))) (name "eng") (declared-name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Flow Definition Example::Vehicle")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Flow Definition Example::vehicle::tankAssy"))) (name "tankAssy") (declared-name "tankAssy") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Flow Definition Example::Vehicle")))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flow Definition Example::vehicle"))) (to (node (document "d0") (qualified-name "Flow Definition Example::Vehicle"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Flow Definition Example::FuelFlow"))) (status missing-prerequisite) (target "Flows::MessageAction"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Flow Definition Example::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Flow Definition Example::vehicle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Flow Definition Example::vehicle::eng"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Flow Definition Example::vehicle::tankAssy"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/13_flow_definition_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 2) (end 7 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 2) (end 8 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 18) (end 12 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 13) (end 13 19))
      )
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "sysml")
        (range (start 15 2) (end 15 110))
      )
    )
  )
)
~~~
