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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f2096242123231750825f4828c8c41f6633c58b1a98cbf2feae7f604d2809c09") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AssignmentTest"))) (kind "package") (name "AssignmentTest") (declared-name "AssignmentTest"))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counter"))) (kind "part def") (name "Counter") (declared-name "Counter") (parent (node (document "d0") (qualified-name "AssignmentTest"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counter::count"))) (kind "attribute") (name "count") (declared-name "count") (parent (node (document "d0") (qualified-name "AssignmentTest::Counter"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer")) (typing (reference "ScalarValues::Integer")))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counter::decr"))) (kind "action") (name "decr") (declared-name "decr") (parent (node (document "d0") (qualified-name "AssignmentTest::Counter"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counter::decr::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "AssignmentTest::Counter::decr"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counter::incr"))) (kind "action") (name "incr") (declared-name "incr") (parent (node (document "d0") (qualified-name "AssignmentTest::Counter"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counter::incr::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "AssignmentTest::Counter::incr"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (kind "state def") (name "Counting") (declared-name "Counting") (parent (node (document "d0") (qualified-name "AssignmentTest"))) (authored (membership (kind Owning)) (relationships (transition (reference "AssignmentTest::Counting::increment")) (transition (reference "AssignmentTest::Counting::decrement")) (initial-state (reference "AssignmentTest::Counting::wait")) (initial-state (reference "AssignmentTest::Counting::wait")))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counting::decrement"))) (kind "state") (name "decrement") (declared-name "decrement") (parent (node (document "d0") (qualified-name "AssignmentTest::Counting"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counting::increment"))) (kind "state") (name "increment") (declared-name "increment") (parent (node (document "d0") (qualified-name "AssignmentTest::Counting"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_decrement"))) (kind "transition") (name "transition_Counting_to_decrement") (declared-name "transition_Counting_to_decrement") (parent (node (document "d0") (qualified-name "AssignmentTest::Counting"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_decrement::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_decrement"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_increment"))) (kind "transition") (name "transition_Counting_to_increment") (declared-name "transition_Counting_to_increment") (parent (node (document "d0") (qualified-name "AssignmentTest::Counting"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_increment::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "AssignmentTest::Counting::transition_Counting_to_increment"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Decr"))) (kind "attribute def") (name "Decr") (declared-name "Decr") (parent (node (document "d0") (qualified-name "AssignmentTest"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Incr"))) (kind "attribute def") (name "Incr") (declared-name "Incr") (parent (node (document "d0") (qualified-name "AssignmentTest"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Increment"))) (kind "calc def") (name "Increment") (declared-name "Increment") (parent (node (document "d0") (qualified-name "AssignmentTest"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Increment::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "AssignmentTest::Increment"))) (authored (relationships (typing (reference "Counter")))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::Increment::c"))) (kind "in out parameter") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "AssignmentTest::Increment"))) (authored (relationships (typing (reference "Counter")))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::a"))) (kind "action") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "AssignmentTest"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::a::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "AssignmentTest::a"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::a::_assign#assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "AssignmentTest::a"))))
    (element (id (node (document "d0") (qualified-name "AssignmentTest::a::counting"))) (kind "state") (name "counting") (declared-name "counting") (parent (node (document "d0") (qualified-name "AssignmentTest::a"))) (authored (membership (kind Feature)) (relationships (typing (reference "Counting")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Counter::count"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Counter::count"))) (kind featureTyping) (ordinal 1)) (authored-target "ScalarValues::Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (kind transitionSource) (ordinal 0)) (authored-target "AssignmentTest::Counting::increment") (outcome (status resolved) (target (node (document "d0") (qualified-name "AssignmentTest::Counting::increment")))))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (kind transitionSource) (ordinal 1)) (authored-target "AssignmentTest::Counting::decrement") (outcome (status resolved) (target (node (document "d0") (qualified-name "AssignmentTest::Counting::decrement")))))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (kind initialStateSource) (ordinal 0)) (authored-target "AssignmentTest::Counting::wait") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Counting"))) (kind initialStateSource) (ordinal 1)) (authored-target "AssignmentTest::Counting::wait") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Increment::"))) (kind featureTyping) (ordinal 0)) (authored-target "Counter") (outcome (status resolved) (target (node (document "d0") (qualified-name "AssignmentTest::Counter")))))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::Increment::c"))) (kind featureTyping) (ordinal 0)) (authored-target "Counter") (outcome (status resolved) (target (node (document "d0") (qualified-name "AssignmentTest::Counter")))))
    (reference (id (source (node (document "d0") (qualified-name "AssignmentTest::a::counting"))) (kind featureTyping) (ordinal 0)) (authored-target "Counting") (outcome (status resolved) (target (node (document "d0") (qualified-name "AssignmentTest::Counting")))))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 20) (end 3 41)) (probe (position 3 20))
      (reference
        (source (document "d0") (qualified-name "AssignmentTest::Counter::count"))
        (kind featureTyping) (ordinal 1) (authored-target "ScalarValues::Integer")
        (range (start 3 20) (end 3 41))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
