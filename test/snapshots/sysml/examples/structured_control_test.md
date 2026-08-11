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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "StructuredControlTest"))) (name "StructuredControlTest") (declared-name "StructuredControlTest")
      (contains
        (element (kind "action") (id (node (document "d0") (qualified-name "StructuredControlTest::"))) (name "") (declared)
          (contains
            (element (kind "if") (id (node (document "d0") (qualified-name "StructuredControlTest::::_if"))) (name "if") (declared-name "if")
              (contains
                (element (kind "assign") (id (node (document "d0") (qualified-name "StructuredControlTest::::_if::_assign"))) (name "assign") (declared-name "assign"))
              )
            )
            (element (kind "if") (id (node (document "d0") (qualified-name "StructuredControlTest::::_if#if"))) (name "if") (declared-name "if")
              (contains
                (element (kind "assign") (id (node (document "d0") (qualified-name "StructuredControlTest::::_if#if::_assign"))) (name "assign") (declared-name "assign"))
              )
            )
            (element (kind "loop") (id (node (document "d0") (qualified-name "StructuredControlTest::::_loop"))) (name "loop") (declared-name "loop")
              (contains
                (element (kind "assign") (id (node (document "d0") (qualified-name "StructuredControlTest::::_loop::_assign"))) (name "assign") (declared-name "assign"))
              )
            )
            (element (kind "while") (id (node (document "d0") (qualified-name "StructuredControlTest::::_while"))) (name "while") (declared-name "while")
              (contains
                (element (kind "assign") (id (node (document "d0") (qualified-name "StructuredControlTest::::_while::_assign"))) (name "assign") (declared-name "assign"))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "StructuredControlTest::::aLoop"))) (name "aLoop") (declared-name "aLoop"))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "StructuredControlTest::::b : ScalarValues::Boolean"))) (name "b : ScalarValues::Boolean") (declared-name "b : ScalarValues::Boolean"))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "StructuredControlTest::::i : ScalarValues::Integer := 0"))) (name "i : ScalarValues::Integer := 0") (declared-name "i : ScalarValues::Integer := 0"))
          )
        )
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "StructuredControlTest::"))) (to (node (document "d0") (qualified-name "StructuredControlTest::::aLoop"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StructuredControlTest::"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StructuredControlTest::::_if"))) (status missing-prerequisite) (target "Actions::ifThenActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StructuredControlTest::::_if#if"))) (status missing-prerequisite) (target "Actions::ifThenActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StructuredControlTest::::_if#if::_assign"))) (status missing-prerequisite) (target "Actions::assignmentActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StructuredControlTest::::_if::_assign"))) (status missing-prerequisite) (target "Actions::assignmentActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StructuredControlTest::::_loop::_assign"))) (status missing-prerequisite) (target "Actions::assignmentActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StructuredControlTest::::_while"))) (status missing-prerequisite) (target "Actions::whileLoopActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StructuredControlTest::::_while::_assign"))) (status missing-prerequisite) (target "Actions::assignmentActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StructuredControlTest::::aLoop"))) (status missing-prerequisite) (target "Actions::actions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/structured_control_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 8 4) (end 8 43))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 10 4) (end 10 42))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 21 4) (end 21 18))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 23 2) (end 23 52))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 29 4) (end 29 20))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 31 2) (end 31 74))
      )
    )
  )
)
~~~
