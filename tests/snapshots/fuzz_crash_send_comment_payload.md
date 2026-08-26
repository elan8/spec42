# META
~~~ini
description=Fuzzer crash: send node with comment-only payload causing semicolon absorption
type=file
~~~
# SOURCE
~~~sysml
package P {
action def A {
    for
in send// nd port for HTT3prin  pq  for y  // nd port for HTT3prin items { }
  send pq   }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_crash_send_comment_payload.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 2 4) (end 4 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 7) (end 4 9))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:aeb710a8d0ad90ed1f589691d9c9f54cd7f016be1c763ccf63758b62e0100a8e"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_crash_send_comment_payload.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_crash_send_comment_payload.md") (qualified-name "P::A"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_crash_send_comment_payload.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind send-action) (ordinal 0))))) (kind send-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "pq")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/fuzz_crash_send_comment_payload.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind send-action) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "pq")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_crash_send_comment_payload.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind send-action) (ordinal 0))))) (target (node (document "memory://snapshot/fuzz_crash_send_comment_payload.md") (qualified-name "P::A"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/fuzz_crash_send_comment_payload.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind send-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/fuzz_crash_send_comment_payload.md") (qualified-name "P::A")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/fuzz_crash_send_comment_payload.md") (range (start 4 7) (end 4 9)) (probe (position 4 7))
    (reference (id (source (node (document "memory://snapshot/fuzz_crash_send_comment_payload.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind send-action) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "pq")
      (outcome (status unresolved)))
    )
  )
)
~~~
