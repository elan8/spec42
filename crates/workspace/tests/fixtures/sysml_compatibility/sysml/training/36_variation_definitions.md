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
        attribute diameter : Diameter [1];
    }

    part def Engine {
        part cylinder : Cylinder [2..*];
    }

    part '4cylEngine' : Engine {
        part redefines cylinder [4];
    }

    part '6cylEngine' : Engine {
        part redefines cylinder [6];
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
(model
  (namespace
    (package 'Variation Definitions'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (membership_import private -> 'SI::mm'[unresolved])
      (attribute_def 'Diameter' :> 'ISQ::LengthValue'[unresolved])
      (part_def 'Cylinder'
        (attribute_usage composite 'diameter' : 'Variation Definitions::Diameter'[attribute_def]
          (multiplicity_range [1])))
      (part_def 'Engine'
        (part_usage composite 'cylinder' : 'Variation Definitions::Cylinder'[part_def]
          (multiplicity_range [2..*])))
      (part_usage '4cylEngine' : 'Variation Definitions::Engine'[part_def]
        (part_usage composite :>> 'Variation Definitions::Engine::cylinder'[part_usage]
          (multiplicity_range [4])))
      (part_usage '6cylEngine' : 'Variation Definitions::Engine'[part_def]
        (part_usage composite :>> 'Variation Definitions::Engine::cylinder'[part_usage]
          (multiplicity_range [6])))
      (attribute_def variation 'DiameterChoices' :> 'Variation Definitions::Diameter'[attribute_def]
        (variant_usage
          (attribute_usage composite 'diameterSmall'
            (feature_value (=))))
        (variant_usage
          (attribute_usage composite 'diameterLarge'
            (feature_value (=)))))
      (part_def variation 'EngineChoices' :> 'Variation Definitions::Engine'[part_def]
        (variant_usage
          (reference_usage reference '4cylEngine'))
        (variant_usage
          (reference_usage reference '6cylEngine'))))))
~~~
