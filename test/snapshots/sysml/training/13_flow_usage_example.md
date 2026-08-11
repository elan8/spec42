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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "13_flow_usage_example.md"
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
        (range (start 6 18) (end 6 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 13) (end 7 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 9) (end 10 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 6) (end 11 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 9) (end 14 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 6) (end 15 38))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e1c21fc60f630af35f595658aa20a15ee602a19bb14861d7d41a6b72ac46f8a4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Flow Usage Example"))) (kind "package") (name "Flow Usage Example") (declared-name "Flow Usage Example") (range (start (line 0) (character 0)) (end (line 0) (character 372))))
    (element (id (node (document "d0") (qualified-name "Flow Usage Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 34))) (parent (node (document "d0") (qualified-name "Flow Usage Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Port Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Flow Usage Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 3) (character 1)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "Flow Usage Example"))))
    (element (id (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 5) (character 1)) (end (line 5) (character 280))) (parent (node (document "d0") (qualified-name "Flow Usage Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 5) (character 16)) (end (line 5) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Flow Usage Example::vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 7) (character 2)) (end (line 7) (character 20))) (parent (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 7) (character 13)) (end (line 7) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Flow Usage Example::vehicle::tankAssy"))) (kind "part") (name "tankAssy") (declared-name "tankAssy") (range (start (line 6) (character 2)) (end (line 6) (character 35))) (parent (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelTankAssembly") (range (start (line 6) (character 18)) (end (line 6) (character 34)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Port Example::*") (range (start (line 1) (character 16)) (end (line 1) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 5) (character 16)) (end (line 5) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flow Usage Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (kind flowSource) (ordinal 0)) (authored-target "tankAssy::fuelTankPort::fuelSupply") (range (start (line 10) (character 9)) (end (line 10) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (kind flowSource) (ordinal 1)) (authored-target "eng::engineFuelPort::fuelReturn") (range (start (line 14) (character 9)) (end (line 14) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (kind flowTarget) (ordinal 0)) (authored-target "eng::engineFuelPort::fuelSupply") (range (start (line 11) (character 6)) (end (line 11) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (kind flowTarget) (ordinal 1)) (authored-target "tankAssy::fuelTankPort::fuelReturn") (range (start (line 15) (character 6)) (end (line 15) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 7) (character 13)) (end (line 7) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelTankAssembly") (range (start (line 6) (character 18)) (end (line 6) (character 34))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (target (node (document "d0") (qualified-name "Flow Usage Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
