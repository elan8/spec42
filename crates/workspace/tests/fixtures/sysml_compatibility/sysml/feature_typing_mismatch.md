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
# EXPECTED
~~~
semantic.feature_typing_kind_mismatch
~~~
# PROBLEMS
~~~
semantic.feature_typing_kind_mismatch
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
# FORMAT
~~~sysml
attribute def Foo {}
part p : Foo;

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "attribute def") (id (node (document "d0") (qualified-name "Foo"))) (name "Foo") (declared-name "Foo") (declared (properties (ordered false) (unique true))))
    (element (kind "part") (id (node (document "d0") (qualified-name "p"))) (name "p") (declared-name "p") (declared (properties (composite true) (reference false) (ordered false))))
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "p"))) (to (node (document "d0") (qualified-name "Foo"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
