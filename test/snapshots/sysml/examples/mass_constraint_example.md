# META
~~~ini
description=SysML Example (Mass Roll-up): MassConstraintExample
type=file
~~~
# SOURCE
~~~sysml
package MassConstraintExample {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	part def Engine {
		attribute m :> mass;
	}
	
	part def Transmission {
		attribute m :> mass;
	}
	
	part def Vehicle1 {
		attribute m : MassValue = eng.m + trans.m;
		
		part eng : Engine {
			attribute :>> m : MassValue;
		}
		
		part trans : Transmission {
			attribute :>> m : MassValue;
		}
	}
	
	part def Vehicle2 {
		assert constraint { m == eng.m + trans.m }
		
		attribute m : MassValue;
		
		part eng : Engine {
			attribute :>> m : MassValue;
		}
		
		part trans : Transmission {
			attribute :>> m : MassValue;
		}
	}
	
	constraint def MassConstraint3 {
		in totalMass : MassValue; 
		in partMasses : MassValue[0..*];
			
		totalMass == sum(partMasses)
	}
	
	part def Vehicle3 {
		assert constraint massConstraint : MassConstraint3 {
			in totalMass = m;
			in partMasses = (eng.m, trans.m);
		}
		
		attribute m : MassValue;
		
		part eng {
			attribute m : MassValue;
		}
		
		part trans {
			attribute m : MassValue;
		}
	}
	
	constraint def MassConstraint4 {
		in totalMass : MassValue;
		in partMasses : MassValue[0..*];
	}
	
	constraint mc : MassConstraint4 {
		in totalMass : MassValue; 
		in partMasses : MassValue[0..*];
		
		totalMass == sum(partMasses)
	}
	
	part def Vehicle4 {
		assert mc {
			in totalMass = m;
			in partMasses = (eng.m, trans.m);
		}
		
		attribute m : MassValue;
		
		part eng : Engine {
			attribute :>> m : MassValue;
		}
		
		part trans : Transmission {
			attribute :>> m : MassValue;
		}
	}
	
	constraint def MassLimit {
		in mass : MassValue; 
		in maxMass : MassValue;
			
		mass <= maxMass
	}
	
	part def Vehicle5 {
		assert constraint ml : MassLimit {
			in mass = m;
			in maxMass = 2500 [kg];
		}
		
		attribute m : MassValue = eng.m + trans.m;
		
		part eng : Engine {
			attribute :>> m : MassValue;
		}
		
		part trans : Transmission {
			attribute :>> m : MassValue;
		}
	}	
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/mass_constraint_example.md"
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
        (code "unsupported_reference")
        (source "semantic")
        (range (start 6 17) (end 6 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 10 17) (end 10 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 16) (end 14 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 17 17) (end 17 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 21) (end 17 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 21 17) (end 21 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 21) (end 21 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 26 2) (end 26 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 16) (end 28 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 31 17) (end 31 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 21) (end 31 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 35 17) (end 35 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 21) (end 35 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 39 1) (end 44 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 47 2) (end 50 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 16) (end 52 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 17) (end 55 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 17) (end 59 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 63 1) (end 66 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 68 1) (end 73 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 76 2) (end 79 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 16) (end 81 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 84 17) (end 84 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 84 21) (end 84 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 88 17) (end 88 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 88 21) (end 88 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 92 1) (end 97 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 100 2) (end 103 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 16) (end 105 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 108 17) (end 108 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 108 21) (end 108 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 112 17) (end 112 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 21) (end 112 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:1d2684d0a7f09166084f2e2a5d7168b0f44a3fef8251785edc5141512a68536c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine::m"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission::m"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle1"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")) (redefinition (reference "m"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle1::m"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")) (redefinition (reference "m"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle2"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")) (redefinition (reference "m"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle2::m"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")) (redefinition (reference "m"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle3"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle3::eng"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle3::eng::m"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle3::m"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle3::trans"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle3::trans::m"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle4"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")) (redefinition (reference "m"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle4::m"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")) (redefinition (reference "m"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle5"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")) (redefinition (reference "m"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle5::m"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")) (redefinition (reference "m"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine::m"))) (kind subsetting) (ordinal 0))
      (authored-target "mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission::m"))) (kind subsetting) (ordinal 0))
      (authored-target "mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine")))))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "m")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle1::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "m")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine")))))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "m")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle2::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "m")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle3::eng::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle3::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle3::trans::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine")))))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "m")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle4::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "m")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine")))))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "m")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle5::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "m")
      (outcome (status unsupported)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 1 16) (end 1 22)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 2 16) (end 2 21)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 3 16) (end 3 37)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 6 17) (end 6 21)) (probe (position 6 17))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine::m"))) (kind subsetting) (ordinal 0) (authored-target "mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 10 17) (end 10 21)) (probe (position 10 17))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission::m"))) (kind subsetting) (ordinal 0) (authored-target "mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 16 13) (end 16 19)) (probe (position 16 13))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine")))))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 17 21) (end 17 30)) (probe (position 17 21))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 17 17) (end 17 18)) (probe (position 17 17))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "m")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 14 16) (end 14 25)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle1::m"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 20 15) (end 20 27)) (probe (position 20 15))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission")))))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 21 21) (end 21 30)) (probe (position 21 21))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 21 17) (end 21 18)) (probe (position 21 17))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "m")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 30 13) (end 30 19)) (probe (position 30 13))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine")))))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 31 21) (end 31 30)) (probe (position 31 21))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 31 17) (end 31 18)) (probe (position 31 17))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "m")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 28 16) (end 28 25)) (probe (position 28 16))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle2::m"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 34 15) (end 34 27)) (probe (position 34 15))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission")))))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 35 21) (end 35 30)) (probe (position 35 21))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 35 17) (end 35 18)) (probe (position 35 17))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "m")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 55 17) (end 55 26)) (probe (position 55 17))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle3::eng::m"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 52 16) (end 52 25)) (probe (position 52 16))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle3::m"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 59 17) (end 59 26)) (probe (position 59 17))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle3::trans::m"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 83 13) (end 83 19)) (probe (position 83 13))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine")))))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 84 21) (end 84 30)) (probe (position 84 21))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 84 17) (end 84 18)) (probe (position 84 17))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "m")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 81 16) (end 81 25)) (probe (position 81 16))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle4::m"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 87 15) (end 87 27)) (probe (position 87 15))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission")))))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 88 21) (end 88 30)) (probe (position 88 21))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 88 17) (end 88 18)) (probe (position 88 17))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "m")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 107 13) (end 107 19)) (probe (position 107 13))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Engine")))))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 108 21) (end 108 30)) (probe (position 108 21))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 108 17) (end 108 18)) (probe (position 108 17))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "m")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 105 16) (end 105 25)) (probe (position 105 16))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle5::m"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 111 15) (end 111 27)) (probe (position 111 15))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_constraint_example.md") (qualified-name "MassConstraintExample::Transmission")))))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 112 21) (end 112 30)) (probe (position 112 21))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_constraint_example.md") (range (start 112 17) (end 112 18)) (probe (position 112 17))
    (reference (id (source (node (document "memory://snapshot/mass_constraint_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "m")
      (outcome (status unsupported)))
  )
)
~~~
