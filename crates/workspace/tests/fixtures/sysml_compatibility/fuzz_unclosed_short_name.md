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
# FORMAT
~~~sysml
package ion {
    class A {
        in<f;
    }

    class A {
        in #su f;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'ion'
      (class_def 'A'
        (not_implemented 'malformed'))
      (class_def 'A'
        (feature_def in 'f')))))
~~~
