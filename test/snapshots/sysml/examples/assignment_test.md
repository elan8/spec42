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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "AssignmentTest"))) (name "AssignmentTest") (declared-name "AssignmentTest")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "AssignmentTest::Counter"))) (name "Counter") (declared-name "Counter") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AssignmentTest::Counter::count"))) (name "count") (declared-name "count") (declared (properties (ordered false) (unique true)) (feature-value (kind initial) (expression (kind "integerLiteral") (literal (integer 0))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AssignmentTest::Counter")))) (evaluation (expression (status "ok") (value (integer 0)))))
            (element (kind "action") (id (node (document "d0") (qualified-name "AssignmentTest::Counter::decr"))) (name "decr") (declared-name "decr") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AssignmentTest::Counter"))))
              (contains
                (element (kind "assign") (id (node (document "d0") (qualified-name "AssignmentTest::Counter::decr::_assign"))) (name "assign") (declared-name "assign") (effective (featuring-type (node (document "d0") (qualified-name "AssignmentTest::Counter")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "AssignmentTest::Counter::incr"))) (name "incr") (declared-name "incr") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AssignmentTest::Counter"))))
              (contains
                (element (kind "assign") (id (node (document "d0") (qualified-name "AssignmentTest::Counter::incr::_assign"))) (name "assign") (declared-name "assign") (effective (featuring-type (node (document "d0") (qualified-name "AssignmentTest::Counter")))))
              )
            )
          )
        )
        (element (kind "state def") (id (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (name "Counting") (declared-name "Counting")
          (contains
            (element (kind "state") (id (node (document "d0") (qualified-name "AssignmentTest::Counting::decrement"))) (name "decrement") (declared-name "decrement") (effective (featuring-type (node (document "d0") (qualified-name "AssignmentTest::Counting")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "AssignmentTest::Counting::increment"))) (name "increment") (declared-name "increment") (effective (featuring-type (node (document "d0") (qualified-name "AssignmentTest::Counting")))))
            (element (kind "transition") (id (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_decrement"))) (name "transition_Counting_to_decrement") (declared-name "transition_Counting_to_decrement") (effective (featuring-type (node (document "d0") (qualified-name "AssignmentTest::Counting"))))
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_decrement::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "AssignmentTest::Counting")))))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_increment"))) (name "transition_Counting_to_increment") (declared-name "transition_Counting_to_increment") (effective (featuring-type (node (document "d0") (qualified-name "AssignmentTest::Counting"))))
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_increment::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "AssignmentTest::Counting")))))
              )
            )
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "AssignmentTest::Decr"))) (name "Decr") (declared-name "Decr") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "AssignmentTest::Incr"))) (name "Incr") (declared-name "Incr") (declared (properties (ordered false) (unique true))))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "AssignmentTest::Increment"))) (name "Increment") (declared-name "Increment") (declared (own-expression (expression (kind "featureReference") (reference "perform")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference")))
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "AssignmentTest::Increment::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AssignmentTest::Increment")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AssignmentTest::Increment::c"))) (name "c") (declared-name "c") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "AssignmentTest::Increment")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "AssignmentTest::a"))) (name "a") (declared-name "a") (declared)
          (contains
            (element (kind "assign") (id (node (document "d0") (qualified-name "AssignmentTest::a::_assign"))) (name "assign") (declared-name "assign"))
            (element (kind "assign") (id (node (document "d0") (qualified-name "AssignmentTest::a::_assign#assign"))) (name "assign") (declared-name "assign"))
            (element (kind "state") (id (node (document "d0") (qualified-name "AssignmentTest::a::counting"))) (name "counting") (declared-name "counting") (declared) (effective (implied-feature-ownership (composite true) (reference false))))
          )
        )
      )
    )
  )
  (relationships
    (transition (status resolved) (from (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (to (node (document "d0") (qualified-name "AssignmentTest::Counting::decrement"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (to (node (document "d0") (qualified-name "AssignmentTest::Counting::increment"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AssignmentTest::Increment::"))) (to (node (document "d0") (qualified-name "AssignmentTest::Counter"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AssignmentTest::Increment::c"))) (to (node (document "d0") (qualified-name "AssignmentTest::Counter"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AssignmentTest::a::counting"))) (to (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (provenance authored))
  )
  (pending-relationships
    (initialState (status pending) (document "d0") (source-qualified "AssignmentTest::Counting") (target-qualified "AssignmentTest::Counting::wait"))
    (initialState (status pending) (document "d0") (source-qualified "AssignmentTest::Counting") (target-qualified "AssignmentTest::Counting::wait"))
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Counter"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Counter::count"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Counter::decr"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Counter::decr::_assign"))) (status missing-prerequisite) (target "Actions::assignmentActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Counter::incr"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Counter::incr::_assign"))) (status missing-prerequisite) (target "Actions::assignmentActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (status missing-prerequisite) (target "States::StateAction"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Counting::decrement"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Counting::increment"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_decrement"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_decrement::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_increment"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_increment::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Decr"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Incr"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::Increment"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::a"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::a::_assign"))) (status missing-prerequisite) (target "Actions::assignmentActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::a::_assign#assign"))) (status missing-prerequisite) (target "Actions::assignmentActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignmentTest::a::counting"))) (status missing-prerequisite) (target "States::stateActions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/assignment_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 2) (end 3 47))
      )
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 17 1) (end 17 351))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 17 1) (end 17 351))
      )
    )
  )
)
~~~
