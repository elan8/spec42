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
  (document "mass_constraint_example.md"
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 17) (end 6 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 17) (end 10 21))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 14 2) (end 14 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 2) (end 14 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 16) (end 14 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 3) (end 17 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 21) (end 17 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 3) (end 21 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 21) (end 21 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 2) (end 28 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 16) (end 28 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 3) (end 31 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 21) (end 31 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 3) (end 35 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 21) (end 35 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 2) (end 52 26))
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
        (range (start 55 3) (end 55 27))
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
        (range (start 59 3) (end 59 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 17) (end 59 26))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 68 1) (end 68 135))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 76 2) (end 76 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 2) (end 81 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 16) (end 81 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 84 3) (end 84 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 84 21) (end 84 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 88 3) (end 88 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 88 21) (end 88 30))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 105 2) (end 105 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 2) (end 105 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 16) (end 105 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 108 3) (end 108 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 108 21) (end 108 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 3) (end 112 31))
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
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "893a985a9c0f3a36a11a7aad03a0196d32cd733639f508f6e9f8c6b635520bd1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MassConstraintExample"))) (kind "package") (name "MassConstraintExample") (declared-name "MassConstraintExample"))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "MassConstraintExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "MassConstraintExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "MassConstraintExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Engine::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Engine"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "mass")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::MassConstraint3"))) (kind "constraint def") (name "MassConstraint3") (declared-name "MassConstraint3") (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::MassConstraint4"))) (kind "constraint def") (name "MassConstraint4") (declared-name "MassConstraint4") (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::MassLimit"))) (kind "constraint def") (name "MassLimit") (declared-name "MassLimit") (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Transmission::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "mass")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1"))) (kind "part def") (name "Vehicle1") (declared-name "Vehicle1") (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (kind "part") (name "eng") (declared-name "eng") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")) (redefinition (reference "m")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (kind "part") (name "trans") (declared-name "trans") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")) (redefinition (reference "m")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2"))) (kind "part def") (name "Vehicle2") (declared-name "Vehicle2") (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (kind "part") (name "eng") (declared-name "eng") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")) (redefinition (reference "m")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (kind "part") (name "trans") (declared-name "trans") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")) (redefinition (reference "m")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3"))) (kind "part def") (name "Vehicle3") (declared-name "Vehicle3") (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::eng"))) (kind "part") (name "eng") (declared-name "eng") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::eng::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::eng"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::trans"))) (kind "part") (name "trans") (declared-name "trans") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::trans::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::trans"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4"))) (kind "part def") (name "Vehicle4") (declared-name "Vehicle4") (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (kind "part") (name "eng") (declared-name "eng") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")) (redefinition (reference "m")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (kind "part") (name "trans") (declared-name "trans") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")) (redefinition (reference "m")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5"))) (kind "part def") (name "Vehicle5") (declared-name "Vehicle5") (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (kind "part") (name "eng") (declared-name "eng") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")) (redefinition (reference "m")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (kind "part") (name "trans") (declared-name "trans") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")) (redefinition (reference "m")))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::mc"))) (kind "constraint") (name "mc") (declared-name "mc") (parent (node (document "d0") (qualified-name "MassConstraintExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassConstraint4")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Engine::m"))) (kind subsetting) (ordinal 0)) (authored-target "mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Transmission::m"))) (kind subsetting) (ordinal 0)) (authored-target "mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::eng::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::eng::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::trans::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::trans::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::mc"))) (kind featureTyping) (ordinal 0)) (authored-target "MassConstraint4") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::MassConstraint4")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::mc"))) (target (node (document "d0") (qualified-name "MassConstraintExample::MassConstraint4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::mc"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "MassConstraintExample::MassConstraint3")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "MassConstraintExample::MassLimit")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::m")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::m")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "MassConstraintExample::mc")) (expression (status "unresolved") (error "expression has an unresolved reference")) (analysis (status "unresolved")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 17 17) (end 17 18)) (probe (position 17 17))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))
        (kind redefinition) (ordinal 0) (authored-target "m")
        (range (start 17 17) (end 17 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m") (range (start 17 3) (end 17 31)))
        )
      )
    )
    (query (range (start 21 17) (end 21 18)) (probe (position 21 17))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))
        (kind redefinition) (ordinal 0) (authored-target "m")
        (range (start 21 17) (end 21 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m") (range (start 21 3) (end 21 31)))
        )
      )
    )
    (query (range (start 31 17) (end 31 18)) (probe (position 31 17))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))
        (kind redefinition) (ordinal 0) (authored-target "m")
        (range (start 31 17) (end 31 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m") (range (start 31 3) (end 31 31)))
        )
      )
    )
    (query (range (start 35 17) (end 35 18)) (probe (position 35 17))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))
        (kind redefinition) (ordinal 0) (authored-target "m")
        (range (start 35 17) (end 35 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m") (range (start 35 3) (end 35 31)))
        )
      )
    )
    (query (range (start 84 17) (end 84 18)) (probe (position 84 17))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))
        (kind redefinition) (ordinal 0) (authored-target "m")
        (range (start 84 17) (end 84 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m") (range (start 84 3) (end 84 31)))
        )
      )
    )
    (query (range (start 88 17) (end 88 18)) (probe (position 88 17))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))
        (kind redefinition) (ordinal 0) (authored-target "m")
        (range (start 88 17) (end 88 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m") (range (start 88 3) (end 88 31)))
        )
      )
    )
    (query (range (start 108 17) (end 108 18)) (probe (position 108 17))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))
        (kind redefinition) (ordinal 0) (authored-target "m")
        (range (start 108 17) (end 108 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m") (range (start 108 3) (end 108 31)))
        )
      )
    )
    (query (range (start 112 17) (end 112 18)) (probe (position 112 17))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))
        (kind redefinition) (ordinal 0) (authored-target "m")
        (range (start 112 17) (end 112 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m") (range (start 112 3) (end 112 31)))
        )
      )
    )
    (query (range (start 2 16) (end 2 18)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 2 16) (end 2 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 19)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 1 16) (end 1 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 17) (end 6 21)) (probe (position 6 17))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Engine::m"))
        (kind subsetting) (ordinal 0) (authored-target "mass")
        (range (start 6 17) (end 6 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 17) (end 10 21)) (probe (position 10 17))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Transmission::m"))
        (kind subsetting) (ordinal 0) (authored-target "mass")
        (range (start 10 17) (end 10 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 13) (end 16 19)) (probe (position 16 13))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 16 13) (end 16 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Engine") (range (start 5 1) (end 5 44)))
        )
      )
    )
    (query (range (start 30 13) (end 30 19)) (probe (position 30 13))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 30 13) (end 30 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Engine") (range (start 5 1) (end 5 44)))
        )
      )
    )
    (query (range (start 83 13) (end 83 19)) (probe (position 83 13))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 83 13) (end 83 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Engine") (range (start 5 1) (end 5 44)))
        )
      )
    )
    (query (range (start 107 13) (end 107 19)) (probe (position 107 13))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 107 13) (end 107 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Engine") (range (start 5 1) (end 5 44)))
        )
      )
    )
    (query (range (start 14 16) (end 14 25)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle1::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 14 16) (end 14 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 21) (end 17 30)) (probe (position 17 21))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 17 21) (end 17 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 21) (end 21 30)) (probe (position 21 21))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 21 21) (end 21 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 28 16) (end 28 25)) (probe (position 28 16))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle2::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 28 16) (end 28 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 31 21) (end 31 30)) (probe (position 31 21))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 31 21) (end 31 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 35 21) (end 35 30)) (probe (position 35 21))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 35 21) (end 35 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 52 16) (end 52 25)) (probe (position 52 16))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle3::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 52 16) (end 52 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 55 17) (end 55 26)) (probe (position 55 17))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle3::eng::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 55 17) (end 55 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 59 17) (end 59 26)) (probe (position 59 17))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle3::trans::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 59 17) (end 59 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 81 16) (end 81 25)) (probe (position 81 16))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle4::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 81 16) (end 81 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 84 21) (end 84 30)) (probe (position 84 21))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 84 21) (end 84 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 88 21) (end 88 30)) (probe (position 88 21))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 88 21) (end 88 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 105 16) (end 105 25)) (probe (position 105 16))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle5::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 105 16) (end 105 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 108 21) (end 108 30)) (probe (position 108 21))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 108 21) (end 108 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 112 21) (end 112 30)) (probe (position 112 21))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 112 21) (end 112 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 15) (end 20 27)) (probe (position 20 15))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 20 15) (end 20 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Transmission") (range (start 9 1) (end 9 50)))
        )
      )
    )
    (query (range (start 34 15) (end 34 27)) (probe (position 34 15))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 34 15) (end 34 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Transmission") (range (start 9 1) (end 9 50)))
        )
      )
    )
    (query (range (start 87 15) (end 87 27)) (probe (position 87 15))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 87 15) (end 87 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Transmission") (range (start 9 1) (end 9 50)))
        )
      )
    )
    (query (range (start 111 15) (end 111 27)) (probe (position 111 15))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 111 15) (end 111 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassConstraintExample::Transmission") (range (start 9 1) (end 9 50)))
        )
      )
    )
    (query (range (start 3 16) (end 3 34)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "MassConstraintExample::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions::*")
        (range (start 3 16) (end 3 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
