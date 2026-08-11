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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "13_flow_definition_example.md"
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "616d9df1d8eac0a03c300a33c28332612d5ad0ec7b44112e2d2176bf7ab408bb") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Flow Definition Example"))) (kind "package") (name "Flow Definition Example") (declared-name "Flow Definition Example") (range (start (line 0) (character 0)) (end (line 0) (character 423))))
    (element (id (node (document "d0") (qualified-name "Flow Definition Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 34))) (parent (node (document "d0") (qualified-name "Flow Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Port Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Flow Definition Example::FuelFlow"))) (kind "flow def") (name "FuelFlow") (declared-name "FuelFlow") (range (start (line 5) (character 1)) (end (line 5) (character 126))) (parent (node (document "d0") (qualified-name "Flow Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Flow Definition Example::FuelFlow::consumerPort"))) (kind "interface end") (name "consumerPort") (declared-name "consumerPort") (range (start (line 8) (character 2)) (end (line 8) (character 37))) (parent (node (document "d0") (qualified-name "Flow Definition Example::FuelFlow"))) (authored (relationships (typing (reference "FuelInPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flow Definition Example::FuelFlow::supplierPort"))) (kind "interface end") (name "supplierPort") (declared-name "supplierPort") (range (start (line 7) (character 2)) (end (line 7) (character 38))) (parent (node (document "d0") (qualified-name "Flow Definition Example::FuelFlow"))) (authored (relationships (typing (reference "FuelOutPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flow Definition Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 3) (character 1)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "Flow Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Flow Definition Example::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 11) (character 1)) (end (line 11) (character 197))) (parent (node (document "d0") (qualified-name "Flow Definition Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 11) (character 16)) (end (line 11) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Flow Definition Example::vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 13) (character 2)) (end (line 13) (character 20))) (parent (node (document "d0") (qualified-name "Flow Definition Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 13) (character 13)) (end (line 13) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Flow Definition Example::vehicle::tankAssy"))) (kind "part") (name "tankAssy") (declared-name "tankAssy") (range (start (line 12) (character 2)) (end (line 12) (character 35))) (parent (node (document "d0") (qualified-name "Flow Definition Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelTankAssembly") (range (start (line 12) (character 18)) (end (line 12) (character 34)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Flow Definition Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Port Example::*") (range (start (line 1) (character 16)) (end (line 1) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Definition Example::FuelFlow::consumerPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelInPort") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Definition Example::FuelFlow::supplierPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelOutPort") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Definition Example::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 11) (character 16)) (end (line 11) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flow Definition Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Flow Definition Example::vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 13) (character 13)) (end (line 13) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Definition Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelTankAssembly") (range (start (line 12) (character 18)) (end (line 12) (character 34))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flow Definition Example::vehicle"))) (target (node (document "d0") (qualified-name "Flow Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flow Definition Example::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
