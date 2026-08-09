# META
~~~ini
description=SysML Example (Simple Tests): StructuredControlTest
type=file
~~~
# SOURCE
~~~sysml
package StructuredControlTest {
	
	action {
		attribute i : ScalarValues::Integer := 0;
		attribute b : ScalarValues::Boolean;
		
		if i < 0 {
			assign i := 0;
		} else if i == 0 {
			assign i := 1;
		} else {
			assign i := i + 1;
		}
		
		if i > 0 {
			assign i := i + 1;
		}
		
		then action aLoop
		while i > 0 {
			assign i := i - 1;
		} until b;
		
		then while i > 0 {
			assign i := i - 1;
		}
		
		loop {
			assign i := i - 1;
		} until b;
				
		for n : ScalarValues::Integer in (1, 2, 3) {
			assign i := i * n;
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwAction,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,ColonEq,DecimalValue,Semicolon,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwIf,Ident,OpenAngle,DecimalValue,OpenCurly,
KwAssign,Ident,ColonEq,DecimalValue,Semicolon,
CloseCurly,KwElse,KwIf,Ident,EqEq,DecimalValue,OpenCurly,
KwAssign,Ident,ColonEq,DecimalValue,Semicolon,
CloseCurly,KwElse,OpenCurly,
KwAssign,Ident,ColonEq,Ident,Plus,DecimalValue,Semicolon,
CloseCurly,
KwIf,Ident,CloseAngle,DecimalValue,OpenCurly,
KwAssign,Ident,ColonEq,Ident,Plus,DecimalValue,Semicolon,
CloseCurly,
KwThen,KwAction,Ident,
KwWhile,Ident,CloseAngle,DecimalValue,OpenCurly,
KwAssign,Ident,ColonEq,Ident,Minus,DecimalValue,Semicolon,
CloseCurly,KwUntil,Ident,Semicolon,
KwThen,KwWhile,Ident,CloseAngle,DecimalValue,OpenCurly,
KwAssign,Ident,ColonEq,Ident,Minus,DecimalValue,Semicolon,
CloseCurly,
KwLoop,OpenCurly,
KwAssign,Ident,ColonEq,Ident,Minus,DecimalValue,Semicolon,
CloseCurly,KwUntil,Ident,Semicolon,
KwFor,Ident,Colon,Ident,ColonColon,Ident,KwIn,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenCurly,
KwAssign,Ident,ColonEq,Ident,Star,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'StructuredControlTest'
    (action_usage
      (attribute_usage 'i' : 'ScalarValues::Integer' value)
      (attribute_usage 'b' : 'ScalarValues::Boolean')
      (if_node)
      (if_node then+else)
      (if_node)
      (source_succession
        (action_usage 'aLoop'))
      (while_loop_node)
      (source_succession
        (while_loop_node))
      (while_loop_node)
      (for_loop_node)
      (malformed))))
~~~
# FORMAT
~~~sysml
package StructuredControlTest {
    action {
        attribute i : ScalarValues::Integer := 0;
        attribute b : ScalarValues::Boolean;

        if i < 0 {
            assign i := 0;
        }
        if i == 0 {
            assign i := 1;
        } else {
            assign i := i + 1;
        }

        if i > 0 {
            assign i := i + 1;
        }

        then action aLoop
        while i > 0 {
            assign i := i - 1;
        } until b;

        then while i > 0 {
			assign i := i - 1;
		}

        loop {
            assign i := i - 1;
        } until b;

        for n in : ScalarValues::Integer { }
        in (1, 2, 3) {
			assign i := i * n;
		}
    }
}
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'ScalarValues::Boolean'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'ScalarValues::Boolean'
~~~
# SMG
~~~
(model
  (namespace
    (package 'StructuredControlTest'
      (action_usage
        (attribute_usage composite 'i' : 'ScalarValues::Integer'[unresolved]
          (feature_value (:=)))
        (attribute_usage composite 'b' : 'ScalarValues::Boolean'[unresolved])
        (if_action_usage
          (assignment_action_usage))
        (if_action_usage
          (assignment_action_usage)
          (assignment_action_usage))
        (if_action_usage
          (assignment_action_usage))
        (source_succession
          (action_usage 'aLoop'))
        (while_loop_action_usage
          (assignment_action_usage))
        (source_succession
          (while_loop_action_usage
            (assignment_action_usage)))
        (while_loop_action_usage
          (assignment_action_usage))
        (for_loop_action_usage)
        (not_implemented 'malformed')))))
~~~
