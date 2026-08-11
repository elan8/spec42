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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_unclosed_short_name_metadata.md"
    (diagnostics
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6ff7ebad902c219bba7c5c4bd8f1ca67afd5d511bfbc3787ef7f1790d9dfdd85") (contract-version "canonical-resolution-v1"))
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
