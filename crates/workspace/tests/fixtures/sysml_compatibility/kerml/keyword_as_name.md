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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "KeywordAsName"))) (name "KeywordAsName") (declared-name "KeywordAsName")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "KeywordAsName::Container"))) (name "Container") (declared-name "Container"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KeywordAsName::IfThenElse"))) (name "IfThenElse") (declared-name "IfThenElse"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "KeywordAsName::MyStruct"))) (name "MyStruct") (declared-name "MyStruct"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "KeywordAsName::SpatialFrame"))) (name "SpatialFrame") (declared-name "SpatialFrame"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "KeywordAsName::TestBehavior"))) (name "TestBehavior") (declared-name "TestBehavior"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "KeywordAsName::do"))) (name "do") (declared-name "do"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "KeywordAsName::multiplicity"))) (name "multiplicity") (declared-name "multiplicity"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
