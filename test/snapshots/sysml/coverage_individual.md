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
  (document "coverage_individual.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 2 1) (end 2 32))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 2 1) (end 2 32))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "e6303e694313bae52dbf13599fc1cb9173f27ad8ffb059a92daa7cacae3dc96b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "CoverageIndividual"))) (kind "package") (name "CoverageIndividual") (declared-name "CoverageIndividual"))
    (element (id (node (document "d0") (qualified-name "CoverageIndividual::D1"))) (kind "individual def") (name "D1") (declared-name "D1") (parent (node (document "d0") (qualified-name "CoverageIndividual"))))
    (element (id (node (document "d0") (qualified-name "CoverageIndividual::D4"))) (kind "part def") (name "D4") (declared-name "D4") (parent (node (document "d0") (qualified-name "CoverageIndividual"))))
    (element (id (node (document "d0") (qualified-name "CoverageIndividual::p1"))) (kind "occurrence") (name "p1") (declared-name "p1") (parent (node (document "d0") (qualified-name "CoverageIndividual"))))
    (element (id (node (document "d0") (qualified-name "CoverageIndividual::p2"))) (kind "part") (name "p2") (declared-name "p2") (parent (node (document "d0") (qualified-name "CoverageIndividual"))))
    (element (id (node (document "d0") (qualified-name "CoverageIndividual::s1"))) (kind "state") (name "s1") (declared-name "s1") (parent (node (document "d0") (qualified-name "CoverageIndividual"))))
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
