# META
~~~ini
description=SysML Training 12 (Binding Connectors): Binding Connectors Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Binding Connectors Example-2' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	part def FuelPump;
	part def FuelTank;
	
	part vehicle : Vehicle {	
		part tank : FuelTankAssembly {
			port redefines fuelTankPort {
				out item redefines fuelSupply;
				in item redefines fuelReturn;
			}
			
			part pump : FuelPump {
				out item pumpOut : Fuel = fuelTankPort.fuelSupply;
				in item pumpIn : Fuel;
			}
			
			part tank : FuelTank {
				out item fuelOut : Fuel;
				in item fuelIn : Fuel = fuelTankPort.fuelReturn;
			}
		}
	} 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "12_binding_connectors_example_2.md"
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
        (range (start 8 14) (end 8 30))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 15 4) (end 15 85))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 20 4) (end 20 85))
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
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,KwRedefines,Ident,OpenCurly,
KwOut,KwItem,KwRedefines,Ident,Semicolon,
KwIn,KwItem,KwRedefines,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwOut,KwItem,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
KwIn,KwItem,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Binding Connectors Example-2''
    (import_decl private ''Port Example'::*')
    (part_def 'Vehicle')
    (part_def 'FuelPump')
    (part_def 'FuelTank')
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'tank' : 'FuelTankAssembly'
        (port_usage :>> 'fuelTankPort'
          (item_usage out :>> 'fuelSupply')
          (item_usage in :>> 'fuelReturn'))
        (part_usage 'pump' : 'FuelPump'
          (item_usage out 'pumpOut' : 'Fuel' value)
          (item_usage in 'pumpIn' : 'Fuel'))
        (part_usage 'tank' : 'FuelTank'
          (item_usage out 'fuelOut' : 'Fuel')
          (item_usage in 'fuelIn' : 'Fuel' value))))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'fuelTankPort'
semantic.unresolved_name 'fuelSupply'
semantic.unresolved_name 'fuelReturn'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'Fuel'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'fuelTankPort'
semantic.unresolved_name 'fuelSupply'
semantic.unresolved_name 'fuelReturn'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'Fuel'
~~~
# FORMAT
~~~sysml
package 'Binding Connectors Example-2' {
    private import 'Port Example'::*;

    part def Vehicle;
    part def FuelPump;
    part def FuelTank;

    part vehicle : Vehicle {
        part tank : FuelTankAssembly {
            port redefines fuelTankPort {
                out item redefines fuelSupply;
                in item redefines fuelReturn;
            }

            part pump : FuelPump {
                out item pumpOut : Fuel = fuelTankPort.fuelSupply;
                in item pumpIn : Fuel;
            }

            part tank : FuelTank {
                out item fuelOut : Fuel;
                in item fuelIn : Fuel = fuelTankPort.fuelReturn;
            }
        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "5f091724d4201101968b698453d4114f702cfc546874dea05c74a1c3a169e237") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-2"))) (kind "package") (name "Binding Connectors Example-2") (declared-name "Binding Connectors Example-2") (range (start (line 0) (character 0)) (end (line 0) (character 549))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 34))) (parent (node (document "d0") (qualified-name "Binding Connectors Example-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Port Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-2::FuelPump"))) (kind "part def") (name "FuelPump") (declared-name "FuelPump") (range (start (line 4) (character 1)) (end (line 4) (character 19))) (parent (node (document "d0") (qualified-name "Binding Connectors Example-2"))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-2::FuelTank"))) (kind "part def") (name "FuelTank") (declared-name "FuelTank") (range (start (line 5) (character 1)) (end (line 5) (character 19))) (parent (node (document "d0") (qualified-name "Binding Connectors Example-2"))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-2::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 3) (character 1)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "Binding Connectors Example-2"))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 7) (character 1)) (end (line 7) (character 407))) (parent (node (document "d0") (qualified-name "Binding Connectors Example-2"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 7) (character 16)) (end (line 7) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank"))) (kind "part") (name "tank") (declared-name "tank") (range (start (line 8) (character 2)) (end (line 8) (character 377))) (parent (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelTankAssembly") (range (start (line 8) (character 14)) (end (line 8) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::fuelTankPort"))) (kind "port") (name "fuelTankPort") (declared-name "fuelTankPort") (range (start (line 9) (character 3)) (end (line 9) (character 106))) (parent (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelTankPort") (range (start (line 9) (character 18)) (end (line 9) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::pump"))) (kind "part") (name "pump") (declared-name "pump") (range (start (line 14) (character 3)) (end (line 14) (character 112))) (parent (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelPump") (range (start (line 14) (character 15)) (end (line 14) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::tank"))) (kind "part") (name "tank") (declared-name "tank") (range (start (line 19) (character 3)) (end (line 19) (character 112))) (parent (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelTank") (range (start (line 19) (character 15)) (end (line 19) (character 23)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Port Example::*") (range (start (line 1) (character 16)) (end (line 1) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 7) (character 16)) (end (line 7) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Binding Connectors Example-2::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelTankAssembly") (range (start (line 8) (character 14)) (end (line 8) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::fuelTankPort"))) (kind redefinition) (ordinal 0)) (authored-target "fuelTankPort") (range (start (line 9) (character 18)) (end (line 9) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::fuelTankPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::pump"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelPump") (range (start (line 14) (character 15)) (end (line 14) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Binding Connectors Example-2::FuelPump")))))
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::tank"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelTank") (range (start (line 19) (character 15)) (end (line 19) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Binding Connectors Example-2::FuelTank")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle"))) (target (node (document "d0") (qualified-name "Binding Connectors Example-2::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::fuelTankPort"))) (target (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::fuelTankPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::fuelTankPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::pump"))) (target (node (document "d0") (qualified-name "Binding Connectors Example-2::FuelPump"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::pump"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::tank"))) (target (node (document "d0") (qualified-name "Binding Connectors Example-2::FuelTank"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::tank"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
