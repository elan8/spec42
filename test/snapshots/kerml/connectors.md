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
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 3 2) (end 4 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 4 2) (end 6 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 6 2) (end 7 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
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
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 9 3) (end 10 3))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 10 3) (end 11 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 13 2) (end 13 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 14 2) (end 14 22))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 16 3) (end 17 3))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 17 3) (end 18 2))
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
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 23 3) (end 24 3))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 24 3) (end 25 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 29 5) (end 30 5))
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:3158d76107a71413e335a2356ac61791c4750462b6335681a61637e4b2b8bc5b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::binding"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::connector"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "c2"))))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::succession"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::B"))) (kind class-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::connector"))) (kind expressionOperand) (ordinal 0))
      (authored-target "c2")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::connector"))) (value (kind unresolved-operand)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/connectors.md") (range (start 8 14) (end 8 16)) (probe (position 8 14))
    (reference (id (source (node (document "memory://snapshot/connectors.md") (qualified-name "Connectors::A::connector"))) (kind expressionOperand) (ordinal 0) (authored-target "c2")
      (outcome (status unresolved)))
  )
)
~~~
