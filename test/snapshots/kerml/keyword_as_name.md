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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d9ceab923dbb202c7b1bc5655129b0c10fb5feeead12aabacc5f9437982edbe2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "KeywordAsName"))) (kind "package") (name "KeywordAsName") (declared-name "KeywordAsName") (range (start (line 0) (character 0)) (end (line 0) (character 734))))
    (element (id (node (document "d0") (qualified-name "KeywordAsName::Container"))) (kind "classifier decl") (name "Container") (declared-name "Container") (range (start (line 28) (character 1)) (end (line 28) (character 52))) (parent (node (document "d0") (qualified-name "KeywordAsName"))))
    (element (id (node (document "d0") (qualified-name "KeywordAsName::IfThenElse"))) (kind "kermlDecl") (name "IfThenElse") (declared-name "IfThenElse") (range (start (line 2) (character 1)) (end (line 2) (character 122))) (parent (node (document "d0") (qualified-name "KeywordAsName"))))
    (element (id (node (document "d0") (qualified-name "KeywordAsName::MyStruct"))) (kind "classifier decl") (name "MyStruct") (declared-name "MyStruct") (range (start (line 16) (character 1)) (end (line 16) (character 77))) (parent (node (document "d0") (qualified-name "KeywordAsName"))))
    (element (id (node (document "d0") (qualified-name "KeywordAsName::SpatialFrame"))) (kind "classifier decl") (name "SpatialFrame") (declared-name "SpatialFrame") (range (start (line 15) (character 1)) (end (line 15) (character 25))) (parent (node (document "d0") (qualified-name "KeywordAsName"))))
    (element (id (node (document "d0") (qualified-name "KeywordAsName::TestBehavior"))) (kind "kermlDecl") (name "TestBehavior") (declared-name "TestBehavior") (range (start (line 9) (character 1)) (end (line 9) (character 91))) (parent (node (document "d0") (qualified-name "KeywordAsName"))))
    (element (id (node (document "d0") (qualified-name "KeywordAsName::do"))) (kind "feature decl") (name "do") (declared-name "do") (range (start (line 25) (character 1)) (end (line 25) (character 29))) (parent (node (document "d0") (qualified-name "KeywordAsName"))))
    (element (id (node (document "d0") (qualified-name "KeywordAsName::multiplicity"))) (kind "alias") (name "multiplicity") (declared-name "multiplicity") (range (start (line 22) (character 1)) (end (line 22) (character 37))) (parent (node (document "d0") (qualified-name "KeywordAsName"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
