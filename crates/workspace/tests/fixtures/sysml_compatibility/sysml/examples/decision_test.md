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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "action def") (id (node (document "d0") (qualified-name "DecisionTest"))) (name "DecisionTest") (declared-name "DecisionTest")
      (contains
        (element (kind "action") (id (node (document "d0") (qualified-name "DecisionTest::A1"))) (name "A1") (declared-name "A1") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "DecisionTest")))))
        (element (kind "action") (id (node (document "d0") (qualified-name "DecisionTest::A2"))) (name "A2") (declared-name "A2") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "DecisionTest")))))
        (element (kind "action") (id (node (document "d0") (qualified-name "DecisionTest::A3"))) (name "A3") (declared-name "A3") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "DecisionTest")))))
        (element (kind "initial") (id (node (document "d0") (qualified-name "DecisionTest::_initial"))) (name "_initial") (effective (featuring-type (node (document "d0") (qualified-name "DecisionTest")))))
        (element (kind "decide") (id (node (document "d0") (qualified-name "DecisionTest::test x"))) (name "decide") (declared-name "decide") (effective (featuring-type (node (document "d0") (qualified-name "DecisionTest")))))
        (element (kind "action body decl") (id (node (document "d0") (qualified-name "DecisionTest::x = 1"))) (name "x = 1") (declared-name "x = 1") (effective (featuring-type (node (document "d0") (qualified-name "DecisionTest")))))
      )
    )
  )
  (relationships
    (flow (status resolved) (from (node (document "d0") (qualified-name "DecisionTest::_initial"))) (to (node (document "d0") (qualified-name "DecisionTest::A3"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "DecisionTest"))) (to (node (document "d0") (qualified-name "DecisionTest::A1"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "DecisionTest"))) (to (node (document "d0") (qualified-name "DecisionTest::A2"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "DecisionTest"))) (to (node (document "d0") (qualified-name "DecisionTest::A3"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
