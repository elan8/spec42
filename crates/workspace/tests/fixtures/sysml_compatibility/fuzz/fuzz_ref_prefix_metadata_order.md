# META
~~~ini
description=Fuzz: ref keyword precedes prefix metadata annotations for correct reparse
type=file
~~~
# SOURCE
~~~sysml
package P {
    class C {
        ref #MyAnnotation self : C;
    }
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
KwRef,Hash,Ident,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (class_def 'C'
      (ref_usage ref #'MyAnnotation' 'self' : 'C'))))
~~~
# FORMAT
~~~sysml
package P {
    class C {
        ref #MyAnnotation self : C;
    }
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "P"))) (name "P") (declared-name "P")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "P::C"))) (name "C") (declared-name "C"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
