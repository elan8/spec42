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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "structured_control_test.md"
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "e33ad92654de39546b911b228ef7fdb2b61de9e6e3dbe50d56614604b31b281d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "StructuredControlTest"))) (kind "package") (name "StructuredControlTest") (declared-name "StructuredControlTest") (range (start (line 0) (character 0)) (end (line 0) (character 532))))
    (element (id (node (document "d0") (qualified-name "StructuredControlTest::"))) (kind "action") (name "") (range (start (line 2) (character 1)) (end (line 2) (character 496))) (parent (node (document "d0") (qualified-name "StructuredControlTest"))) (authored (membership (kind Feature)) (relationships (perform (reference "StructuredControlTest::::aLoop") (range none)))))
    (element (id (node (document "d0") (qualified-name "StructuredControlTest::::_if"))) (kind "if") (name "if") (declared-name "if") (range (start (line 6) (character 2)) (end (line 6) (character 34))) (parent (node (document "d0") (qualified-name "StructuredControlTest::"))))
    (element (id (node (document "d0") (qualified-name "StructuredControlTest::::_if#if"))) (kind "if") (name "if") (declared-name "if") (range (start (line 14) (character 2)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "StructuredControlTest::"))))
    (element (id (node (document "d0") (qualified-name "StructuredControlTest::::_if#if::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 15) (character 3)) (end (line 15) (character 21))) (parent (node (document "d0") (qualified-name "StructuredControlTest::::_if#if"))))
    (element (id (node (document "d0") (qualified-name "StructuredControlTest::::_if::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 7) (character 3)) (end (line 7) (character 17))) (parent (node (document "d0") (qualified-name "StructuredControlTest::::_if"))))
    (element (id (node (document "d0") (qualified-name "StructuredControlTest::::_loop"))) (kind "loop") (name "loop") (declared-name "loop") (range (start (line 27) (character 2)) (end (line 27) (character 34))) (parent (node (document "d0") (qualified-name "StructuredControlTest::"))))
    (element (id (node (document "d0") (qualified-name "StructuredControlTest::::_loop::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 28) (character 3)) (end (line 28) (character 21))) (parent (node (document "d0") (qualified-name "StructuredControlTest::::_loop"))))
    (element (id (node (document "d0") (qualified-name "StructuredControlTest::::_while"))) (kind "while") (name "while") (declared-name "while") (range (start (line 19) (character 2)) (end (line 19) (character 41))) (parent (node (document "d0") (qualified-name "StructuredControlTest::"))))
    (element (id (node (document "d0") (qualified-name "StructuredControlTest::::_while::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 20) (character 3)) (end (line 20) (character 21))) (parent (node (document "d0") (qualified-name "StructuredControlTest::::_while"))))
    (element (id (node (document "d0") (qualified-name "StructuredControlTest::::aLoop"))) (kind "action") (name "aLoop") (declared-name "aLoop") (range (start (line 18) (character 2)) (end (line 18) (character 22))) (parent (node (document "d0") (qualified-name "StructuredControlTest::"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "StructuredControlTest::::b : ScalarValues::Boolean"))) (kind "action body decl") (name "b : ScalarValues::Boolean") (declared-name "b : ScalarValues::Boolean") (range (start (line 4) (character 2)) (end (line 4) (character 38))) (parent (node (document "d0") (qualified-name "StructuredControlTest::"))))
    (element (id (node (document "d0") (qualified-name "StructuredControlTest::::i : ScalarValues::Integer := 0"))) (kind "action body decl") (name "i : ScalarValues::Integer := 0") (declared-name "i : ScalarValues::Integer := 0") (range (start (line 3) (character 2)) (end (line 3) (character 43))) (parent (node (document "d0") (qualified-name "StructuredControlTest::"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "StructuredControlTest::"))) (kind performSource) (ordinal 0)) (authored-target "StructuredControlTest::::aLoop") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StructuredControlTest::::aLoop")))))
    (reference (id (source (node (document "d0") (qualified-name "StructuredControlTest::::aLoop"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StructuredControlTest::")))))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "StructuredControlTest::"))) (target (node (document "d0") (qualified-name "StructuredControlTest::::aLoop"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StructuredControlTest::"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StructuredControlTest::::aLoop"))) (target (node (document "d0") (qualified-name "StructuredControlTest::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StructuredControlTest::::aLoop"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
