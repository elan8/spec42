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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwPart,UnrestrictedName,Colon,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,UnrestrictedName,Colon,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
LineComment,
KwVariation,KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwVariant,KwAttribute,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwVariant,KwAttribute,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwVariation,KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwVariant,UnrestrictedName,Semicolon,
KwVariant,UnrestrictedName,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Variation Definitions''
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'SI::mm')
    (attribute_def 'Diameter' :> 'ISQ::LengthValue')
    (part_def 'Cylinder'
      (attribute_usage 'diameter' : 'Diameter' multiplicity))
    (part_def 'Engine'
      (part_usage 'cylinder' : 'Cylinder' multiplicity))
    (part_usage ''4cylEngine'' : 'Engine'
      (part_usage :>> 'cylinder' multiplicity))
    (part_usage ''6cylEngine'' : 'Engine'
      (part_usage :>> 'cylinder' multiplicity))
    (line_comment)
    (attribute_def variation 'DiameterChoices' :> 'Diameter'
      (variant_usage
        (attribute_usage 'diameterSmall' value))
      (variant_usage
        (attribute_usage 'diameterLarge' value)))
    (part_def variation 'EngineChoices' :> 'Engine'
      (variant_usage
        (default_ref_usage ''4cylEngine''))
      (variant_usage
        (default_ref_usage ''6cylEngine'')))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::LengthValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ISQ::LengthValue'
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "dc3367ce9081a8f723213b71f664ff017f10b994ac042a535f57fe9d6bebc795") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Variation Definitions"))) (kind "package") (name "Variation Definitions") (declared-name "Variation Definitions") (range (start (line 0) (character 0)) (end (line 0) (character 717))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::4cylEngine"))) (kind "part") (name "4cylEngine") (declared-name "4cylEngine") (range (start (line 14) (character 4)) (end (line 14) (character 71))) (parent (node (document "d0") (qualified-name "Variation Definitions"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 14) (character 24)) (end (line 14) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::4cylEngine::cylinder"))) (kind "part") (name "cylinder") (range (start (line 15) (character 5)) (end (line 15) (character 32))) (parent (node (document "d0") (qualified-name "Variation Definitions::4cylEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cylinder") (range (start (line 15) (character 20)) (end (line 15) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::6cylEngine"))) (kind "part") (name "6cylEngine") (declared-name "6cylEngine") (range (start (line 18) (character 4)) (end (line 18) (character 71))) (parent (node (document "d0") (qualified-name "Variation Definitions"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 18) (character 24)) (end (line 18) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::6cylEngine::cylinder"))) (kind "part") (name "cylinder") (range (start (line 19) (character 5)) (end (line 19) (character 32))) (parent (node (document "d0") (qualified-name "Variation Definitions::6cylEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cylinder") (range (start (line 19) (character 20)) (end (line 19) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::Cylinder"))) (kind "part def") (name "Cylinder") (declared-name "Cylinder") (range (start (line 6) (character 4)) (end (line 6) (character 71))) (parent (node (document "d0") (qualified-name "Variation Definitions"))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::Cylinder::diameter"))) (kind "attribute") (name "diameter") (declared-name "diameter") (range (start (line 7) (character 8)) (end (line 7) (character 41))) (parent (node (document "d0") (qualified-name "Variation Definitions::Cylinder"))) (authored (membership (kind Feature)) (relationships (typing (reference "Diameter") (range none)) (typing (reference "Diameter") (range (start (line 7) (character 29)) (end (line 7) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::Diameter"))) (kind "attribute def") (name "Diameter") (declared-name "Diameter") (range (start (line 4) (character 1)) (end (line 4) (character 44))) (parent (node (document "d0") (qualified-name "Variation Definitions"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::DiameterChoices"))) (kind "kermlDecl") (name "DiameterChoices") (declared-name "DiameterChoices") (range (start (line 24) (character 1)) (end (line 24) (character 146))) (parent (node (document "d0") (qualified-name "Variation Definitions"))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 10) (character 4)) (end (line 10) (character 64))) (parent (node (document "d0") (qualified-name "Variation Definitions"))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::Engine::cylinder"))) (kind "part") (name "cylinder") (declared-name "cylinder") (range (start (line 11) (character 5)) (end (line 11) (character 36))) (parent (node (document "d0") (qualified-name "Variation Definitions::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder") (range (start (line 11) (character 21)) (end (line 11) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::EngineChoices"))) (kind "part def") (name "EngineChoices") (declared-name "EngineChoices") (range (start (line 29) (character 1)) (end (line 29) (character 98))) (parent (node (document "d0") (qualified-name "Variation Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine") (range (start (line 29) (character 37)) (end (line 29) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::EngineChoices::4cylEngine"))) (kind "variant") (name "4cylEngine") (declared-name "4cylEngine") (range (start (line 30) (character 2)) (end (line 30) (character 23))) (parent (node (document "d0") (qualified-name "Variation Definitions::EngineChoices"))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::EngineChoices::6cylEngine"))) (kind "variant") (name "6cylEngine") (declared-name "6cylEngine") (range (start (line 31) (character 2)) (end (line 31) (character 23))) (parent (node (document "d0") (qualified-name "Variation Definitions::EngineChoices"))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "Variation Definitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Variation Definitions::mm"))) (kind "import") (name "mm") (declared-name "mm") (range (start (line 2) (character 1)) (end (line 2) (character 23))) (parent (node (document "d0") (qualified-name "Variation Definitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::mm") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 22))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::4cylEngine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 14) (character 24)) (end (line 14) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::4cylEngine::cylinder"))) (kind redefinition) (ordinal 0)) (authored-target "cylinder") (range (start (line 15) (character 20)) (end (line 15) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::4cylEngine::cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::6cylEngine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 18) (character 24)) (end (line 18) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::6cylEngine::cylinder"))) (kind redefinition) (ordinal 0)) (authored-target "cylinder") (range (start (line 19) (character 20)) (end (line 19) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::6cylEngine::cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 0)) (authored-target "Diameter") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::Diameter")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 1)) (authored-target "Diameter") (range (start (line 7) (character 29)) (end (line 7) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::Diameter")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::Diameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::Engine::cylinder"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (range (start (line 11) (character 21)) (end (line 11) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::EngineChoices"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (range (start (line 29) (character 37)) (end (line 29) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Variation Definitions::mm"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::mm") (range (start (line 2) (character 16)) (end (line 2) (character 22))) (outcome (status unresolved)))
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
