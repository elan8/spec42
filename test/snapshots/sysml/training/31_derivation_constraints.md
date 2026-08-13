# META
~~~ini
description=SysML Training 31 (Constraints): Derivation Constraints
type=file
~~~
# SOURCE
~~~sysml
package 'Derivation Constraints' {
	private import SI::*;
	private import 'Constraints Example-1'::*;
	
	part vehicle1 : Vehicle {
		attribute totalMass : MassValue;			
		assert constraint {totalMass == chassisMass + engine.mass + transmission.mass}	
	}
	
	part vehicle2 : Vehicle {
		attribute totalMass : MassValue = chassisMass + engine.mass + transmission.mass;
	}
	
	constraint def Dynamics {
		in mass: MassValue;
		in initialSpeed : SpeedValue;
		in finalSpeed : SpeedValue;
		in deltaT : TimeValue;
		in force : ForceValue;

		force * deltaT == mass * (finalSpeed - initialSpeed) and
		mass > 0[kg]
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/31_derivation_constraints.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 17) (end 4 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 24) (end 5 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 6 2) (end 6 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 17) (end 9 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 24) (end 10 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 1) (end 22 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:b103183131f6dc9065479456841ca5d51390221aaf1718370a2dd0d824fff34b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/31_derivation_constraints.md") (qualified-name "Derivation Constraints"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_derivation_constraints.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_derivation_constraints.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Constraints Example-1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_derivation_constraints.md") (qualified-name "Derivation Constraints::vehicle1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/31_derivation_constraints.md") (qualified-name "Derivation Constraints::vehicle1::totalMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/31_derivation_constraints.md") (qualified-name "Derivation Constraints::vehicle2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/31_derivation_constraints.md") (qualified-name "Derivation Constraints::vehicle2::totalMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/31_derivation_constraints.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_derivation_constraints.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Constraints Example-1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_derivation_constraints.md") (qualified-name "Derivation Constraints::vehicle1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_derivation_constraints.md") (qualified-name "Derivation Constraints::vehicle1::totalMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_derivation_constraints.md") (qualified-name "Derivation Constraints::vehicle2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_derivation_constraints.md") (qualified-name "Derivation Constraints::vehicle2::totalMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
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
  (query (document "memory://snapshot/31_derivation_constraints.md") (range (start 1 16) (end 1 21)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/31_derivation_constraints.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_derivation_constraints.md") (range (start 2 16) (end 2 42)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/31_derivation_constraints.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Constraints Example-1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_derivation_constraints.md") (range (start 4 17) (end 4 24)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/31_derivation_constraints.md") (qualified-name "Derivation Constraints::vehicle1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_derivation_constraints.md") (range (start 5 24) (end 5 33)) (probe (position 5 24))
    (reference (id (source (node (document "memory://snapshot/31_derivation_constraints.md") (qualified-name "Derivation Constraints::vehicle1::totalMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_derivation_constraints.md") (range (start 9 17) (end 9 24)) (probe (position 9 17))
    (reference (id (source (node (document "memory://snapshot/31_derivation_constraints.md") (qualified-name "Derivation Constraints::vehicle2"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_derivation_constraints.md") (range (start 10 24) (end 10 33)) (probe (position 10 24))
    (reference (id (source (node (document "memory://snapshot/31_derivation_constraints.md") (qualified-name "Derivation Constraints::vehicle2::totalMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
)
~~~
