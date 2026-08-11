# META
~~~ini
description=Fuzz: featured by must precede value assignment for idempotent reparse
type=file
~~~
# SOURCE
~~~sysml
package P {
    feature g featured by c = 42;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_featured_by_value.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwFeature,Ident,KwFeatured,KwBy,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (feature_def 'g' value featured by 'c')))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'c'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'c'
~~~
# FORMAT
~~~sysml
package P {
    feature g featured by c = 42;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a028bb4efb42e1f7b094d4943aa6b1638442821faada43694ab06f4db26026f6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 0) (character 0)) (end (line 0) (character 47))))
    (element (id (node (document "d0") (qualified-name "P::g"))) (kind "feature decl") (name "g") (declared-name "g") (range (start (line 1) (character 4)) (end (line 1) (character 33))) (parent (node (document "d0") (qualified-name "P"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
