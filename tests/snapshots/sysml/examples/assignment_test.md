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
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 17 1) (end 36 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 18 2) (end 18 25))
      )
      (diagnostic
        (severity error)
        (code "recovered_state_body_element")
        (source "parser")
        (range (start 21 2) (end 22 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 7) (end 30 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 35 7) (end 35 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 48 9) (end 48 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 48 35) (end 48 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 49 9) (end 49 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 49 35) (end 49 68))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:9a9130e71b13757d4c986fc821a94556e68b5ad1a1afe13a2cc4ca1a081e88c4") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind assign)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Integer")))))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::decr"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "decr")) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "count")) (assignTarget (reference "count")))))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::incr"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "incr")) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "count")) (assignTarget (reference "count")))))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "increment")) (transitionTrigger (reference "Incr")))))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "decrement")) (transitionTrigger (reference "Decr")))))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "wait")))))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind initial-state) (ordinal 1))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "wait")))))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting::decrement"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting::increment"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Decr"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Incr"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "c")))))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Counter")))))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "c::incr")))))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment::c"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Counter") (direction in)))))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind action) (name "a")) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "counting::counter::count")) (memberAccessOperand (reference "counting::counter::count")))))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind action) (name "a")) (anonymous (kind assign) (ordinal 1))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "counting::counter::count")))))
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a::counting"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Counting")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "decr")) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count")))))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "decr")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count")))))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "incr")) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count")))))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "incr")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count")))))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "wait")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind initial-state) (ordinal 1))))) (kind initialState) (ordinal 0))
      (authored-target "wait")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "increment")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting::increment")))))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0))
      (authored-target "decrement")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting::decrement")))))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Incr")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Incr")))))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Decr")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Decr")))))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment"))) (kind expressionOperand) (ordinal 0))
      (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment::c")))))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Counter")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter")))))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "c::incr")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::incr")))))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "Counter")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter")))))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind action) (name "a")) (anonymous (kind assign) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "counting::counter::count")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind action) (name "a")) (anonymous (kind assign) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "counting::counter::count")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind action) (name "a")) (anonymous (kind assign) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "counting::counter::count")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a::counting"))) (kind featureTyping) (ordinal 0))
      (authored-target "Counting")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "decr")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "decr")) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "decr")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "decr")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "incr")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "incr")) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "incr")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "incr")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting::increment"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting::decrement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Incr"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Decr"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment"))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment::c"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::incr"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment::c"))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a::counting"))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a::counting"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count"))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::decr"))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "decr")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::decr"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::incr"))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "incr")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::incr"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind initial-state) (ordinal 1))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting::decrement"))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting::increment"))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment::c"))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind action) (name "a")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind action) (name "a")) (anonymous (kind assign) (ordinal 1))))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a::counting"))) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count"))) (state literal) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "decr")) (anonymous (kind assign) (ordinal 0))))) (state evaluated) (value (kind integer) (integer -1)))
    (evaluated (declaration (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "incr")) (anonymous (kind assign) (ordinal 0))))) (state evaluated) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind action) (name "a")) (anonymous (kind assign) (ordinal 0))))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind action) (name "a")) (anonymous (kind assign) (ordinal 1))))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter")))
      (subtype (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind parameter) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment::c")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count")))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter")))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::decr")))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter")))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "decr")) (anonymous (kind assign) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::decr")))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::incr")))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter")))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "incr")) (anonymous (kind assign) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::incr")))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting")))
      (subtype (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a::counting")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting")))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting")))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting")))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind initial-state) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting")))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting::decrement")))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting")))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting::increment")))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting")))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment")))
      (type (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter")) (provenance authored))
      (effective-type (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter")) (source direct))
      (supertype (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment")))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment::c")))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment")))
      (type (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter")) (provenance authored))
      (effective-type (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter")) (source direct))
      (supertype (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind action) (name "a")) (anonymous (kind assign) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a")))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind action) (name "a")) (anonymous (kind assign) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a")))
    )
    (declaration (id (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a::counting")))
      (featured-by (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a")))
      (type (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting")) (provenance authored))
      (effective-type (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting")) (source direct))
      (supertype (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting")) (scopes any))
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
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 10 19) (end 10 24)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "decr")) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count")))))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 10 10) (end 10 15)) (probe (position 10 10))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "decr")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count")))))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 6 19) (end 6 24)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "incr")) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count")))))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 6 10) (end 6 15)) (probe (position 6 10))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind part-def) (name "Counter")) (named (kind action) (name "incr")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::count")))))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 30 7) (end 30 11)) (probe (position 30 7))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "wait")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 35 7) (end 35 11)) (probe (position 35 7))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind initial-state) (ordinal 1))))) (kind initialState) (ordinal 0) (authored-target "wait")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 23 8) (end 23 17)) (probe (position 23 8))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "increment")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting::increment")))))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 25 8) (end 25 17)) (probe (position 25 8))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0) (authored-target "decrement")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting::decrement")))))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 22 9) (end 22 13)) (probe (position 22 9))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0) (authored-target "Incr")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Incr")))))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 24 9) (end 24 13)) (probe (position 24 9))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind state-def) (name "Counting")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0) (authored-target "Decr")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Decr")))))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 43 2) (end 43 3)) (probe (position 43 2))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment"))) (kind expressionOperand) (ordinal 0) (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment::c")))))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 40 11) (end 40 18)) (probe (position 40 11))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Counter")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter")))))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 42 10) (end 42 16)) (probe (position 42 10))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind calc-def) (name "Increment")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "c::incr")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter::incr")))))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 39 9) (end 39 16)) (probe (position 39 9))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Increment::c"))) (kind featureTyping) (ordinal 0) (authored-target "Counter")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counter")))))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 48 9) (end 48 31)) (probe (position 48 9))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind action) (name "a")) (anonymous (kind assign) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "counting::counter::count")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 49 9) (end 49 31)) (probe (position 49 9))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind action) (name "a")) (anonymous (kind assign) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "counting::counter::count")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 48 35) (end 48 57)) (probe (position 48 35))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (path (named (kind package) (name "AssignmentTest")) (named (kind action) (name "a")) (anonymous (kind assign) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "counting::counter::count")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/assignment_test.md") (range (start 47 19) (end 47 27)) (probe (position 47 19))
    (reference (id (source (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::a::counting"))) (kind featureTyping) (ordinal 0) (authored-target "Counting")
      (outcome (status resolved) (target (node (document "memory://snapshot/assignment_test.md") (qualified-name "AssignmentTest::Counting")))))
    )
  )
)
~~~
