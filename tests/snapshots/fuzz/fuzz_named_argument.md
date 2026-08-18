# META
~~~ini
description=Fuzz: named arguments in invocations use = not => for idempotent reparse
type=file
~~~
# SOURCE
~~~sysml
package P {
    calc def F { in p : A; }
    attribute f = F(q = 1, p = a);
    attribute b = new A(y = a, x = "");
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_named_argument.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 24) (end 1 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 31) (end 2 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 22) (end 3 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 28) (end 3 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a30c87320530daeaa6ba1ba5a93f49193903c5d98385c4e7c0c8f5aa187a0a0d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F::p"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A") (direction in)))))
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::b"))) (kind attribute-def) (membership (kind owning) (visibility default)) (feature-value (kind bind)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "a")) (invocationCallee (reference "A")))))
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::f"))) (kind attribute-def) (membership (kind owning) (visibility default)) (feature-value (kind bind)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "a")) (invocationCallee (reference "F")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::b"))) (kind expressionOperand) (ordinal 0))
      (authored-target "a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::b"))) (kind invocationCallee) (ordinal 0))
      (authored-target "A")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::f"))) (kind expressionOperand) (ordinal 0))
      (authored-target "a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::f"))) (kind invocationCallee) (ordinal 0))
      (authored-target "F")
      (outcome (status resolved) (target (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F")))))
  )
  (relationships
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::f"))) (target (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::f"))) (kind invocationCallee) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::b"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::f"))) (state non-constant))
    (invocation (declaration (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::f"))) (callee (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F"))) (supplied 2) (required 1) (start 2 18) (end 2 33))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F::p")))
      (featured-by (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/fuzz_named_argument.md") (range (start 1 24) (end 1 25)) (probe (position 1 24))
    (reference (id (source (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F::p"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/fuzz_named_argument.md") (range (start 3 28) (end 3 29)) (probe (position 3 28))
    (reference (id (source (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::b"))) (kind expressionOperand) (ordinal 0) (authored-target "a")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/fuzz_named_argument.md") (range (start 3 22) (end 3 23)) (probe (position 3 22))
    (reference (id (source (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::b"))) (kind invocationCallee) (ordinal 0) (authored-target "A")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/fuzz_named_argument.md") (range (start 2 31) (end 2 32)) (probe (position 2 31))
    (reference (id (source (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::f"))) (kind expressionOperand) (ordinal 0) (authored-target "a")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/fuzz_named_argument.md") (range (start 2 18) (end 2 19)) (probe (position 2 18))
    (reference (id (source (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::f"))) (kind invocationCallee) (ordinal 0) (authored-target "F")
      (outcome (status resolved) (target (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F")))))
    )
  )
)
~~~
