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
  (document "08_items_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 2) (end 7 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 19) (end 7 23))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "dbcdfd0cb66ec00a9b48308d56557e51a2fd5b66f8b5ec4882aca513005a774a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Items Example"))) (kind "package") (name "Items Example") (declared-name "Items Example") (range (start (line 0) (character 0)) (end (line 0) (character 224))))
    (element (id (node (document "d0") (qualified-name "Items Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "Items Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Items Example::Fuel"))) (kind "item def") (name "Fuel") (declared-name "Fuel") (range (start (line 3) (character 1)) (end (line 3) (character 15))) (parent (node (document "d0") (qualified-name "Items Example"))))
    (element (id (node (document "d0") (qualified-name "Items Example::Person"))) (kind "item def") (name "Person") (declared-name "Person") (range (start (line 4) (character 1)) (end (line 4) (character 17))) (parent (node (document "d0") (qualified-name "Items Example"))))
    (element (id (node (document "d0") (qualified-name "Items Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 6) (character 1)) (end (line 6) (character 123))) (parent (node (document "d0") (qualified-name "Items Example"))))
    (element (id (node (document "d0") (qualified-name "Items Example::Vehicle::fuelTank"))) (kind "part") (name "fuelTank") (declared-name "fuelTank") (range (start (line 11) (character 2)) (end (line 11) (character 41))) (parent (node (document "d0") (qualified-name "Items Example::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Items Example::Vehicle::item"))) (kind "opaque member") (name "item") (declared-name "item") (range (start (line 9) (character 2)) (end (line 9) (character 27))) (parent (node (document "d0") (qualified-name "Items Example::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Items Example::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 7) (character 2)) (end (line 7) (character 24))) (parent (node (document "d0") (qualified-name "Items Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 7) (character 19)) (end (line 7) (character 23)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Items Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items Example::Vehicle::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items Example::Vehicle::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 7) (character 19)) (end (line 7) (character 23))) (outcome (status unresolved)))
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
    (query (range (start 7 19) (end 7 23)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "Items Example::Vehicle::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 7 19) (end 7 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Items Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
