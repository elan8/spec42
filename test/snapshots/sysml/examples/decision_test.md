# META
~~~ini
description=SysML Example (Simple Tests): DecisionTest
type=file
~~~
# SOURCE
~~~sysml
action def DecisionTest {
	attribute x = 1;
	
	decide 'test x';
	if x == 1 then A1; 
	if x > 1 then A2;
	else A3; 
	
	then decide D; 
	if true then A1;
	if false then A2;
	
	action A1;
	action A2;
	action A3;
	
	succession S first A1 
		if x == 0 then A2;
		
	first A3;
		if x > 0 then 'test x';
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "decision_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 4 1) (end 4 22))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 4 1) (end 4 22))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 6 1) (end 6 14))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 8 1) (end 8 18))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 8 1) (end 8 18))
      )
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "sysml")
        (range (start 16 1) (end 16 49))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 20 2) (end 20 26))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "06a130201bd0ea4b3b2afcd6608e5b97cfafccfbc601414fec3c0ad3cf472971") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DecisionTest"))) (kind "action def") (name "DecisionTest") (declared-name "DecisionTest") (authored (membership (kind Owning)) (relationships (perform (reference "DecisionTest::A1")) (perform (reference "DecisionTest::A2")) (perform (reference "DecisionTest::A3")))))
    (element (id (node (document "d0") (qualified-name "DecisionTest::A1"))) (kind "action") (name "A1") (declared-name "A1") (parent (node (document "d0") (qualified-name "DecisionTest"))))
    (element (id (node (document "d0") (qualified-name "DecisionTest::A2"))) (kind "action") (name "A2") (declared-name "A2") (parent (node (document "d0") (qualified-name "DecisionTest"))))
    (element (id (node (document "d0") (qualified-name "DecisionTest::A3"))) (kind "action") (name "A3") (declared-name "A3") (parent (node (document "d0") (qualified-name "DecisionTest"))))
    (element (id (node (document "d0") (qualified-name "DecisionTest::_initial"))) (kind "initial") (name "_initial") (parent (node (document "d0") (qualified-name "DecisionTest"))) (authored (relationships (flow (reference "DecisionTest::A3")))))
    (element (id (node (document "d0") (qualified-name "DecisionTest::test x"))) (kind "decide") (name "decide") (declared-name "decide") (parent (node (document "d0") (qualified-name "DecisionTest"))))
    (element (id (node (document "d0") (qualified-name "DecisionTest::x = 1"))) (kind "action body decl") (name "x = 1") (declared-name "x = 1") (parent (node (document "d0") (qualified-name "DecisionTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DecisionTest"))) (kind performSource) (ordinal 0)) (authored-target "DecisionTest::A1") (outcome (status resolved) (target (node (document "d0") (qualified-name "DecisionTest::A1")))))
    (reference (id (source (node (document "d0") (qualified-name "DecisionTest"))) (kind performSource) (ordinal 1)) (authored-target "DecisionTest::A2") (outcome (status resolved) (target (node (document "d0") (qualified-name "DecisionTest::A2")))))
    (reference (id (source (node (document "d0") (qualified-name "DecisionTest"))) (kind performSource) (ordinal 2)) (authored-target "DecisionTest::A3") (outcome (status resolved) (target (node (document "d0") (qualified-name "DecisionTest::A3")))))
    (reference (id (source (node (document "d0") (qualified-name "DecisionTest::_initial"))) (kind flowSource) (ordinal 0)) (authored-target "DecisionTest::A3") (outcome (status resolved) (target (node (document "d0") (qualified-name "DecisionTest::A3")))))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "DecisionTest"))) (target (node (document "d0") (qualified-name "DecisionTest::A1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DecisionTest"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "DecisionTest"))) (target (node (document "d0") (qualified-name "DecisionTest::A2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DecisionTest"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "DecisionTest"))) (target (node (document "d0") (qualified-name "DecisionTest::A3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DecisionTest"))) (kind performSource) (ordinal 2)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "DecisionTest::_initial"))) (target (node (document "d0") (qualified-name "DecisionTest::A3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DecisionTest::_initial"))) (kind flowSource) (ordinal 0)))
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
