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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 39))
      )
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:394e3aff4c70bccf48382dd297df0ea08aede2973f794ce5bf1cc3775328ad63") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Transition Actions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0))))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller"))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind) (value (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "vehicleController")))))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle"))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind) (value (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle::vehicleController"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleController")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Transition Actions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "vehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle::vehicleController")))))
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle::vehicleController"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleController")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle::vehicleController"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0))))) (target (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller"))))) (target (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle"))))) (target (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle::vehicleController"))) (target (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")))))
      (featured-by (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")))))
      (featured-by (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle::vehicleController")))
      (featured-by (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/26_state_exhibition_example.md") (range (start 1 16) (end 1 39)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Transition Actions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/26_state_exhibition_example.md") (range (start 3 16) (end 3 23)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/26_state_exhibition_example.md") (range (start 9 19) (end 9 36)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "controller")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "vehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle::vehicleController")))))
    )
  )
  (query (document "memory://snapshot/26_state_exhibition_example.md") (range (start 8 25) (end 8 32)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (path (named (kind package) (name "State Exhibition Example")) (named (kind part) (name "vehicle")) (anonymous (kind state) (ordinal 0)) (named (kind parameter) (name "operatingVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle")))))
    )
  )
  (query (document "memory://snapshot/26_state_exhibition_example.md") (range (start 5 27) (end 5 44)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/26_state_exhibition_example.md") (qualified-name "State Exhibition Example::vehicle::vehicleController"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleController")
      (outcome (status unresolved)))
    )
  )
)
~~~
