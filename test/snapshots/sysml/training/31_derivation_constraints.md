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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c842fe30fd52ea054a67376e72bed44294bdb406a188ac6d98e838c92dd62745") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Derivation Constraints"))) (kind "package") (name "Derivation Constraints") (declared-name "Derivation Constraints") (range (start (line 0) (character 0)) (end (line 0) (character 613))))
    (element (id (node (document "d0") (qualified-name "Derivation Constraints::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 22))) (parent (node (document "d0") (qualified-name "Derivation Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 18))))))
    (element (id (node (document "d0") (qualified-name "Derivation Constraints::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 43))) (parent (node (document "d0") (qualified-name "Derivation Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Constraints Example-1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 39))))))
    (element (id (node (document "d0") (qualified-name "Derivation Constraints::Dynamics"))) (kind "constraint def") (name "Dynamics") (declared-name "Dynamics") (range (start (line 13) (character 1)) (end (line 13) (character 238))) (parent (node (document "d0") (qualified-name "Derivation Constraints"))))
    (element (id (node (document "d0") (qualified-name "Derivation Constraints::vehicle1"))) (kind "part") (name "vehicle1") (declared-name "vehicle1") (range (start (line 4) (character 1)) (end (line 4) (character 149))) (parent (node (document "d0") (qualified-name "Derivation Constraints"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 4) (character 17)) (end (line 4) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "Derivation Constraints::vehicle1::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (range (start (line 5) (character 2)) (end (line 5) (character 34))) (parent (node (document "d0") (qualified-name "Derivation Constraints::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 5) (character 24)) (end (line 5) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Derivation Constraints::vehicle2"))) (kind "part") (name "vehicle2") (declared-name "vehicle2") (range (start (line 9) (character 1)) (end (line 9) (character 112))) (parent (node (document "d0") (qualified-name "Derivation Constraints"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 9) (character 17)) (end (line 9) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "Derivation Constraints::vehicle2::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (range (start (line 10) (character 2)) (end (line 10) (character 82))) (parent (node (document "d0") (qualified-name "Derivation Constraints::vehicle2"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 10) (character 24)) (end (line 10) (character 33)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 1) (character 16)) (end (line 1) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Constraints Example-1::*") (range (start (line 2) (character 16)) (end (line 2) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::vehicle1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 4) (character 17)) (end (line 4) (character 24))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::vehicle1::totalMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::vehicle1::totalMass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 5) (character 24)) (end (line 5) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::vehicle2"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 9) (character 17)) (end (line 9) (character 24))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::vehicle2::totalMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Derivation Constraints::vehicle2::totalMass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 10) (character 24)) (end (line 10) (character 33))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Derivation Constraints::Dynamics")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Derivation Constraints::vehicle2::totalMass")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
