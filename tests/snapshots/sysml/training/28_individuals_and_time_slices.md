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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 54))
      )
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 14) (end 6 23))
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
        (range (start 12 8) (end 12 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 16) (end 15 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 8) (end 16 12))
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
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:0cd6b18f9e0f56c83b2bd04b31954c8933a32cf235596a7b9dc36065798f6956") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Individuals and Snapshots Example") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle_1")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving"))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice)))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers individual reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Alice")) (redefinition (reference "driver")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "start")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 1))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "done")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving"))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice)))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving")) (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers individual reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bob")) (redefinition (reference "driver")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Alice"))) (kind item-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Bob"))) (kind item-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Person")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Individuals and Snapshots Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle_1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Alice")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Alice")))))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "driver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "done")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving")) (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Bob")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Bob")))))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "driver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Alice"))) (kind specialization) (ordinal 0))
      (authored-target "Person")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Bob"))) (kind specialization) (ordinal 0))
      (authored-target "Person")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Alice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving")) (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Bob"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving")) (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving"))))) (target (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 0))))) (target (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 1))))) (target (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving"))))) (target (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving")) (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving"))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind real) (real 2000)))
    (evaluated (declaration (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind real) (real 1500)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind item) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")))))
      (type (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Alice")) (provenance authored))
      (effective-type (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Alice")) (source direct))
      (supertype (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Alice")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")))))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")))))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving")))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving")) (anonymous (kind item) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving")))))
      (type (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Bob")) (provenance authored))
      (effective-type (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Bob")) (source direct))
      (supertype (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Bob")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Alice")))
      (subtype (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind item) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Bob")))
      (subtype (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving")) (anonymous (kind item) (ordinal 0)))) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 1 16) (end 1 54)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Individuals and Snapshots Example")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 6 14) (end 6 23)) (probe (position 6 14))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle_1")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 9 36) (end 9 41)) (probe (position 9 36))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Alice")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Alice")))))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 9 27) (end 9 33)) (probe (position 9 27))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "driver")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 11 16) (end 11 21)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "start")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 15 16) (end 15 20)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "done")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 12 8) (end 12 12)) (probe (position 12 8))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 16 8) (end 16 12)) (probe (position 16 8))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "aliceDriving")) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 21 36) (end 21 39)) (probe (position 21 36))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving")) (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Bob")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Bob")))))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 21 27) (end 21 33)) (probe (position 21 27))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (path (named (kind package) (name "Individuals and Time Slices")) (anonymous (kind occurrence) (ordinal 0)) (named (kind occurrence) (name "bobDriving")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "driver")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 3 30) (end 3 36)) (probe (position 3 30))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Alice"))) (kind specialization) (ordinal 0) (authored-target "Person")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_time_slices.md") (range (start 4 28) (end 4 34)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_time_slices.md") (qualified-name "Individuals and Time Slices::Bob"))) (kind specialization) (ordinal 0) (authored-target "Person")
      (outcome (status unresolved)))
    )
  )
)
~~~
