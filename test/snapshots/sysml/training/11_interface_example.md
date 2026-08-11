# META
~~~ini
description=SysML Training 11 (Interfaces): Interface Example
type=file
~~~
# SOURCE
~~~sysml
package 'Interface Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	interface def FuelInterface {
		end supplierPort : FuelOutPort;
		end consumerPort : FuelInPort;
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
  (document "11_interface_example.md"
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 18) (end 11 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 13) (end 12 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 20) (end 15 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 20) (end 16 38))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Interface Example' {
    private import 'Port Example'::*;

    part def Vehicle;

    interface def FuelInterface {
        end supplierPort : FuelOutPort;
        end consumerPort : FuelInPort;
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6df93f3caecb225ed597f54b87a3fdea3e2e2d428817c6f9c4a2d8cd016c4c85") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Interface Example"))) (kind "package") (name "Interface Example") (declared-name "Interface Example") (range (start (line 0) (character 0)) (end (line 0) (character 408))))
    (element (id (node (document "d0") (qualified-name "Interface Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 34))) (parent (node (document "d0") (qualified-name "Interface Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Port Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Interface Example::FuelInterface"))) (kind "interface def") (name "FuelInterface") (declared-name "FuelInterface") (range (start (line 5) (character 1)) (end (line 5) (character 100))) (parent (node (document "d0") (qualified-name "Interface Example"))))
    (element (id (node (document "d0") (qualified-name "Interface Example::FuelInterface::consumerPort"))) (kind "interface end") (name "consumerPort") (declared-name "consumerPort") (range (start (line 7) (character 2)) (end (line 7) (character 32))) (parent (node (document "d0") (qualified-name "Interface Example::FuelInterface"))) (authored (relationships (typing (reference "FuelInPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interface Example::FuelInterface::supplierPort"))) (kind "interface end") (name "supplierPort") (declared-name "supplierPort") (range (start (line 6) (character 2)) (end (line 6) (character 33))) (parent (node (document "d0") (qualified-name "Interface Example::FuelInterface"))) (authored (relationships (typing (reference "FuelOutPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interface Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 3) (character 1)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "Interface Example"))))
    (element (id (node (document "d0") (qualified-name "Interface Example::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 10) (character 1)) (end (line 10) (character 214))) (parent (node (document "d0") (qualified-name "Interface Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 10) (character 16)) (end (line 10) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Interface Example::vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 12) (character 2)) (end (line 12) (character 20))) (parent (node (document "d0") (qualified-name "Interface Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 12) (character 13)) (end (line 12) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Interface Example::vehicle::tankAssy"))) (kind "part") (name "tankAssy") (declared-name "tankAssy") (range (start (line 11) (character 2)) (end (line 11) (character 35))) (parent (node (document "d0") (qualified-name "Interface Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelTankAssembly") (range (start (line 11) (character 18)) (end (line 11) (character 34)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Port Example::*") (range (start (line 1) (character 16)) (end (line 1) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::FuelInterface::consumerPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelInPort") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::FuelInterface::supplierPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelOutPort") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 10) (character 16)) (end (line 10) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interface Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::vehicle"))) (kind connectionSource) (ordinal 0)) (authored-target "tankAssy::fuelTankPort") (range (start (line 15) (character 20)) (end (line 15) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::vehicle"))) (kind connectionTarget) (ordinal 0)) (authored-target "eng::engineFuelPort") (range (start (line 16) (character 20)) (end (line 16) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 12) (character 13)) (end (line 12) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelTankAssembly") (range (start (line 11) (character 18)) (end (line 11) (character 34))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Interface Example::vehicle"))) (target (node (document "d0") (qualified-name "Interface Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interface Example::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
