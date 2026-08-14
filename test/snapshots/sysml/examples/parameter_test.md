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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 31) (end 8 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 64) (end 8 85))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:8e305049c91ab2ce85183f91e42ea59caff5c4bbbaa247997f62b8943c736fb6") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::x"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::String"))))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::y"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (path (named (kind package) (name "ParameterTest")) (named (kind calc-def) (name "F")) (anonymous (kind parameter) (ordinal 0)))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Integer"))))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F::p"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A") (direction in))))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F::q"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Integer") (direction in))))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::b"))) (kind attribute-def) (membership (kind owning) (visibility default)) (feature-value (kind bind)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "a")) (invocationCallee (reference "A"))))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::c"))) (kind attribute-def) (membership (kind owning) (visibility default)) (feature-value (kind bind)) (authored (membership (kind owning) (visibility default)) (relationships (invocationCallee (reference "A"))))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::f"))) (kind attribute-def) (membership (kind owning) (visibility default)) (feature-value (kind bind)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "a")) (invocationCallee (reference "F"))))
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::g"))) (kind attribute-def) (membership (kind owning) (visibility default)) (feature-value (kind bind)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "a")) (invocationCallee (reference "F"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")))))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (path (named (kind package) (name "ParameterTest")) (named (kind calc-def) (name "F")) (anonymous (kind parameter) (ordinal 0)))))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")))))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")))))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::b"))) (kind expressionOperand) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a")))))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::b"))) (kind invocationCallee) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")))))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::c"))) (kind invocationCallee) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")))))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::f"))) (kind expressionOperand) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a")))))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::f"))) (kind invocationCallee) (ordinal 0))
      (authored-target "F")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F")))))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::g"))) (kind expressionOperand) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a")))))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::g"))) (kind invocationCallee) (ordinal 0))
      (authored-target "F")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::y"))) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F::p"))) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a"))) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::b"))) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::b"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::b"))) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::b"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::c"))) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::c"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::f"))) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::f"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::f"))) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::f"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::g"))) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::g"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::g"))) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::g"))) (kind invocationCallee) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::b"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::c"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::f"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::g"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A::y")))
      (supertype (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F::p")))
      (supertype (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a")))
      (supertype (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")) (scopes any))
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
  (query (document "memory://snapshot/parameter_test.md") (range (start 8 64) (end 8 85)) (probe (position 8 64))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (path (named (kind package) (name "ParameterTest")) (named (kind calc-def) (name "F")) (anonymous (kind parameter) (ordinal 0)))))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/parameter_test.md") (range (start 8 21) (end 8 22)) (probe (position 8 21))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F::p"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")))))
  )
  (query (document "memory://snapshot/parameter_test.md") (range (start 8 31) (end 8 52)) (probe (position 8 31))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F::q"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/parameter_test.md") (range (start 6 15) (end 6 16)) (probe (position 6 15))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")))))
  )
  (query (document "memory://snapshot/parameter_test.md") (range (start 13 23) (end 13 24)) (probe (position 13 23))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::b"))) (kind expressionOperand) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a")))))
  )
  (query (document "memory://snapshot/parameter_test.md") (range (start 13 19) (end 13 20)) (probe (position 13 19))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::b"))) (kind invocationCallee) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")))))
  )
  (query (document "memory://snapshot/parameter_test.md") (range (start 14 19) (end 14 20)) (probe (position 14 19))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::c"))) (kind invocationCallee) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::A")))))
  )
  (query (document "memory://snapshot/parameter_test.md") (range (start 10 17) (end 10 18)) (probe (position 10 17))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::f"))) (kind expressionOperand) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a")))))
  )
  (query (document "memory://snapshot/parameter_test.md") (range (start 10 15) (end 10 16)) (probe (position 10 15))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::f"))) (kind invocationCallee) (ordinal 0) (authored-target "F")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F")))))
  )
  (query (document "memory://snapshot/parameter_test.md") (range (start 11 28) (end 11 29)) (probe (position 11 28))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::g"))) (kind expressionOperand) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::a")))))
  )
  (query (document "memory://snapshot/parameter_test.md") (range (start 11 15) (end 11 16)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::g"))) (kind invocationCallee) (ordinal 0) (authored-target "F")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameter_test.md") (qualified-name "ParameterTest::F")))))
  )
)
~~~
