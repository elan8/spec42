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
  (document "memory://snapshot/decision_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 1 1) (end 1 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 8) (end 3 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 4 1) (end 4 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 5 1) (end 6 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 13) (end 8 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 9 1) (end 9 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 10 1) (end 10 18))
      )
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "parser")
        (range (start 16 1) (end 19 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 20 2) (end 20 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:040cf77bfeccd08e8998112b0cb9f03441d2a7e57bec5bd99dd79ad132370220") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/decision_test.md") (qualified-name "DecisionTest"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/decision_test.md") (anonymous (kind decide) (ordinal 0))))) (kind decide) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (decisionInput (reference "test x"))))
    (declaration (id (node (document "memory://snapshot/decision_test.md") (anonymous (kind decide) (ordinal 1))))) (kind decide) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (decisionInput (reference "D"))))
    (declaration (id (node (document "memory://snapshot/decision_test.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "A3"))))
    (declaration (id (node (document "memory://snapshot/decision_test.md") (qualified-name "DecisionTest::A1"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/decision_test.md") (qualified-name "DecisionTest::A2"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/decision_test.md") (qualified-name "DecisionTest::A3"))) (kind action) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/decision_test.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "A3")
      (outcome (status resolved) (target (node (document "memory://snapshot/decision_test.md") (qualified-name "DecisionTest::A3")))))
    (reference (id (source (node (document "memory://snapshot/decision_test.md") (anonymous (kind decide) (ordinal 0))))) (kind decisionInput) (ordinal 0))
      (authored-target "test x")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/decision_test.md") (anonymous (kind decide) (ordinal 1))))) (kind decisionInput) (ordinal 0))
      (authored-target "D")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/decision_test.md") (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/decision_test.md") (qualified-name "DecisionTest::A3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/decision_test.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/decision_test.md") (range (start 19 7) (end 19 9)) (probe (position 19 7))
    (reference (id (source (node (document "memory://snapshot/decision_test.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "A3")
      (outcome (status resolved) (target (node (document "memory://snapshot/decision_test.md") (qualified-name "DecisionTest::A3")))))
  )
  (query (document "memory://snapshot/decision_test.md") (range (start 3 8) (end 3 16)) (probe (position 3 8))
    (reference (id (source (node (document "memory://snapshot/decision_test.md") (anonymous (kind decide) (ordinal 0))))) (kind decisionInput) (ordinal 0) (authored-target "test x")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/decision_test.md") (range (start 8 13) (end 8 14)) (probe (position 8 13))
    (reference (id (source (node (document "memory://snapshot/decision_test.md") (anonymous (kind decide) (ordinal 1))))) (kind decisionInput) (ordinal 0) (authored-target "D")
      (outcome (status unresolved)))
  )
)
~~~
