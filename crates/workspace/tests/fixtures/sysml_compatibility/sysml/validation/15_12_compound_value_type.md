# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_12-Compound Value Type
type=file
~~~
# SOURCE
~~~sysml
package '15_12-Compound Value Type' {
	private import ScalarValues::*;
	private import USCustomaryUnits::'in';
	
	/*
	 * Real world user models would use quantity and vector types
	 * from library models. They are included here for the purpose
	 * of showing how such attribute defs can be defined.
	 */

    attribute def PositionVector {
        attribute x: Real[1];
        attribute y: Real[1];
        attribute z: Real[1];
    }
    
    attribute def LengthValue :> Real;

    attribute def TireInfo {
    	attribute manufacturer: String;
        attribute hubDiameter: LengthValue;
        attribute width: Integer;
        attribute placement: PositionVector[0..1];
    }
    
    attribute frenchTireInfo: TireInfo {
    	attribute :>> manufacturer = "Michelin";
    	attribute :>> hubDiameter = 18.0['in'];
    	attribute :>> width = 245;
    }
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,UnrestrictedName,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,StringValue,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_12-Compound Value Type''
    (import_decl private 'ScalarValues::*')
    (import_decl private 'USCustomaryUnits::'in'')
    (comment)
    (attribute_def 'PositionVector'
      (attribute_usage 'x' : 'Real' multiplicity)
      (attribute_usage 'y' : 'Real' multiplicity)
      (attribute_usage 'z' : 'Real' multiplicity))
    (attribute_def 'LengthValue' :> 'Real')
    (attribute_def 'TireInfo'
      (attribute_usage 'manufacturer' : 'String')
      (attribute_usage 'hubDiameter' : 'LengthValue')
      (attribute_usage 'width' : 'Integer')
      (attribute_usage 'placement' : 'PositionVector' multiplicity))
    (attribute_usage 'frenchTireInfo' : 'TireInfo'
      (attribute_usage :>> 'manufacturer' value)
      (attribute_usage :>> 'hubDiameter' value)
      (attribute_usage :>> 'width' value))))
~~~
# FORMAT
~~~sysml
package '15_12-Compound Value Type' {
    private import ScalarValues::*;
    private import USCustomaryUnits::'in';

    /*
	 * Real world user models would use quantity and vector types
	 * from library models. They are included here for the purpose
	 * of showing how such attribute defs can be defined.
	 */

    attribute def PositionVector {
        attribute x : Real [1];
        attribute y : Real [1];
        attribute z : Real [1];
    }

    attribute def LengthValue :> Real;

    attribute def TireInfo {
        attribute manufacturer : String;
        attribute hubDiameter : LengthValue;
        attribute width : Integer;
        attribute placement : PositionVector [0..1];
    }

    attribute frenchTireInfo : TireInfo {
        attribute :>> manufacturer = "Michelin";
        attribute :>> hubDiameter = 18.0['in'];
        attribute :>> width = 245;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
~~~
# SMG
~~~
(model
  (namespace
    (package '15_12-Compound Value Type'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (membership_import private -> 'USCustomaryUnits::in'[unresolved])
      (attribute_def 'PositionVector'
        (attribute_usage composite 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite 'z' : 'Real'[unresolved]
          (multiplicity_range [1])))
      (attribute_def 'LengthValue' :> 'Real'[unresolved])
      (attribute_def 'TireInfo'
        (attribute_usage composite 'manufacturer' : 'String'[unresolved])
        (attribute_usage composite 'hubDiameter' : '15_12-Compound Value Type::LengthValue'[attribute_def])
        (attribute_usage composite 'width' : 'Integer'[unresolved])
        (attribute_usage composite 'placement' : '15_12-Compound Value Type::PositionVector'[attribute_def]
          (multiplicity_range [0..1])))
      (attribute_usage 'frenchTireInfo' : '15_12-Compound Value Type::TireInfo'[attribute_def]
        (attribute_usage composite :>> '15_12-Compound Value Type::TireInfo::manufacturer'[attribute_usage]
          (feature_value (=)))
        (attribute_usage composite :>> '15_12-Compound Value Type::TireInfo::hubDiameter'[attribute_usage]
          (feature_value (=)))
        (attribute_usage composite :>> '15_12-Compound Value Type::TireInfo::width'[attribute_usage]
          (feature_value (=)))))))
~~~
