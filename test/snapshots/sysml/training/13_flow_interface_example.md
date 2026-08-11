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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "13_flow_interface_example.md"
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
        (range (start 6 2) (end 6 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 2) (end 7 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 7) (end 9 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 34) (end 9 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 7) (end 10 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 34) (end 10 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 18) (end 14 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 13) (end 15 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 20) (end 18 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 20) (end 19 38))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "31183c67023ea64975868a5f7302d1a89a1cc5fda8f903b94b03f0f3c94b89ab") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Flow Interface Example"))) (kind "package") (name "Flow Interface Example") (declared-name "Flow Interface Example") (range (start (line 0) (character 0)) (end (line 0) (character 537))))
    (element (id (node (document "d0") (qualified-name "Flow Interface Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 34))) (parent (node (document "d0") (qualified-name "Flow Interface Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Port Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface"))) (kind "interface def") (name "FuelInterface") (declared-name "FuelInterface") (range (start (line 5) (character 1)) (end (line 5) (character 224))) (parent (node (document "d0") (qualified-name "Flow Interface Example"))))
    (element (id (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface::consumerPort"))) (kind "interface end") (name "consumerPort") (declared-name "consumerPort") (range (start (line 7) (character 2)) (end (line 7) (character 32))) (parent (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface"))) (authored (relationships (typing (reference "FuelInPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface::supplierPort"))) (kind "interface end") (name "supplierPort") (declared-name "supplierPort") (range (start (line 6) (character 2)) (end (line 6) (character 33))) (parent (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface"))) (authored (relationships (typing (reference "FuelOutPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flow Interface Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 3) (character 1)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "Flow Interface Example"))))
    (element (id (node (document "d0") (qualified-name "Flow Interface Example::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 13) (character 1)) (end (line 13) (character 214))) (parent (node (document "d0") (qualified-name "Flow Interface Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 13) (character 16)) (end (line 13) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Flow Interface Example::vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 15) (character 2)) (end (line 15) (character 20))) (parent (node (document "d0") (qualified-name "Flow Interface Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 15) (character 13)) (end (line 15) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Flow Interface Example::vehicle::tankAssy"))) (kind "part") (name "tankAssy") (declared-name "tankAssy") (range (start (line 14) (character 2)) (end (line 14) (character 35))) (parent (node (document "d0") (qualified-name "Flow Interface Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelTankAssembly") (range (start (line 14) (character 18)) (end (line 14) (character 34)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Flow Interface Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Port Example::*") (range (start (line 1) (character 16)) (end (line 1) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface"))) (kind flowSource) (ordinal 0)) (authored-target "supplierPort::fuelSupply") (range (start (line 9) (character 7)) (end (line 9) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface"))) (kind flowSource) (ordinal 1)) (authored-target "consumerPort::fuelReturn") (range (start (line 10) (character 7)) (end (line 10) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface"))) (kind flowTarget) (ordinal 0)) (authored-target "consumerPort::fuelSupply") (range (start (line 9) (character 34)) (end (line 9) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface"))) (kind flowTarget) (ordinal 1)) (authored-target "supplierPort::fuelReturn") (range (start (line 10) (character 34)) (end (line 10) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface::consumerPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelInPort") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Interface Example::FuelInterface::supplierPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelOutPort") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Interface Example::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 13) (character 16)) (end (line 13) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flow Interface Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Flow Interface Example::vehicle"))) (kind connectionSource) (ordinal 0)) (authored-target "tankAssy::fuelTankPort") (range (start (line 18) (character 20)) (end (line 18) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Interface Example::vehicle"))) (kind connectionTarget) (ordinal 0)) (authored-target "eng::engineFuelPort") (range (start (line 19) (character 20)) (end (line 19) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Interface Example::vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 15) (character 13)) (end (line 15) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Interface Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelTankAssembly") (range (start (line 14) (character 18)) (end (line 14) (character 34))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flow Interface Example::vehicle"))) (target (node (document "d0") (qualified-name "Flow Interface Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flow Interface Example::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
