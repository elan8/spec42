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
  (document "memory://snapshot/structured_control_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 3 2) (end 3 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 4 2) (end 4 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 5) (end 6 6))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 10) (end 7 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 12) (end 8 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 10) (end 9 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 10) (end 11 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 15) (end 11 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 5) (end 14 6))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 10) (end 15 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 15) (end 15 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 8) (end 19 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 10) (end 20 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 15) (end 20 16))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 21 4) (end 23 2))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 23 2) (end 27 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 10) (end 28 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 15) (end 28 16))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 29 4) (end 31 2))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 31 2) (end 34 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:84b55a1873c4b0df5137f643091f1de7c1aa033cb67835cd4609e1ade340c657") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (qualified-name "StructuredControlTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)))))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)))))) (kind if) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "i"))))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 1)))))) (kind if) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "i"))))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind while) (ordinal 0)))))) (kind while) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "i"))))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind loop) (ordinal 0)))))) (kind loop) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "i"))))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind if) (ordinal 0)))))) (kind if) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "i"))))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 1)) (anonymous (kind assign) (ordinal 0)))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "i")) (assignTarget (reference "i"))))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "i")) (assignTarget (reference "i"))))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind loop) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "i")) (assignTarget (reference "i"))))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "i"))))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind assign) (ordinal 1)))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "i")) (assignTarget (reference "i"))))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (named (kind action) (name "aLoop")))))) (kind action) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "i")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 1)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "i")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind while) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "i")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind if) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "i")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 1)) (anonymous (kind assign) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "i")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "i")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind loop) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "i")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind assignTarget) (ordinal 0))
      (authored-target "i")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 1)) (anonymous (kind assign) (ordinal 0)))))) (kind assignTarget) (ordinal 0))
      (authored-target "i")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind assignTarget) (ordinal 0))
      (authored-target "i")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind loop) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind assignTarget) (ordinal 0))
      (authored-target "i")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind assign) (ordinal 1)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "i")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind assignTarget) (ordinal 0))
      (authored-target "i")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind assign) (ordinal 1)))))) (kind assignTarget) (ordinal 0))
      (authored-target "i")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 1)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind while) (ordinal 0)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (state literal) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind if) (ordinal 0)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 1)) (anonymous (kind assign) (ordinal 0)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind loop) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind assign) (ordinal 1)))))) (state unresolved-operand))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/structured_control_test.md") (range (start 6 5) (end 6 6)) (probe (position 6 5))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "i")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/structured_control_test.md") (range (start 14 5) (end 14 6)) (probe (position 14 5))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 1)))))) (kind expressionOperand) (ordinal 0) (authored-target "i")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/structured_control_test.md") (range (start 19 8) (end 19 9)) (probe (position 19 8))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind while) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "i")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/structured_control_test.md") (range (start 8 12) (end 8 13)) (probe (position 8 12))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind if) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "i")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/structured_control_test.md") (range (start 15 15) (end 15 16)) (probe (position 15 15))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 1)) (anonymous (kind assign) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "i")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/structured_control_test.md") (range (start 20 15) (end 20 16)) (probe (position 20 15))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "i")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/structured_control_test.md") (range (start 28 15) (end 28 16)) (probe (position 28 15))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind loop) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "i")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/structured_control_test.md") (range (start 7 10) (end 7 11)) (probe (position 7 10))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind assignTarget) (ordinal 0) (authored-target "i")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/structured_control_test.md") (range (start 15 10) (end 15 11)) (probe (position 15 10))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 1)) (anonymous (kind assign) (ordinal 0)))))) (kind assignTarget) (ordinal 0) (authored-target "i")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/structured_control_test.md") (range (start 20 10) (end 20 11)) (probe (position 20 10))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind assignTarget) (ordinal 0) (authored-target "i")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/structured_control_test.md") (range (start 28 10) (end 28 11)) (probe (position 28 10))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind loop) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind assignTarget) (ordinal 0) (authored-target "i")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/structured_control_test.md") (range (start 11 15) (end 11 16)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind assign) (ordinal 1)))))) (kind expressionOperand) (ordinal 0) (authored-target "i")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/structured_control_test.md") (range (start 9 10) (end 9 11)) (probe (position 9 10))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind assignTarget) (ordinal 0) (authored-target "i")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/structured_control_test.md") (range (start 11 10) (end 11 11)) (probe (position 11 10))
    (reference (id (source (node (document "memory://snapshot/structured_control_test.md") (path (named (kind package) (name "StructuredControlTest")) (anonymous (kind action) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (anonymous (kind assign) (ordinal 1)))))) (kind assignTarget) (ordinal 0) (authored-target "i")
      (outcome (status unresolved)))
  )
)
~~~
