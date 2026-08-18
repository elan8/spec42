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
  (document "memory://snapshot/keyword_as_name.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 25 1) (end 25 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 25 1) (end 25 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:00159580b8a880f642649626ab8582199534da466003ec034252e9ff802726fd") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::Container"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (path (named (kind package) (name "KeywordAsName")) (named (kind kerml-classifier) (name "Container")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialFrame")))))
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::IfThenElse"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::IfThenElse::condition"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::IfThenElse::elseValue"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::IfThenElse::thenValue"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct::frame"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialFrame") (direction in)))))
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct::type"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialFrame") (direction in)))))
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::TestBehavior"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::TestBehavior::guardCondition"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::TestBehavior::whileTest"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::multiplicity"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "SpatialFrame")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/keyword_as_name.md") (path (named (kind package) (name "KeywordAsName")) (named (kind kerml-classifier) (name "Container")) (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")))))
    (reference (id (source (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct::frame"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")))))
    (reference (id (source (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct::type"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")))))
    (reference (id (source (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::multiplicity"))) (kind aliasBinding) (ordinal 0))
      (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/keyword_as_name.md") (path (named (kind package) (name "KeywordAsName")) (named (kind kerml-classifier) (name "Container")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/keyword_as_name.md") (path (named (kind package) (name "KeywordAsName")) (named (kind kerml-classifier) (name "Container")) (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct::frame"))) (target (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct::frame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct::type"))) (target (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct::type"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::multiplicity"))) (target (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::multiplicity"))) (kind aliasBinding) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::IfThenElse::condition"))) (state literal) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::IfThenElse::elseValue"))) (state literal) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::IfThenElse::thenValue"))) (state literal) (value (kind integer) (integer 42)))
    (evaluated (declaration (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::TestBehavior::guardCondition"))) (state literal) (value (kind boolean) (boolean false)))
    (evaluated (declaration (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::TestBehavior::whileTest"))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (path (named (kind package) (name "KeywordAsName")) (named (kind kerml-classifier) (name "Container")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::Container")))
      (type (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")) (provenance authored))
      (effective-type (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")) (source direct))
      (supertype (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::IfThenElse::condition")))
      (featured-by (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::IfThenElse")))
    )
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::IfThenElse::elseValue")))
      (featured-by (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::IfThenElse")))
    )
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::IfThenElse::thenValue")))
      (featured-by (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::IfThenElse")))
    )
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct::frame")))
      (featured-by (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct")))
      (type (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")) (provenance authored))
      (effective-type (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")) (source direct))
      (supertype (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct::type")))
      (featured-by (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct")))
      (type (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")) (provenance authored))
      (effective-type (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")) (source direct))
      (supertype (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")))
      (subtype (node (document "memory://snapshot/keyword_as_name.md") (path (named (kind package) (name "KeywordAsName")) (named (kind kerml-classifier) (name "Container")) (anonymous (kind part) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct::frame")) (scopes any))
      (subtype (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct::type")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::TestBehavior::guardCondition")))
      (featured-by (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::TestBehavior")))
    )
    (declaration (id (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::TestBehavior::whileTest")))
      (featured-by (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::TestBehavior")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/keyword_as_name.md") (range (start 29 12) (end 29 24)) (probe (position 29 12))
    (reference (id (source (node (document "memory://snapshot/keyword_as_name.md") (path (named (kind package) (name "KeywordAsName")) (named (kind kerml-classifier) (name "Container")) (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")))))
    )
  )
  (query (document "memory://snapshot/keyword_as_name.md") (range (start 17 13) (end 17 25)) (probe (position 17 13))
    (reference (id (source (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct::frame"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")))))
    )
  )
  (query (document "memory://snapshot/keyword_as_name.md") (range (start 18 12) (end 18 24)) (probe (position 18 12))
    (reference (id (source (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::MyStruct::type"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")))))
    )
  )
  (query (document "memory://snapshot/keyword_as_name.md") (range (start 22 24) (end 22 36)) (probe (position 22 24))
    (reference (id (source (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::multiplicity"))) (kind aliasBinding) (ordinal 0) (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/keyword_as_name.md") (qualified-name "KeywordAsName::SpatialFrame")))))
    )
  )
)
~~~
