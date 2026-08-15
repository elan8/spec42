# META
~~~ini
description=KerML Simple Tests: Types
type=file
~~~
# SOURCE
~~~kerml
package Types {
	abstract type A specializes Base::Anything;
	type all x specializes A, Base::things;
	
	// This Type has exactly one instance.
	type Singleton[1] specializes Base::Anything;
	
	type Super specializes Base::Anything {
	    private package P {
	        type Sub specializes Super;
	    }
	    protected feature f : P::Sub;
	}
	
	type B :> Base::Anything;
	
	specialization Gen subtype A specializes B;
	specialization subtype x :> Base::things;
	
	type Original specializes Base::Anything {
	    in feature Input; 
	}
	type Conjugate1 specializes Base::Anything;
	type Conjugate2 specializes Base::Anything;
	conjugation c1 conjugate Conjugate1 conjugates Original; 
	conjugation c2 conjugate Conjugate2 ~ Original; 
	
	type Conjugate3 conjugates Original;
	type Conjugate4 ~ Conjugate1;
	
	type C :> B disjoint from A;
	
	type D :> Base::Anything unions A, B;
	type E :> Base::Anything intersects A, B;
	type F :> Base::Anything differences A, B;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/types.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1 29) (end 1 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 2 27) (end 2 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 5 31) (end 5 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 7 24) (end 7 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 5) (end 8 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 13) (end 8 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 21) (end 8 22))
      )
      (diagnostic
        (severity error)
        (code "recovered_calc_body_element")
        (source "parser")
        (range (start 8 23) (end 11 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 27) (end 11 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 14 11) (end 14 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 16 1) (end 16 44))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 17 1) (end 35 0))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:92bf859ed276e45c4017951a46cdb187a56914e606c9142aeeb6ef6f42b23613") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/types.md") (qualified-name "Types"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/types.md") (qualified-name "Types::A"))) (kind kerml-type) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base::Anything")))))
    (declaration (id (node (document "memory://snapshot/types.md") (qualified-name "Types::B"))) (kind kerml-type) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base::Anything")))))
    (declaration (id (node (document "memory://snapshot/types.md") (qualified-name "Types::Singleton"))) (kind kerml-type) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base::Anything")))))
    (declaration (id (node (document "memory://snapshot/types.md") (qualified-name "Types::Super"))) (kind kerml-type) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base::Anything")) (expressionOperand (reference "private")) (expressionOperand (reference "package")) (expressionOperand (reference "P")))))
    (declaration (id (node (document "memory://snapshot/types.md") (qualified-name "Types::Super::f"))) (kind kerml-feature) (membership (kind feature) (visibility protected)) (authored (membership (kind feature) (visibility protected)) (relationships (featureTyping (reference "P::Sub")))))
    (declaration (id (node (document "memory://snapshot/types.md") (qualified-name "Types::x"))) (kind kerml-type) (membership (kind owning) (visibility default)) (facts (modifiers all)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")) (specialization (reference "Base::things")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::A"))) (kind specialization) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::B"))) (kind specialization) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::Singleton"))) (kind specialization) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::Super"))) (kind specialization) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::Super"))) (kind expressionOperand) (ordinal 0))
      (authored-target "private")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::Super"))) (kind expressionOperand) (ordinal 1))
      (authored-target "package")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::Super"))) (kind expressionOperand) (ordinal 2))
      (authored-target "P")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::Super::f"))) (kind featureTyping) (ordinal 0))
      (authored-target "P::Sub")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::x"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/types.md") (qualified-name "Types::A")))))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::x"))) (kind specialization) (ordinal 1))
      (authored-target "Base::things")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/types.md") (qualified-name "Types::x"))) (target (node (document "memory://snapshot/types.md") (qualified-name "Types::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/types.md") (qualified-name "Types::x"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/types.md") (qualified-name "Types::Super"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/types.md") (qualified-name "Types::Super"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/types.md") (qualified-name "Types::Super"))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/types.md") (qualified-name "Types::A")))
      (subtype (node (document "memory://snapshot/types.md") (qualified-name "Types::x")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/types.md") (qualified-name "Types::Super::f")))
      (featured-by (node (document "memory://snapshot/types.md") (qualified-name "Types::Super")))
    )
    (declaration (id (node (document "memory://snapshot/types.md") (qualified-name "Types::x")))
      (supertype (node (document "memory://snapshot/types.md") (qualified-name "Types::A")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/types.md") (range (start 1 29) (end 1 43)) (probe (position 1 29))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::A"))) (kind specialization) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/types.md") (range (start 14 11) (end 14 25)) (probe (position 14 11))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::B"))) (kind specialization) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/types.md") (range (start 5 31) (end 5 45)) (probe (position 5 31))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::Singleton"))) (kind specialization) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/types.md") (range (start 7 24) (end 7 38)) (probe (position 7 24))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::Super"))) (kind specialization) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/types.md") (range (start 8 5) (end 8 12)) (probe (position 8 5))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::Super"))) (kind expressionOperand) (ordinal 0) (authored-target "private")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/types.md") (range (start 8 13) (end 8 20)) (probe (position 8 13))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::Super"))) (kind expressionOperand) (ordinal 1) (authored-target "package")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/types.md") (range (start 8 21) (end 8 22)) (probe (position 8 21))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::Super"))) (kind expressionOperand) (ordinal 2) (authored-target "P")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/types.md") (range (start 11 27) (end 11 33)) (probe (position 11 27))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::Super::f"))) (kind featureTyping) (ordinal 0) (authored-target "P::Sub")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/types.md") (range (start 2 24) (end 2 25)) (probe (position 2 24))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::x"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/types.md") (qualified-name "Types::A")))))
    )
  )
  (query (document "memory://snapshot/types.md") (range (start 2 27) (end 2 39)) (probe (position 2 27))
    (reference (id (source (node (document "memory://snapshot/types.md") (qualified-name "Types::x"))) (kind specialization) (ordinal 1) (authored-target "Base::things")
      (outcome (status unresolved)))
    )
  )
)
~~~
