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
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 8 2) (end 10 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 15 2) (end 17 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 22 1) (end 25 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 22 1) (end 25 2))
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
        (range (start 32 4) (end 32 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 32 4) (end 32 26))
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
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 37 5) (end 37 28))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:16b888c6815444dfd8aaff94acf942263750ae25ffaac0a3d4031275a0720967") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::B"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P::a"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "P"))))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "P"))))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Q"))))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::r"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (modifiers variation)))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::v"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P") (variation true))))
    (declaration (id (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::y"))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q"))) (kind specialization) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V"))) (kind specialization) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")))))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::v"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::y"))) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::y"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
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
  (query (document "memory://snapshot/variability_test.md") (range (start 7 25) (end 7 26)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::V"))) (kind specialization) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
  )
  (query (document "memory://snapshot/variability_test.md") (range (start 13 10) (end 13 11)) (probe (position 13 10))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::q"))) (kind featureTyping) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::Q")))))
  )
  (query (document "memory://snapshot/variability_test.md") (range (start 14 20) (end 14 21)) (probe (position 14 20))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::v"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
  )
  (query (document "memory://snapshot/variability_test.md") (range (start 20 10) (end 20 11)) (probe (position 20 10))
    (reference (id (source (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::y"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/variability_test.md") (qualified-name "VariabilityTest::P")))))
  )
)
~~~
