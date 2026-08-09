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
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::LengthValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ISQ::LengthValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Variation Definitions"))) (name "Variation Definitions") (declared-name "Variation Definitions")
      (contains
        (element (kind "part") (id (node (document "d0") (qualified-name "Variation Definitions::4cylEngine"))) (name "4cylEngine") (declared-name "4cylEngine") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Variation Definitions::4cylEngine::cylinder"))) (name "cylinder") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Variation Definitions::Engine")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Variation Definitions::6cylEngine"))) (name "6cylEngine") (declared-name "6cylEngine") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Variation Definitions::6cylEngine::cylinder"))) (name "cylinder") (declared (properties (ordered false)) (multiplicity (lower 6) (upper 6) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Variation Definitions::Engine")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Variation Definitions::Cylinder"))) (name "Cylinder") (declared-name "Cylinder") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Variation Definitions::Cylinder::diameter"))) (name "diameter") (declared-name "diameter") (declared (properties (ordered false) (unique true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Variation Definitions::Cylinder")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Variation Definitions::Diameter"))) (name "Diameter") (declared-name "Diameter") (declared (properties (ordered false) (unique true))))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Variation Definitions::DiameterChoices"))) (name "DiameterChoices") (declared-name "DiameterChoices"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Variation Definitions::Engine"))) (name "Engine") (declared-name "Engine") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Variation Definitions::Engine::cylinder"))) (name "cylinder") (declared-name "cylinder") (declared (properties (ordered false)) (multiplicity (lower 2) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Variation Definitions::Engine")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Variation Definitions::EngineChoices"))) (name "EngineChoices") (declared-name "EngineChoices") (declared (properties (variation true)))
          (contains
            (element (kind "variant") (id (node (document "d0") (qualified-name "Variation Definitions::EngineChoices::4cylEngine"))) (name "4cylEngine") (declared-name "4cylEngine") (effective (featuring-type (node (document "d0") (qualified-name "Variation Definitions::EngineChoices")))))
            (element (kind "variant") (id (node (document "d0") (qualified-name "Variation Definitions::EngineChoices::6cylEngine"))) (name "6cylEngine") (declared-name "6cylEngine") (effective (featuring-type (node (document "d0") (qualified-name "Variation Definitions::EngineChoices")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Variation Definitions::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Variation Definitions::mm"))) (name "mm") (declared-name "mm"))
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Variation Definitions::4cylEngine::cylinder"))) (to (node (document "d0") (qualified-name "Variation Definitions::Engine::cylinder"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Variation Definitions::6cylEngine::cylinder"))) (to (node (document "d0") (qualified-name "Variation Definitions::Engine::cylinder"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Variation Definitions::EngineChoices"))) (to (node (document "d0") (qualified-name "Variation Definitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Variation Definitions::4cylEngine"))) (to (node (document "d0") (qualified-name "Variation Definitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Variation Definitions::6cylEngine"))) (to (node (document "d0") (qualified-name "Variation Definitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Variation Definitions::Cylinder::diameter"))) (to (node (document "d0") (qualified-name "Variation Definitions::Diameter"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Variation Definitions::Engine::cylinder"))) (to (node (document "d0") (qualified-name "Variation Definitions::Cylinder"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/36_variation_definitions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 1) (end 4 44))
      )
      (diagnostic
        (severity warning)
        (code "invalid_variation_member_kind")
        (source "semantic")
        (range (start 30 2) (end 30 23))
      )
      (diagnostic
        (severity warning)
        (code "invalid_variation_member_kind")
        (source "semantic")
        (range (start 31 2) (end 31 23))
      )
    )
  )
)
~~~
