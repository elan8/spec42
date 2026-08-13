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
  (document "memory://snapshot/parameter_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 16) (end 2 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 1) (end 8 88))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:8e305049c91ab2ce85183f91e42ea59caff5c4bbbaa247997f62b8943c736fb6") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::x"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::String"))))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::y"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::b"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::c"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::f"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::g"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")))))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::y"))) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a"))) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/parameter_test.md") (range (start 2 16) (end 2 36)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::x"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/parameter_test.md") (range (start 3 16) (end 3 17)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::y"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")))))
  )
  (query (document "memory://snapshot/parameter_test.md") (range (start 6 15) (end 6 16)) (probe (position 6 15))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")))))
  )
)
~~~
