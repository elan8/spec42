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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "15_12-Compound Value Type"))) (name "15_12-Compound Value Type") (declared-name "15_12-Compound Value Type")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::*"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::LengthValue"))) (name "LengthValue") (declared-name "LengthValue") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector"))) (name "PositionVector") (declared-name "PositionVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::x"))) (name "x") (declared-name "x") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::y"))) (name "y") (declared-name "y") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::z"))) (name "z") (declared-name "z") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo"))) (name "TireInfo") (declared-name "TireInfo") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (name "hubDiameter") (declared-name "hubDiameter") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer"))) (name "manufacturer") (declared-name "manufacturer") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (name "placement") (declared-name "placement") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::width"))) (name "width") (declared-name "width") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (name "frenchTireInfo") (declared-name "frenchTireInfo") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter"))) (name "hubDiameter") (declared-name "hubDiameter") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer"))) (name "manufacturer") (declared-name "manufacturer") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width"))) (name "width") (declared-name "width") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "15_12-Compound Value Type::in"))) (name "in") (declared-name "in"))
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter"))) (to (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer"))) (to (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width"))) (to (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::width"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (to (node (document "d0") (qualified-name "15_12-Compound Value Type::LengthValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (to (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (to (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo"))))
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
  (document "sysml/validation/15_12_compound_value_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 8) (end 11 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 8) (end 12 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 8) (end 13 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 4) (end 16 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 8) (end 21 33))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 27 5) (end 27 44))
      )
    )
  )
)
~~~
