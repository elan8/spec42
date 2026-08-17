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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 6) (end 3 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 17 3) (end 17 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 2) (end 20 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 28) (end 20 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:71d66e748beeaff18ed1e468cffbe9f8a327239ede88af42ec57d23a8d8502cb") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::R"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " ... "))))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V::m"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Integer")))))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase"))) (kind verification-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (memberAccessOperand (reference "v::m")) (invocationCallee (reference "VerificationCases::PassIf")))))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase::objective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase::v"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "V")))))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan"))) (kind verification-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::objective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (path (named (kind package) (name "VerificationTest")) (named (kind verification-def) (name "VerificationPlan")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verify-requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (verifyRequirementTarget (reference "r")))))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::v"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "V")))))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::verificationCase"))) (kind verification) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerificationCase")))))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "R")))))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::verificationContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::verificationContext::verificationPlan"))) (kind verification) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerificationPlan")))))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::verificationContext::verificationPlan::v"))) (kind subject) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::vv"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "V")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "v::m")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase"))) (kind invocationCallee) (ordinal 0))
      (authored-target "VerificationCases::PassIf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")))))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (path (named (kind package) (name "VerificationTest")) (named (kind verification-def) (name "VerificationPlan")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0))
      (authored-target "r")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r")))))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")))))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::verificationCase"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerificationCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase")))))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r"))) (kind featureTyping) (ordinal 0))
      (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::R")))))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::verificationContext::verificationPlan"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerificationPlan")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan")))))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::vv"))) (kind featureTyping) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase::v"))) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind verifyRequirementTarget) (source (node (document "memory://snapshot/verification_test.md") (path (named (kind package) (name "VerificationTest")) (named (kind verification-def) (name "VerificationPlan")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_test.md") (path (named (kind package) (name "VerificationTest")) (named (kind verification-def) (name "VerificationPlan")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::v"))) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::verificationCase"))) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::verificationCase"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r"))) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::R"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::verificationContext::verificationPlan"))) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::verificationContext::verificationPlan"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::vv"))) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::vv"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase"))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::R")))
      (subtype (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")))
      (subtype (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase::v")) (scopes any))
      (subtype (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::v")) (scopes any))
      (subtype (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::vv")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V::m")))
      (featured-by (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")))
    )
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase")))
      (subtype (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::verificationCase")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase::objective")))
      (featured-by (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase")))
    )
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase::v")))
      (featured-by (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase")))
      (type (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")) (provenance authored))
      (effective-type (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")) (source direct))
      (supertype (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan")))
      (subtype (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::verificationContext::verificationPlan")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::objective")))
      (featured-by (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan")))
    )
    (declaration (id (node (document "memory://snapshot/verification_test.md") (path (named (kind package) (name "VerificationTest")) (named (kind verification-def) (name "VerificationPlan")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::objective")))
    )
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::v")))
      (featured-by (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan")))
      (type (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")) (provenance authored))
      (effective-type (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")) (source direct))
      (supertype (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::verificationCase")))
      (featured-by (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan")))
      (type (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase")) (source direct))
      (supertype (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r")))
      (type (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::R")) (provenance authored))
      (effective-type (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::R")) (source direct))
      (supertype (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::R")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::verificationContext::verificationPlan")))
      (featured-by (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::verificationContext")))
      (type (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan")) (provenance authored))
      (effective-type (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan")) (source direct))
      (supertype (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::verificationContext::verificationPlan::v")))
      (featured-by (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::verificationContext::verificationPlan")))
    )
    (declaration (id (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::vv")))
      (type (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")) (provenance authored))
      (effective-type (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")) (source direct))
      (supertype (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/verification_test.md") (range (start 3 6) (end 3 27)) (probe (position 3 6))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V::m"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_test.md") (range (start 20 28) (end 20 31)) (probe (position 20 28))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase"))) (kind memberAccessOperand) (ordinal 0) (authored-target "v::m")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_test.md") (range (start 20 2) (end 20 27)) (probe (position 20 2))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase"))) (kind invocationCallee) (ordinal 0) (authored-target "VerificationCases::PassIf")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_test.md") (range (start 15 14) (end 15 15)) (probe (position 15 14))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase::v"))) (kind featureTyping) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")))))
    )
  )
  (query (document "memory://snapshot/verification_test.md") (range (start 27 10) (end 27 11)) (probe (position 27 10))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (path (named (kind package) (name "VerificationTest")) (named (kind verification-def) (name "VerificationPlan")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0) (authored-target "r")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r")))))
    )
  )
  (query (document "memory://snapshot/verification_test.md") (range (start 24 14) (end 24 15)) (probe (position 24 14))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::v"))) (kind featureTyping) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")))))
    )
  )
  (query (document "memory://snapshot/verification_test.md") (range (start 30 34) (end 30 50)) (probe (position 30 34))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan::verificationCase"))) (kind featureTyping) (ordinal 0) (authored-target "VerificationCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationCase")))))
    )
  )
  (query (document "memory://snapshot/verification_test.md") (range (start 12 17) (end 12 18)) (probe (position 12 17))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::r"))) (kind featureTyping) (ordinal 0) (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::R")))))
    )
  )
  (query (document "memory://snapshot/verification_test.md") (range (start 34 34) (end 34 50)) (probe (position 34 34))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::verificationContext::verificationPlan"))) (kind featureTyping) (ordinal 0) (authored-target "VerificationPlan")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::VerificationPlan")))))
    )
  )
  (query (document "memory://snapshot/verification_test.md") (range (start 6 11) (end 6 12)) (probe (position 6 11))
    (reference (id (source (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::vv"))) (kind featureTyping) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_test.md") (qualified-name "VerificationTest::V")))))
    )
  )
)
~~~
