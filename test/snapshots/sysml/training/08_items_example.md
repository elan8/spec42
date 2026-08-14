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
        (severity error)
        (code "recovered_part_def_body_element")
        (source "parser")
        (range (start 9 2) (end 11 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:f9b280e7e1f643151513a7985884a425eea5f643b599a947a57892cba08a9e5c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (path (named (kind package) (name "Items Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Fuel"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Person"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank::fuel"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel")))))
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/08_items_example.md") (path (named (kind package) (name "Items Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank::fuel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank::fuel"))) (target (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank::fuel"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Vehicle::fuelTank::fuel")))
      (supertype (node (document "memory://snapshot/08_items_example.md") (qualified-name "Items Example::Fuel")) (scopes any))
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
