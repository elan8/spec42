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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwClass,UnrestrictedName,Semicolon,
KwType,UnrestrictedName,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwClass,UnrestrictedName,Semicolon,
KwClass,UnrestrictedName,ColonGt,Ident,ColonColon,UnrestrictedName,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''αβ''
    (class_def ''漢字'')
    (type_def ''🧪'' :> 'Base::Anything')
    (class_def ''é'')
    (class_def ''Ω'' :> 'Pkg::'β'')))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Pkg::β'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Pkg::β'
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
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "ec0536340af6e805b9d849d2373afd97adef8a8ec7b9575f097f24a2f2f6d5a2") (contract-version "canonical-resolution-v1"))
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
