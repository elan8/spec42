# META
~~~ini
description=SysML Training 28 (Individuals): Individuals and Roles-1
type=file
~~~
# SOURCE
~~~sysml
package 'Individuals and Roles' {
	private import 'Part Definition Example'::*;
	
	part def Wheel;
	
	individual part def Vehicle_1 :> Vehicle {
		part leftFrontWheel : Wheel;
		part rightFrontWheel : Wheel;
	}
	
	individual part def Wheel_1 :> Wheel;
	
	individual part vehicle_1 : Vehicle_1 {
		snapshot part vehicle_1_t0 {
			snapshot leftFrontWheel_t0 : Wheel_1 :>> leftFrontWheel;
		}
		
		then snapshot part vehicle_1_t1 {
			snapshot rightFrontWheel_t1 : Wheel_1 :>> rightFrontWheel;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/28_individuals_and_roles_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 5 34) (end 5 41))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 13 2) (end 20 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:cff966f014c18aeb0707ebc6f25bb04248a25c55679d878a6453cdccdefe645c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (path (named (kind package) (name "Individuals and Roles")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Part Definition Example") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle_1"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (path (named (kind package) (name "Individuals and Roles")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Part Definition Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1"))) (kind specialization) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel")))
      (supertype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel")))
      (supertype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1")))
      (supertype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1")))
      (supertype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 1 16) (end 1 44)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (path (named (kind package) (name "Individuals and Roles")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "Part Definition Example")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 5 34) (end 5 41)) (probe (position 5 34))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 6 24) (end 6 29)) (probe (position 6 24))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")))))
  )
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 7 25) (end 7 30)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")))))
  )
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 10 32) (end 10 37)) (probe (position 10 32))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1"))) (kind specialization) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")))))
  )
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 12 29) (end 12 38)) (probe (position 12 29))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1")))))
  )
)
~~~
