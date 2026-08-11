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
  (document "28_individuals_and_roles_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 5 34) (end 5 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 24) (end 6 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 25) (end 7 30))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 13 2) (end 13 201))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "ddc7839d525f5f08f9247427e5750b72acf146740fb7a4bb7e0b3988f089a4db") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Individuals and Roles"))) (kind "package") (name "Individuals and Roles") (declared-name "Individuals and Roles"))
    (element (id (node (document "d0") (qualified-name "Individuals and Roles::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Individuals and Roles"))) (authored (membership (kind Import) (visibility "private") (import (reference "Part Definition Example::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1"))) (kind "part def") (name "Vehicle_1") (declared-name "Vehicle_1") (parent (node (document "d0") (qualified-name "Individuals and Roles"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (kind "part") (name "leftFrontWheel") (declared-name "leftFrontWheel") (parent (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (kind "part") (name "rightFrontWheel") (declared-name "rightFrontWheel") (parent (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "Individuals and Roles::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "Individuals and Roles"))))
    (element (id (node (document "d0") (qualified-name "Individuals and Roles::Wheel_1"))) (kind "part def") (name "Wheel_1") (declared-name "Wheel_1") (parent (node (document "d0") (qualified-name "Individuals and Roles"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "Individuals and Roles::vehicle_1"))) (kind "part") (name "vehicle_1") (declared-name "vehicle_1") (parent (node (document "d0") (qualified-name "Individuals and Roles"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle_1")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Roles::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Part Definition Example::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Roles::Wheel_1"))) (kind specialization) (ordinal 0)) (authored-target "Wheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "Individuals and Roles::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Roles::vehicle_1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle_1") (outcome (status resolved) (target (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Individuals and Roles::Wheel_1"))) (target (node (document "d0") (qualified-name "Individuals and Roles::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Individuals and Roles::Wheel_1"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Individuals and Roles::vehicle_1"))) (target (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Individuals and Roles::vehicle_1"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 6 24) (end 6 29)) (probe (position 6 24))
      (reference
        (source (document "d0") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 6 24) (end 6 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 25) (end 7 30)) (probe (position 7 25))
      (reference
        (source (document "d0") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 7 25) (end 7 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 32) (end 10 37)) (probe (position 10 32))
      (reference
        (source (document "d0") (qualified-name "Individuals and Roles::Wheel_1"))
        (kind specialization) (ordinal 0) (authored-target "Wheel")
        (range (start 10 32) (end 10 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Individuals and Roles::Wheel") (range (start 3 1) (end 3 16)))
        )
      )
    )
    (query (range (start 5 34) (end 5 41)) (probe (position 5 34))
      (reference
        (source (document "d0") (qualified-name "Individuals and Roles::Vehicle_1"))
        (kind specialization) (ordinal 0) (authored-target "Vehicle")
        (range (start 5 34) (end 5 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 29) (end 12 38)) (probe (position 12 29))
      (reference
        (source (document "d0") (qualified-name "Individuals and Roles::vehicle_1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle_1")
        (range (start 12 29) (end 12 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Individuals and Roles::Vehicle_1") (range (start 5 1) (end 5 109)))
        )
      )
    )
    (query (range (start 1 16) (end 1 41)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Individuals and Roles::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Part Definition Example::*")
        (range (start 1 16) (end 1 41))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
