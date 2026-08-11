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
  (document "31_constraint_assertions_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 34))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 13 1) (end 13 146))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 21 2) (end 21 129))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 2) (end 26 36))
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
        (range (start 29 3) (end 29 30))
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
        (range (start 33 3) (end 33 30))
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
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "ab0db59faa4337f266a6fec8fdd401c1798b03ecf9bfce5f4a8aba4491bccd49") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2"))) (kind "package") (name "Constraint Assertions-2") (declared-name "Constraint Assertions-2"))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::MassConstraint"))) (kind "constraint def") (name "MassConstraint") (declared-name "MassConstraint") (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::chassisMass"))) (kind "attribute") (name "chassisMass") (declared-name "chassisMass") (parent (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::massConstraint"))) (kind "constraint") (name "massConstraint") (declared-name "massConstraint") (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassConstraint")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::chassisMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::chassisMass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Constraint Assertions-2::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Constraint Assertions-2::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::massConstraint"))) (kind featureTyping) (ordinal 0)) (authored-target "MassConstraint") (outcome (status resolved) (target (node (document "d0") (qualified-name "Constraint Assertions-2::MassConstraint")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (target (node (document "d0") (qualified-name "Constraint Assertions-2::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (target (node (document "d0") (qualified-name "Constraint Assertions-2::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Constraint Assertions-2::massConstraint"))) (target (node (document "d0") (qualified-name "Constraint Assertions-2::MassConstraint"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Constraint Assertions-2::massConstraint"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Constraint Assertions-2::massConstraint")) (expression (status "unresolved") (error "expression has an unresolved reference")) (analysis (status "unresolved")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 16) (end 2 18)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Constraint Assertions-2::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 2 16) (end 2 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 19)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Constraint Assertions-2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 1 16) (end 1 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 28 16) (end 28 22)) (probe (position 28 16))
      (reference
        (source (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 28 16) (end 28 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Constraint Assertions-2::Engine") (range (start 5 1) (end 5 17)))
        )
      )
    )
    (query (range (start 32 22) (end 32 28)) (probe (position 32 22))
      (reference
        (source (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 32 22) (end 32 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Constraint Assertions-2::Engine") (range (start 5 1) (end 5 17)))
        )
      )
    )
    (query (range (start 26 26) (end 26 35)) (probe (position 26 26))
      (reference
        (source (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::chassisMass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 26 26) (end 26 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 29 20) (end 29 29)) (probe (position 29 20))
      (reference
        (source (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 29 20) (end 29 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 20) (end 33 29)) (probe (position 33 20))
      (reference
        (source (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 33 20) (end 33 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 34)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "Constraint Assertions-2::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions::*")
        (range (start 3 16) (end 3 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
