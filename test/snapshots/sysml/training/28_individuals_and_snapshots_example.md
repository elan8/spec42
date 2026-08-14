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
  (document "memory://snapshot/28_individuals_and_snapshots_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 43))
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
        (source "parser")
        (range (start 5 2) (end 13 2))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 5 2) (end 13 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 8) (end 21 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 26) (end 21 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:b258241d80976f581202691a49bd3dc13faf10a7cb0fc91a5f0fe26e74b83a19") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Part Definition Example") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "vehicle_1_t0")) (succession (reference "vehicle_1_t1")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Part Definition Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "vehicle_1_t0")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "vehicle_1_t1")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 1 15) (end 1 43)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Part Definition Example")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 3 34) (end 3 41)) (probe (position 3 34))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 21 8) (end 21 20)) (probe (position 21 8))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "vehicle_1_t0")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 21 26) (end 21 38)) (probe (position 21 26))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "vehicle_1_t1")
      (outcome (status unresolved)))
    )
  )
)
~~~
