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
  (document "memory://snapshot/requirement_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 1) (end 1 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 2 1) (end 2 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 3 16) (end 3 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 5 2) (end 5 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 6 2) (end 6 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 8 5) (end 8 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 9 5) (end 12 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 15 2) (end 15 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 20 2) (end 20 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 21 2) (end 21 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 25 1) (end 25 21))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 26 1) (end 28 0))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:019bc8ad926c802bf2064f60ae8e65cb5ff26b03094888cb9acab2fdfebaf1aa") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_test.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "q") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::R"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::R1"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::p"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::q"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::q::r"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "R"))))
    (declaration (id (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::r1"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "R1"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/requirement_test.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "q")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::q::r"))) (kind featureTyping) (ordinal 0))
      (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::R")))))
    (reference (id (source (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::r1"))) (kind featureTyping) (ordinal 0))
      (authored-target "R1")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::R1")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::q::r"))) (target (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::R"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::q::r"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::r1"))) (target (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::R1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::r1"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/requirement_test.md") (range (start 3 16) (end 3 21)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/requirement_test.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "q")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/requirement_test.md") (range (start 19 18) (end 19 19)) (probe (position 19 18))
    (reference (id (source (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::q::r"))) (kind featureTyping) (ordinal 0) (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::R")))))
  )
  (query (document "memory://snapshot/requirement_test.md") (range (start 24 18) (end 24 20)) (probe (position 24 18))
    (reference (id (source (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::r1"))) (kind featureTyping) (ordinal 0) (authored-target "R1")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_test.md") (qualified-name "RequirementTest::R1")))))
  )
)
~~~
