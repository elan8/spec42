# META
~~~ini
description=SysML Training 13 (Flows): Flow Interface Example
type=file
~~~
# SOURCE
~~~sysml
package 'Flow Interface Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	interface def FuelInterface {
		end supplierPort : FuelOutPort;
		end consumerPort : FuelInPort;
		
		flow supplierPort.fuelSupply to consumerPort.fuelSupply;			
		flow consumerPort.fuelReturn to supplierPort.fuelReturn;
	}
	
	part vehicle : Vehicle {	
		part tankAssy : FuelTankAssembly;		
		part eng : Engine;
		
		interface : FuelInterface connect 
			supplierPort ::> tankAssy.fuelTankPort to 
			consumerPort ::> eng.engineFuelPort;
	} 
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwInterface,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwInterface,Colon,Ident,KwConnect,
Ident,ColonColonGt,Ident,Dot,Ident,KwTo,
Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Flow Interface Example''
    (import_decl private ''Port Example'::*')
    (part_def 'Vehicle')
    (interface_def 'FuelInterface'
      (interface_end end 'supplierPort' : 'FuelOutPort')
      (interface_end end 'consumerPort' : 'FuelInPort')
      (flow_usage 'supplierPort')
      (flow_usage 'consumerPort'))
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'tankAssy' : 'FuelTankAssembly')
      (part_usage 'eng' : 'Engine')
      (interface_usage 'FuelInterface'
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package 'Flow Interface Example' {
    private import 'Port Example'::*;

    part def Vehicle;

    interface def FuelInterface {
        end supplierPort : FuelOutPort;
        end consumerPort : FuelInPort;

        flow supplierPort.fuelSupply to consumerPort.fuelSupply;
        flow consumerPort.fuelReturn to supplierPort.fuelReturn;
    }

    part vehicle : Vehicle {
        part tankAssy : FuelTankAssembly;
        part eng : Engine;

        interface : FuelInterface connect
        supplierPort ::> tankAssy.fuelTankPort to
        consumerPort ::> eng.engineFuelPort;
    }
}

~~~
# EXPECTED
~~~
semantic.duplicate_name 'supplierPort'
semantic.duplicate_name 'consumerPort'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'FuelOutPort'
semantic.unresolved_name 'FuelInPort'
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'tankAssy::fuelTankPort'
semantic.unresolved_name 'eng::engineFuelPort'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'supplierPort'
semantic.duplicate_name 'consumerPort'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'FuelOutPort'
semantic.unresolved_name 'FuelInPort'
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'tankAssy::fuelTankPort'
semantic.unresolved_name 'eng::engineFuelPort'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Flow Interface Example"))) (name "Flow Interface Example") (declared-name "Flow Interface Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Flow Interface Example::*"))) (name "*") (declared-name "*"))
        (element (kind "interface def") (id (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface"))) (name "FuelInterface") (declared-name "FuelInterface")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface::consumerPort"))) (name "consumerPort") (declared-name "consumerPort") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface::supplierPort"))) (name "supplierPort") (declared-name "supplierPort") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Flow Interface Example::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "Flow Interface Example::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Flow Interface Example::vehicle::eng"))) (name "eng") (declared-name "eng") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Flow Interface Example::Vehicle")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Flow Interface Example::vehicle::tankAssy"))) (name "tankAssy") (declared-name "tankAssy") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Flow Interface Example::Vehicle")))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flow Interface Example::vehicle"))) (to (node (document "d0") (qualified-name "Flow Interface Example::Vehicle"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (connection (status pending-expression) (document "d0") (source-expression "tankAssy::fuelTankPort") (target-expression "eng::engineFuelPort") (container-prefix "Flow Interface Example::vehicle") (interface-usage true) (interface-type "FuelInterface"))
  )
)
~~~
