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
  (document "memory://snapshot/assignment_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 20) (end 3 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 6 3) (end 6 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 10 3) (end 10 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 18 2) (end 19 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 19 2) (end 21 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 21 2) (end 22 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 22 2) (end 22 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 24 2) (end 24 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 28 3) (end 29 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 30 2) (end 30 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 33 3) (end 34 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 35 2) (end 35 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 38 1) (end 44 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 48 2) (end 48 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 49 2) (end 49 69))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:9a9130e71b13757d4c986fc821a94556e68b5ad1a1afe13a2cc4ca1a081e88c4") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Integer"))))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::decr"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::incr"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting::decrement"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting::increment"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Decr"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Incr"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a::counting"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Counting"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a::counting"))) (kind featureTyping) (ordinal 0))
      (authored-target "Counting")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a::counting"))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a::counting"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/assignment_test.md") (range (start 3 20) (end 3 41)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 47 19) (end 47 27)) (probe (position 47 19))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a::counting"))) (kind featureTyping) (ordinal 0) (authored-target "Counting")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting")))))
  )
)
~~~
