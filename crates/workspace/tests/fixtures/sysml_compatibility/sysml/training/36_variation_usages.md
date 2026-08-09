# META
~~~ini
description=SysML Training 36 (Variability): Variation Usages
type=file
~~~
# SOURCE
~~~sysml
package 'Variation Usages' {
	private import 'Variation Definitions'::*;
	
	part def Vehicle;
	part def Transmission;
	part manualTransmission;
	part automaticTransmission;
	
	abstract part vehicleFamily : Vehicle {
		part engine : EngineChoices[1];
		
		variation part transmission : Transmission[1] {
			variant manualTransmission;
			variant automaticTransmission;
		}
		
		assert constraint {
			(engine == engine::'4cylEngine' and
			 transmission == transmission::manualTransmission) xor
			(engine == engine::'6cylEngine' and 
			 transmission == transmission::automaticTransmission)
		}	
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwAbstract,KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwVariation,KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwVariant,Ident,Semicolon,
KwVariant,Ident,Semicolon,
CloseCurly,
KwAssert,KwConstraint,OpenCurly,
OpenParen,Ident,EqEq,Ident,ColonColon,UnrestrictedName,KwAnd,
Ident,EqEq,Ident,ColonColon,Ident,CloseParen,KwXor,
OpenParen,Ident,EqEq,Ident,ColonColon,UnrestrictedName,KwAnd,
Ident,EqEq,Ident,ColonColon,Ident,CloseParen,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Variation Usages''
    (import_decl private ''Variation Definitions'::*')
    (part_def 'Vehicle')
    (part_def 'Transmission')
    (part_usage 'manualTransmission')
    (part_usage 'automaticTransmission')
    (part_usage abstract 'vehicleFamily' : 'Vehicle'
      (part_usage 'engine' : 'EngineChoices' multiplicity)
      (part_usage variation 'transmission' : 'Transmission' multiplicity
        (variant_usage
          (default_ref_usage 'manualTransmission'))
        (variant_usage
          (default_ref_usage 'automaticTransmission')))
      (sysml_decl
        (result_expr_member)))))
~~~
# FORMAT
~~~sysml
package 'Variation Usages' {
    private import 'Variation Definitions'::*;

    part def Vehicle;
    part def Transmission;
    part manualTransmission;
    part automaticTransmission;

    abstract part vehicleFamily : Vehicle {
        part engine : EngineChoices [1];

        variation part transmission : Transmission [1] {
            variant manualTransmission;
            variant automaticTransmission;
        }

        assert constraint {
            = (engine == engine::'4cylEngine' and transmission == transmission::manualTransmission) xor (engine == engine::'6cylEngine' and transmission == transmission::automaticTransmission);
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'EngineChoices'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'EngineChoices'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Variation Usages'
      (namespace_import private -> 'Variation Definitions'[unresolved])
      (part_def 'Vehicle')
      (part_def 'Transmission')
      (part_usage 'manualTransmission')
      (part_usage 'automaticTransmission')
      (part_usage abstract 'vehicleFamily' : 'Variation Usages::Vehicle'[part_def]
        (part_usage composite 'engine' : 'EngineChoices'[unresolved]
          (multiplicity_range [1]))
        (part_usage variation composite 'transmission' : 'Variation Usages::Transmission'[part_def]
          (multiplicity_range [1])
          (variant_usage
            (reference_usage reference 'manualTransmission'))
          (variant_usage
            (reference_usage reference 'automaticTransmission')))
        (assert_constraint_usage
          (result_expr_membership))))))
~~~
