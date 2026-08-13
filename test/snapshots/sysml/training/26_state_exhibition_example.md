# META
~~~ini
description=SysML Training 26 (State Exhibition): State Exhibition Example
type=file
~~~
# SOURCE
~~~sysml
package 'State Exhibition Example' {
	private import 'Transition Actions'::*;
	
	part vehicle : Vehicle {
		
		part vehicleController : VehicleController;
		
		exhibit vehicleStates {
			in operatingVehicle = vehicle;
			in controller = vehicleController;
		}

	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/26_state_exhibition_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 16) (end 3 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 27) (end 5 44))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:394e3aff4c70bccf48382dd297df0ea08aede2973f794ce5bf1cc3775328ad63") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Transition Actions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (anonymous (kind state) (ordinal 0))))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle::::controller"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle::::operatingVehicle"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle::vehicleController"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleController"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Transition Actions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle::vehicleController"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleController")
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
  (query (document "memory://snapshot/26_state_exhibition_example.md") (range (start 1 16) (end 1 39)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Transition Actions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/26_state_exhibition_example.md") (range (start 3 16) (end 3 23)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/26_state_exhibition_example.md") (range (start 5 27) (end 5 44)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle::vehicleController"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleController")
      (outcome (status unresolved)))
  )
)
~~~
