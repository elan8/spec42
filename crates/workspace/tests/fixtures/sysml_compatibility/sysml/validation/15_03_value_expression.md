# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_03-Value Expression
type=file
~~~
# SOURCE
~~~sysml
package '15_03-Value Expression' {
    private import SI::*;
    private import USCustomaryUnits::*;

    part def Vehicle_1 {
        attribute mass: MassValue = 1200 [kg];
        attribute length: LengthValue = 4.82 [m];
        part leftFrontWheel : Wheel;
        part rightFrontWheel : Wheel;
    }

    part def Wheel {
    	attribute hubDiameter: LengthValue = 18 ['in'];
        attribute width: LengthValue = 245 [mm];
        attribute outerDiameter: LengthValue = (hubDiameter + 2 * tire.height) [mm] {
	        doc
	        /*
	         * This binds 'outDiameter' to the result of a computed attribute.
	         * There is no need to mark it as "derived".
	         */
        }
        part tire: Tire[1];
    }
    
    part def Tire {
    	attribute profileDepth: LengthValue default 6.0 [mm];
        constraint hasLegalProfileDepth {profileDepth >= 3.5 [mm]}
    	attribute height: LengthValue = 45 [mm];
    }
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,OpenParen,Ident,Plus,DecimalValue,Star,Ident,Dot,Ident,CloseParen,OpenSquare,Ident,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,KwDefault,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwConstraint,Ident,OpenCurly,Ident,GtEq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,CloseCurly,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_03-Value Expression''
    (import_decl private 'SI::*')
    (import_decl private 'USCustomaryUnits::*')
    (part_def 'Vehicle_1'
      (attribute_usage 'mass' : 'MassValue' value)
      (attribute_usage 'length' : 'LengthValue' value)
      (part_usage 'leftFrontWheel' : 'Wheel')
      (part_usage 'rightFrontWheel' : 'Wheel'))
    (part_def 'Wheel'
      (attribute_usage 'hubDiameter' : 'LengthValue' value)
      (attribute_usage 'width' : 'LengthValue' value)
      (attribute_usage 'outerDiameter' : 'LengthValue' value
        (documentation))
      (part_usage 'tire' : 'Tire' multiplicity))
    (part_def 'Tire'
      (attribute_usage 'profileDepth' : 'LengthValue' value)
      (constraint_usage 'hasLegalProfileDepth'
        (result_expr_member))
      (attribute_usage 'height' : 'LengthValue' value))))
~~~
# FORMAT
~~~sysml
package '15_03-Value Expression' {
    private import SI::*;
    private import USCustomaryUnits::*;

    part def Vehicle_1 {
        attribute mass : MassValue = 1200 [kg];
        attribute length : LengthValue = 4.82 [m];
        part leftFrontWheel : Wheel;
        part rightFrontWheel : Wheel;
    }

    part def Wheel {
        attribute hubDiameter : LengthValue = 18 ['in'];
        attribute width : LengthValue = 245 [mm];
        attribute outerDiameter : LengthValue = (hubDiameter + 2 * tire.height) [mm] {
            doc /*
	         * This binds 'outDiameter' to the result of a computed attribute.
	         * There is no need to mark it as "derived".
	         */
        }
        part tire : Tire [1];
    }

    part def Tire {
        attribute profileDepth : LengthValue default = 6.0 [mm];
        constraint hasLegalProfileDepth {
            = profileDepth >= 3.5[mm];
        }
        attribute height : LengthValue = 45 [mm];
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
~~~
# SMG
~~~
(model
  (namespace
    (package '15_03-Value Expression'
      (namespace_import private -> 'SI'[unresolved])
      (namespace_import private -> 'USCustomaryUnits'[unresolved])
      (part_def 'Vehicle_1'
        (attribute_usage composite 'mass' : 'MassValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'length' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (part_usage composite 'leftFrontWheel' : '15_03-Value Expression::Wheel'[part_def])
        (part_usage composite 'rightFrontWheel' : '15_03-Value Expression::Wheel'[part_def]))
      (part_def 'Wheel'
        (attribute_usage composite 'hubDiameter' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'width' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'outerDiameter' : 'LengthValue'[unresolved]
          (feature_value (=))
          (documentation))
        (part_usage composite 'tire' : '15_03-Value Expression::Tire'[part_def]
          (multiplicity_range [1])))
      (part_def 'Tire'
        (attribute_usage composite 'profileDepth' : 'LengthValue'[unresolved]
          (feature_value (default =)))
        (constraint_usage composite 'hasLegalProfileDepth'
          (result_expr_membership))
        (attribute_usage composite 'height' : 'LengthValue'[unresolved]
          (feature_value (=)))))))
~~~
