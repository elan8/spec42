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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "assignment_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 3 2) (end 3 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 2) (end 3 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 20) (end 3 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 1) (end 17 351))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 1) (end 17 351))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f2096242123231750825f4828c8c41f6633c58b1a98cbf2feae7f604d2809c09") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AssignmentTest"))) (kind "package") (name "AssignmentTest") (declared-name "AssignmentTest") (range (start (line 0) (character 0)) (end (line 0) (character 873))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counter"))) (kind "part def") (name "Counter") (declared-name "Counter") (range (start (line 2) (character 1)) (end (line 2) (character 176))) (parent (node (document "d0") (qualified-name "AssignmentTest"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counter::count"))) (kind "attribute") (name "count") (declared-name "count") (range (start (line 3) (character 2)) (end (line 3) (character 47))) (parent (node (document "d0") (qualified-name "AssignmentTest::Counter"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer") (range none)) (typing (reference "ScalarValues::Integer") (range (start (line 3) (character 20)) (end (line 3) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counter::decr"))) (kind "action") (name "decr") (declared-name "decr") (range (start (line 9) (character 2)) (end (line 9) (character 49))) (parent (node (document "d0") (qualified-name "AssignmentTest::Counter"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counter::decr::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 10) (character 3)) (end (line 10) (character 29))) (parent (node (document "d0") (qualified-name "AssignmentTest::Counter::decr"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counter::incr"))) (kind "action") (name "incr") (declared-name "incr") (range (start (line 5) (character 2)) (end (line 5) (character 49))) (parent (node (document "d0") (qualified-name "AssignmentTest::Counter"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counter::incr::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 6) (character 3)) (end (line 6) (character 29))) (parent (node (document "d0") (qualified-name "AssignmentTest::Counter::incr"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (kind "state def") (name "Counting") (declared-name "Counting") (range (start (line 17) (character 1)) (end (line 17) (character 351))) (parent (node (document "d0") (qualified-name "AssignmentTest"))) (authored (membership (kind Owning)) (relationships (transition (reference "AssignmentTest::Counting::increment") (range none)) (transition (reference "AssignmentTest::Counting::decrement") (range none)) (initial-state (reference "AssignmentTest::Counting::wait") (range none)) (initial-state (reference "AssignmentTest::Counting::wait") (range none)))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counting::decrement"))) (kind "state") (name "decrement") (declared-name "decrement") (range (start (line 32) (character 2)) (end (line 32) (character 72))) (parent (node (document "d0") (qualified-name "AssignmentTest::Counting"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counting::increment"))) (kind "state") (name "increment") (declared-name "increment") (range (start (line 27) (character 2)) (end (line 27) (character 72))) (parent (node (document "d0") (qualified-name "AssignmentTest::Counting"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_decrement"))) (kind "transition") (name "transition_Counting_to_decrement") (declared-name "transition_Counting_to_decrement") (range (start (line 24) (character 2)) (end (line 24) (character 32))) (parent (node (document "d0") (qualified-name "AssignmentTest::Counting"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_decrement::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 24) (character 2)) (end (line 24) (character 32))) (parent (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_decrement"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_increment"))) (kind "transition") (name "transition_Counting_to_increment") (declared-name "transition_Counting_to_increment") (range (start (line 22) (character 2)) (end (line 22) (character 32))) (parent (node (document "d0") (qualified-name "AssignmentTest::Counting"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_increment::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 22) (character 2)) (end (line 22) (character 32))) (parent (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_increment"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Decr"))) (kind "attribute def") (name "Decr") (declared-name "Decr") (range (start (line 15) (character 1)) (end (line 15) (character 20))) (parent (node (document "d0") (qualified-name "AssignmentTest"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Incr"))) (kind "attribute def") (name "Incr") (declared-name "Incr") (range (start (line 14) (character 1)) (end (line 14) (character 20))) (parent (node (document "d0") (qualified-name "AssignmentTest"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Increment"))) (kind "calc def") (name "Increment") (declared-name "Increment") (range (start (line 38) (character 1)) (end (line 38) (character 88))) (parent (node (document "d0") (qualified-name "AssignmentTest"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Increment::"))) (kind "return parameter") (name "") (range (start (line 40) (character 2)) (end (line 40) (character 19))) (parent (node (document "d0") (qualified-name "AssignmentTest::Increment"))) (authored (relationships (typing (reference "Counter") (range none)))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Increment::c"))) (kind "in out parameter") (name "c") (declared-name "c") (range (start (line 39) (character 2)) (end (line 39) (character 17))) (parent (node (document "d0") (qualified-name "AssignmentTest::Increment"))) (authored (relationships (typing (reference "Counter") (range none)))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::a"))) (kind "action") (name "a") (declared-name "a") (range (start (line 46) (character 1)) (end (line 46) (character 176))) (parent (node (document "d0") (qualified-name "AssignmentTest"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::a::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 48) (character 2)) (end (line 48) (character 62))) (parent (node (document "d0") (qualified-name "AssignmentTest::a"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::a::_assign#assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 49) (character 2)) (end (line 49) (character 69))) (parent (node (document "d0") (qualified-name "AssignmentTest::a"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::a::counting"))) (kind "state") (name "counting") (declared-name "counting") (range (start (line 47) (character 2)) (end (line 47) (character 28))) (parent (node (document "d0") (qualified-name "AssignmentTest::a"))) (authored (membership (kind Feature)) (relationships (typing (reference "Counting") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Counter::count"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Counter::count"))) (kind featureTyping) (ordinal 1)) (authored-target "ScalarValues::Integer") (range (start (line 3) (character 20)) (end (line 3) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (kind transitionSource) (ordinal 0)) (authored-target "AssignmentTest::Counting::increment") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AssignmentTest::Counting::increment")))))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (kind transitionSource) (ordinal 1)) (authored-target "AssignmentTest::Counting::decrement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AssignmentTest::Counting::decrement")))))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (kind initialStateSource) (ordinal 0)) (authored-target "AssignmentTest::Counting::wait") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (kind initialStateSource) (ordinal 1)) (authored-target "AssignmentTest::Counting::wait") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Increment::"))) (kind featureTyping) (ordinal 0)) (authored-target "Counter") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AssignmentTest::Counter")))))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Increment::c"))) (kind featureTyping) (ordinal 0)) (authored-target "Counter") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AssignmentTest::Counter")))))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::a::counting"))) (kind featureTyping) (ordinal 0)) (authored-target "Counting") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AssignmentTest::Counting")))))
  )
  (relationships
    (relationship (kind transition) (source (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (target (node (document "d0") (qualified-name "AssignmentTest::Counting::decrement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (target (node (document "d0") (qualified-name "AssignmentTest::Counting::increment"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AssignmentTest::Increment::"))) (target (node (document "d0") (qualified-name "AssignmentTest::Counter"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AssignmentTest::Increment::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AssignmentTest::Increment::c"))) (target (node (document "d0") (qualified-name "AssignmentTest::Counter"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AssignmentTest::Increment::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AssignmentTest::a::counting"))) (target (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AssignmentTest::a::counting"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "AssignmentTest::Counter::count")) (expression (status "ok") (value (integer 0))))
    (node (node (document "d0") (qualified-name "AssignmentTest::Increment")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
