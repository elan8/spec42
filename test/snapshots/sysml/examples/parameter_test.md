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
    (element (id (node (document "d0") (qualified-name "ParameterTest"))) (kind "package") (name "ParameterTest") (declared-name "ParameterTest"))
    (element (id (node (document "d0") (qualified-name "ParameterTest::A"))) (kind "attribute def") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "ParameterTest"))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::A::x"))) (kind "attribute") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "ParameterTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::A::y"))) (kind "attribute") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "ParameterTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::F"))) (kind "calc def") (name "F") (declared-name "F") (parent (node (document "d0") (qualified-name "ParameterTest"))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::F::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "ParameterTest::F"))) (authored (relationships (typing (reference "ScalarValues::Integer")))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::F::p"))) (kind "in out parameter") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "ParameterTest::F"))) (authored (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::F::q"))) (kind "in out parameter") (name "q") (declared-name "q") (parent (node (document "d0") (qualified-name "ParameterTest::F"))) (authored (relationships (typing (reference "ScalarValues::Integer")))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::a"))) (kind "attribute def") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "ParameterTest"))) (authored (membership (kind Owning)) (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::b"))) (kind "attribute def") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "ParameterTest"))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::c"))) (kind "attribute def") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "ParameterTest"))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::f"))) (kind "attribute def") (name "f") (declared-name "f") (parent (node (document "d0") (qualified-name "ParameterTest"))))
    (element (id (node (document "d0") (qualified-name "ParameterTest::g"))) (kind "attribute def") (name "g") (declared-name "g") (parent (node (document "d0") (qualified-name "ParameterTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ParameterTest::A::x"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ParameterTest::A::y"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "ParameterTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ParameterTest::F::"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarValues::Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ParameterTest::F::p"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "ParameterTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ParameterTest::F::q"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarValues::Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ParameterTest::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "ParameterTest::A")))))
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
# NAVIGATION
~~~sexpr
(navigation
)
~~~
