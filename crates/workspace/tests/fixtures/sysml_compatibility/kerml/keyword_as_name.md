# META
~~~ini
description=KerML keyword-as-name: keywords used as declared names and KerML usage keywords with direction prefixes
type=file
~~~
# SOURCE
~~~kerml
package KeywordAsName {
	// P1: KerML usage keywords with direction prefixes
	function IfThenElse {
		in bool condition[1] { true }
		in expr thenValue[0..*] { 42 }
		in expr elseValue[0..*] { 0 }
	}

	// P1: direction prefix with expr
	behavior TestBehavior {
		in expr whileTest { true }
		in bool guardCondition { false }
	}

	// P3: keywords used as names in features
	classifier SpatialFrame;
	struct MyStruct {
		in frame : SpatialFrame[1];
		in type : SpatialFrame;
	}

	// P3: keyword as name in alias
	alias multiplicity for SpatialFrame;

	// P3: keyword as short name
	feature <do> : SpatialFrame;

	// Regression: usage dispatch keywords must NOT be consumed as names
	classifier Container {
		in part : SpatialFrame;
	}
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
LineComment,
KwFunction,Ident,OpenCurly,
KwIn,KwBool,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,KwTrue,CloseCurly,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,DecimalValue,CloseCurly,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,DecimalValue,CloseCurly,
CloseCurly,
LineComment,
KwBehavior,Ident,OpenCurly,
KwIn,KwExpr,Ident,OpenCurly,KwTrue,CloseCurly,
KwIn,KwBool,Ident,OpenCurly,KwFalse,CloseCurly,
CloseCurly,
LineComment,
KwClassifier,Ident,Semicolon,
KwStruct,Ident,OpenCurly,
KwIn,KwFrame,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,KwType,Colon,Ident,Semicolon,
CloseCurly,
LineComment,
KwAlias,KwMultiplicity,KwFor,Ident,Semicolon,
LineComment,
KwFeature,OpenAngle,KwDo,CloseAngle,Colon,Ident,Semicolon,
LineComment,
KwClassifier,Ident,OpenCurly,
KwIn,KwPart,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'KeywordAsName'
    (line_comment)
    (function_def
      (boolean_expr_usage
        (result_expr_member))
      (expression_usage
        (result_expr_member))
      (expression_usage
        (result_expr_member)))
    (line_comment)
    (behavior_def
      (expression_usage
        (result_expr_member))
      (boolean_expr_usage
        (result_expr_member)))
    (line_comment)
    (classifier_def 'SpatialFrame')
    (structure_def 'MyStruct'
      (feature_def in 'frame' : 'SpatialFrame' multiplicity)
      (feature_def in 'type' : 'SpatialFrame'))
    (line_comment)
    (alias_member 'multiplicity' for 'SpatialFrame')
    (line_comment)
    (feature_def 'do' : 'SpatialFrame')
    (line_comment)
    (classifier_def 'Container'
      (part_usage in : 'SpatialFrame'))))
~~~
# FORMAT
~~~sysml
package KeywordAsName {
    // P1: KerML usage keywords with direction prefixes
    function IfThenElse {
		in bool condition[1] { true }
		in expr thenValue[0..*] { 42 }
		in expr elseValue[0..*] { 0 }
	}

    // P1: direction prefix with expr
    behavior TestBehavior {
        in expr whileTest { true }
        in bool guardCondition { false }
    }

    // P3: keywords used as names in features
    classifier SpatialFrame;
    struct MyStruct {
        in frame: SpatialFrame [1];
        in type: SpatialFrame;
    }

    // P3: keyword as name in alias
    alias multiplicity for SpatialFrame;

    // P3: keyword as short name
    feature <do> : SpatialFrame;

    // Regression: usage dispatch keywords must NOT be consumed as names
    classifier Container {
        in part : SpatialFrame;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'KeywordAsName'
      (function_def 'IfThenElse'
        (boolean_expr_usage in 'condition'
          (multiplicity_range [1])
          (result_expr_membership))
        (expression_usage in 'thenValue'
          (multiplicity_range [0..*])
          (result_expr_membership))
        (expression_usage in 'elseValue'
          (multiplicity_range [0..*])
          (result_expr_membership)))
      (behavior_def 'TestBehavior'
        (expression_usage in 'whileTest'
          (result_expr_membership))
        (boolean_expr_usage in 'guardCondition'
          (result_expr_membership)))
      (classifier_def 'SpatialFrame')
      (structure_def 'MyStruct'
        (feature_def in 'frame' : 'KeywordAsName::SpatialFrame'[classifier_def]
          (multiplicity_range [1]))
        (feature_def in 'type' : 'KeywordAsName::SpatialFrame'[classifier_def]))
      (alias_member 'multiplicity' -> 'KeywordAsName::SpatialFrame'[classifier_def])
      (feature_def : 'KeywordAsName::SpatialFrame'[classifier_def])
      (classifier_def 'Container'
        (part_usage in : 'KeywordAsName::SpatialFrame'[classifier_def])))))
~~~
