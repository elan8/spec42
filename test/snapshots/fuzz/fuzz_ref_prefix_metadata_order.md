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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_ref_prefix_metadata_order.md"
    (diagnostics
    )
  )
)
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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "494594fc27d6e4bffca4753540d7cc8ceac8925ac8474c56cfe9a61ba09cbac3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 0) (character 0)) (end (line 0) (character 69))))
    (element (id (node (document "d0") (qualified-name "P::C"))) (kind "classifier decl") (name "C") (declared-name "C") (range (start (line 1) (character 4)) (end (line 1) (character 55))) (parent (node (document "d0") (qualified-name "P"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
