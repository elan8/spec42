# META
~~~ini
description=SysML Example (Simple Tests): MultiplicityTest
type=file
~~~
# SOURCE
~~~sysml
package MultiplicityTest {
	
	part def P;
	attribute n : ScalarValues::Integer = 5;
	
	part a[1];
	part b[0..2] : P;
	part c : P[2..*];
	part d[*];
	
	part e[n];
	part f[n..*];
	part g[1..n];

	attribute def A {
		attribute i :ScalarValues::Integer;
		attribute x : A[i];
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,Ident,DotDot,Star,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,DotDot,Ident,CloseSquare,Semicolon,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MultiplicityTest'
    (part_def 'P')
    (attribute_usage 'n' : 'ScalarValues::Integer' value)
    (part_usage 'a' multiplicity)
    (part_usage 'b' : 'P' multiplicity)
    (part_usage 'c' : 'P' multiplicity)
    (part_usage 'd' multiplicity)
    (part_usage 'e' multiplicity)
    (part_usage 'f' multiplicity)
    (part_usage 'g' multiplicity)
    (attribute_def 'A'
      (attribute_usage 'i' : 'ScalarValues::Integer')
      (attribute_usage 'x' : 'A' multiplicity))))
~~~
# FORMAT
~~~sysml
package MultiplicityTest {

    part def P;
    attribute n : ScalarValues::Integer = 5;

    part a[1];
    part b[0..2] : P;
    part c : P[2..*];
    part d[*];

    part e[n];
    part f[n..*];
    part g[1..n];

    attribute def A {
        attribute i :ScalarValues::Integer;
        attribute x : A[i];
    }
}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'ScalarValues::Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'ScalarValues::Integer'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "MultiplicityTest"))) (name "MultiplicityTest") (declared-name "MultiplicityTest")
      (contains
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MultiplicityTest::A"))) (name "A") (declared-name "A") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MultiplicityTest::A::i"))) (name "i") (declared-name "i") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MultiplicityTest::A")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MultiplicityTest::A::x"))) (name "x") (declared-name "x") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MultiplicityTest::A")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "MultiplicityTest::P"))) (name "P") (declared-name "P") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "MultiplicityTest::a"))) (name "a") (declared-name "a") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))))
        (element (kind "part") (id (node (document "d0") (qualified-name "MultiplicityTest::b"))) (name "b") (declared-name "b") (declared (properties (ordered false)) (multiplicity (lower 0) (upper 2) (ordered false) (provenance authored))))
        (element (kind "part") (id (node (document "d0") (qualified-name "MultiplicityTest::c"))) (name "c") (declared-name "c") (declared (properties (ordered false)) (multiplicity (lower 2) (upper unbounded) (ordered false) (provenance authored))))
        (element (kind "part") (id (node (document "d0") (qualified-name "MultiplicityTest::d"))) (name "d") (declared-name "d") (declared (properties (ordered false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))))
        (element (kind "part") (id (node (document "d0") (qualified-name "MultiplicityTest::e"))) (name "e") (declared-name "e") (declared (properties (ordered false)) (multiplicity (lower unevaluated) (upper unevaluated) (ordered false) (provenance authored))))
        (element (kind "part") (id (node (document "d0") (qualified-name "MultiplicityTest::f"))) (name "f") (declared-name "f") (declared (properties (ordered false)) (multiplicity (lower unevaluated) (upper unbounded) (ordered false) (provenance authored))))
        (element (kind "part") (id (node (document "d0") (qualified-name "MultiplicityTest::g"))) (name "g") (declared-name "g") (declared (properties (ordered false)) (multiplicity (lower 1) (upper unevaluated) (ordered false) (provenance authored))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MultiplicityTest::n"))) (name "n") (declared-name "n") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "integerLiteral") (literal 5)))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "MultiplicityTest::n"))) (role feature-value))))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "MultiplicityTest::A::x"))) (to (node (document "d0") (qualified-name "MultiplicityTest::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MultiplicityTest::b"))) (to (node (document "d0") (qualified-name "MultiplicityTest::P"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MultiplicityTest::c"))) (to (node (document "d0") (qualified-name "MultiplicityTest::P"))))
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
  (document "sysml/examples/multiplicity_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 1) (end 3 41))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 5 1) (end 5 11))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 8 1) (end 8 11))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 10 1) (end 10 11))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 11 1) (end 11 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 12 1) (end 12 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 37))
      )
    )
  )
)
~~~
