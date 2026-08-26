# META
~~~ini
description=SysML Training 08 (Items): Items Example
type=file
~~~
# SOURCE
~~~sysml
package 'Items Example' {
	private import ScalarValues::*;
	
	item def Fuel;
	item def Person;
	
	part def Vehicle {
		attribute mass : Real;
		
		ref item driver : Person;

		part fuelTank {
			item fuel: Fuel;
		}		
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/08_items_example.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 19) (end 7 23))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 11 2) (end 13 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:f9b280e7e1f643151513a7985884a425eea5f643b599a947a57892cba08a9e5c") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (path (named (kind package) (name "Items Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Fuel"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Person"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::driver"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank::fuel"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel")))))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/08_items_example.md") (path (named (kind package) (name "Items Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Person")))))
    (reference (id (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank::fuel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::driver"))) (target (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank::fuel"))) (target (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank::fuel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::driver"))) (target (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank"))) (target (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank::fuel"))) (target (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::mass"))) (target (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Fuel")))
      (subtype (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank::fuel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Person")))
      (subtype (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::driver")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::driver")))
      (featured-by (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle")))
      (type (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Person")) (source direct))
      (supertype (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank")))
      (featured-by (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank::fuel")))
      (featured-by (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank")))
      (type (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Fuel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Fuel")) (source direct))
      (supertype (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Fuel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/08_items_example.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/08_items_example.md") (path (named (kind package) (name "Items Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/08_items_example.md") (range (start 9 20) (end 9 26)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Person")))))
    )
  )
  (query (document "memory://snapshot/08_items_example.md") (range (start 12 14) (end 12 18)) (probe (position 12 14))
    (reference (id (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank::fuel"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Fuel")))))
    )
  )
  (query (document "memory://snapshot/08_items_example.md") (range (start 7 19) (end 7 23)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
)
~~~
