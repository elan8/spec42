# META
~~~ini
description=SysML Training 31 (Constraints): Constraint Assertions-2
type=file
~~~
# SOURCE
~~~sysml
package 'Constraint Assertions-2' {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	part def Engine;
	part def Transmission;
	
	constraint def MassConstraint {
		in partMasses : MassValue[0..*];
		in massLimit : MassValue;
	}
	
	constraint massConstraint : MassConstraint {
		in partMasses : MassValue[0..*];
		in massLimit : MassValue;
			
		sum(partMasses) <= massLimit
	}
	
	part def Vehicle {
		assert massConstraint {
			in partMasses = (chassisMass, engine.mass, transmission.mass);
			in massLimit = 2500[kg];
		}
		
		attribute chassisMass : MassValue;
		
		part engine : Engine {
			attribute mass : MassValue;
		}
		
		part transmission : Engine {
			attribute mass : MassValue;
		}
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/31_constraint_assertions_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 18) (end 9 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 17) (end 10 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 18) (end 14 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 17) (end 15 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 17 2) (end 17 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 21 2) (end 24 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 26) (end 26 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 20) (end 29 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 20) (end 33 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:6e436aa60e2789297c06a200b91b91fed61c45875d253d75869a4e233bea8f26") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::MassConstraint"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::MassConstraint::massLimit"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::MassConstraint::partMasses"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::chassisMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::engine::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::transmission::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::massConstraint"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassConstraint"))))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::massConstraint::massLimit"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::massConstraint::partMasses"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::MassConstraint::massLimit"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::MassConstraint::partMasses"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::chassisMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Engine")))))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::engine::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Engine")))))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::transmission::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::massConstraint"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassConstraint")
      (outcome (status resolved) (target (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::MassConstraint")))))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::massConstraint::massLimit"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::massConstraint::partMasses"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (target (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (target (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::massConstraint"))) (target (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::MassConstraint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::massConstraint"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/31_constraint_assertions_2.md") (range (start 1 16) (end 1 22)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraint_assertions_2.md") (range (start 2 16) (end 2 21)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraint_assertions_2.md") (range (start 3 16) (end 3 37)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraint_assertions_2.md") (range (start 10 17) (end 10 26)) (probe (position 10 17))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::MassConstraint::massLimit"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraint_assertions_2.md") (range (start 9 18) (end 9 27)) (probe (position 9 18))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::MassConstraint::partMasses"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraint_assertions_2.md") (range (start 26 26) (end 26 35)) (probe (position 26 26))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::chassisMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraint_assertions_2.md") (range (start 28 16) (end 28 22)) (probe (position 28 16))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Engine")))))
  )
  (query (document "memory://snapshot/31_constraint_assertions_2.md") (range (start 29 20) (end 29 29)) (probe (position 29 20))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::engine::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraint_assertions_2.md") (range (start 32 22) (end 32 28)) (probe (position 32 22))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Engine")))))
  )
  (query (document "memory://snapshot/31_constraint_assertions_2.md") (range (start 33 20) (end 33 29)) (probe (position 33 20))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::Vehicle::transmission::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraint_assertions_2.md") (range (start 13 29) (end 13 43)) (probe (position 13 29))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::massConstraint"))) (kind featureTyping) (ordinal 0) (authored-target "MassConstraint")
      (outcome (status resolved) (target (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::MassConstraint")))))
  )
  (query (document "memory://snapshot/31_constraint_assertions_2.md") (range (start 15 17) (end 15 26)) (probe (position 15 17))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::massConstraint::massLimit"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraint_assertions_2.md") (range (start 14 18) (end 14 27)) (probe (position 14 18))
    (reference (id (source (node (document "memory://snapshot/31_constraint_assertions_2.md") (qualified-name "Constraint Assertions-2::massConstraint::partMasses"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
)
~~~
