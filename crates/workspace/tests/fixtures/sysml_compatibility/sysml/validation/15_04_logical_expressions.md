# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_04-Logical Expressions
type=file
~~~
# SOURCE
~~~sysml
package '15_04-Logical Expressions' {
	private import ScalarValues::*;
	
	part def Engine;
	part def '4CylEngine' :> Engine;
	part def '6CylEngine' :> Engine;
	
	part def Transmission;
	part def ManualTransmission :> Transmission;
	part def AutomaticTransmission :> Transmission;
	
	part def Vehicle {
		attribute isHighPerformance: Boolean;
		
		part engine: Engine[1];
		part transmission: Transmission[1];
		
		assert constraint {
			if isHighPerformance? engine istype '6CylEngine'
			else engine istype '4CylEngine'
		}
		
		assert constraint {
			(engine istype '4CylEngine' and 
			 transmission istype ManualTransmission) xor
			(engine istype '6CylEngine' and
			 transmission istype AutomaticTransmission)
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,UnrestrictedName,ColonGt,Ident,Semicolon,
KwPart,KwDef,UnrestrictedName,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAssert,KwConstraint,OpenCurly,
KwIf,Ident,Question,Ident,KwIstype,UnrestrictedName,
KwElse,Ident,KwIstype,UnrestrictedName,
CloseCurly,
KwAssert,KwConstraint,OpenCurly,
OpenParen,Ident,KwIstype,UnrestrictedName,KwAnd,
Ident,KwIstype,Ident,CloseParen,KwXor,
OpenParen,Ident,KwIstype,UnrestrictedName,KwAnd,
Ident,KwIstype,Ident,CloseParen,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_04-Logical Expressions''
    (import_decl private 'ScalarValues::*')
    (part_def 'Engine')
    (part_def ''4CylEngine'' :> 'Engine')
    (part_def ''6CylEngine'' :> 'Engine')
    (part_def 'Transmission')
    (part_def 'ManualTransmission' :> 'Transmission')
    (part_def 'AutomaticTransmission' :> 'Transmission')
    (part_def 'Vehicle'
      (attribute_usage 'isHighPerformance' : 'Boolean')
      (part_usage 'engine' : 'Engine' multiplicity)
      (part_usage 'transmission' : 'Transmission' multiplicity)
      (sysml_decl
        (result_expr_member))
      (sysml_decl
        (result_expr_member)))))
~~~
# FORMAT
~~~sysml
package '15_04-Logical Expressions' {
    private import ScalarValues::*;

    part def Engine;
    part def '4CylEngine' :> Engine;
    part def '6CylEngine' :> Engine;

    part def Transmission;
    part def ManualTransmission :> Transmission;
    part def AutomaticTransmission :> Transmission;

    part def Vehicle {
        attribute isHighPerformance : Boolean;

        part engine : Engine [1];
        part transmission : Transmission [1];

        assert constraint {
            = if isHighPerformance ? engine istype '6CylEngine' else engine istype '4CylEngine';
        }

        assert constraint {
            = (engine istype '4CylEngine' and transmission istype ManualTransmission) xor (engine istype '6CylEngine' and transmission istype AutomaticTransmission);
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Boolean'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Boolean'
~~~
# SMG
~~~
(model
  (namespace
    (package '15_04-Logical Expressions'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (part_def 'Engine')
      (part_def '4CylEngine' :> '15_04-Logical Expressions::Engine'[part_def])
      (part_def '6CylEngine' :> '15_04-Logical Expressions::Engine'[part_def])
      (part_def 'Transmission')
      (part_def 'ManualTransmission' :> '15_04-Logical Expressions::Transmission'[part_def])
      (part_def 'AutomaticTransmission' :> '15_04-Logical Expressions::Transmission'[part_def])
      (part_def 'Vehicle'
        (attribute_usage composite 'isHighPerformance' : 'Boolean'[unresolved])
        (part_usage composite 'engine' : '15_04-Logical Expressions::Engine'[part_def]
          (multiplicity_range [1]))
        (part_usage composite 'transmission' : '15_04-Logical Expressions::Transmission'[part_def]
          (multiplicity_range [1]))
        (assert_constraint_usage
          (result_expr_membership))
        (assert_constraint_usage
          (result_expr_membership))))))
~~~
