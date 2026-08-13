# META
~~~ini
description=KerML Simple Tests: Associations
type=file
~~~
# SOURCE
~~~kerml
package Associations {
    datatype X;
    class Y;
    
	assoc A {
		end x_cross [1..1] feature x : X; 
		end y_cross [1..*] feature y : Y;
	}
	
	assoc B specializes A {
		end x1;
		end [0..*] feature y1 redefines y;
	}
	
	assoc struct C {
		const end [1] feature a;
		const end feature b;
	}
	
	metaclass M;	
	assoc XY {
		end [0..1] feature x : X {
			@M;
		}
		end feature y : Y;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/associations.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 4) (end 1 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 5 2) (end 5 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 6 2) (end 6 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 11 2) (end 11 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 2) (end 15 7))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 15 8) (end 15 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 2) (end 16 7))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 19 1) (end 19 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 21 2) (end 23 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:9c1cfd6ca73422e202a633654de22826f37b6d9381a913651af6e4a2d2704f0a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A"))))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B::x1"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::C"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "const")) (expressionOperand (reference "const"))))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::C::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Y"))))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y"))) (kind class-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A")))))
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::C"))) (kind expressionOperand) (ordinal 0))
      (authored-target "const")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::C"))) (kind expressionOperand) (ordinal 1))
      (authored-target "const")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "Y")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B"))) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY::y"))) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY::y"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/associations.md") (qualified-name "Associations::C"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/associations.md") (qualified-name "Associations::C"))) (value (kind unresolved-operand)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/associations.md") (range (start 9 21) (end 9 22)) (probe (position 9 21))
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A")))))
  )
  (query (document "memory://snapshot/associations.md") (range (start 15 2) (end 15 7)) (probe (position 15 2))
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::C"))) (kind expressionOperand) (ordinal 0) (authored-target "const")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/associations.md") (range (start 16 2) (end 16 7)) (probe (position 16 2))
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::C"))) (kind expressionOperand) (ordinal 1) (authored-target "const")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/associations.md") (range (start 24 18) (end 24 19)) (probe (position 24 18))
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY::y"))) (kind featureTyping) (ordinal 0) (authored-target "Y")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y")))))
  )
)
~~~
