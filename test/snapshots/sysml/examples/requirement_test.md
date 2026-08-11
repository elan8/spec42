# META
~~~ini
description=SysML Example (Simple Tests): RequirementTest
type=file
~~~
# SOURCE
~~~sysml
package RequirementTest {
	constraint def C;
	constraint c : C;
	private import q::**;
	requirement def R {
		assume constraint c1 : C;
		require c;
		doc /* */
    	requirement;
    	requirement def <'1'> A {
    		doc /* Text */
    		subject s;
    	}
	}
	requirement def R1 {
		require constraint c1 :>> c;
	}
	part p;
	part q {
		requirement r : R;
		satisfy r by p;
		assert satisfy r by q;
	}
	
	requirement r1 : R1;
	not satisfy r1 by p;
	assert not satisfy r1 by q;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "requirement_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "invalid_recursive_import")
        (source "semantic")
        (range (start 3 16) (end 3 17))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 17 1) (end 17 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 10) (end 20 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 17) (end 21 18))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 21 22) (end 21 23))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/requirement_test.md")
            (range (start 3 1) (end 3 22))
          )
          (related
            (uri "memory://snapshot/snapshot/requirement_test.md")
            (range (start 18 1) (end 18 76))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "satisfy_target_invalid_kind")
        (source "semantic")
        (range (start 25 13) (end 25 15))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 26 26) (end 26 27))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/requirement_test.md")
            (range (start 3 1) (end 3 22))
          )
          (related
            (uri "memory://snapshot/snapshot/requirement_test.md")
            (range (start 18 1) (end 18 76))
          )
        )
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package RequirementTest {
	constraint def C;
	constraint c : C;
	private import q::**;
	requirement def R {
		assume constraint c1 : C;
		require c;
		doc /* */
    	requirement;
    	requirement def <'1'> A {
    		doc /* Text */
    		subject s;
    	}
	}
	requirement def R1 {
		require constraint c1 :>> c;
	}
	part p;
	part q {
		requirement r : R;
		satisfy r by p;
		assert satisfy r by q;
	}
	
	requirement r1 : R1;
	not satisfy r1 by p;
	assert not satisfy r1 by q;
	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "7c8c2df235a9d35a9e3a33163a79576669b79128edc59d5b21949429d6322ee6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RequirementTest"))) (kind "package") (name "RequirementTest") (declared-name "RequirementTest"))
    (element (id (node (document "d0") (qualified-name "RequirementTest::C"))) (kind "constraint def") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "RequirementTest"))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::R"))) (kind "requirement def") (name "R") (declared-name "R") (parent (node (document "d0") (qualified-name "RequirementTest"))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::R1"))) (kind "requirement def") (name "R1") (declared-name "R1") (parent (node (document "d0") (qualified-name "RequirementTest"))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::R::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "RequirementTest::R"))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::c"))) (kind "constraint") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "RequirementTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "C")))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::p"))) (kind "part") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "RequirementTest"))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::q"))) (kind "import") (name "q") (declared-name "q") (parent (node (document "d0") (qualified-name "RequirementTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "q") (origin Import) (shape Membership) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::q#part"))) (kind "part") (name "q") (declared-name "q") (parent (node (document "d0") (qualified-name "RequirementTest"))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::r1"))) (kind "requirement") (name "r1") (declared-name "r1") (parent (node (document "d0") (qualified-name "RequirementTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "R1")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest"))) (kind satisfySource) (ordinal 0)) (authored-target "r1") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementTest::r1")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest"))) (kind satisfySource) (ordinal 1)) (authored-target "r1") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementTest::r1")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest"))) (kind satisfyTarget) (ordinal 0)) (authored-target "p") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementTest::p")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest"))) (kind satisfyTarget) (ordinal 1)) (authored-target "q") (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "RequirementTest::q")) (node (document "d0") (qualified-name "RequirementTest::q#part")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest::c"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest::q"))) (kind membershipImport) (ordinal 0)) (authored-target "q") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementTest::q#part")))) (import (origin import) (shape membership) (recursive true) (conformance recursive-non-namespace (actual-kind "part"))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest::q#part"))) (kind satisfySource) (ordinal 0)) (authored-target "r") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest::q#part"))) (kind satisfySource) (ordinal 1)) (authored-target "r") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest::q#part"))) (kind satisfyTarget) (ordinal 0)) (authored-target "p") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementTest::p")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest::q#part"))) (kind satisfyTarget) (ordinal 1)) (authored-target "q") (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "RequirementTest::q")) (node (document "d0") (qualified-name "RequirementTest::q#part")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest::r1"))) (kind featureTyping) (ordinal 0)) (authored-target "R1") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementTest::R1")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RequirementTest::c"))) (target (node (document "d0") (qualified-name "RequirementTest::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementTest::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RequirementTest::r1"))) (target (node (document "d0") (qualified-name "RequirementTest::R1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementTest::r1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind satisfy) (source (node (document "d0") (qualified-name "RequirementTest::r1"))) (target (node (document "d0") (qualified-name "RequirementTest::p"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementTest"))) (kind satisfySource) (ordinal 0)) (expression (kind satisfy) (source "r1") (target "p")))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 16) (end 3 17)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "RequirementTest::q"))
        (kind membershipImport) (ordinal 0) (authored-target "q")
        (range (start 3 16) (end 3 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementTest::q#part") (range (start 18 1) (end 18 76)))
        )
      )
    )
    (query (range (start 20 10) (end 20 11)) (probe (position 20 10))
      (reference
        (source (document "d0") (qualified-name "RequirementTest::q#part"))
        (kind satisfySource) (ordinal 0) (authored-target "r")
        (range (start 20 10) (end 20 11))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 15) (end 20 16)) (probe (position 20 15))
      (reference
        (source (document "d0") (qualified-name "RequirementTest::q#part"))
        (kind satisfyTarget) (ordinal 0) (authored-target "p")
        (range (start 20 15) (end 20 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementTest::p") (range (start 17 1) (end 17 8)))
        )
      )
    )
    (query (range (start 21 17) (end 21 18)) (probe (position 21 17))
      (reference
        (source (document "d0") (qualified-name "RequirementTest::q#part"))
        (kind satisfySource) (ordinal 1) (authored-target "r")
        (range (start 21 17) (end 21 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 22) (end 21 23)) (probe (position 21 22))
      (reference
        (source (document "d0") (qualified-name "RequirementTest::q#part"))
        (kind satisfyTarget) (ordinal 1) (authored-target "q")
        (range (start 21 22) (end 21 23))
        (outcome (status ambiguous)
          (target (document "d0") (qualified-name "RequirementTest::q") (range (start 3 1) (end 3 22)))
          (target (document "d0") (qualified-name "RequirementTest::q#part") (range (start 18 1) (end 18 76)))
        )
      )
    )
    (query (range (start 25 19) (end 25 20)) (probe (position 25 19))
      (reference
        (source (document "d0") (qualified-name "RequirementTest"))
        (kind satisfyTarget) (ordinal 0) (authored-target "p")
        (range (start 25 19) (end 25 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementTest::p") (range (start 17 1) (end 17 8)))
        )
      )
    )
    (query (range (start 26 26) (end 26 27)) (probe (position 26 26))
      (reference
        (source (document "d0") (qualified-name "RequirementTest"))
        (kind satisfyTarget) (ordinal 1) (authored-target "q")
        (range (start 26 26) (end 26 27))
        (outcome (status ambiguous)
          (target (document "d0") (qualified-name "RequirementTest::q") (range (start 3 1) (end 3 22)))
          (target (document "d0") (qualified-name "RequirementTest::q#part") (range (start 18 1) (end 18 76)))
        )
      )
    )
    (query (range (start 25 13) (end 25 15)) (probe (position 25 13))
      (reference
        (source (document "d0") (qualified-name "RequirementTest"))
        (kind satisfySource) (ordinal 0) (authored-target "r1")
        (range (start 25 13) (end 25 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementTest::r1") (range (start 24 1) (end 24 21)))
        )
      )
    )
    (query (range (start 26 20) (end 26 22)) (probe (position 26 20))
      (reference
        (source (document "d0") (qualified-name "RequirementTest"))
        (kind satisfySource) (ordinal 1) (authored-target "r1")
        (range (start 26 20) (end 26 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementTest::r1") (range (start 24 1) (end 24 21)))
        )
      )
    )
  )
)
~~~
