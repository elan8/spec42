# META
~~~ini
description=SysML Training 31 (Constraints): Constraint Assertions-2
type=file
~~~
# SOURCE
~~~sysml
package 'Constraint Assertions-2' {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	part def Engine;
	part def Transmission;
	
	constraint def MassConstraint {
		in partMasses : MassValue[0..*];
		in massLimit : MassValue;
	}
	
	constraint massConstraint : MassConstraint {
		in partMasses : MassValue[0..*];
		in massLimit : MassValue;
			
		sum(partMasses) <= massLimit
	}
	
	part def Vehicle {
		assert massConstraint {
			in partMasses = (chassisMass, engine.mass, transmission.mass);
			in massLimit = 2500[kg];
		}
		
		attribute chassisMass : MassValue;
		
		part engine : Engine {
			attribute mass : MassValue;
		}
		
		part transmission : Engine {
			attribute mass : MassValue;
		}
	}	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwConstraint,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwConstraint,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
Ident,OpenParen,Ident,CloseParen,LtEq,Ident,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAssert,Ident,OpenCurly,
KwIn,Ident,Eq,OpenParen,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
KwIn,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Constraint Assertions-2''
    (import_decl private 'ISQ::*')
    (import_decl private 'SI::*')
    (import_decl private 'NumericalFunctions::*')
    (part_def 'Engine')
    (part_def 'Transmission')
    (constraint_def 'MassConstraint'
      (default_ref_usage in 'partMasses' : 'MassValue' multiplicity)
      (default_ref_usage in 'massLimit' : 'MassValue'))
    (constraint_usage 'massConstraint' : 'MassConstraint'
      (default_ref_usage in 'partMasses' : 'MassValue' multiplicity)
      (default_ref_usage in 'massLimit' : 'MassValue')
      (result_expr_member))
    (part_def 'Vehicle'
      (sysml_decl 'massConstraint'
        (default_ref_usage in 'partMasses' value)
        (default_ref_usage in 'massLimit' value))
      (attribute_usage 'chassisMass' : 'MassValue')
      (part_usage 'engine' : 'Engine'
        (attribute_usage 'mass' : 'MassValue'))
      (part_usage 'transmission' : 'Engine'
        (attribute_usage 'mass' : 'MassValue')))))
~~~
# FORMAT
~~~sysml
package 'Constraint Assertions-2' {
    private import ISQ::*;
    private import SI::*;
    private import NumericalFunctions::*;

    part def Engine;
    part def Transmission;

    constraint def MassConstraint {
        in partMasses : MassValue [0..*];
        in massLimit : MassValue;
    }

    constraint massConstraint : MassConstraint {
        in partMasses : MassValue [0..*];
        in massLimit : MassValue;

        = sum(partMasses) <= massLimit;
    }

    part def Vehicle {
        assert constraint massConstraint {
            in partMasses = (chassisMass, engine.mass, transmission.mass);
            in massLimit = 2500[kg];
        }

        attribute chassisMass : MassValue;

        part engine : Engine {
            attribute mass : MassValue;
        }

        part transmission : Engine {
            attribute mass : MassValue;
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Constraint Assertions-2'
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'SI'[unresolved])
      (namespace_import private -> 'NumericalFunctions'[unresolved])
      (part_def 'Engine')
      (part_def 'Transmission')
      (constraint_def 'MassConstraint'
        (reference_usage in reference 'partMasses' : 'MassValue'[unresolved]
          (multiplicity_range [0..*]))
        (reference_usage in reference 'massLimit' : 'MassValue'[unresolved]))
      (constraint_usage 'massConstraint' : 'Constraint Assertions-2::MassConstraint'[constraint_def]
        (reference_usage in reference 'partMasses' : 'MassValue'[unresolved]
          (multiplicity_range [0..*]))
        (reference_usage in reference 'massLimit' : 'MassValue'[unresolved])
        (result_expr_membership))
      (part_def 'Vehicle'
        (assert_constraint_usage 'massConstraint'
          (reference_usage in reference 'partMasses'
            (feature_value (=)))
          (reference_usage in reference 'massLimit'
            (feature_value (=))))
        (attribute_usage composite 'chassisMass' : 'MassValue'[unresolved])
        (part_usage composite 'engine' : 'Constraint Assertions-2::Engine'[part_def]
          (attribute_usage composite 'mass' : 'MassValue'[unresolved]))
        (part_usage composite 'transmission' : 'Constraint Assertions-2::Engine'[part_def]
          (attribute_usage composite 'mass' : 'MassValue'[unresolved]))))))
~~~
