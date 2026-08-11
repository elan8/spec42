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
# TOKENS
~~~zig
KwAction,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Eq,DecimalValue,Semicolon,
KwDecide,UnrestrictedName,Semicolon,
KwIf,Ident,EqEq,DecimalValue,KwThen,Ident,Semicolon,
KwIf,Ident,CloseAngle,DecimalValue,KwThen,Ident,Semicolon,
KwElse,Ident,Semicolon,
KwThen,KwDecide,Ident,Semicolon,
KwIf,KwTrue,KwThen,Ident,Semicolon,
KwIf,KwFalse,KwThen,Ident,Semicolon,
KwAction,Ident,Semicolon,
KwAction,Ident,Semicolon,
KwAction,Ident,Semicolon,
KwSuccession,Ident,KwFirst,Ident,
KwIf,Ident,EqEq,DecimalValue,KwThen,Ident,Semicolon,
KwFirst,Ident,Semicolon,
KwIf,Ident,CloseAngle,DecimalValue,KwThen,UnrestrictedName,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (action_def 'DecisionTest'
    (attribute_usage 'x' value)
    (sysml_decl ''test x'')
    (if_node)
    (source_succession
      (default_ref_usage 'A1'))
    (if_node)
    (source_succession
      (default_ref_usage 'A2'))
    (source_succession
      (default_ref_usage 'A3'))
    (source_succession
      (sysml_decl 'D'))
    (if_node)
    (source_succession
      (default_ref_usage 'A1'))
    (if_node)
    (source_succession
      (default_ref_usage 'A2'))
    (action_usage 'A1')
    (action_usage 'A2')
    (action_usage 'A3')
    (succession_as_usage 'S'
      (connector_end)
      (connector_end))
    (initial_node A3)
    (if_node)
    (source_succession
      (default_ref_usage ''test x''))))
~~~
# EXPECTED
~~~
semantic.duplicate_name 'A1'
semantic.duplicate_name 'A2'
semantic.duplicate_name 'A1'
semantic.duplicate_name 'A2'
semantic.duplicate_name 'A3'
semantic.duplicate_name 'test x'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'A1'
semantic.duplicate_name 'A2'
semantic.duplicate_name 'A1'
semantic.duplicate_name 'A2'
semantic.duplicate_name 'A3'
semantic.duplicate_name 'test x'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "06a130201bd0ea4b3b2afcd6608e5b97cfafccfbc601414fec3c0ad3cf472971") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DecisionTest"))) (kind "action def") (name "DecisionTest") (declared-name "DecisionTest") (range (start (line 0) (character 0)) (end (line 0) (character 297))) (authored (membership (kind Owning)) (relationships (perform (reference "DecisionTest::A1") (range none)) (perform (reference "DecisionTest::A2") (range none)) (perform (reference "DecisionTest::A3") (range none)))))
    (element (id (node (document "d0") (qualified-name "DecisionTest::A1"))) (kind "action") (name "A1") (declared-name "A1") (range (start (line 12) (character 1)) (end (line 12) (character 11))) (parent (node (document "d0") (qualified-name "DecisionTest"))))
    (element (id (node (document "d0") (qualified-name "DecisionTest::A2"))) (kind "action") (name "A2") (declared-name "A2") (range (start (line 13) (character 1)) (end (line 13) (character 11))) (parent (node (document "d0") (qualified-name "DecisionTest"))))
    (element (id (node (document "d0") (qualified-name "DecisionTest::A3"))) (kind "action") (name "A3") (declared-name "A3") (range (start (line 14) (character 1)) (end (line 14) (character 11))) (parent (node (document "d0") (qualified-name "DecisionTest"))))
    (element (id (node (document "d0") (qualified-name "DecisionTest::_initial"))) (kind "initial") (name "_initial") (range (start (line 19) (character 1)) (end (line 19) (character 10))) (parent (node (document "d0") (qualified-name "DecisionTest"))) (authored (relationships (flow (reference "DecisionTest::A3") (range none)))))
    (element (id (node (document "d0") (qualified-name "DecisionTest::test x"))) (kind "decide") (name "decide") (declared-name "decide") (range (start (line 3) (character 1)) (end (line 3) (character 17))) (parent (node (document "d0") (qualified-name "DecisionTest"))))
    (element (id (node (document "d0") (qualified-name "DecisionTest::x = 1"))) (kind "action body decl") (name "x = 1") (declared-name "x = 1") (range (start (line 1) (character 1)) (end (line 1) (character 17))) (parent (node (document "d0") (qualified-name "DecisionTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DecisionTest"))) (kind performSource) (ordinal 0)) (authored-target "DecisionTest::A1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "DecisionTest::A1")))))
    (reference (id (source (node (document "d0") (qualified-name "DecisionTest"))) (kind performSource) (ordinal 1)) (authored-target "DecisionTest::A2") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "DecisionTest::A2")))))
    (reference (id (source (node (document "d0") (qualified-name "DecisionTest"))) (kind performSource) (ordinal 2)) (authored-target "DecisionTest::A3") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "DecisionTest::A3")))))
    (reference (id (source (node (document "d0") (qualified-name "DecisionTest::_initial"))) (kind flowSource) (ordinal 0)) (authored-target "DecisionTest::A3") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "DecisionTest::A3")))))
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
