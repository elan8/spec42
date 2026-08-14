# META
~~~ini
description=KerML Simple Tests: FeatureChains
type=file
~~~
# SOURCE
~~~kerml
package FeatureChains {
	classifier F {
		feature a : A;
	}
	  
	feature f : F;
	  
	classifier A {
		feature g = f.a;
	}
	  
	classifier B {
	  	feature f : F;
	  	feature a : A;
	}
	  
	feature b : B {
	  	connector f.a to a.g;
	  	binding f.a = a.g;
	}
	  
	feature g subsets f.a;
	subset g.g subsets b.f.a;
	redefinition b.f redefines b.a;
	  
	subtype g.g specializes b.f.a;
	
	disjoint b.f.a from b.a;
	
	feature h1 unions f, b.f, b.a;
	feature h2 differences b.f, b.a intersects f.a, g disjoint from h1;
	
	feature b_f_a chains b chains f.a;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/feature_chains.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 17 4) (end 18 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 12) (end 18 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 18) (end 18 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 19) (end 21 22))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 22 1) (end 29 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 29 1) (end 29 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 29 1) (end 29 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 30 1) (end 30 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 30 1) (end 30 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 32 1) (end 32 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 32 1) (end 32 35))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:48b26c0fd1f2ee265267ec9d5aa8c99bcbc717e80655b227e364c9b1741bccf9") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::A::g"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "f::a"))))
    (declaration (id (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::B::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::B::f"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "F"))))
    (declaration (id (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B"))))
    (declaration (id (node (document "memory://snapshot/feature_chains.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind kerml-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "f::a")) (bindTarget (reference "a::g"))))
    (declaration (id (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::f"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "F"))))
    (declaration (id (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::g"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "f::a"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::A::g"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "f::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F::a")))))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::B::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::A")))))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::B::f"))) (kind featureTyping) (ordinal 0))
      (authored-target "F")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F")))))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::A")))))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::B")))))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "f::a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "a::g")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::f"))) (kind featureTyping) (ordinal 0))
      (authored-target "F")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F")))))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::g"))) (kind subsetting) (ordinal 0))
      (authored-target "f::a")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::A::g"))) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::A::g"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::B::a"))) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::B::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::B::f"))) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::B::f"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F::a"))) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::b"))) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::f"))) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::f"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/feature_chains.md") (range (start 8 14) (end 8 17)) (probe (position 8 14))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::A::g"))) (kind memberAccessOperand) (ordinal 0) (authored-target "f::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F::a")))))
  )
  (query (document "memory://snapshot/feature_chains.md") (range (start 13 16) (end 13 17)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::B::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::A")))))
  )
  (query (document "memory://snapshot/feature_chains.md") (range (start 12 16) (end 12 17)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::B::f"))) (kind featureTyping) (ordinal 0) (authored-target "F")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F")))))
  )
  (query (document "memory://snapshot/feature_chains.md") (range (start 2 14) (end 2 15)) (probe (position 2 14))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::A")))))
  )
  (query (document "memory://snapshot/feature_chains.md") (range (start 16 13) (end 16 14)) (probe (position 16 13))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::B")))))
  )
  (query (document "memory://snapshot/feature_chains.md") (range (start 18 12) (end 18 15)) (probe (position 18 12))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "f::a")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_chains.md") (range (start 18 18) (end 18 21)) (probe (position 18 18))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "a::g")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_chains.md") (range (start 5 13) (end 5 14)) (probe (position 5 13))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::f"))) (kind featureTyping) (ordinal 0) (authored-target "F")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::F")))))
  )
  (query (document "memory://snapshot/feature_chains.md") (range (start 21 19) (end 21 22)) (probe (position 21 19))
    (reference (id (source (node (document "memory://snapshot/feature_chains.md") (qualified-name "FeatureChains::g"))) (kind subsetting) (ordinal 0) (authored-target "f::a")
      (outcome (status unresolved)))
  )
)
~~~
