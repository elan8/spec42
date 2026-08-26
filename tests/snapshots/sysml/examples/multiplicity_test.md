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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/multiplicity_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 15) (end 3 36))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 5 1) (end 5 11))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 8 1) (end 8 11))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 10 1) (end 10 11))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 11 1) (end 11 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 12 1) (end 12 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 15) (end 15 36))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:deba5cfab2f49ffa063a5603afc4584eb30210640721e7a314122342e88550a4") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A::i"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Integer")))))
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A::x"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower expression) (upper expression))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::P"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::a"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::b"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::c"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::d"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::e"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower expression) (upper expression))))
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::f"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower expression) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::g"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper expression))))
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::n"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/multiplicity_test.md") (path (named (kind package) (name "MultiplicityTest")) (named (kind attribute) (name "n")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/multiplicity_test.md") (path (named (kind package) (name "MultiplicityTest")) (named (kind attribute) (name "n")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Integer")))))
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (path (named (kind package) (name "MultiplicityTest")) (named (kind attribute) (name "n")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/multiplicity_test.md") (path (named (kind package) (name "MultiplicityTest")) (named (kind attribute) (name "n")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (path (named (kind package) (name "MultiplicityTest")) (named (kind attribute) (name "n")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A::i"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A")))))
    (reference (id (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::P")))))
    (reference (id (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::P")))))
    (reference (id (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::n"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A::x"))) (target (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::b"))) (target (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::c"))) (target (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A::i"))) (target (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A::x"))) (target (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/multiplicity_test.md") (path (named (kind package) (name "MultiplicityTest")) (named (kind attribute) (name "n")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/multiplicity_test.md") (path (named (kind package) (name "MultiplicityTest")) (named (kind attribute) (name "n")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/multiplicity_test.md") (path (named (kind package) (name "MultiplicityTest")) (named (kind attribute) (name "n")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 5)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A")))
      (subtype (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A::x")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A::i")))
      (featured-by (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A")))
    )
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A::x")))
      (featured-by (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A")))
      (type (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A")) (source direct))
      (supertype (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::P")))
      (subtype (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::b")) (scopes any))
      (subtype (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::c")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::b")))
      (type (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::P")) (source direct))
      (supertype (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::P")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::c")))
      (type (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::P")) (source direct))
      (supertype (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::P")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/multiplicity_test.md") (path (named (kind package) (name "MultiplicityTest")) (named (kind attribute) (name "n")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/multiplicity_test.md") (path (named (kind package) (name "MultiplicityTest")) (named (kind attribute) (name "n")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/multiplicity_test.md") (range (start 15 15) (end 15 36)) (probe (position 15 15))
    (reference (id (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A::i"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/multiplicity_test.md") (range (start 16 16) (end 16 17)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A::x"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::A")))))
    )
  )
  (query (document "memory://snapshot/multiplicity_test.md") (range (start 6 16) (end 6 17)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::b"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::P")))))
    )
  )
  (query (document "memory://snapshot/multiplicity_test.md") (range (start 7 10) (end 7 11)) (probe (position 7 10))
    (reference (id (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::c"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::P")))))
    )
  )
  (query (document "memory://snapshot/multiplicity_test.md") (range (start 3 15) (end 3 36)) (probe (position 3 15))
    (reference (id (source (node (document "memory://snapshot/multiplicity_test.md") (qualified-name "MultiplicityTest::n"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    )
  )
)
~~~
