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
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 6 2) (end 12 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 14 2) (end 16 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 19 2) (end 21 3))
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
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 27 2) (end 29 3))
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:84b55a1873c4b0df5137f643091f1de7c1aa033cb67835cd4609e1ade340c657") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (qualified-name "StructuredControlTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (anonymous (kind action) (ordinal 0))))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structured_control_test.md") (qualified-name "StructuredControlTest::::aLoop"))) (kind action) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
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
