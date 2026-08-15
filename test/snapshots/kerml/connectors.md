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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 7 2) (end 7 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_parser_construct")
        (source "semantic")
        (range (start 7 2) (end 7 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 14) (end 8 16))
      )
      (diagnostic
        (severity error)
        (code "recovered_attribute_body_element")
        (source "parser")
        (range (start 9 3) (end 10 3))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 9 3) (end 10 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 13 2) (end 13 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_parser_construct")
        (source "semantic")
        (range (start 13 2) (end 13 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 14 2) (end 14 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_parser_construct")
        (source "semantic")
        (range (start 14 2) (end 14 22))
      )
      (diagnostic
        (severity error)
        (code "recovered_attribute_body_element")
        (source "parser")
        (range (start 16 3) (end 17 3))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 16 3) (end 17 3))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 20 2) (end 21 2))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 21 2) (end 22 2))
      )
      (diagnostic
        (severity error)
        (code "recovered_attribute_body_element")
        (source "parser")
        (range (start 23 3) (end 24 3))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 23 3) (end 24 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:3158d76107a71413e335a2356ac61791c4750462b6335681a61637e4b2b8bc5b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::binding"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "a")) (connectorEnd (reference "b")))))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::connector"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "c2")))))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::succession"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (kind connectorEnd) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (kind connectorEnd) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b")))))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::connector"))) (kind expressionOperand) (ordinal 0))
      (authored-target "c2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B::a"))) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B::a"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::connector"))) (state unresolved-operand))
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
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a")))
      (featured-by (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))
      (type (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (source direct))
      (supertype (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::b")))
      (featured-by (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))
      (type (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (source direct))
      (supertype (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::binding")))
      (featured-by (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))
    )
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::c1")))
      (featured-by (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))
    )
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::connector")))
      (featured-by (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))
    )
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::succession")))
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
  (query (document "memory://snapshot/connectors.md") (range (start 3 14) (end 3 15)) (probe (position 3 14))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))))
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
  (query (document "memory://snapshot/connectors.md") (range (start 8 14) (end 8 16)) (probe (position 8 14))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::connector"))) (kind expressionOperand) (ordinal 0) (authored-target "c2")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connectors.md") (range (start 29 17) (end 29 18)) (probe (position 29 17))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A")))))
    )
  )
)
~~~
