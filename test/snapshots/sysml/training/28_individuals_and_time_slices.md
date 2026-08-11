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
  (document "28_individuals_and_time_slices.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 51))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 3 1) (end 3 39))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 3 1) (end 3 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 12) (end 6 297))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 18) (end 9 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 27) (end 9 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 16) (end 11 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 16) (end 15 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 18) (end 21 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 27) (end 21 33))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "b53e77bc3a07ff75f3b156bdb80be8cf06d3384b978a6454046fe8953286b117") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices"))) (kind "package") (name "Individuals and Time Slices") (declared-name "Individuals and Time Slices"))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::"))) (kind "occurrence") (name "") (parent (node (document "d0") (qualified-name "Individuals and Time Slices"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle_1")))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Individuals and Time Slices"))) (authored (membership (kind Import) (visibility "private") (import (reference "Individuals and Snapshots Example::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving"))) (kind "occurrence") (name "aliceDriving") (declared-name "aliceDriving") (parent (node (document "d0") (qualified-name "Individuals and Time Slices::"))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::"))) (kind "occurrence") (name "") (parent (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "start")))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence"))) (kind "occurrence") (name "") (parent (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "done")))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass")))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass")))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::item"))) (kind "occurrence") (name "item") (declared-name "item") (parent (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving"))) (authored (membership (kind Feature)) (relationships (typing (reference "Alice")) (redefinition (reference "driver")))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving"))) (kind "occurrence") (name "bobDriving") (declared-name "bobDriving") (parent (node (document "d0") (qualified-name "Individuals and Time Slices::"))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving::item"))) (kind "occurrence") (name "item") (declared-name "item") (parent (node (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving"))) (authored (membership (kind Feature)) (relationships (typing (reference "Bob")) (redefinition (reference "driver")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle_1") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Individuals and Snapshots Example::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::"))) (kind redefinition) (ordinal 0)) (authored-target "start") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence"))) (kind redefinition) (ordinal 0)) (authored-target "done") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (outcome (status resolved) (target (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (outcome (status resolved) (target (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::item"))) (kind featureTyping) (ordinal 0)) (authored-target "Alice") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::item"))) (kind redefinition) (ordinal 0)) (authored-target "driver") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving::item"))) (kind featureTyping) (ordinal 0)) (authored-target "Bob") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving::item"))) (kind redefinition) (ordinal 0)) (authored-target "driver") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass"))) (target (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass"))) (target (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass")) (expression (status "ok") (value (integer 1500))))
    (node (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass")) (expression (status "ok") (value (integer 2000))))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 15 16) (end 15 20)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence"))
        (kind redefinition) (ordinal 0) (authored-target "done")
        (range (start 15 16) (end 15 20))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 21)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::"))
        (kind redefinition) (ordinal 0) (authored-target "start")
        (range (start 11 16) (end 11 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 27) (end 9 33)) (probe (position 9 27))
      (reference
        (source (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::item"))
        (kind redefinition) (ordinal 0) (authored-target "driver")
        (range (start 9 27) (end 9 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 27) (end 21 33)) (probe (position 21 27))
      (reference
        (source (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving::item"))
        (kind redefinition) (ordinal 0) (authored-target "driver")
        (range (start 21 27) (end 21 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 4) (end 12 12)) (probe (position 12 4))
      (reference
        (source (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 12 4) (end 12 12))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass") (range (start 12 4) (end 12 22)))
        )
      )
    )
    (query (range (start 16 4) (end 16 12)) (probe (position 16 4))
      (reference
        (source (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 16 4) (end 16 12))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass") (range (start 16 4) (end 16 22)))
        )
      )
    )
    (query (range (start 1 16) (end 1 51)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Individuals and Time Slices::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Individuals and Snapshots Example::*")
        (range (start 1 16) (end 1 51))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
