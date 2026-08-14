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
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 8 1) (end 9 1))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 8 1) (end 9 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:cc260e42efc2f0574416b1986958ea8b3cd611f779804213b16cf55f2d510c57") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D1"))) (kind individual-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D13"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D2"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D3"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D4"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D5"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D6"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::D7"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::a1"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::i1"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::o1"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::p1"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::p2"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::po1"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_individual.md") (qualified-name "CoverageIndividual::s1"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
