# META
~~~ini
description=SysML Example (Simple Tests): VerificationTest
type=file
~~~
# SOURCE
~~~sysml
package VerificationTest {

	part def V {
		m : ScalarValues::Integer;
	}
	
	part vv : V;
	
	requirement def R {
		doc /* ... */
	}
	
	requirement r : R;

	verification def VerificationCase {		
		subject v : V;	
		objective {
			verify requirement : R;
		}
		
		VerificationCases::PassIf(v.m == 0)
	}
	
	verification def VerificationPlan {
		subject v : V;
		
		objective {
			verify r;
		}
		
		verification verificationCase : VerificationCase;
	}
	
	part verificationContext {
		verification verificationPlan : VerificationPlan {
			subject v = vv;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/verification_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 3 2) (end 3 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 15 2) (end 15 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 16 2) (end 18 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 20 2) (end 20 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 24 2) (end 24 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 26 2) (end 28 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 30 2) (end 30 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 30 15) (end 30 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 30 32) (end 30 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 34 2) (end 36 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:71d66e748beeaff18ed1e468cffbe9f8a327239ede88af42ec57d23a8d8502cb") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::R"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase"))) (kind verification-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan"))) (kind verification-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "R"))))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::verificationContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::vv"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "V"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r"))) (kind featureTyping) (ordinal 0))
      (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::R")))))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::vv"))) (kind featureTyping) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r"))) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::R"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::vv"))) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::vv"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/verification_test.md") (range (start 12 17) (end 12 18)) (probe (position 12 17))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r"))) (kind featureTyping) (ordinal 0) (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::R")))))
  )
  (query (document "memory://snapshot/verification_test.md") (range (start 6 11) (end 6 12)) (probe (position 6 11))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::vv"))) (kind featureTyping) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")))))
  )
)
~~~
