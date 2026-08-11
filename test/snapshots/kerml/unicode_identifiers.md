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
  (document "unicode_identifiers.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 2 4) (end 2 81))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'αβ' {
    class '漢字';
    type '🧪' :> Base::Anything;
    class 'é';
    class 'Ω' :> Pkg::'β';
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "bc71cc2126695b9b4f16f905938197687270248bed2b21f4400605202a6021c9") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "αβ"))) (kind "package") (name "αβ") (declared-name "αβ") (range (start (line 0) (character 0)) (end (line 0) (character 119))))
    (element (id (node (document "d0") (qualified-name "αβ::class"))) (kind "classifier decl") (name "class") (declared-name "class") (range (start (line 1) (character 4)) (end (line 1) (character 19))) (parent (node (document "d0") (qualified-name "αβ"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
