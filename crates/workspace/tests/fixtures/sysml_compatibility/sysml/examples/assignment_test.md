# META
~~~ini
description=SysML Example (Simple Tests): AssignmentTest
type=file
~~~
# SOURCE
~~~sysml
package AssignmentTest {
	
	part def Counter {
		attribute count : ScalarValues::Integer := 0;
		
		action incr {
			assign count := count + 1;
		}
		
		action decr {
			assign count := count - 1;
		}
	}
	
	attribute def Incr;
	attribute def Decr;
	
	state def Counting {
		part counter : Counter;
		entry assign counter.count := 0;
		
		then state wait;
		accept Incr
			then increment;
		accept Decr
			then decrement;
		
		state increment {
			do assign counter.count := counter.count + 1;
		}
		then wait;
		
		state decrement {
			do assign counter.count := counter.count - 1;
		}
		then wait;
	}
	
	calc def Increment { 
		in c : Counter;
		return : Counter;
		
		perform c.incr;
		c
	}
	
	action a {
		state counting : Counting;
		assign counting.counter.count := counting.counter.count + 1;
		assign counting.counter.count := Increment(counting.counter).count;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,ColonEq,DecimalValue,Semicolon,
KwAction,Ident,OpenCurly,
KwAssign,Ident,ColonEq,Ident,Plus,DecimalValue,Semicolon,
CloseCurly,
KwAction,Ident,OpenCurly,
KwAssign,Ident,ColonEq,Ident,Minus,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwState,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwEntry,KwAssign,Ident,Dot,Ident,ColonEq,DecimalValue,Semicolon,
KwThen,KwState,Ident,Semicolon,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,OpenCurly,
KwDo,KwAssign,Ident,Dot,Ident,ColonEq,Ident,Dot,Ident,Plus,DecimalValue,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
KwState,Ident,OpenCurly,
KwDo,KwAssign,Ident,Dot,Ident,ColonEq,Ident,Dot,Ident,Minus,DecimalValue,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,Semicolon,
Ident,
CloseCurly,
KwAction,Ident,OpenCurly,
KwState,Ident,Colon,Ident,Semicolon,
KwAssign,Ident,Dot,Ident,Dot,Ident,ColonEq,Ident,Dot,Ident,Dot,Ident,Plus,DecimalValue,Semicolon,
KwAssign,Ident,Dot,Ident,Dot,Ident,ColonEq,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'AssignmentTest'
    (part_def 'Counter'
      (attribute_usage 'count' : 'ScalarValues::Integer' value)
      (action_usage 'incr'
        (assign_node))
      (action_usage 'decr'
        (assign_node)))
    (attribute_def 'Incr')
    (attribute_def 'Decr')
    (state_def 'Counting'
      (part_usage 'counter' : 'Counter')
      (malformed)
      (state_usage 'wait')
      (target_transition)
      (target_transition)
      (state_usage 'increment'
        (malformed)
        (source_succession
          (default_ref_usage 'wait'))
        (state_usage 'decrement'
          (malformed)
          (source_succession
            (default_ref_usage 'wait')))
        (calc_def 'Increment'
          (default_ref_usage in 'c' : 'Counter')
          (return_member)
          (perform_action :>> 'c.incr')
          (result_expr_member))
        (action_usage 'a'
          (state_usage 'counting' : 'Counting')
          (assign_node)
          (assign_node))))))
~~~
# FORMAT
~~~sysml
package AssignmentTest {
    part def Counter {
        attribute count : ScalarValues::Integer := 0;

        action incr {
            assign count := count + 1;
        }

        action decr {
            assign count := count - 1;
        }
    }

    attribute def Incr;
    attribute def Decr;

    state def Counting {
        part counter : Counter;

        then
        state wait;
        accept Incr then increment;
        accept Decr then decrement;

        state increment {
            }
            then wait;

            state decrement {
                }
                then wait;
            }

            calc def Increment {
                in c : Counter;
                return : Counter;

                perform :>> c.incr;
                = c;
            }

            action a {
                state counting : Counting;
                assign counting.counter.count := counting.counter.count + 1;
                assign counting.counter.count := Increment(counting.counter).count;
            }
        }
    }
}
~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_close_curly
parse.expected_close_curly
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'c::incr'
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_close_curly
parse.expected_close_curly
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'c::incr'
~~~
# SMG
~~~
(model
  (namespace
    (package 'AssignmentTest'
      (part_def 'Counter'
        (attribute_usage composite 'count' : 'ScalarValues::Integer'[unresolved]
          (feature_value (:=)))
        (action_usage composite 'incr'
          (assignment_action_usage))
        (action_usage composite 'decr'
          (assignment_action_usage)))
      (attribute_def 'Incr')
      (attribute_def 'Decr')
      (state_def 'Counting'
        (part_usage composite 'counter' : 'AssignmentTest::Counter'[part_def])
        (not_implemented 'malformed')
        (state_usage composite 'wait')
        (transition_usage)
        (transition_usage)
        (state_usage composite 'increment'
          (not_implemented 'malformed')
          (source_succession
            (reference_usage reference 'wait'))
          (state_usage composite 'decrement'
            (not_implemented 'malformed')
            (source_succession
              (reference_usage reference 'wait')))
          (calculation_def 'Increment'
            (reference_usage in reference 'c' : 'AssignmentTest::Counter'[part_def])
            (return_parameter_membership
              (feature_def out : 'AssignmentTest::Counter'[part_def]))
            (perform_action_usage :>> 'c::incr'[unresolved])
            (result_expr_membership))
          (action_usage composite 'a'
            (state_usage composite 'counting' : 'AssignmentTest::Counting'[state_def])
            (assignment_action_usage)
            (assignment_action_usage)))))))
~~~
