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
  (document "memory://snapshot/constraint_test.md"
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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 1) (end 10 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 18) (end 13 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 22 19) (end 22 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 23 2) (end 26 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 28 2) (end 28 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 29 2) (end 29 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 30 2) (end 30 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 31 2) (end 31 60))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 34 19) (end 34 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 35 2) (end 38 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 41 1) (end 46 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 48 19) (end 48 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 49 2) (end 49 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 51 2) (end 51 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 52 2) (end 52 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 53 2) (end 53 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 54 2) (end 54 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 57 19) (end 57 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 58 2) (end 61 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 64 1) (end 67 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 69 1) (end 74 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 76 18) (end 76 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 77 2) (end 80 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 83 18) (end 83 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 84 2) (end 84 92))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 87 1) (end 87 88))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 88 1) (end 88 87))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:97e018ed81220c84820a4c857b5eb4b015afff7d271f32bc06ca8de652f1a910") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQ::MassValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::kg") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "NumericalFunctions::sum") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle3"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle4"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQ::MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::kg")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "NumericalFunctions::sum")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component")))))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1a"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1b"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2a"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2b"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle3"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle4"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component")))))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component")))))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::engine"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/constraint_test.md") (range (start 1 16) (end 1 30)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ISQ::MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 2 16) (end 2 22)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "SI::kg")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 3 16) (end 3 39)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "NumericalFunctions::sum")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 13 18) (end 13 27)) (probe (position 13 18))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 16 16) (end 16 25)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component")))))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 22 19) (end 22 26)) (probe (position 22 19))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1a"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 34 19) (end 34 26)) (probe (position 34 19))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1b"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 48 19) (end 48 26)) (probe (position 48 19))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2a"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 57 19) (end 57 26)) (probe (position 57 19))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2b"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 76 18) (end 76 25)) (probe (position 76 18))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle3"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 83 18) (end 83 25)) (probe (position 83 18))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle4"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 17 16) (end 17 25)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component")))))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 18 27) (end 18 36)) (probe (position 18 27))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component")))))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 19 26) (end 19 35)) (probe (position 19 26))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component")))))
  )
)
~~~
