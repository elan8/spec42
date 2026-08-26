# META
~~~ini
description=Coverage: individual keyword parsing paths
type=file
~~~
# SOURCE
~~~sysml
package CoverageIndividual {
	individual def D1;
	individual occurrence def D2;
	individual item def D3;
	individual part def D4;
	individual action def D5;
	individual state def D6;
	individual connection def D7;
	individual calc def D8;
	individual constraint def D9;
	individual requirement def D10;
	individual concern def D11;
	individual case def D12;
	individual analysis def D13;
	individual verification def D14;
	individual view def D15;
	individual viewpoint def D16;
	individual rendering def D17;

	individual p1;
	individual occurrence o1;
	individual item i1;
	individual part p2;
	individual port po1;
	individual action a1;
	individual state s1;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/coverage_individual.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 22 1) (end 22 20))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 23 1) (end 23 21))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:cc260e42efc2f0574416b1986958ea8b3cd611f779804213b16cf55f2d510c57") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D1"))) (kind individual-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D10"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D11"))) (kind concern-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D12"))) (kind case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D13"))) (kind analysis-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D14"))) (kind verification-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D15"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D16"))) (kind viewpoint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D17"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D2"))) (kind occurrence-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D3"))) (kind item-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D4"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D5"))) (kind action-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D6"))) (kind state-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D7"))) (kind connection-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D8"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D9"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::a1"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers individual composite)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::i1"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::o1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::p1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::p2"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::po1"))) (kind port) (membership (kind feature) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::s1"))) (kind state) (membership (kind feature) (visibility default)) (facts (modifiers individual)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
