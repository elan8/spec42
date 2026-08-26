# META
~~~ini
description=KerML Simple Tests: Connectors
type=file
~~~
# SOURCE
~~~kerml
package Connectors {
	
	class A {
		feature a : A;
		feature b : A;
		
		connector c1 from a to b;
		abstract connector c2 = c1;
		connector = c2 {
			end feature references a;
			end feature references b;
		}
		
		binding a = b;
		binding ab of a = b;
		binding {
			end feature references a;
			end feature references b;
		}
		
		succession a then b;
		succession s first a then b;
		succession {
			end feature references a;
			end feature references b;
		}
	}
	
	class B {
	    feature a : A;	    
	    connector :> a.c1 from a.a to a.b;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/connectors.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 2) (end 7 10))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 7 11) (end 8 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 8 2) (end 13 2))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 15 2) (end 20 2))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 22 2) (end 26 1))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 30 5) (end 31 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:3158d76107a71413e335a2356ac61791c4750462b6335681a61637e4b2b8bc5b") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "abstract")))))
    (declaration (id (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind kerml-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "a")) (bindTarget (reference "b")))))
    (declaration (id (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "a")) (succession (reference "b")))))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::ab"))) (kind kerml-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "a")) (bindTarget (reference "b")))))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "a")) (connectorEnd (reference "b")))))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::s"))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "a")) (succession (reference "b")))))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (kind expressionOperand) (ordinal 0))
      (authored-target "abstract")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::ab"))) (kind bindSource) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::ab"))) (kind bindTarget) (ordinal 0))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (kind connectorEnd) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (kind connectorEnd) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::s"))) (kind succession) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::s"))) (kind succession) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind bindSource) (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind kerml-binding) (ordinal 0))))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0)))
    (relationship (kind bindTarget) (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind kerml-binding) (ordinal 0))))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind bindSource) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::ab"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::ab"))) (kind bindSource) (ordinal 0)))
    (relationship (kind bindTarget) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::ab"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::ab"))) (kind bindTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::s"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::s"))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::s"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::s"))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B::a"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind kerml-binding) (ordinal 0))))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::ab"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::s"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B::a"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))
      (subtype (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a")) (scopes any))
      (subtype (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b")) (scopes any))
      (subtype (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B::a")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind kerml-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))
    )
    (declaration (id (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))
    )
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a")))
      (featured-by (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))
      (type (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (source direct))
      (supertype (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::ab")))
      (featured-by (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))
    )
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b")))
      (featured-by (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))
      (type (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (source direct))
      (supertype (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1")))
      (featured-by (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))
    )
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::s")))
      (featured-by (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))
    )
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B::a")))
      (featured-by (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B")))
      (type (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (source direct))
      (supertype (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/connectors.md") (range (start 7 2) (end 7 10)) (probe (position 7 2))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (kind expressionOperand) (ordinal 0) (authored-target "abstract")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connectors.md") (range (start 20 13) (end 20 14)) (probe (position 20 13))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a")))))
    )
  )
  (query (document "memory://snapshot/connectors.md") (range (start 20 20) (end 20 21)) (probe (position 20 20))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b")))))
    )
  )
  (query (document "memory://snapshot/connectors.md") (range (start 13 10) (end 13 11)) (probe (position 13 10))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a")))))
    )
  )
  (query (document "memory://snapshot/connectors.md") (range (start 13 14) (end 13 15)) (probe (position 13 14))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (path (named (kind package) (name "Connectors")) (named (kind class-def) (name "A")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b")))))
    )
  )
  (query (document "memory://snapshot/connectors.md") (range (start 3 14) (end 3 15)) (probe (position 3 14))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))))
    )
  )
  (query (document "memory://snapshot/connectors.md") (range (start 14 16) (end 14 17)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::ab"))) (kind bindSource) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a")))))
    )
  )
  (query (document "memory://snapshot/connectors.md") (range (start 14 20) (end 14 21)) (probe (position 14 20))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::ab"))) (kind bindTarget) (ordinal 0) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b")))))
    )
  )
  (query (document "memory://snapshot/connectors.md") (range (start 4 14) (end 4 15)) (probe (position 4 14))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))))
    )
  )
  (query (document "memory://snapshot/connectors.md") (range (start 6 20) (end 6 21)) (probe (position 6 20))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (kind connectorEnd) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a")))))
    )
  )
  (query (document "memory://snapshot/connectors.md") (range (start 6 25) (end 6 26)) (probe (position 6 25))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (kind connectorEnd) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b")))))
    )
  )
  (query (document "memory://snapshot/connectors.md") (range (start 21 21) (end 21 22)) (probe (position 21 21))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::s"))) (kind succession) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a")))))
    )
  )
  (query (document "memory://snapshot/connectors.md") (range (start 21 28) (end 21 29)) (probe (position 21 28))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::s"))) (kind succession) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b")))))
    )
  )
  (query (document "memory://snapshot/connectors.md") (range (start 29 17) (end 29 18)) (probe (position 29 17))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))))
    )
  )
)
~~~
