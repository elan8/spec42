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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "90318d6907e828593e533e6147594b83a27f8b799d5222e124ef066ea10c5241") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VerificationTest"))) (kind "package") (name "VerificationTest") (declared-name "VerificationTest") (range (start (line 0) (character 0)) (end (line 0) (character 559))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::R"))) (kind "requirement def") (name "R") (declared-name "R") (range (start (line 8) (character 1)) (end (line 8) (character 39))) (parent (node (document "d0") (qualified-name "VerificationTest"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::R::_documentation"))) (kind "documentation") (name "") (range (start (line 8) (character 1)) (end (line 8) (character 39))) (parent (node (document "d0") (qualified-name "VerificationTest::R"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::V"))) (kind "part def") (name "V") (declared-name "V") (range (start (line 2) (character 1)) (end (line 2) (character 45))) (parent (node (document "d0") (qualified-name "VerificationTest"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationCase"))) (kind "verification def") (name "VerificationCase") (declared-name "VerificationCase") (range (start (line 14) (character 1)) (end (line 14) (character 145))) (parent (node (document "d0") (qualified-name "VerificationTest"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective"))) (kind "objective") (name "objective") (declared-name "objective") (range (start (line 16) (character 2)) (end (line 16) (character 44))) (parent (node (document "d0") (qualified-name "VerificationTest::VerificationCase"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (kind "verified requirement") (name "R") (declared-name "R") (range (start (line 17) (character 3)) (end (line 17) (character 26))) (parent (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective"))) (authored (relationships (typing (reference "R") (range none)) (subject (reference "R") (range none)))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationCase::v"))) (kind "subject") (name "v") (declared-name "v") (range (start (line 15) (character 2)) (end (line 15) (character 16))) (parent (node (document "d0") (qualified-name "VerificationTest::VerificationCase"))) (authored (relationships (typing (reference "V") (range none)))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationPlan"))) (kind "verification def") (name "VerificationPlan") (declared-name "VerificationPlan") (range (start (line 23) (character 1)) (end (line 23) (character 145))) (parent (node (document "d0") (qualified-name "VerificationTest"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective"))) (kind "objective") (name "objective") (declared-name "objective") (range (start (line 26) (character 2)) (end (line 26) (character 30))) (parent (node (document "d0") (qualified-name "VerificationTest::VerificationPlan"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (kind "verified requirement") (name "r") (declared-name "r") (range (start (line 27) (character 3)) (end (line 27) (character 12))) (parent (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective"))) (authored (relationships (typing (reference "r") (range none)) (subject (reference "r") (range none)))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::v"))) (kind "subject") (name "v") (declared-name "v") (range (start (line 24) (character 2)) (end (line 24) (character 16))) (parent (node (document "d0") (qualified-name "VerificationTest::VerificationPlan"))) (authored (relationships (typing (reference "V") (range none)))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::r"))) (kind "requirement") (name "r") (declared-name "r") (range (start (line 12) (character 1)) (end (line 12) (character 19))) (parent (node (document "d0") (qualified-name "VerificationTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "R") (range none)))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::verificationContext"))) (kind "part") (name "verificationContext") (declared-name "verificationContext") (range (start (line 33) (character 1)) (end (line 33) (character 106))) (parent (node (document "d0") (qualified-name "VerificationTest"))))
    (element (id (node (document "d0") (qualified-name "VerificationTest::vv"))) (kind "part") (name "vv") (declared-name "vv") (range (start (line 6) (character 1)) (end (line 6) (character 13))) (parent (node (document "d0") (qualified-name "VerificationTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "V") (range (start (line 6) (character 11)) (end (line 6) (character 12)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (kind featureTyping) (ordinal 0)) (authored-target "R") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "R") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::VerificationCase::v"))) (kind featureTyping) (ordinal 0)) (authored-target "V") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::V")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (kind featureTyping) (ordinal 0)) (authored-target "r") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "r") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::v"))) (kind featureTyping) (ordinal 0)) (authored-target "V") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::V")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::r"))) (kind featureTyping) (ordinal 0)) (authored-target "R") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::R")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationTest::vv"))) (kind featureTyping) (ordinal 0)) (authored-target "V") (range (start (line 6) (character 11)) (end (line 6) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationTest::V")))))
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
