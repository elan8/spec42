# META
~~~ini
description=KerML succession with structured parsing (stdlib patterns from StatePerformances/TransitionPerformances)
type=file
~~~
# SOURCE
~~~kerml
package SuccessionStructured {
    succession all [*] trigger then [*] guard;
    succession [1] entry then [*] middle;
    succession first X then Y;
    succession s first A then B;
    succession all [*] acceptable then [1] exit;
    succession x;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/succession_structured.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 1 4) (end 1 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 4) (end 1 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 2 4) (end 2 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 2 4) (end 2 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 21) (end 3 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 28) (end 3 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 23) (end 4 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 30) (end 4 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 5 4) (end 5 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 4) (end 5 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 6 4) (end 6 17))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery,unsupported-syntax) (has-evaluation false) (source-digest "blake3:b08e203ed72d12009c7ad384a8071acbd7c4f1a389b32d77e1a1bee0d231a3db") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/succession_structured.md") (qualified-name "SuccessionStructured"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "X")) (succession (reference "Y")))))
    (declaration (id (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "A")) (succession (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "X")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "A")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "Y")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "B")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/succession_structured.md") (range (start 3 21) (end 3 22)) (probe (position 3 21))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "X")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/succession_structured.md") (range (start 4 23) (end 4 24)) (probe (position 4 23))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "A")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/succession_structured.md") (range (start 3 28) (end 3 29)) (probe (position 3 28))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "Y")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/succession_structured.md") (range (start 4 30) (end 4 31)) (probe (position 4 30))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "B")
      (outcome (status unresolved)))
    )
  )
)
~~~
