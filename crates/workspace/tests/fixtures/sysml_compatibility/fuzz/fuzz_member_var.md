# META
~~~ini
description=Fuzz: var feature in definition body should not emit spurious 'member' keyword
type=file
~~~
# SOURCE
~~~sysml
package P {
    requirement r {
        var x :>> y = 42;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'y'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'y'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwRequirement,Ident,OpenCurly,
KwVar,Ident,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (requirement_usage 'r'
      (feature_def var 'x' :>> 'y' value))))
~~~
# FORMAT
~~~sysml
package P {
    requirement r {
        var x:>> y = 42;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'P'
      (requirement_usage 'r'
        (feature_def 'x' :>> 'y'[unresolved]
          (feature_value (=)))))))
~~~
