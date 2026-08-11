# META
~~~ini
description=SysML Feature Typing Kind Mismatch (SC-4)
type=file
~~~
# SOURCE
~~~sysml
attribute def Foo {}
part p : Foo;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "feature_typing_mismatch.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwAttribute,KwDef,Ident,OpenCurly,CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,EndOfFile,
~~~
# AST
~~~
(root
  (attribute_def 'Foo')
  (part_usage 'p' : 'Foo'))
~~~
# EXPECTED
~~~
semantic.feature_typing_kind_mismatch
~~~
# PROBLEMS
~~~
semantic.feature_typing_kind_mismatch
~~~
# FORMAT
~~~sysml
attribute def Foo {}
part p : Foo;

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "2814bc9eca9a81de17357b4c5d2fc69bf205bd5e276ec4beee1fe10870deb7f6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Foo"))) (kind "attribute def") (name "Foo") (declared-name "Foo") (range (start (line 0) (character 0)) (end (line 0) (character 20))))
    (element (id (node (document "d0") (qualified-name "p"))) (kind "part") (name "p") (declared-name "p") (range (start (line 1) (character 0)) (end (line 1) (character 13))) (authored (membership (kind Feature)) (relationships (typing (reference "Foo") (range (start (line 1) (character 9)) (end (line 1) (character 12)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "p"))) (kind featureTyping) (ordinal 0)) (authored-target "Foo") (range (start (line 1) (character 9)) (end (line 1) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Foo")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "p"))) (target (node (document "d0") (qualified-name "Foo"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "p"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
