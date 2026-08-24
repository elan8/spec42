# META
~~~ini
description=KerML Unicode Identifier Tests
type=file
~~~
# SOURCE
~~~kerml
package 'αβ' {
    class '漢字';
    type '🧪' :> Base::Anything;
    class 'é';
    class 'Ω' :> Pkg::'β';
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/unicode_identifiers.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 2 19) (end 2 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 4 18) (end 4 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:f8a23e6174d62e17d3c65b92b6caa884baa28c0d45ccde488656c537a2982539") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/unicode_identifiers.md") (qualified-name "αβ"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/unicode_identifiers.md") (qualified-name "αβ::é"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/unicode_identifiers.md") (qualified-name "αβ::Ω"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Pkg::β")))))
    (declaration (id (node (document "memory://snapshot/unicode_identifiers.md") (qualified-name "αβ::漢字"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/unicode_identifiers.md") (qualified-name "αβ::🧪"))) (kind kerml-type) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base::Anything")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/unicode_identifiers.md") (qualified-name "αβ::Ω"))) (kind specialization) (ordinal 0))
      (authored-target "Pkg::β")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/unicode_identifiers.md") (qualified-name "αβ::🧪"))) (kind specialization) (ordinal 0))
      (authored-target "Base::Anything")
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
  (query (document "memory://snapshot/unicode_identifiers.md") (range (start 4 18) (end 4 27)) (probe (position 4 18))
    (reference (id (source (node (document "memory://snapshot/unicode_identifiers.md") (qualified-name "αβ::Ω"))) (kind specialization) (ordinal 0) (authored-target "Pkg::β")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/unicode_identifiers.md") (range (start 2 19) (end 2 33)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/unicode_identifiers.md") (qualified-name "αβ::🧪"))) (kind specialization) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
    )
  )
)
~~~
