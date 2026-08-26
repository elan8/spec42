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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 44))
      )
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
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 13 2) (end 15 3))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 17 2) (end 19 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:cff966f014c18aeb0707ebc6f25bb04248a25c55679d878a6453cdccdefe645c") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (path (named (kind package) (name "Individuals and Roles")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Part Definition Example") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle_1")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0::leftFrontWheel_t0"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel_1")) (redefinition (reference "leftFrontWheel")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1::rightFrontWheel_t1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel_1")) (redefinition (reference "rightFrontWheel")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (path (named (kind package) (name "Individuals and Roles")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
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
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0::leftFrontWheel_t0"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1")))))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0::leftFrontWheel_t0"))) (kind redefinition) (ordinal 0))
      (authored-target "leftFrontWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel")))))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1::rightFrontWheel_t1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1")))))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1::rightFrontWheel_t1"))) (kind redefinition) (ordinal 0))
      (authored-target "rightFrontWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0::leftFrontWheel_t0"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0::leftFrontWheel_t0"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0::leftFrontWheel_t0"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0::leftFrontWheel_t0"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1::rightFrontWheel_t1"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1::rightFrontWheel_t1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1::rightFrontWheel_t1"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1::rightFrontWheel_t1"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0::leftFrontWheel_t0"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1::rightFrontWheel_t1"))) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1")))
      (subtype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel")))
      (featured-by (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1")))
      (type (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")) (source direct))
      (supertype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")) (scopes any))
      (subtype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0::leftFrontWheel_t0")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel")))
      (featured-by (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1")))
      (type (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")) (source direct))
      (supertype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")) (scopes any))
      (subtype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1::rightFrontWheel_t1")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")))
      (subtype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel")) (scopes any))
      (subtype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel")) (scopes any))
      (subtype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1")))
      (supertype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0::leftFrontWheel_t0")) (scopes any))
      (subtype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1::rightFrontWheel_t1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1")))
      (type (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1")) (source direct))
      (supertype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0")))
      (featured-by (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1")))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0::leftFrontWheel_t0")))
      (featured-by (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0")))
      (type (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")) (source inherited) (from (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))))
      (effective-type (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1")) (source direct))
      (supertype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel")) (scopes any feature))
      (supertype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")) (scopes any))
      (supertype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1")))
      (featured-by (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1")))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1::rightFrontWheel_t1")))
      (featured-by (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1")))
      (type (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")) (source inherited) (from (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))))
      (effective-type (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1")) (source direct))
      (supertype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel")) (scopes any feature))
      (supertype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")) (scopes any))
      (supertype (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 1 16) (end 1 44)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (path (named (kind package) (name "Individuals and Roles")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Part Definition Example")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 5 34) (end 5 41)) (probe (position 5 34))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 6 24) (end 6 29)) (probe (position 6 24))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")))))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 7 25) (end 7 30)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")))))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 10 32) (end 10 37)) (probe (position 10 32))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1"))) (kind specialization) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel")))))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 12 29) (end 12 38)) (probe (position 12 29))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1")))))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 14 32) (end 14 39)) (probe (position 14 32))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0::leftFrontWheel_t0"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1")))))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 14 44) (end 14 58)) (probe (position 14 44))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t0::leftFrontWheel_t0"))) (kind redefinition) (ordinal 0) (authored-target "leftFrontWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel")))))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 18 33) (end 18 40)) (probe (position 18 33))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1::rightFrontWheel_t1"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Wheel_1")))))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_roles_1.md") (range (start 18 45) (end 18 60)) (probe (position 18 45))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::vehicle_1::vehicle_1_t1::rightFrontWheel_t1"))) (kind redefinition) (ordinal 0) (authored-target "rightFrontWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_roles_1.md") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel")))))
    )
  )
)
~~~
