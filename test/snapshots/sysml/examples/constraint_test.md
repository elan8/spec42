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
    (element (id (node (document "d0") (qualified-name "ConstraintTest"))) (kind "package") (name "ConstraintTest") (declared-name "ConstraintTest") (range (start (line 0) (character 0)) (end (line 0) (character 2477))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::Component"))) (kind "part def") (name "Component") (declared-name "Component") (range (start (line 12) (character 1)) (end (line 12) (character 53))) (parent (node (document "d0") (qualified-name "ConstraintTest"))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::Component::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 13) (character 2)) (end (line 13) (character 28))) (parent (node (document "d0") (qualified-name "ConstraintTest::Component"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 13) (character 18)) (end (line 13) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis"))) (kind "constraint def") (name "MassAnalysis") (declared-name "MassAnalysis") (range (start (line 5) (character 1)) (end (line 5) (character 152))) (parent (node (document "d0") (qualified-name "ConstraintTest"))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis2"))) (kind "constraint def") (name "MassAnalysis2") (declared-name "MassAnalysis2") (range (start (line 41) (character 1)) (end (line 41) (character 141))) (parent (node (document "d0") (qualified-name "ConstraintTest"))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis3"))) (kind "constraint def") (name "MassAnalysis3") (declared-name "MassAnalysis3") (range (start (line 64) (character 1)) (end (line 64) (character 101))) (parent (node (document "d0") (qualified-name "ConstraintTest"))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::MassValue"))) (kind "import") (name "MassValue") (declared-name "MassValue") (range (start (line 1) (character 1)) (end (line 1) (character 31))) (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::MassValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 30))))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::kg"))) (kind "import") (name "kg") (declared-name "kg") (range (start (line 2) (character 1)) (end (line 2) (character 23))) (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::kg") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 22))))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::massAnalysis3"))) (kind "constraint") (name "massAnalysis3") (declared-name "massAnalysis3") (range (start (line 69) (character 1)) (end (line 69) (character 152))) (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassAnalysis3") (range none)))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::massLimitation"))) (kind "constraint") (name "massLimitation") (declared-name "massLimitation") (range (start (line 87) (character 1)) (end (line 87) (character 88))) (parent (node (document "d0") (qualified-name "ConstraintTest"))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::sum"))) (kind "import") (name "sum") (declared-name "sum") (range (start (line 3) (character 1)) (end (line 3) (character 40))) (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::sum") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 39))))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 16) (character 1)) (end (line 16) (character 134))) (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Component") (range (start (line 16) (character 16)) (end (line 16) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind "part") (name "vehicle1a") (declared-name "vehicle1a") (range (start (line 22) (character 1)) (end (line 22) (character 374))) (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 22) (character 19)) (end (line 22) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle1b"))) (kind "part") (name "vehicle1b") (declared-name "vehicle1b") (range (start (line 34) (character 1)) (end (line 34) (character 234))) (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 34) (character 19)) (end (line 34) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind "part") (name "vehicle2a") (declared-name "vehicle2a") (range (start (line 48) (character 1)) (end (line 48) (character 306))) (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 48) (character 19)) (end (line 48) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle2b"))) (kind "part") (name "vehicle2b") (declared-name "vehicle2b") (range (start (line 57) (character 1)) (end (line 57) (character 197))) (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 57) (character 19)) (end (line 57) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle3"))) (kind "part") (name "vehicle3") (declared-name "vehicle3") (range (start (line 76) (character 1)) (end (line 76) (character 169))) (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 76) (character 18)) (end (line 76) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle4"))) (kind "part") (name "vehicle4") (declared-name "vehicle4") (range (start (line 83) (character 1)) (end (line 83) (character 123))) (parent (node (document "d0") (qualified-name "ConstraintTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 83) (character 18)) (end (line 83) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 17) (character 2)) (end (line 17) (character 26))) (parent (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Component") (range (start (line 17) (character 16)) (end (line 17) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (kind "part") (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (range (start (line 18) (character 2)) (end (line 18) (character 37))) (parent (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Component") (range (start (line 18) (character 27)) (end (line 18) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (range (start (line 19) (character 2)) (end (line 19) (character 36))) (parent (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Component") (range (start (line 19) (character 26)) (end (line 19) (character 35)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::Component::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::MassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::Component::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 13) (character 18)) (end (line 13) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::MassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::MassValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::MassValue") (range (start (line 1) (character 16)) (end (line 1) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::kg"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::kg") (range (start (line 2) (character 16)) (end (line 2) (character 22))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::massAnalysis3"))) (kind featureTyping) (ordinal 0)) (authored-target "MassAnalysis3") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis3")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::sum"))) (kind membershipImport) (ordinal 0)) (authored-target "NumericalFunctions::sum") (range (start (line 3) (character 16)) (end (line 3) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Component") (range (start (line 16) (character 16)) (end (line 16) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::Component")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 22) (character 19)) (end (line 22) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindSource) (ordinal 0)) (authored-target "massAnalysis::totalMass") (range (start (line 28) (character 7)) (end (line 28) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindSource) (ordinal 1)) (authored-target "massAnalysis::componentMasses") (range (start (line 29) (character 7)) (end (line 29) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindSource) (ordinal 2)) (authored-target "massAnalysis::componentMasses") (range (start (line 30) (character 7)) (end (line 30) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindSource) (ordinal 3)) (authored-target "massAnalysis::componentMasses") (range (start (line 31) (character 7)) (end (line 31) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindTarget) (ordinal 0)) (authored-target "mass") (range (start (line 28) (character 32)) (end (line 28) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindTarget) (ordinal 1)) (authored-target "engine::mass") (range (start (line 29) (character 38)) (end (line 29) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindTarget) (ordinal 2)) (authored-target "frontAxleAssembly::mass") (range (start (line 30) (character 38)) (end (line 30) (character 60))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (kind bindTarget) (ordinal 3)) (authored-target "rearAxleAssembly::mass") (range (start (line 31) (character 38)) (end (line 31) (character 59))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle1b"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 34) (character 19)) (end (line 34) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 48) (character 19)) (end (line 48) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindSource) (ordinal 0)) (authored-target "massConstraint::totalMass") (range (start (line 51) (character 7)) (end (line 51) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindSource) (ordinal 1)) (authored-target "massConstraint::componentMasses") (range (start (line 52) (character 7)) (end (line 52) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindSource) (ordinal 2)) (authored-target "massConstraint::componentMasses") (range (start (line 53) (character 7)) (end (line 53) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindSource) (ordinal 3)) (authored-target "massConstraint::componentMasses") (range (start (line 54) (character 7)) (end (line 54) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindTarget) (ordinal 0)) (authored-target "mass") (range (start (line 51) (character 34)) (end (line 51) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindTarget) (ordinal 1)) (authored-target "engine::mass") (range (start (line 52) (character 40)) (end (line 52) (character 51))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindTarget) (ordinal 2)) (authored-target "frontAxleAssembly::mass") (range (start (line 53) (character 40)) (end (line 53) (character 62))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (kind bindTarget) (ordinal 3)) (authored-target "rearAxleAssembly::mass") (range (start (line 54) (character 40)) (end (line 54) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle2b"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 57) (character 19)) (end (line 57) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle3"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 76) (character 18)) (end (line 76) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle4"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 83) (character 18)) (end (line 83) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Component") (range (start (line 17) (character 16)) (end (line 17) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::Component")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "Component") (range (start (line 18) (character 27)) (end (line 18) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::Component")))))
    (reference (id (source (node (document "d0") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "Component") (range (start (line 19) (character 26)) (end (line 19) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConstraintTest::Component")))))
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
