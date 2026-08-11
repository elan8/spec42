# META
~~~ini
description=Fuzzer crash: unclosed short name `<f` without `>` causes idempotence violation
type=file
~~~
# SOURCE
~~~sysml
package ion {
  class A {
    in<f;
  }

  class A { in #su f;
  }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_unclosed_short_name.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
KwIn,OpenAngle,Ident,Semicolon,
CloseCurly,
KwClass,Ident,OpenCurly,KwIn,Hash,Ident,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ion'
    (class_def 'A'
      (malformed))
    (class_def 'A'
      (feature_def in #'su' 'f'))))
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
semantic.duplicate_name 'A'
semantic.ambiguous_member 'A'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
semantic.duplicate_name 'A'
semantic.ambiguous_member 'A'
~~~
# FORMAT
~~~sysml
package ion {
  class A {
    in<f;
  }

  class A { in #su f;
  }
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6f0cfc58889cc520cc0acaf3e9a0d5df9664402d52925df6ab57f1595c1d7949") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ion"))) (kind "package") (name "ion") (declared-name "ion") (range (start (line 0) (character 0)) (end (line 0) (character 68))))
    (element (id (node (document "d0") (qualified-name "ion::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 1) (character 2)) (end (line 1) (character 25))) (parent (node (document "d0") (qualified-name "ion"))))
    (element (id (node (document "d0") (qualified-name "ion::A#classifier_decl"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 5) (character 2)) (end (line 5) (character 25))) (parent (node (document "d0") (qualified-name "ion"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
