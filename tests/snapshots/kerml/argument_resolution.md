# META
~~~ini
description=KerML Simple Tests: ArgumentResolution
type=file
~~~
# SOURCE
~~~kerml
package ArgumentResolutionBug {
	class A {
		feature x;
	}
	
	behavior B  {
		in feature x;
		out feature : A = new A(x);
	}
	
	class C {
		feature a : A;
		feature b : B;
		
		connector a ::> a.x to b;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/argument_resolution.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:b753ec2a1688d588b7fc25704884c3a36dbeb6ed77aafa28f53b6a331363e5bc") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)) (feature-value (kind bind) (value (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A") (direction out)))))
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "x")) (invocationCallee (reference "A")))))
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind class-def) (name "C")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "a")) (connectorEnd (reference "b")))))
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A")))))
    (reference (id (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B::x")))))
    (reference (id (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A")))))
    (reference (id (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind class-def) (name "C")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::a")))))
    (reference (id (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind class-def) (name "C")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::b")))))
    (reference (id (source (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A")))))
    (reference (id (source (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B")))))
  )
  (relationships
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind class-def) (name "C")) (anonymous (kind kerml-connector) (ordinal 0))))) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind class-def) (name "C")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind class-def) (name "C")) (anonymous (kind kerml-connector) (ordinal 0))))) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind class-def) (name "C")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::a"))) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::b"))) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A::x"))) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B::x"))) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind class-def) (name "C")) (anonymous (kind kerml-connector) (ordinal 0))))) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::a"))) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::b"))) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A")))
      (subtype (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::a")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A::x")))
      (featured-by (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A")))
    )
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B")))
      (subtype (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::b")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B")))
      (type (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A")) (source direct))
      (supertype (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B::x")))
      (featured-by (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B")))
    )
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind class-def) (name "C")) (anonymous (kind kerml-connector) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C")))
    )
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::a")))
      (featured-by (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C")))
      (type (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A")) (source direct))
      (supertype (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::b")))
      (featured-by (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C")))
      (type (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B")) (source direct))
      (supertype (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/argument_resolution.md") (range (start 7 16) (end 7 17)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A")))))
    )
  )
  (query (document "memory://snapshot/argument_resolution.md") (range (start 7 26) (end 7 27)) (probe (position 7 26))
    (reference (id (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B::x")))))
    )
  )
  (query (document "memory://snapshot/argument_resolution.md") (range (start 7 24) (end 7 25)) (probe (position 7 24))
    (reference (id (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind kerml-behavior) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A")))))
    )
  )
  (query (document "memory://snapshot/argument_resolution.md") (range (start 14 12) (end 14 13)) (probe (position 14 12))
    (reference (id (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind class-def) (name "C")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::a")))))
    )
  )
  (query (document "memory://snapshot/argument_resolution.md") (range (start 14 25) (end 14 26)) (probe (position 14 25))
    (reference (id (source (node (document "memory://snapshot/argument_resolution.md") (path (named (kind package) (name "ArgumentResolutionBug")) (named (kind class-def) (name "C")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::b")))))
    )
  )
  (query (document "memory://snapshot/argument_resolution.md") (range (start 11 14) (end 11 15)) (probe (position 11 14))
    (reference (id (source (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::A")))))
    )
  )
  (query (document "memory://snapshot/argument_resolution.md") (range (start 12 14) (end 12 15)) (probe (position 12 14))
    (reference (id (source (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::C::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/argument_resolution.md") (qualified-name "ArgumentResolutionBug::B")))))
    )
  )
)
~~~
