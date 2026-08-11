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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parameter_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 2) (end 2 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 24) (end 8 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 54) (end 8 86))
      )
    )
  )
)
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "22e86f60b3f7cd03878476e30a1c79ad6b72ac3e1e54df48a295623f974c230a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ParameterTest"))) (kind "package") (name "ParameterTest") (declared-name "ParameterTest") (range (start (line 0) (character 0)) (end (line 0) (character 340))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::A"))) (kind "attribute def") (name "A") (declared-name "A") (range (start (line 1) (character 1)) (end (line 1) (character 78))) (parent (node (document "d0") (qualified-name "ParameterTest"))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::A::x"))) (kind "attribute") (name "x") (declared-name "x") (range (start (line 2) (character 2)) (end (line 2) (character 37))) (parent (node (document "d0") (qualified-name "ParameterTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::A::y"))) (kind "attribute") (name "y") (declared-name "y") (range (start (line 3) (character 2)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "ParameterTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range none)))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::F"))) (kind "calc def") (name "F") (declared-name "F") (range (start (line 8) (character 1)) (end (line 8) (character 88))) (parent (node (document "d0") (qualified-name "ParameterTest"))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::F::"))) (kind "return parameter") (name "") (range (start (line 8) (character 54)) (end (line 8) (character 86))) (parent (node (document "d0") (qualified-name "ParameterTest::F"))) (authored (relationships (typing (reference "ScalarValues::Integer") (range none)))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::F::p"))) (kind "in out parameter") (name "p") (declared-name "p") (range (start (line 8) (character 14)) (end (line 8) (character 23))) (parent (node (document "d0") (qualified-name "ParameterTest::F"))) (authored (relationships (typing (reference "A") (range none)))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::F::q"))) (kind "in out parameter") (name "q") (declared-name "q") (range (start (line 8) (character 24)) (end (line 8) (character 53))) (parent (node (document "d0") (qualified-name "ParameterTest::F"))) (authored (relationships (typing (reference "ScalarValues::Integer") (range none)))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::a"))) (kind "attribute def") (name "a") (declared-name "a") (range (start (line 6) (character 1)) (end (line 6) (character 17))) (parent (node (document "d0") (qualified-name "ParameterTest"))) (authored (membership (kind Owning)) (relationships (typing (reference "A") (range none)))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::b"))) (kind "attribute def") (name "b") (declared-name "b") (range (start (line 13) (character 1)) (end (line 13) (character 32))) (parent (node (document "d0") (qualified-name "ParameterTest"))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::c"))) (kind "attribute def") (name "c") (declared-name "c") (range (start (line 14) (character 1)) (end (line 14) (character 30))) (parent (node (document "d0") (qualified-name "ParameterTest"))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::f"))) (kind "attribute def") (name "f") (declared-name "f") (range (start (line 10) (character 1)) (end (line 10) (character 23))) (parent (node (document "d0") (qualified-name "ParameterTest"))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::g"))) (kind "attribute def") (name "g") (declared-name "g") (range (start (line 11) (character 1)) (end (line 11) (character 31))) (parent (node (document "d0") (qualified-name "ParameterTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ParameterTest::A::x"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ParameterTest::A::y"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ParameterTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ParameterTest::F::"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarValues::Integer") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ParameterTest::F::p"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ParameterTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ParameterTest::F::q"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarValues::Integer") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ParameterTest::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ParameterTest::A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ParameterTest::A::y"))) (target (node (document "d0") (qualified-name "ParameterTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ParameterTest::A::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ParameterTest::F::p"))) (target (node (document "d0") (qualified-name "ParameterTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ParameterTest::F::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ParameterTest::a"))) (target (node (document "d0") (qualified-name "ParameterTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ParameterTest::a"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "ParameterTest::b")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "ParameterTest::c")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "ParameterTest::f")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "ParameterTest::g")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
