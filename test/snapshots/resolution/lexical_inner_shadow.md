# META
~~~ini
description=Lexical inner binding shadows an incompatible imported binding
type=file
~~~
# SOURCE
~~~sysml
package A {
    part def T;
}
package C {
    import A::*;
    part T;
    part p : T;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "lexical_inner_shadow.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 5 4) (end 5 11))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package A {
    part def T;
}
package C {
    import A::*;
    part T;
    part p : T;
}

~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "089d80923fa71dcfa1fe7d67d903e50ce9d4d995cbfefd15e3f10f6118f6f896") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "A"))) (kind "package") (name "A") (declared-name "A"))
    (element (id (node (document "d0") (qualified-name "A::T"))) (kind "part def") (name "T") (declared-name "T") (parent (node (document "d0") (qualified-name "A"))))
    (element (id (node (document "d0") (qualified-name "C"))) (kind "package") (name "C") (declared-name "C"))
    (element (id (node (document "d0") (qualified-name "C::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "C"))) (authored (membership (kind Import) (import (reference "A::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "C::T"))) (kind "part") (name "T") (declared-name "T") (parent (node (document "d0") (qualified-name "C"))))
    (element (id (node (document "d0") (qualified-name "C::p"))) (kind "part") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "C"))) (authored (membership (kind Feature)) (relationships (typing (reference "T")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "C::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "A::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "A")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "C::p"))) (kind featureTyping) (ordinal 0)) (authored-target "T") (outcome (status resolved) (target (node (document "d0") (qualified-name "C::T")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "C::p"))) (target (node (document "d0") (qualified-name "C::T"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "C::p"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 11) (end 4 12)) (probe (position 4 11))
      (reference
        (source (document "d0") (qualified-name "C::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "A::*")
        (range (start 4 11) (end 4 12))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "A") (range (start 0 0) (end 0 29)))
        )
      )
    )
    (query (range (start 6 13) (end 6 14)) (probe (position 6 13))
      (reference
        (source (document "d0") (qualified-name "C::p"))
        (kind featureTyping) (ordinal 0) (authored-target "T")
        (range (start 6 13) (end 6 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "C::T") (range (start 5 4) (end 5 11)))
        )
      )
    )
  )
)
~~~
