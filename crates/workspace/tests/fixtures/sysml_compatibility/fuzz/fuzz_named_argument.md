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
    calc def F { in p : A; }
    attribute f = F(q = 1, p = a);
    attribute b = new A(y = a, x = "");
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "P"))) (name "P") (declared-name "P")
      (contains
        (element (kind "calc def") (id (node (document "d0") (qualified-name "P::F"))) (name "F") (declared-name "F")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "P::F::p"))) (name "p") (declared-name "p") (effective (featuring-type (node (document "d0") (qualified-name "P::F")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "P::b"))) (name "b") (declared-name "b") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "constructor") (reference "A") (arguments (argument (name "y") (expression (kind "featureReference") (reference "a"))) (argument (name "x") (expression (kind "stringLiteral") (literal ""))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "P::b"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "P::f"))) (name "f") (declared-name "f") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "F"))) (arguments (argument (name "q") (expression (kind "integerLiteral") (literal 1))) (argument (name "p") (expression (kind "featureReference") (reference "a"))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "P::f"))) (role feature-value))))
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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz/fuzz_named_argument.md"
    (diagnostics
    )
  )
)
~~~
