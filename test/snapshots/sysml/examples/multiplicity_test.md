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
  (document "multiplicity_test.md"
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "59a2167d72bc5bd96dcbbfb5b3d2d23f50bbe005786fafdf3e49513d8cd98ff5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MultiplicityTest"))) (kind "package") (name "MultiplicityTest") (declared-name "MultiplicityTest"))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::A"))) (kind "attribute def") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "MultiplicityTest"))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::A::i"))) (kind "attribute") (name "i") (declared-name "i") (parent (node (document "d0") (qualified-name "MultiplicityTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer")))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::A::x"))) (kind "attribute") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "MultiplicityTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::P"))) (kind "part def") (name "P") (declared-name "P") (parent (node (document "d0") (qualified-name "MultiplicityTest"))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::a"))) (kind "part") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "MultiplicityTest"))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::b"))) (kind "part") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "MultiplicityTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "P")))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::c"))) (kind "part") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "MultiplicityTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "P")))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::d"))) (kind "part") (name "d") (declared-name "d") (parent (node (document "d0") (qualified-name "MultiplicityTest"))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::e"))) (kind "part") (name "e") (declared-name "e") (parent (node (document "d0") (qualified-name "MultiplicityTest"))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::f"))) (kind "part") (name "f") (declared-name "f") (parent (node (document "d0") (qualified-name "MultiplicityTest"))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::g"))) (kind "part") (name "g") (declared-name "g") (parent (node (document "d0") (qualified-name "MultiplicityTest"))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::n"))) (kind "attribute def") (name "n") (declared-name "n") (parent (node (document "d0") (qualified-name "MultiplicityTest"))) (authored (membership (kind Owning)) (relationships (typing (reference "Integer")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MultiplicityTest::A::i"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MultiplicityTest::A::x"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "MultiplicityTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "MultiplicityTest::b"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (outcome (status resolved) (target (node (document "d0") (qualified-name "MultiplicityTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "MultiplicityTest::c"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (outcome (status resolved) (target (node (document "d0") (qualified-name "MultiplicityTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "MultiplicityTest::n"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MultiplicityTest::A::x"))) (target (node (document "d0") (qualified-name "MultiplicityTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MultiplicityTest::A::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MultiplicityTest::b"))) (target (node (document "d0") (qualified-name "MultiplicityTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MultiplicityTest::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MultiplicityTest::c"))) (target (node (document "d0") (qualified-name "MultiplicityTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MultiplicityTest::c"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "MultiplicityTest::n")) (expression (status "ok") (value (integer 5))))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 6 16) (end 6 17)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "MultiplicityTest::b"))
        (kind featureTyping) (ordinal 0) (authored-target "P")
        (range (start 6 16) (end 6 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MultiplicityTest::P") (range (start 2 1) (end 2 12)))
        )
      )
    )
    (query (range (start 7 10) (end 7 11)) (probe (position 7 10))
      (reference
        (source (document "d0") (qualified-name "MultiplicityTest::c"))
        (kind featureTyping) (ordinal 0) (authored-target "P")
        (range (start 7 10) (end 7 11))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MultiplicityTest::P") (range (start 2 1) (end 2 12)))
        )
      )
    )
  )
)
~~~
