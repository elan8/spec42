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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:a30c87320530daeaa6ba1ba5a93f49193903c5d98385c4e7c0c8f5aa187a0a0d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F::p"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A") (direction in))))
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::b"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::f"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/fuzz_named_argument.md") (range (start 1 24) (end 1 25)) (probe (position 1 24))
    (reference (id (source (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F::p"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status unresolved)))
  )
)
~~~
