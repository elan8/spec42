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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "keyword_as_name.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d9ceab923dbb202c7b1bc5655129b0c10fb5feeead12aabacc5f9437982edbe2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "KeywordAsName"))) (kind "package") (name "KeywordAsName") (declared-name "KeywordAsName"))
    (element (id (node (document "d0") (qualified-name "KeywordAsName::Container"))) (kind "classifier decl") (name "Container") (declared-name "Container") (parent (node (document "d0") (qualified-name "KeywordAsName"))))
    (element (id (node (document "d0") (qualified-name "KeywordAsName::IfThenElse"))) (kind "kermlDecl") (name "IfThenElse") (declared-name "IfThenElse") (parent (node (document "d0") (qualified-name "KeywordAsName"))))
    (element (id (node (document "d0") (qualified-name "KeywordAsName::MyStruct"))) (kind "classifier decl") (name "MyStruct") (declared-name "MyStruct") (parent (node (document "d0") (qualified-name "KeywordAsName"))))
    (element (id (node (document "d0") (qualified-name "KeywordAsName::SpatialFrame"))) (kind "classifier decl") (name "SpatialFrame") (declared-name "SpatialFrame") (parent (node (document "d0") (qualified-name "KeywordAsName"))))
    (element (id (node (document "d0") (qualified-name "KeywordAsName::TestBehavior"))) (kind "kermlDecl") (name "TestBehavior") (declared-name "TestBehavior") (parent (node (document "d0") (qualified-name "KeywordAsName"))))
    (element (id (node (document "d0") (qualified-name "KeywordAsName::do"))) (kind "feature decl") (name "do") (declared-name "do") (parent (node (document "d0") (qualified-name "KeywordAsName"))))
    (element (id (node (document "d0") (qualified-name "KeywordAsName::multiplicity"))) (kind "alias") (name "multiplicity") (declared-name "multiplicity") (parent (node (document "d0") (qualified-name "KeywordAsName"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
