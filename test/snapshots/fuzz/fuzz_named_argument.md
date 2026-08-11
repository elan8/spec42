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
  (document "fuzz_named_argument.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 17) (end 1 26))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    calc def F { in p : A; }
    attribute f = F(q = 1, p = a);
    attribute b = new A(y = a, x = "");
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "65c4980a5d3ea68d799182f9d32ee290054819c9ab98219064dd33f0f321b555") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 0) (character 0)) (end (line 0) (character 117))))
    (element (id (node (document "d0") (qualified-name "P::F"))) (kind "calc def") (name "F") (declared-name "F") (range (start (line 1) (character 4)) (end (line 1) (character 28))) (parent (node (document "d0") (qualified-name "P"))))
    (element (id (node (document "d0") (qualified-name "P::F::p"))) (kind "in out parameter") (name "p") (declared-name "p") (range (start (line 1) (character 17)) (end (line 1) (character 26))) (parent (node (document "d0") (qualified-name "P::F"))) (authored (relationships (typing (reference "A") (range none)))))
    (element (id (node (document "d0") (qualified-name "P::b"))) (kind "attribute def") (name "b") (declared-name "b") (range (start (line 3) (character 4)) (end (line 3) (character 39))) (parent (node (document "d0") (qualified-name "P"))))
    (element (id (node (document "d0") (qualified-name "P::f"))) (kind "attribute def") (name "f") (declared-name "f") (range (start (line 2) (character 4)) (end (line 2) (character 34))) (parent (node (document "d0") (qualified-name "P"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "P::F::p"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range none) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (node (node (document "d0") (qualified-name "P::b")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "P::f")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
