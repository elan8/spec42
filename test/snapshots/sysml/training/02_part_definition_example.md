# META
~~~ini
description=SysML Training 02 (Part Definitions): Part Definition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Part Definition Example' {
	private import ScalarValues::*;
	
	part def Vehicle {
		attribute mass : Real;
		attribute status : VehicleStatus;
		
		part eng : Engine;
		
		ref part driver : Person;
	}
	
	attribute def VehicleStatus {
		attribute gearSetting : Integer;
		attribute acceleratorPosition : Real;
	}
	
	part def Engine;	
	part def Person;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/02_part_definition_example.md"
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
        (range (start 4 19) (end 4 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 26) (end 13 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 34) (end 14 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:acdb51406f99c0c590ae7188a5528530f6059abf3d66f527e8fb367a820d8898") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/02_part_definition_example.md") (path (name "Part Definition Example") (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Person"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::driver"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person"))))
    (declaration (id (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::status"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleStatus"))))
    (declaration (id (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::VehicleStatus"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::VehicleStatus::acceleratorPosition"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::VehicleStatus::gearSetting"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/02_part_definition_example.md") (path (name "Part Definition Example") (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Person")))))
    (reference (id (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Engine")))))
    (reference (id (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::status"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleStatus")
      (outcome (status resolved) (target (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::VehicleStatus")))))
    (reference (id (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::VehicleStatus::acceleratorPosition"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::VehicleStatus::gearSetting"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::driver"))) (target (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::eng"))) (target (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::status"))) (target (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::VehicleStatus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::status"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/02_part_definition_example.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/02_part_definition_example.md") (path (name "Part Definition Example") (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/02_part_definition_example.md") (range (start 9 20) (end 9 26)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Person")))))
  )
  (query (document "memory://snapshot/02_part_definition_example.md") (range (start 7 13) (end 7 19)) (probe (position 7 13))
    (reference (id (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Engine")))))
  )
  (query (document "memory://snapshot/02_part_definition_example.md") (range (start 4 19) (end 4 23)) (probe (position 4 19))
    (reference (id (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/02_part_definition_example.md") (range (start 5 21) (end 5 34)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::Vehicle::status"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleStatus")
      (outcome (status resolved) (target (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::VehicleStatus")))))
  )
  (query (document "memory://snapshot/02_part_definition_example.md") (range (start 14 34) (end 14 38)) (probe (position 14 34))
    (reference (id (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::VehicleStatus::acceleratorPosition"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/02_part_definition_example.md") (range (start 13 26) (end 13 33)) (probe (position 13 26))
    (reference (id (source (node (document "memory://snapshot/02_part_definition_example.md") (qualified-name "Part Definition Example::VehicleStatus::gearSetting"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
  )
)
~~~
