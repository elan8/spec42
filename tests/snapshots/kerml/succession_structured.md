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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 1 23) (end 1 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 1 40) (end 1 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 19) (end 2 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 34) (end 2 40))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 23) (end 5 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 43) (end 5 47))
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
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:92cb70d46ded883e9f3d27d98daab27d29229294e0e8db293e70445eb29464ac"))
  (declarations
    (declaration (id (node (document "memory://snapshot/succession_structured.md") (qualified-name "SuccessionStructured"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (facts (modifiers all) (end-multiplicity (end source) (lower unbounded) (upper unbounded)) (end-multiplicity (end target) (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "trigger")) (succession (reference "guard")))))
    (declaration (id (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (facts (end-multiplicity (end source) (lower 1) (upper 1)) (end-multiplicity (end target) (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "entry")) (succession (reference "middle")))))
    (declaration (id (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "X")) (succession (reference "Y")))))
    (declaration (id (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (membership (kind feature) (visibility default)) (facts (modifiers all) (end-multiplicity (end source) (lower unbounded) (upper unbounded)) (end-multiplicity (end target) (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "acceptable")) (succession (reference "exit")))))
    (declaration (id (node (document "memory://snapshot/succession_structured.md") (qualified-name "SuccessionStructured::s"))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "A")) (succession (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "trigger")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "entry")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0))
      (authored-target "X")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 0))
      (authored-target "acceptable")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "guard")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "middle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1))
      (authored-target "Y")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 1))
      (authored-target "exit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (qualified-name "SuccessionStructured::s"))) (kind succession) (ordinal 0))
      (authored-target "A")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (qualified-name "SuccessionStructured::s"))) (kind succession) (ordinal 1))
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
  (query (document "memory://snapshot/succession_structured.md") (range (start 1 23) (end 1 30)) (probe (position 1 23))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "trigger")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/succession_structured.md") (range (start 2 19) (end 2 24)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "entry")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/succession_structured.md") (range (start 3 21) (end 3 22)) (probe (position 3 21))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0) (authored-target "X")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/succession_structured.md") (range (start 5 23) (end 5 33)) (probe (position 5 23))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 0) (authored-target "acceptable")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/succession_structured.md") (range (start 1 40) (end 1 45)) (probe (position 1 40))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "guard")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/succession_structured.md") (range (start 2 34) (end 2 40)) (probe (position 2 34))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "middle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/succession_structured.md") (range (start 3 28) (end 3 29)) (probe (position 3 28))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1) (authored-target "Y")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/succession_structured.md") (range (start 5 43) (end 5 47)) (probe (position 5 43))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (path (named (kind package) (name "SuccessionStructured")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 1) (authored-target "exit")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/succession_structured.md") (range (start 4 23) (end 4 24)) (probe (position 4 23))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (qualified-name "SuccessionStructured::s"))) (kind succession) (ordinal 0) (authored-target "A")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/succession_structured.md") (range (start 4 30) (end 4 31)) (probe (position 4 30))
    (reference (id (source (node (document "memory://snapshot/succession_structured.md") (qualified-name "SuccessionStructured::s"))) (kind succession) (ordinal 1) (authored-target "B")
      (outcome (status unresolved)))
    )
  )
)
~~~
