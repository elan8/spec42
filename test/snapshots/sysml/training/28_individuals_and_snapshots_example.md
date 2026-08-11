# META
~~~ini
description=SysML Training 28 (Individuals): Individuals and Snapshots Example
type=file
~~~
# SOURCE
~~~sysml
package 'Individuals and Snapshots Example' {
	public import 'Part Definition Example'::*;
	
	individual part def Vehicle_1 :> Vehicle {
		
		snapshot part vehicle_1_t0 {
			:>> mass = 2000.0;
			:>> status {
				:>> gearSetting = 0;
				:>> acceleratorPosition = 0.0;
			}
		}
		
		snapshot part vehicle_1_t1 {
			:>> mass = 1500.0;
			:>> status {
				:>> gearSetting = 2;
				:>> acceleratorPosition = 0.5;
			}
		}
		
		first vehicle_1_t0 then vehicle_1_t1;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "28_individuals_and_snapshots_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 3 34) (end 3 41))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 5 2) (end 5 143))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 5 2) (end 5 143))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "441947d6f85108b620190eb3115d90d76ac5f4d84d86cfbab5ccd319a35221f9") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Individuals and Snapshots Example"))) (kind "package") (name "Individuals and Snapshots Example") (declared-name "Individuals and Snapshots Example"))
    (element (id (node (document "d0") (qualified-name "Individuals and Snapshots Example::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Individuals and Snapshots Example"))) (authored (membership (kind Import) (visibility "public") (import (reference "Part Definition Example::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))) (kind "part def") (name "Vehicle_1") (declared-name "Vehicle_1") (parent (node (document "d0") (qualified-name "Individuals and Snapshots Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Snapshots Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Part Definition Example::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 34) (end 3 41)) (probe (position 3 34))
      (reference
        (source (document "d0") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))
        (kind specialization) (ordinal 0) (authored-target "Vehicle")
        (range (start 3 34) (end 3 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 15) (end 1 40)) (probe (position 1 15))
      (reference
        (source (document "d0") (qualified-name "Individuals and Snapshots Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Part Definition Example::*")
        (range (start 1 15) (end 1 40))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
