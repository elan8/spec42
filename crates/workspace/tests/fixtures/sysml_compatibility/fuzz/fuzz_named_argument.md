# META
~~~ini
description=Fuzz: named arguments in invocations use = not => for idempotent reparse
type=file
~~~
# SOURCE
~~~sysml
package P {
    calc def F { in p : A; }
    attribute f = F(q = 1, p = a);
    attribute b = new A(y = a, x = "");
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'A'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'A'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAttribute,Ident,Eq,Ident,OpenParen,Ident,Eq,DecimalValue,Comma,Ident,Eq,Ident,CloseParen,Semicolon,
KwAttribute,Ident,Eq,Ident,Ident,OpenParen,Ident,Eq,Ident,Comma,Ident,Eq,StringValue,CloseParen,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (calc_def 'F'
      (default_ref_usage in 'p' : 'A'))
    (attribute_usage 'f' value)
    (attribute_usage 'b' value)))
~~~
# FORMAT
~~~sysml
package P {
    calc def F {
        in p : A;
    }
    attribute f = F(q = 1, p = a);
    attribute b = new A(y = a, x = "");
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'P'
      (calculation_def 'F'
        (reference_usage in reference 'p' : 'A'[unresolved]))
      (attribute_usage 'f'
        (feature_value (=)))
      (attribute_usage 'b'
        (feature_value (=))))))
~~~
