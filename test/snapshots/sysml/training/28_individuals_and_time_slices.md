# META
~~~ini
description=SysML Training 28 (Individuals): Individuals and Time Slices
type=file
~~~
# SOURCE
~~~sysml
package 'Individuals and Time Slices' {
	private import 'Individuals and Snapshots Example'::*;
	
	individual item def Alice :> Person;
	individual item def Bob :> Person;
	
	individual : Vehicle_1 {
		
		timeslice aliceDriving {
			ref individual item :>> driver : Alice;

			snapshot :>> start {
				:>> mass = 2000.0;
			}
			
			snapshot :>> done {
				:>> mass = 1500.0;
			}			
		}
		
		then timeslice bobDriving {
			ref individual item :>> driver : Bob;
		}
		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/28_individuals_and_time_slices.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 3 30) (end 3 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 4 28) (end 4 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 6 12) (end 24 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:0cd6b18f9e0f56c83b2bd04b31954c8933a32cf235596a7b9dc36065798f6956") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Individuals and Snapshots Example") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Alice"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Person"))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Bob"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Person"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Individuals and Snapshots Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Alice"))) (kind specialization) (ordinal 0))
      (authored-target "Person")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Bob"))) (kind specialization) (ordinal 0))
      (authored-target "Person")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 1 16) (end 1 54)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Individuals and Snapshots Example")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 3 30) (end 3 36)) (probe (position 3 30))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Alice"))) (kind specialization) (ordinal 0) (authored-target "Person")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 4 28) (end 4 34)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Bob"))) (kind specialization) (ordinal 0) (authored-target "Person")
      (outcome (status unresolved)))
  )
)
~~~
