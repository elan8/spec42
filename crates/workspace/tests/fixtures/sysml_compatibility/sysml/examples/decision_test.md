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
    if x == 1;
    then A1;
    if x > 1;
    then A2;
    else A3;

    then decide D;
    if true;
    then A1;
    if false;
    then A2;

    action A1;
    action A2;
    action A3;

    succession S first A1 
		if x == 0 then A2;

    first A3;
    if x > 0;
    then 'test x';
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
(model
  (namespace
    (action_def 'DecisionTest'
      (attribute_usage composite 'x'
        (feature_value (=)))
      (decide_node 'test x')
      (if_action_usage)
      (source_succession
        (reference_usage reference 'A1'))
      (if_action_usage)
      (source_succession
        (reference_usage reference 'A2'))
      (source_succession
        (reference_usage reference 'A3'))
      (source_succession
        (decide_node 'D'))
      (if_action_usage)
      (source_succession
        (reference_usage reference 'A1'))
      (if_action_usage)
      (source_succession
        (reference_usage reference 'A2'))
      (action_usage composite 'A1')
      (action_usage composite 'A2')
      (action_usage composite 'A3')
      (succession_def 'S'
        (connector_end 'A1')
        (connector_end 'A2'))
      (initial_node)
      (if_action_usage)
      (source_succession
        (reference_usage reference 'test x')))))
~~~
