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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "15_03-Value Expression"))) (name "15_03-Value Expression") (declared-name "15_03-Value Expression")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "15_03-Value Expression::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "15_03-Value Expression::*#import"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_03-Value Expression::Tire"))) (name "Tire") (declared-name "Tire") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_03-Value Expression::Tire::height"))) (name "height") (declared-name "height") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 45)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_03-Value Expression::Tire"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_03-Value Expression::Tire::height"))) (role feature-value))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_03-Value Expression::Tire::profileDepth"))) (name "profileDepth") (declared-name "profileDepth") (declared (properties (ordered false) (unique true)) (feature-value (kind default) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal "6.0")) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_03-Value Expression::Tire")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1"))) (name "Vehicle_1") (declared-name "Vehicle_1") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (name "leftFrontWheel") (declared-name "leftFrontWheel") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::length"))) (name "length") (declared-name "length") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal "4.82")) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "m")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::length"))) (role feature-value))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 1200)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::mass"))) (role feature-value))))
            (element (kind "part") (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (name "rightFrontWheel") (declared-name "rightFrontWheel") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::hubDiameter"))) (name "hubDiameter") (declared-name "hubDiameter") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 18)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "in")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::hubDiameter"))) (role feature-value))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (name "outerDiameter") (declared-name "outerDiameter") (declared (properties (ordered false) (unique true)) (multiplicity (lower unevaluated) (upper unevaluated) (ordered false) (provenance authored)) (feature-value (kind bound) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "+") (children (expression (kind "featureReference") (reference "hubDiameter")) (expression (kind "binary") (operator "*") (children (expression (kind "integerLiteral") (literal 2)) (expression (kind "memberAccess") (reference "height") (children (expression (kind "featureReference") (reference "tire")))))))))))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (role feature-value))))
            (element (kind "part") (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::tire"))) (name "tire") (declared-name "tire") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_03-Value Expression::Wheel")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::width"))) (name "width") (declared-name "width") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 245)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::width"))) (role feature-value))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (to (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (to (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::tire"))) (to (node (document "d0") (qualified-name "15_03-Value Expression::Tire"))))
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
  (document "sysml/validation/15_03_value_expression.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 35))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 5 8) (end 5 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 8) (end 5 46))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 6 8) (end 6 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 8) (end 6 49))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 12 5) (end 12 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 5) (end 12 52))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 13 8) (end 13 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 8) (end 13 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 8) (end 14 263))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 25 5) (end 25 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 5) (end 25 58))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 27 5) (end 27 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 5) (end 27 45))
      )
    )
  )
)
~~~
