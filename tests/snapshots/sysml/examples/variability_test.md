# META
~~~ini
description=SysML Example (Simple Tests): VariabilityTest
type=file
~~~
# SOURCE
~~~sysml
package VariabilityTest {
	part def P {
		attribute a;
	}
	
	part def Q :> P;
	attribute def B;
	variation part def V :> P {
		variant part x : Q {
			attribute b : B :>> a;
		}
	}
	
	part q : Q;
	variation part v : P {
		variant q {
			attribute b : B :>> a;
		}
	}
	
	part y : P = v::q;
	
	variation action def A {
		variant action a1;
		variant action a2;
	}
	
	variation use case uc1 {
    	variant use case uc11;
    	variant use case uc12;
    }

    variation analysis a1;
    
    variation verification v1;
    
    variation requirement r {
    	variant requirement r1;
    }
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/variability_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 23 2) (end 23 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 24 2) (end 24 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 27 1) (end 30 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 27 1) (end 30 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 34 4) (end 34 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 34 4) (end 34 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery,unsupported-syntax) (has-evaluation false) (source-digest "blake3:16b888c6815444dfd8aaff94acf942263750ae25ffaac0a3d4031275a0720967") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::A"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P::a"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "P")))))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "P")))))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Q")))))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x::b"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")) (redefinition (reference "a")))))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::a1"))) (kind analysis) (membership (kind feature) (visibility default)) (facts (modifiers variation)))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Q")))))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::r"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (modifiers variation)))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::r::r1"))) (kind requirement) (membership (kind owning) (visibility default) (role variant)))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::v"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P") (variation true)))))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (subsetting (reference "q")))))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)) (named (kind attribute) (name "b"))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")) (redefinition (reference "a")))))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::y"))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q"))) (kind specialization) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V"))) (kind specialization) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")))))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B")))))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x::b"))) (kind redefinition) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P::a")))))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")))))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q")))))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)) (named (kind attribute) (name "b"))))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B")))))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)) (named (kind attribute) (name "b"))))) (kind redefinition) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P::a")))))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x::b"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x::b"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x::b"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::v"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)) (named (kind attribute) (name "b"))))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)) (named (kind attribute) (name "b"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)) (named (kind attribute) (name "b"))))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)) (named (kind attribute) (name "b"))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::y"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P::a"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x::b"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)) (named (kind attribute) (name "b"))))) (target (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B")))
      (subtype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x::b")) (scopes any))
      (subtype (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)) (named (kind attribute) (name "b")))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))
      (subtype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::v")) (scopes any))
      (subtype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::y")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P::a")))
      (featured-by (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))
      (subtype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x::b")) (scopes any feature))
      (subtype (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)) (named (kind attribute) (name "b")))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x")) (scopes any))
      (subtype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V")))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x")))
      (type (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")) (provenance authored))
      (effective-type (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")) (source direct))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")) (scopes any))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x::b")))
      (featured-by (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x")))
      (type (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B")) (source direct))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B")) (scopes any))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P::a")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q")))
      (type (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")) (provenance authored))
      (effective-type (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")) (source direct))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")) (scopes any))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")) (scopes any))
      (subtype (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::v")))
      (type (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")) (source direct))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")) (source inherited) (from (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q"))))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")) (scopes any))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")) (scopes any))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)) (named (kind attribute) (name "b")))))
      (featured-by (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)))))
      (type (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B")) (source direct))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B")) (scopes any))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P::a")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::y")))
      (type (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")) (source direct))
      (supertype (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/variability_test.md") (range (start 5 15) (end 5 16)) (probe (position 5 15))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q"))) (kind specialization) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
    )
  )
  (query (document "memory://snapshot/variability_test.md") (range (start 7 25) (end 7 26)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V"))) (kind specialization) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
    )
  )
  (query (document "memory://snapshot/variability_test.md") (range (start 8 19) (end 8 20)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x"))) (kind featureTyping) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")))))
    )
  )
  (query (document "memory://snapshot/variability_test.md") (range (start 9 17) (end 9 18)) (probe (position 9 17))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B")))))
    )
  )
  (query (document "memory://snapshot/variability_test.md") (range (start 9 23) (end 9 24)) (probe (position 9 23))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V::x::b"))) (kind redefinition) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P::a")))))
    )
  )
  (query (document "memory://snapshot/variability_test.md") (range (start 13 10) (end 13 11)) (probe (position 13 10))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q"))) (kind featureTyping) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")))))
    )
  )
  (query (document "memory://snapshot/variability_test.md") (range (start 14 20) (end 14 21)) (probe (position 14 20))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::v"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
    )
  )
  (query (document "memory://snapshot/variability_test.md") (range (start 15 10) (end 15 11)) (probe (position 15 10))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q")))))
    )
  )
  (query (document "memory://snapshot/variability_test.md") (range (start 16 17) (end 16 18)) (probe (position 16 17))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)) (named (kind attribute) (name "b"))))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B")))))
    )
  )
  (query (document "memory://snapshot/variability_test.md") (range (start 16 23) (end 16 24)) (probe (position 16 23))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (path (named (kind package) (name "VariabilityTest")) (named (kind part) (name "v")) (anonymous (kind ref) (ordinal 0)) (named (kind attribute) (name "b"))))) (kind redefinition) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P::a")))))
    )
  )
  (query (document "memory://snapshot/variability_test.md") (range (start 20 10) (end 20 11)) (probe (position 20 10))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::y"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
    )
  )
)
~~~
