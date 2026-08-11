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
  (document "verification_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 34 2) (end 34 77))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "90318d6907e828593e533e6147594b83a27f8b799d5222e124ef066ea10c5241") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VerificationTest"))) (kind "package") (name "VerificationTest") (declared-name "VerificationTest"))
    (element (id (node (document "d0") (qualified-name "VerificationTest::R"))) (kind "requirement def") (name "R") (declared-name "R") (parent (node (document "d0") (qualified-name "VerificationTest"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::R::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VerificationTest::R"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::V"))) (kind "part def") (name "V") (declared-name "V") (parent (node (document "d0") (qualified-name "VerificationTest"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationCase"))) (kind "verification def") (name "VerificationCase") (declared-name "VerificationCase") (parent (node (document "d0") (qualified-name "VerificationTest"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective"))) (kind "objective") (name "objective") (declared-name "objective") (parent (node (document "d0") (qualified-name "VerificationTest::VerificationCase"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (kind "verified requirement") (name "R") (declared-name "R") (parent (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective"))) (authored (relationships (typing (reference "R")) (subject (reference "R")))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationCase::v"))) (kind "subject") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "VerificationTest::VerificationCase"))) (authored (relationships (typing (reference "V")))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationPlan"))) (kind "verification def") (name "VerificationPlan") (declared-name "VerificationPlan") (parent (node (document "d0") (qualified-name "VerificationTest"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective"))) (kind "objective") (name "objective") (declared-name "objective") (parent (node (document "d0") (qualified-name "VerificationTest::VerificationPlan"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (kind "verified requirement") (name "r") (declared-name "r") (parent (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective"))) (authored (relationships (typing (reference "r")) (subject (reference "r")))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::v"))) (kind "subject") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "VerificationTest::VerificationPlan"))) (authored (relationships (typing (reference "V")))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::r"))) (kind "requirement") (name "r") (declared-name "r") (parent (node (document "d0") (qualified-name "VerificationTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "R")))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::verificationContext"))) (kind "part") (name "verificationContext") (declared-name "verificationContext") (parent (node (document "d0") (qualified-name "VerificationTest"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::vv"))) (kind "part") (name "vv") (declared-name "vv") (parent (node (document "d0") (qualified-name "VerificationTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "V")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (kind featureTyping) (ordinal 0)) (authored-target "R") (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "R") (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::VerificationCase::v"))) (kind featureTyping) (ordinal 0)) (authored-target "V") (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::V")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (kind featureTyping) (ordinal 0)) (authored-target "r") (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "r") (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::v"))) (kind featureTyping) (ordinal 0)) (authored-target "V") (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::V")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::r"))) (kind featureTyping) (ordinal 0)) (authored-target "R") (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::R")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::vv"))) (kind featureTyping) (ordinal 0)) (authored-target "V") (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::V")))))
  )
  (relationships
    (relationship (kind subject) (source (node (document "d0") (qualified-name "VerificationTest::VerificationCase"))) (target (node (document "d0") (qualified-name "VerificationTest::V"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "VerificationTest::VerificationCase"))) (target (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (target (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (target (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VerificationTest::VerificationCase::v"))) (target (node (document "d0") (qualified-name "VerificationTest::V"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationTest::VerificationCase::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "VerificationTest::VerificationPlan"))) (target (node (document "d0") (qualified-name "VerificationTest::V"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "VerificationTest::VerificationPlan"))) (target (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (target (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (target (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::v"))) (target (node (document "d0") (qualified-name "VerificationTest::V"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VerificationTest::r"))) (target (node (document "d0") (qualified-name "VerificationTest::R"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationTest::r"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VerificationTest::vv"))) (target (node (document "d0") (qualified-name "VerificationTest::V"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationTest::vv"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 6 11) (end 6 12)) (probe (position 6 11))
      (reference
        (source (document "d0") (qualified-name "VerificationTest::vv"))
        (kind featureTyping) (ordinal 0) (authored-target "V")
        (range (start 6 11) (end 6 12))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VerificationTest::V") (range (start 2 1) (end 2 45)))
        )
      )
    )
  )
)
~~~
