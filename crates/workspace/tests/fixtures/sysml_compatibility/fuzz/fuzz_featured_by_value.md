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
# EXPECTED
~~~
semantic.unresolved_name 'c'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'c'
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
# FORMAT
~~~sysml
package P {
    feature g featured by c = 42;
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'P'
      (feature_def 'g'
        (feature_value (=))))))
~~~
