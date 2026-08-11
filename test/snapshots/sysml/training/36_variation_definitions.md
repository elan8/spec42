# META
~~~ini
description=SysML Training 36 (Variability): Variation Definitions
type=file
~~~
# SOURCE
~~~sysml
package 'Variation Definitions' {
	private import ScalarValues::Real;
	private import SI::mm;
	
	attribute def Diameter :> ISQ::LengthValue;
	
    part def Cylinder {
        attribute diameter : Diameter[1];
    }

    part def Engine {
    	part cylinder : Cylinder[2..*];
    }
    
    part '4cylEngine' : Engine {
    	part redefines cylinder[4];
    }
    
    part '6cylEngine' : Engine {
    	part redefines cylinder[6];
    }
    
    // Variability model
	
	variation attribute def DiameterChoices :> Diameter {
		variant attribute diameterSmall = 70[mm];
		variant attribute diameterLarge = 100[mm];
	}

	variation part def EngineChoices :> Engine {
		variant '4cylEngine';
		variant '6cylEngine';		
	}	

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "36_variation_definitions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 1) (end 4 44))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Variation Definitions' {
	private import ScalarValues::Real;
	private import SI::mm;
	
	attribute def Diameter :> ISQ::LengthValue;
	
    part def Cylinder {
        attribute diameter : Diameter[1];
    }

    part def Engine {
    	part cylinder : Cylinder[2..*];
    }
    
    part '4cylEngine' : Engine {
    	part redefines cylinder[4];
    }
    
    part '6cylEngine' : Engine {
    	part redefines cylinder[6];
    }
    
    // Variability model
	
	variation attribute def DiameterChoices :> Diameter {
		variant attribute diameterSmall = 70[mm];
		variant attribute diameterLarge = 100[mm];
	}

	variation part def EngineChoices :> Engine {
		variant '4cylEngine';
		variant '6cylEngine';		
	}	

}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d708c72e680b085a9851b2ac94d49c6ed340c2f7e70a1052902e9b34ae3a4288") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Variation Definitions"))) (kind "package") (name "Variation Definitions") (declared-name "Variation Definitions"))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::4cylEngine"))) (kind "part") (name "4cylEngine") (declared-name "4cylEngine") (parent (node (document "d0") (qualified-name "Variation Definitions"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::4cylEngine::cylinder"))) (kind "part") (name "cylinder") (parent (node (document "d0") (qualified-name "Variation Definitions::4cylEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cylinder")))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::6cylEngine"))) (kind "part") (name "6cylEngine") (declared-name "6cylEngine") (parent (node (document "d0") (qualified-name "Variation Definitions"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::6cylEngine::cylinder"))) (kind "part") (name "cylinder") (parent (node (document "d0") (qualified-name "Variation Definitions::6cylEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cylinder")))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::Cylinder"))) (kind "part def") (name "Cylinder") (declared-name "Cylinder") (parent (node (document "d0") (qualified-name "Variation Definitions"))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::Cylinder::diameter"))) (kind "attribute") (name "diameter") (declared-name "diameter") (parent (node (document "d0") (qualified-name "Variation Definitions::Cylinder"))) (authored (membership (kind Feature)) (relationships (typing (reference "Diameter")) (typing (reference "Diameter")))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::Diameter"))) (kind "attribute def") (name "Diameter") (declared-name "Diameter") (parent (node (document "d0") (qualified-name "Variation Definitions"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::DiameterChoices"))) (kind "kermlDecl") (name "DiameterChoices") (declared-name "DiameterChoices") (parent (node (document "d0") (qualified-name "Variation Definitions"))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "Variation Definitions"))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::Engine::cylinder"))) (kind "part") (name "cylinder") (declared-name "cylinder") (parent (node (document "d0") (qualified-name "Variation Definitions::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder")))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::EngineChoices"))) (kind "part def") (name "EngineChoices") (declared-name "EngineChoices") (parent (node (document "d0") (qualified-name "Variation Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::EngineChoices::4cylEngine"))) (kind "variant") (name "4cylEngine") (declared-name "4cylEngine") (parent (node (document "d0") (qualified-name "Variation Definitions::EngineChoices"))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::EngineChoices::6cylEngine"))) (kind "variant") (name "6cylEngine") (declared-name "6cylEngine") (parent (node (document "d0") (qualified-name "Variation Definitions::EngineChoices"))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "Variation Definitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::mm"))) (kind "import") (name "mm") (declared-name "mm") (parent (node (document "d0") (qualified-name "Variation Definitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::mm") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::4cylEngine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::4cylEngine::cylinder"))) (kind redefinition) (ordinal 0)) (authored-target "cylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::4cylEngine::cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::6cylEngine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::6cylEngine::cylinder"))) (kind redefinition) (ordinal 0)) (authored-target "cylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::6cylEngine::cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 0)) (authored-target "Diameter") (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::Diameter")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 1)) (authored-target "Diameter") (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::Diameter")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::Diameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::Engine::cylinder"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::EngineChoices"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::mm"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::mm") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Variation Definitions::4cylEngine"))) (target (node (document "d0") (qualified-name "Variation Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Definitions::4cylEngine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Variation Definitions::4cylEngine::cylinder"))) (target (node (document "d0") (qualified-name "Variation Definitions::4cylEngine::cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Definitions::4cylEngine::cylinder"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Variation Definitions::6cylEngine"))) (target (node (document "d0") (qualified-name "Variation Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Definitions::6cylEngine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Variation Definitions::6cylEngine::cylinder"))) (target (node (document "d0") (qualified-name "Variation Definitions::6cylEngine::cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Definitions::6cylEngine::cylinder"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Variation Definitions::Cylinder::diameter"))) (target (node (document "d0") (qualified-name "Variation Definitions::Diameter"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Definitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Variation Definitions::Cylinder::diameter"))) (target (node (document "d0") (qualified-name "Variation Definitions::Diameter"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Definitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Variation Definitions::Engine::cylinder"))) (target (node (document "d0") (qualified-name "Variation Definitions::Cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Definitions::Engine::cylinder"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Variation Definitions::EngineChoices"))) (target (node (document "d0") (qualified-name "Variation Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Definitions::EngineChoices"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 16) (end 2 22)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Variation Definitions::mm"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::mm")
        (range (start 2 16) (end 2 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 24) (end 14 30)) (probe (position 14 24))
      (reference
        (source (document "d0") (qualified-name "Variation Definitions::4cylEngine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 14 24) (end 14 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Variation Definitions::Engine") (range (start 10 4) (end 10 64)))
        )
      )
    )
    (query (range (start 18 24) (end 18 30)) (probe (position 18 24))
      (reference
        (source (document "d0") (qualified-name "Variation Definitions::6cylEngine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 18 24) (end 18 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Variation Definitions::Engine") (range (start 10 4) (end 10 64)))
        )
      )
    )
    (query (range (start 29 37) (end 29 43)) (probe (position 29 37))
      (reference
        (source (document "d0") (qualified-name "Variation Definitions::EngineChoices"))
        (kind specialization) (ordinal 0) (authored-target "Engine")
        (range (start 29 37) (end 29 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Variation Definitions::Engine") (range (start 10 4) (end 10 64)))
        )
      )
    )
    (query (range (start 7 29) (end 7 37)) (probe (position 7 29))
      (reference
        (source (document "d0") (qualified-name "Variation Definitions::Cylinder::diameter"))
        (kind featureTyping) (ordinal 1) (authored-target "Diameter")
        (range (start 7 29) (end 7 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Variation Definitions::Diameter") (range (start 4 1) (end 4 44)))
        )
      )
    )
    (query (range (start 11 21) (end 11 29)) (probe (position 11 21))
      (reference
        (source (document "d0") (qualified-name "Variation Definitions::Engine::cylinder"))
        (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
        (range (start 11 21) (end 11 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Variation Definitions::Cylinder") (range (start 6 4) (end 6 71)))
        )
      )
    )
    (query (range (start 15 20) (end 15 28)) (probe (position 15 20))
      (reference
        (source (document "d0") (qualified-name "Variation Definitions::4cylEngine::cylinder"))
        (kind redefinition) (ordinal 0) (authored-target "cylinder")
        (range (start 15 20) (end 15 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Variation Definitions::4cylEngine::cylinder") (range (start 15 5) (end 15 32)))
        )
      )
    )
    (query (range (start 19 20) (end 19 28)) (probe (position 19 20))
      (reference
        (source (document "d0") (qualified-name "Variation Definitions::6cylEngine::cylinder"))
        (kind redefinition) (ordinal 0) (authored-target "cylinder")
        (range (start 19 20) (end 19 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Variation Definitions::6cylEngine::cylinder") (range (start 19 5) (end 19 32)))
        )
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Variation Definitions::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
