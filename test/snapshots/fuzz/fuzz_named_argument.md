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
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 1 17) (end 1 26))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:a30c87320530daeaa6ba1ba5a93f49193903c5d98385c4e7c0c8f5aa187a0a0d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::F"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::b"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_named_argument.md") (qualified-name "P::f"))) (kind attribute-def) (membership (kind owning) (visibility default)))
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
