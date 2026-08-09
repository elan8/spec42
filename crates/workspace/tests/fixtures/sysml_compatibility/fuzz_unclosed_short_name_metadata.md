# META
~~~ini
description=Fuzzer crash: unclosed short name with prefix metadata `#su<f` causes idempotence violation
type=file
~~~
# SOURCE
~~~sysml
package ion {
  class A {
    in f;
  }

  class A { in #su<f;
  }
}
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
semantic.duplicate_name 'A'
semantic.ambiguous_member 'A'
semantic.ambiguous_member 'malformed'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
semantic.duplicate_name 'A'
semantic.ambiguous_member 'A'
semantic.ambiguous_member 'malformed'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
KwIn,Ident,Semicolon,
CloseCurly,
KwClass,Ident,OpenCurly,KwIn,Hash,Ident,OpenAngle,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ion'
    (class_def 'A'
      (feature_def in 'f'))
    (class_def 'A'
      (malformed)
      (malformed))))
~~~
# FORMAT
~~~sysml
package ion {
  class A {
    in f;
  }

  class A { in #su<f;
  }
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ion"))) (name "ion") (declared-name "ion")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ion::A"))) (name "A") (declared-name "A"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ion::A#classifier_decl"))) (name "A") (declared-name "A"))
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
