# META
~~~ini
description=SysML Example (Simple Tests): ParameterTest
type=file
~~~
# SOURCE
~~~sysml
package ParameterTest {
	attribute def A {
		attribute x : ScalarValues::String;
		attribute y : A;
	}
	
	attribute a : A;
	
	calc def F { in p : A; in q : ScalarValues::Integer; return :  ScalarValues::Integer; }
	
	attribute f = F(a, 2);
	attribute g = F(q = 1, p = a);
	
	attribute b = new A(y=a, x=""); 
	attribute c = new A("test2");
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,ColonColon,Ident,Semicolon,KwReturn,Colon,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwAttribute,Ident,Eq,Ident,OpenParen,Ident,Comma,DecimalValue,CloseParen,Semicolon,
KwAttribute,Ident,Eq,Ident,OpenParen,Ident,Eq,DecimalValue,Comma,Ident,Eq,Ident,CloseParen,Semicolon,
KwAttribute,Ident,Eq,Ident,Ident,OpenParen,Ident,Eq,Ident,Comma,Ident,Eq,StringValue,CloseParen,Semicolon,
KwAttribute,Ident,Eq,Ident,Ident,OpenParen,StringValue,CloseParen,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ParameterTest'
    (attribute_def 'A'
      (attribute_usage 'x' : 'ScalarValues::String')
      (attribute_usage 'y' : 'A'))
    (attribute_usage 'a' : 'A')
    (calc_def 'F'
      (default_ref_usage in 'p' : 'A')
      (default_ref_usage in 'q' : 'ScalarValues::Integer')
      (return_member))
    (attribute_usage 'f' value)
    (attribute_usage 'g' value)
    (attribute_usage 'b' value)
    (attribute_usage 'c' value)))
~~~
# FORMAT
~~~sysml
package ParameterTest {
    attribute def A {
        attribute x : ScalarValues::String;
        attribute y : A;
    }

    attribute a : A;

    calc def F {
        in p : A;
        in q : ScalarValues::Integer;
        return :  ScalarValues::Integer;
    }

    attribute f = F(a, 2);
    attribute g = F(q = 1, p = a);

    attribute b = new A(y=a, x="");
    attribute c = new A("test2");
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ScalarValues::String'
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'ScalarValues::Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarValues::String'
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'ScalarValues::Integer'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ParameterTest"))) (name "ParameterTest") (declared-name "ParameterTest")
      (contains
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ParameterTest::A"))) (name "A") (declared-name "A") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ParameterTest::A::x"))) (name "x") (declared-name "x") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ParameterTest::A")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ParameterTest::A::y"))) (name "y") (declared-name "y") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ParameterTest::A")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "ParameterTest::F"))) (name "F") (declared-name "F")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "ParameterTest::F::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ParameterTest::F")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ParameterTest::F::p"))) (name "p") (declared-name "p") (effective (featuring-type (node (document "d0") (qualified-name "ParameterTest::F")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ParameterTest::F::q"))) (name "q") (declared-name "q") (effective (featuring-type (node (document "d0") (qualified-name "ParameterTest::F")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ParameterTest::a"))) (name "a") (declared-name "a") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ParameterTest::b"))) (name "b") (declared-name "b") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "constructor") (reference "A") (arguments (argument (name "y") (expression (kind "featureReference") (reference "a"))) (argument (name "x") (expression (kind "stringLiteral") (literal ""))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "ParameterTest::b"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ParameterTest::c"))) (name "c") (declared-name "c") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "constructor") (reference "A") (arguments (argument (expression (kind "stringLiteral") (literal "test2"))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "ParameterTest::c"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ParameterTest::f"))) (name "f") (declared-name "f") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "F"))) (arguments (argument (expression (kind "featureReference") (reference "a"))) (argument (expression (kind "integerLiteral") (literal 2))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "ParameterTest::f"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ParameterTest::g"))) (name "g") (declared-name "g") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "F"))) (arguments (argument (name "q") (expression (kind "integerLiteral") (literal 1))) (argument (name "p") (expression (kind "featureReference") (reference "a"))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "ParameterTest::g"))) (role feature-value))))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "ParameterTest::A::y"))) (to (node (document "d0") (qualified-name "ParameterTest::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ParameterTest::F::p"))) (to (node (document "d0") (qualified-name "ParameterTest::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ParameterTest::a"))) (to (node (document "d0") (qualified-name "ParameterTest::A"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
