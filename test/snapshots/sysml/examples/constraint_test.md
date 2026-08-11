# META
~~~ini
description=SysML Example (Simple Tests): ConstraintTest
type=file
~~~
# SOURCE
~~~sysml
package ConstraintTest {
	private import ISQ::MassValue;
	private import SI::kg;
	private import NumericalFunctions::sum;
	
	constraint def MassAnalysis {
		attribute totalMass: MassValue;
		attribute componentMasses: MassValue[0..*];		

		totalMass == sum(componentMasses)
	}
	
	part def Component {
		attribute mass: MassValue;
	}
	
	part vehicle : Component {	
		part engine : Component;
		part frontAxleAssembly : Component;
		part rearAxleAssembly : Component;	
	}
		
	part vehicle1a :> vehicle {
		assert constraint massAnalysis : MassAnalysis {
			attribute redefines totalMass;
			attribute redefines componentMasses;
		}
		
		bind massAnalysis.totalMass = mass;
		bind massAnalysis.componentMasses = engine.mass;
		bind massAnalysis.componentMasses = frontAxleAssembly.mass;
		bind massAnalysis.componentMasses = rearAxleAssembly.mass;
	}
	
	part vehicle1b :> vehicle {		
		assert constraint massAnalysis : MassAnalysis {
			attribute redefines totalMass = mass;
			attribute redefines componentMasses = (engine.mass, frontAxleAssembly.mass, rearAxleAssembly.mass);		
		}	
	}
		
	constraint def MassAnalysis2 { 
		in totalMass : MassValue;
		in componentMasses: MassValue[0..*];
		
		totalMass == sum(componentMasses)
	}
	
	part vehicle2a :> vehicle {
		assert constraint massConstraint : MassAnalysis2;
		
		bind massConstraint.totalMass = mass;
		bind massConstraint.componentMasses = engine.mass;
		bind massConstraint.componentMasses = frontAxleAssembly.mass;
		bind massConstraint.componentMasses = rearAxleAssembly.mass;
	}
		
	part vehicle2b :> vehicle {
		assert constraint massAnalysis2 : MassAnalysis2 {
			in totalMass = mass;
			in componentMasses = (engine.mass, frontAxleAssembly.mass, rearAxleAssembly.mass);
		}
	}
	
	constraint def MassAnalysis3 {
		in totalMass : MassValue;
		in componentMasses: MassValue[0..*];
	}
	
	constraint massAnalysis3 : MassAnalysis3 {
		in totalMass : MassValue;
		in componentMasses: MassValue[0..*];
		
		totalMass == sum(componentMasses)
	}
	
	part vehicle3 :> vehicle {
		assert massAnalysis3 {
			in totalMass = mass;
			in componentMasses = (engine.mass, frontAxleAssembly.mass, rearAxleAssembly.mass);
		}
	}
	
	part vehicle4 :> vehicle {
		assert constraint { mass == engine.mass + frontAxleAssembly.mass + rearAxleAssembly.mass }
	}
	
	constraint massLimitation { mass : MassValue; massLimit : MassValue; mass < massLimit }
	assert not massLimitation { :>> mass = vehicle3.mass; :>> massLimit = vehicle4.mass; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "constraint_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 7) (end 28 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 32) (end 28 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 29 7) (end 29 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 29 38) (end 29 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 7) (end 30 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 38) (end 30 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 7) (end 31 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 38) (end 31 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 51 7) (end 51 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 51 34) (end 51 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 52 7) (end 52 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 52 40) (end 52 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 53 7) (end 53 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 53 40) (end 53 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 54 7) (end 54 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 54 40) (end 54 61))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 69 1) (end 69 152))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 77 2) (end 77 140))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 77 2) (end 77 140))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package ConstraintTest {
	private import ISQ::MassValue;
	private import SI::kg;
	private import NumericalFunctions::sum;
	
	constraint def MassAnalysis {
		attribute totalMass: MassValue;
		attribute componentMasses: MassValue[0..*];		

		totalMass == sum(componentMasses)
	}
	
	part def Component {
		attribute mass: MassValue;
	}
	
	part vehicle : Component {	
		part engine : Component;
		part frontAxleAssembly : Component;
		part rearAxleAssembly : Component;	
	}
		
	part vehicle1a :> vehicle {
		assert constraint massAnalysis : MassAnalysis {
			attribute redefines totalMass;
			attribute redefines componentMasses;
		}
		
		bind massAnalysis.totalMass = mass;
		bind massAnalysis.componentMasses = engine.mass;
		bind massAnalysis.componentMasses = frontAxleAssembly.mass;
		bind massAnalysis.componentMasses = rearAxleAssembly.mass;
	}
	
	part vehicle1b :> vehicle {		
		assert constraint massAnalysis : MassAnalysis {
			attribute redefines totalMass = mass;
			attribute redefines componentMasses = (engine.mass, frontAxleAssembly.mass, rearAxleAssembly.mass);		
		}	
	}
		
	constraint def MassAnalysis2 { 
		in totalMass : MassValue;
		in componentMasses: MassValue[0..*];
		
		totalMass == sum(componentMasses)
	}
	
	part vehicle2a :> vehicle {
		assert constraint massConstraint : MassAnalysis2;
		
		bind massConstraint.totalMass = mass;
		bind massConstraint.componentMasses = engine.mass;
		bind massConstraint.componentMasses = frontAxleAssembly.mass;
		bind massConstraint.componentMasses = rearAxleAssembly.mass;
	}
		
	part vehicle2b :> vehicle {
		assert constraint massAnalysis2 : MassAnalysis2 {
			in totalMass = mass;
			in componentMasses = (engine.mass, frontAxleAssembly.mass, rearAxleAssembly.mass);
		}
	}
	
	constraint def MassAnalysis3 {
		in totalMass : MassValue;
		in componentMasses: MassValue[0..*];
	}
	
	constraint massAnalysis3 : MassAnalysis3 {
		in totalMass : MassValue;
		in componentMasses: MassValue[0..*];
		
		totalMass == sum(componentMasses)
	}
	
	part vehicle3 :> vehicle {
		assert massAnalysis3 {
			in totalMass = mass;
			in componentMasses = (engine.mass, frontAxleAssembly.mass, rearAxleAssembly.mass);
		}
	}
	
	part vehicle4 :> vehicle {
		assert constraint { mass == engine.mass + frontAxleAssembly.mass + rearAxleAssembly.mass }
	}
	
	constraint massLimitation { mass : MassValue; massLimit : MassValue; mass < massLimit }
	assert not massLimitation { :>> mass = vehicle3.mass; :>> massLimit = vehicle4.mass; }
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "22cd871396ec4018b0d27ae481a54a3a607ca11e178bf0a69ff5b02a74365edd") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ConstraintTest"))) (kind "package") (name "ConstraintTest") (declared-name "ConstraintTest"))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::Component"))) (kind "part def") (name "Component") (declared-name "Component") (parent (node (document "d0") (qualified-name "ConstraintTest"))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::Component::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "ConstraintTest::Component"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis"))) (kind "constraint def") (name "MassAnalysis") (declared-name "MassAnalysis") (parent (node (document "d0") (qualified-name "ConstraintTest"))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis2"))) (kind "constraint def") (name "MassAnalysis2") (declared-name "MassAnalysis2") (parent (node (document "d0") (qualified-name "ConstraintTest"))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis3"))) (kind "constraint def") (name "MassAnalysis3") (declared-name "MassAnalysis3") (parent (node (document "d0") (qualified-name "ConstraintTest"))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::MassValue"))) (kind "import") (name "MassValue") (declared-name "MassValue") (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::MassValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::kg"))) (kind "import") (name "kg") (declared-name "kg") (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::kg") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::massAnalysis3"))) (kind "constraint") (name "massAnalysis3") (declared-name "massAnalysis3") (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassAnalysis3")))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::massLimitation"))) (kind "constraint") (name "massLimitation") (declared-name "massLimitation") (parent (node (document "d0") (qualified-name "ConstraintTest"))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::sum"))) (kind "import") (name "sum") (declared-name "sum") (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::sum") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Component")))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind "part") (name "vehicle1a") (declared-name "vehicle1a") (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle")))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle1b"))) (kind "part") (name "vehicle1b") (declared-name "vehicle1b") (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle")))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind "part") (name "vehicle2a") (declared-name "vehicle2a") (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle")))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle2b"))) (kind "part") (name "vehicle2b") (declared-name "vehicle2b") (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle")))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle3"))) (kind "part") (name "vehicle3") (declared-name "vehicle3") (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle")))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle4"))) (kind "part") (name "vehicle4") (declared-name "vehicle4") (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle")))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Component")))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (kind "part") (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (parent (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Component")))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (parent (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Component")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::Component::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::MassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::Component::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::MassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::MassValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::MassValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::kg"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::kg") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::massAnalysis3"))) (kind featureTyping) (ordinal 0)) (authored-target "MassAnalysis3") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis3")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::sum"))) (kind membershipImport) (ordinal 0)) (authored-target "NumericalFunctions::sum") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Component") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::Component")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindSource) (ordinal 0)) (authored-target "massAnalysis::totalMass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindSource) (ordinal 1)) (authored-target "massAnalysis::componentMasses") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindSource) (ordinal 2)) (authored-target "massAnalysis::componentMasses") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindSource) (ordinal 3)) (authored-target "massAnalysis::componentMasses") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindTarget) (ordinal 0)) (authored-target "mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindTarget) (ordinal 1)) (authored-target "engine::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindTarget) (ordinal 2)) (authored-target "frontAxleAssembly::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindTarget) (ordinal 3)) (authored-target "rearAxleAssembly::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1b"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindSource) (ordinal 0)) (authored-target "massConstraint::totalMass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindSource) (ordinal 1)) (authored-target "massConstraint::componentMasses") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindSource) (ordinal 2)) (authored-target "massConstraint::componentMasses") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindSource) (ordinal 3)) (authored-target "massConstraint::componentMasses") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindTarget) (ordinal 0)) (authored-target "mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindTarget) (ordinal 1)) (authored-target "engine::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindTarget) (ordinal 2)) (authored-target "frontAxleAssembly::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindTarget) (ordinal 3)) (authored-target "rearAxleAssembly::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2b"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle3"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle4"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Component") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::Component")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "Component") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::Component")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "Component") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConstraintTest::Component::mass"))) (target (node (document "d0") (qualified-name "ConstraintTest::MassValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConstraintTest::Component::mass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConstraintTest::Component::mass"))) (target (node (document "d0") (qualified-name "ConstraintTest::MassValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConstraintTest::Component::mass"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConstraintTest::massAnalysis3"))) (target (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConstraintTest::massAnalysis3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (target (node (document "d0") (qualified-name "ConstraintTest::Component"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1b"))) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1b"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2b"))) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2b"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ConstraintTest::vehicle3"))) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConstraintTest::vehicle3"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ConstraintTest::vehicle4"))) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConstraintTest::vehicle4"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConstraintTest::vehicle::engine"))) (target (node (document "d0") (qualified-name "ConstraintTest::Component"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConstraintTest::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (target (node (document "d0") (qualified-name "ConstraintTest::Component"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (target (node (document "d0") (qualified-name "ConstraintTest::Component"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis2")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "ConstraintTest::massAnalysis3")) (expression (status "unresolved") (error "expression has an unresolved reference")) (analysis (status "unresolved")))
    (node (node (document "d0") (qualified-name "ConstraintTest::massLimitation")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 28 32) (end 28 36)) (probe (position 28 32))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle1a"))
        (kind bindTarget) (ordinal 0) (authored-target "mass")
        (range (start 28 32) (end 28 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 51 34) (end 51 38)) (probe (position 51 34))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle2a"))
        (kind bindTarget) (ordinal 0) (authored-target "mass")
        (range (start 51 34) (end 51 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 22)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::kg"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::kg")
        (range (start 2 16) (end 2 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 22 19) (end 22 26)) (probe (position 22 19))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle1a"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle")
        (range (start 22 19) (end 22 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConstraintTest::vehicle") (range (start 16 1) (end 16 134)))
        )
      )
    )
    (query (range (start 34 19) (end 34 26)) (probe (position 34 19))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle1b"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle")
        (range (start 34 19) (end 34 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConstraintTest::vehicle") (range (start 16 1) (end 16 134)))
        )
      )
    )
    (query (range (start 48 19) (end 48 26)) (probe (position 48 19))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle2a"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle")
        (range (start 48 19) (end 48 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConstraintTest::vehicle") (range (start 16 1) (end 16 134)))
        )
      )
    )
    (query (range (start 57 19) (end 57 26)) (probe (position 57 19))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle2b"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle")
        (range (start 57 19) (end 57 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConstraintTest::vehicle") (range (start 16 1) (end 16 134)))
        )
      )
    )
    (query (range (start 76 18) (end 76 25)) (probe (position 76 18))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle3"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle")
        (range (start 76 18) (end 76 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConstraintTest::vehicle") (range (start 16 1) (end 16 134)))
        )
      )
    )
    (query (range (start 83 18) (end 83 25)) (probe (position 83 18))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle4"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle")
        (range (start 83 18) (end 83 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConstraintTest::vehicle") (range (start 16 1) (end 16 134)))
        )
      )
    )
    (query (range (start 13 18) (end 13 27)) (probe (position 13 18))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::Component::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 13 18) (end 13 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConstraintTest::MassValue") (range (start 1 1) (end 1 31)))
        )
      )
    )
    (query (range (start 16 16) (end 16 25)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Component")
        (range (start 16 16) (end 16 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConstraintTest::Component") (range (start 12 1) (end 12 53)))
        )
      )
    )
    (query (range (start 17 16) (end 17 25)) (probe (position 17 16))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Component")
        (range (start 17 16) (end 17 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConstraintTest::Component") (range (start 12 1) (end 12 53)))
        )
      )
    )
    (query (range (start 18 27) (end 18 36)) (probe (position 18 27))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "Component")
        (range (start 18 27) (end 18 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConstraintTest::Component") (range (start 12 1) (end 12 53)))
        )
      )
    )
    (query (range (start 19 26) (end 19 35)) (probe (position 19 26))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "Component")
        (range (start 19 26) (end 19 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConstraintTest::Component") (range (start 12 1) (end 12 53)))
        )
      )
    )
    (query (range (start 29 38) (end 29 49)) (probe (position 29 38))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle1a"))
        (kind bindTarget) (ordinal 1) (authored-target "engine::mass")
        (range (start 29 38) (end 29 49))
        (outcome (status unresolved))
      )
    )
    (query (range (start 52 40) (end 52 51)) (probe (position 52 40))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle2a"))
        (kind bindTarget) (ordinal 1) (authored-target "engine::mass")
        (range (start 52 40) (end 52 51))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 30)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::MassValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQ::MassValue")
        (range (start 1 16) (end 1 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 31 38) (end 31 59)) (probe (position 31 38))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle1a"))
        (kind bindTarget) (ordinal 3) (authored-target "rearAxleAssembly::mass")
        (range (start 31 38) (end 31 59))
        (outcome (status unresolved))
      )
    )
    (query (range (start 54 40) (end 54 61)) (probe (position 54 40))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle2a"))
        (kind bindTarget) (ordinal 3) (authored-target "rearAxleAssembly::mass")
        (range (start 54 40) (end 54 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 28 7) (end 28 29)) (probe (position 28 7))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle1a"))
        (kind bindSource) (ordinal 0) (authored-target "massAnalysis::totalMass")
        (range (start 28 7) (end 28 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 30 38) (end 30 60)) (probe (position 30 38))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle1a"))
        (kind bindTarget) (ordinal 2) (authored-target "frontAxleAssembly::mass")
        (range (start 30 38) (end 30 60))
        (outcome (status unresolved))
      )
    )
    (query (range (start 53 40) (end 53 62)) (probe (position 53 40))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle2a"))
        (kind bindTarget) (ordinal 2) (authored-target "frontAxleAssembly::mass")
        (range (start 53 40) (end 53 62))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 39)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::sum"))
        (kind membershipImport) (ordinal 0) (authored-target "NumericalFunctions::sum")
        (range (start 3 16) (end 3 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 51 7) (end 51 31)) (probe (position 51 7))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle2a"))
        (kind bindSource) (ordinal 0) (authored-target "massConstraint::totalMass")
        (range (start 51 7) (end 51 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 29 7) (end 29 35)) (probe (position 29 7))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle1a"))
        (kind bindSource) (ordinal 1) (authored-target "massAnalysis::componentMasses")
        (range (start 29 7) (end 29 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 30 7) (end 30 35)) (probe (position 30 7))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle1a"))
        (kind bindSource) (ordinal 2) (authored-target "massAnalysis::componentMasses")
        (range (start 30 7) (end 30 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 31 7) (end 31 35)) (probe (position 31 7))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle1a"))
        (kind bindSource) (ordinal 3) (authored-target "massAnalysis::componentMasses")
        (range (start 31 7) (end 31 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 52 7) (end 52 37)) (probe (position 52 7))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle2a"))
        (kind bindSource) (ordinal 1) (authored-target "massConstraint::componentMasses")
        (range (start 52 7) (end 52 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 53 7) (end 53 37)) (probe (position 53 7))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle2a"))
        (kind bindSource) (ordinal 2) (authored-target "massConstraint::componentMasses")
        (range (start 53 7) (end 53 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 54 7) (end 54 37)) (probe (position 54 7))
      (reference
        (source (document "d0") (qualified-name "ConstraintTest::vehicle2a"))
        (kind bindSource) (ordinal 3) (authored-target "massConstraint::componentMasses")
        (range (start 54 7) (end 54 37))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
