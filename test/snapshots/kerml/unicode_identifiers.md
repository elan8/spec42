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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 4) (end 1 19))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 2 4) (end 3 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 3 4) (end 3 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 4) (end 4 28))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:f8a23e6174d62e17d3c65b92b6caa884baa28c0d45ccde488656c537a2982539") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/unicode_identifiers.md") (qualified-name "αβ"))) (kind package) (membership (kind owning) (visibility default)))
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
