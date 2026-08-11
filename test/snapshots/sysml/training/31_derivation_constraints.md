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
  (document "31_derivation_constraints.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 39))
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
        (range (start 5 2) (end 5 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 24) (end 5 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 17) (end 9 24))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 10 2) (end 10 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 2) (end 10 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 24) (end 10 33))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c842fe30fd52ea054a67376e72bed44294bdb406a188ac6d98e838c92dd62745") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Derivation Constraints"))) (kind "package") (name "Derivation Constraints") (declared-name "Derivation Constraints"))
    (element (id (node (document "d0") (qualified-name "Derivation Constraints::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Derivation Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Derivation Constraints::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Derivation Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Constraints Example-1::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Derivation Constraints::Dynamics"))) (kind "constraint def") (name "Dynamics") (declared-name "Dynamics") (parent (node (document "d0") (qualified-name "Derivation Constraints"))))
    (element (id (node (document "d0") (qualified-name "Derivation Constraints::vehicle1"))) (kind "part") (name "vehicle1") (declared-name "vehicle1") (parent (node (document "d0") (qualified-name "Derivation Constraints"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Derivation Constraints::vehicle1::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (parent (node (document "d0") (qualified-name "Derivation Constraints::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Derivation Constraints::vehicle2"))) (kind "part") (name "vehicle2") (declared-name "vehicle2") (parent (node (document "d0") (qualified-name "Derivation Constraints"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Derivation Constraints::vehicle2::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (parent (node (document "d0") (qualified-name "Derivation Constraints::vehicle2"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Constraints Example-1::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::vehicle1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::vehicle1::totalMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::vehicle1::totalMass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::vehicle2"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::vehicle2::totalMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::vehicle2::totalMass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Derivation Constraints::Dynamics")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Derivation Constraints::vehicle2::totalMass")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 18)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Derivation Constraints::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 1 16) (end 1 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 4 17) (end 4 24)) (probe (position 4 17))
      (reference
        (source (document "d0") (qualified-name "Derivation Constraints::vehicle1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 4 17) (end 4 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 17) (end 9 24)) (probe (position 9 17))
      (reference
        (source (document "d0") (qualified-name "Derivation Constraints::vehicle2"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 9 17) (end 9 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 24) (end 5 33)) (probe (position 5 24))
      (reference
        (source (document "d0") (qualified-name "Derivation Constraints::vehicle1::totalMass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 5 24) (end 5 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 24) (end 10 33)) (probe (position 10 24))
      (reference
        (source (document "d0") (qualified-name "Derivation Constraints::vehicle2::totalMass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 10 24) (end 10 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 39)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Derivation Constraints::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Constraints Example-1::*")
        (range (start 2 16) (end 2 39))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
