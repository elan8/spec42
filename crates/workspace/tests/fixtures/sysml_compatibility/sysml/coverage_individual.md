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
# EXPECTED
~~~
semantic.invalid_connection_end_count
~~~
# PROBLEMS
~~~
semantic.invalid_connection_end_count
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwIndividual,KwDef,Ident,Semicolon,
KwIndividual,KwOccurrence,KwDef,Ident,Semicolon,
KwIndividual,KwItem,KwDef,Ident,Semicolon,
KwIndividual,KwPart,KwDef,Ident,Semicolon,
KwIndividual,KwAction,KwDef,Ident,Semicolon,
KwIndividual,KwState,KwDef,Ident,Semicolon,
KwIndividual,KwConnection,KwDef,Ident,Semicolon,
KwIndividual,KwCalc,KwDef,Ident,Semicolon,
KwIndividual,KwConstraint,KwDef,Ident,Semicolon,
KwIndividual,KwRequirement,KwDef,Ident,Semicolon,
KwIndividual,KwConcern,KwDef,Ident,Semicolon,
KwIndividual,KwCase,KwDef,Ident,Semicolon,
KwIndividual,KwAnalysis,KwDef,Ident,Semicolon,
KwIndividual,KwVerification,KwDef,Ident,Semicolon,
KwIndividual,KwView,KwDef,Ident,Semicolon,
KwIndividual,KwViewpoint,KwDef,Ident,Semicolon,
KwIndividual,KwRendering,KwDef,Ident,Semicolon,
KwIndividual,Ident,Semicolon,
KwIndividual,KwOccurrence,Ident,Semicolon,
KwIndividual,KwItem,Ident,Semicolon,
KwIndividual,KwPart,Ident,Semicolon,
KwIndividual,KwPort,Ident,Semicolon,
KwIndividual,KwAction,Ident,Semicolon,
KwIndividual,KwState,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'CoverageIndividual'
    (individual_def individual 'D1')
    (occurrence_def individual 'D2')
    (item_def individual 'D3')
    (part_def individual 'D4')
    (action_def individual 'D5')
    (state_def individual 'D6')
    (connection_def individual 'D7')
    (calc_def individual 'D8')
    (constraint_def individual 'D9')
    (requirement_def individual 'D10')
    (concern_def individual 'D11')
    (case_def individual 'D12')
    (analysis_case_def individual 'D13')
    (verification_case_def individual 'D14')
    (view_def individual 'D15')
    (viewpoint_def individual 'D16')
    (rendering_def individual 'D17')
    (individual_usage individual 'p1')
    (occurrence_usage individual 'o1')
    (item_usage individual 'i1')
    (part_usage individual 'p2')
    (port_usage individual 'po1')
    (action_usage individual 'a1')
    (state_usage individual 's1')))
~~~
# FORMAT
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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "CoverageIndividual"))) (name "CoverageIndividual") (declared-name "CoverageIndividual")
      (contains
        (element (kind "individual def") (id (node (document "d0") (qualified-name "CoverageIndividual::D1"))) (name "D1") (declared-name "D1"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "CoverageIndividual::D4"))) (name "D4") (declared-name "D4") (declared (properties (individual true))))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "CoverageIndividual::p1"))) (name "p1") (declared-name "p1") (declared (properties (individual true) (composite true) (reference false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "CoverageIndividual::p2"))) (name "p2") (declared-name "p2") (declared (properties (individual true) (composite true) (reference false) (ordered false))))
        (element (kind "state") (id (node (document "d0") (qualified-name "CoverageIndividual::s1"))) (name "s1") (declared-name "s1") (declared (properties (composite true) (reference false))))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/coverage_individual.md"
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
