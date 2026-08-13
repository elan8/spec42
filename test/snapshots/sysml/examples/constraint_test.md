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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 2) (end 6 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 12) (end 6 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 6 21) (end 7 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 2) (end 7 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 12) (end 7 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 7 27) (end 9 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 2) (end 9 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 9 15) (end 9 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 18) (end 13 27))
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
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 29 7) (end 29 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 29 38) (end 29 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 30 7) (end 30 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 30 38) (end 30 60))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 31 7) (end 31 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 31 38) (end 31 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 35 2) (end 38 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 17) (end 42 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 22) (end 43 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 45 2) (end 45 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 45 15) (end 45 35))
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
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 52 7) (end 52 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 52 40) (end 52 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 53 7) (end 53 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 53 40) (end 53 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 54 7) (end 54 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 54 40) (end 54 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 58 2) (end 61 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 17) (end 65 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 66 22) (end 66 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 70 17) (end 70 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 22) (end 71 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 73 2) (end 73 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 73 15) (end 73 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 77 2) (end 80 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 84 2) (end 84 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 29) (end 87 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 87 34) (end 87 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 47) (end 87 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 87 57) (end 87 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 70) (end 87 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 77) (end 87 86))
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
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:97e018ed81220c84820a4c857b5eb4b015afff7d271f32bc06ca8de652f1a910") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQ::MassValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::kg") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "NumericalFunctions::sum") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (kind constraint-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "attribute")) (expressionOperand (reference "totalMass")) (expressionOperand (reference "attribute")) (expressionOperand (reference "componentMasses")) (expressionOperand (reference "totalMass"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis2"))) (kind constraint-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "totalMass"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis2::componentMasses"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis2::totalMass"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis3"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis3::componentMasses"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis3::totalMass"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massAnalysis3"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassAnalysis3")) (expressionOperand (reference "totalMass"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massAnalysis3::componentMasses"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massAnalysis3::totalMass"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massLimitation"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "mass")) (expressionOperand (reference "massLimit")) (expressionOperand (reference "mass")) (expressionOperand (reference "massLimit"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (anonymous (kind bind) (ordinal 1))))) (kind bind) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (anonymous (kind bind) (ordinal 2))))) (kind bind) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (anonymous (kind bind) (ordinal 3))))) (kind bind) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (anonymous (kind bind) (ordinal 1))))) (kind bind) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (anonymous (kind bind) (ordinal 2))))) (kind bind) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_test.md") (anonymous (kind bind) (ordinal 3))))) (kind bind) (membership (kind feature) (visibility default)))
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
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (kind expressionOperand) (ordinal 0))
      (authored-target "attribute")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (kind expressionOperand) (ordinal 1))
      (authored-target "totalMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (kind expressionOperand) (ordinal 2))
      (authored-target "attribute")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (kind expressionOperand) (ordinal 3))
      (authored-target "componentMasses")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (kind expressionOperand) (ordinal 4))
      (authored-target "totalMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis2"))) (kind expressionOperand) (ordinal 0))
      (authored-target "totalMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis2::componentMasses"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis2::totalMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis3::componentMasses"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis3::totalMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massAnalysis3"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassAnalysis3")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis3")))))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massAnalysis3"))) (kind expressionOperand) (ordinal 0))
      (authored-target "totalMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massAnalysis3::componentMasses"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massAnalysis3::totalMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massLimitation"))) (kind expressionOperand) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massLimitation"))) (kind expressionOperand) (ordinal 1))
      (authored-target "massLimit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massLimitation"))) (kind expressionOperand) (ordinal 2))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massLimitation"))) (kind expressionOperand) (ordinal 3))
      (authored-target "massLimit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component")))))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1a"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1b"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2a"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2b"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle3"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle4"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle")))))
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
    (relationship (kind typing) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massAnalysis3"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massAnalysis3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1a"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1a"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1b"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1b"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2a"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2a"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2b"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2b"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle3"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle3"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle4"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle4"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::engine"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massLimitation"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massLimitation"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massLimitation"))) (value (kind unresolved-operand)))
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
  (query (document "memory://snapshot/constraint_test.md") (range (start 6 2) (end 6 11)) (probe (position 6 2))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (kind expressionOperand) (ordinal 0) (authored-target "attribute")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 6 12) (end 6 21)) (probe (position 6 12))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (kind expressionOperand) (ordinal 1) (authored-target "totalMass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 7 2) (end 7 11)) (probe (position 7 2))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (kind expressionOperand) (ordinal 2) (authored-target "attribute")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 7 12) (end 7 27)) (probe (position 7 12))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (kind expressionOperand) (ordinal 3) (authored-target "componentMasses")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 9 2) (end 9 11)) (probe (position 9 2))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis"))) (kind expressionOperand) (ordinal 4) (authored-target "totalMass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 45 2) (end 45 11)) (probe (position 45 2))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis2"))) (kind expressionOperand) (ordinal 0) (authored-target "totalMass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 43 22) (end 43 31)) (probe (position 43 22))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis2::componentMasses"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 42 17) (end 42 26)) (probe (position 42 17))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis2::totalMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 66 22) (end 66 31)) (probe (position 66 22))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis3::componentMasses"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 65 17) (end 65 26)) (probe (position 65 17))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis3::totalMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 69 28) (end 69 41)) (probe (position 69 28))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massAnalysis3"))) (kind featureTyping) (ordinal 0) (authored-target "MassAnalysis3")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::MassAnalysis3")))))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 73 2) (end 73 11)) (probe (position 73 2))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massAnalysis3"))) (kind expressionOperand) (ordinal 0) (authored-target "totalMass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 71 22) (end 71 31)) (probe (position 71 22))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massAnalysis3::componentMasses"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 70 17) (end 70 26)) (probe (position 70 17))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massAnalysis3::totalMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 87 29) (end 87 33)) (probe (position 87 29))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massLimitation"))) (kind expressionOperand) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 87 47) (end 87 56)) (probe (position 87 47))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massLimitation"))) (kind expressionOperand) (ordinal 1) (authored-target "massLimit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 87 70) (end 87 74)) (probe (position 87 70))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massLimitation"))) (kind expressionOperand) (ordinal 2) (authored-target "mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 87 77) (end 87 86)) (probe (position 87 77))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::massLimitation"))) (kind expressionOperand) (ordinal 3) (authored-target "massLimit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 16 16) (end 16 25)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::Component")))))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 22 19) (end 22 26)) (probe (position 22 19))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1a"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle")))))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 28 32) (end 28 36)) (probe (position 28 32))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 34 19) (end 34 26)) (probe (position 34 19))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle1b"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle")))))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 48 19) (end 48 26)) (probe (position 48 19))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2a"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle")))))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 51 34) (end 51 38)) (probe (position 51 34))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 57 19) (end 57 26)) (probe (position 57 19))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle2b"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle")))))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 76 18) (end 76 25)) (probe (position 76 18))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle3"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle")))))
  )
  (query (document "memory://snapshot/constraint_test.md") (range (start 83 18) (end 83 25)) (probe (position 83 18))
    (reference (id (source (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle4"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_test.md") (qualified-name "ConstraintTest::vehicle")))))
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
