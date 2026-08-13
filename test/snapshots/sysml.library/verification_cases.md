# META
~~~ini
description=Standard Library: Systems Library/VerificationCases
type=file
~~~
# SOURCE
~~~sysml
standard library package VerificationCases {
	doc
	/*
	 * This package defines the base types for verification cases and related behavioral elements 
	 * in the SysML language.
	 */

	private import Cases::Case;
	private import Cases::cases;
	private import Requirements::RequirementCheck;
	private import ScalarValues::Boolean;
	
	abstract verification def VerificationCase :> Case {
		doc
		/*
		 * VerificationCase is the most general class of performances of VerificationCaseDefinitions. 
		 * VericationCase is the base class of all VerificationCaseDefinitions.
		 */
	
		ref verification self : VerificationCase :>> Case::self;		
		subject subj :>> Case::subj;
		return verdict : VerdictKind :>> result;
		
		objective obj :>> Case::obj {
			subject subj = VerificationCase::subj;
			
			requirement requirementVerifications : RequirementCheck[0..*] :> subrequirements {
				doc
				/*
				 * A record of the evaluations of the RequirementChecks of requirements being verified.
				 */
			}
		}
		
		ref requirement requirementVerifications : RequirementCheck[0..*] = obj.requirementVerifications {
			doc
			/*
			 * Checks on whether the verifiedRequirements of the VerificationCase have been satisfied.
			 */
		}
		
		abstract verification subVerificationCases : VerificationCase[0..*] :> verificationCases, subcases {
			doc
			/*
			 * The subcases of this VerificationCase that are VerificationCaseUsages.
			 */
		}
		
	}
	
	abstract verification verificationCases : VerificationCase[0..*] nonunique  :> cases {
		doc
		/*
		 * verificationCases is the base feature of all VerificationCaseUsages.
		 */
	}
	
	enum def VerdictKind {
		doc
		/*
		 * VerdictKind is an enumeration of the possible results of a VerificationCase.
		 */
	
		pass;
		fail;
		inconclusive;
		error;
	}
	
	calc def PassIf {
		doc
		/*
		 * PassIf returns a pass or fail VerdictKind depending on whether its argument is
		 * true or false.
		 */
	
		in attribute isPassing : Boolean;
		return attribute verdict : VerdictKind = if isPassing? VerdictKind::pass else VerdictKind::fail;
	}
	
	metadata def VerificationMethod {
		doc
		/*
		 * VerificationMethod can be used as metadata annotating a verification case or action.
		 */
	
		attribute kind : VerificationMethodKind[1..*];
	}
	
	enum def VerificationMethodKind {
		doc
		/*
		 * VerificationMethodKind is an enumeration of the standard methods by which verification
		 * can be carried out.
		 */
	
		inspect;
		analyze;
		demo;
		test;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/verification_cases.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 12 47) (end 12 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 19 2) (end 19 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 19 6) (end 19 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 19 19) (end 19 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 19 24) (end 19 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 20 2) (end 20 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 21 2) (end 21 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 23 2) (end 32 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 34 2) (end 34 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 45) (end 34 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 41 2) (end 41 10))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 41 11) (end 41 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 41 24) (end 41 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 41 45) (end 46 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 50 1) (end 55 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 76 27) (end 76 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 77 43) (end 77 97))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:384c8158e574ea87aca9b808965096c93cd9d2b821afefb35a2d06f83d3cf900") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Cases::Case") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Cases::cases") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Requirements::RequirementCheck") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerdictKind"))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::error"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::fail"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::inconclusive"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::pass"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind verification-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Case"))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementCheck"))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerificationMethodKind"))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::analyze"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::demo"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::inspect"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::test"))) (kind enum-literal) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Cases::Case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Cases::cases")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Requirements::RequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerdictKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind specialization) (ordinal 0))
      (authored-target "Case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (kind featureTyping) (ordinal 0))
      (authored-target "RequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerificationMethodKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/verification_cases.md") (range (start 7 16) (end 7 27)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Cases::Case")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 8 16) (end 8 28)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Cases::cases")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 9 16) (end 9 46)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Requirements::RequirementCheck")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 10 16) (end 10 37)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 76 27) (end 76 34)) (probe (position 76 27))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 77 29) (end 77 40)) (probe (position 77 29))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind featureTyping) (ordinal 0) (authored-target "VerdictKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")))))
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 12 47) (end 12 51)) (probe (position 12 47))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind specialization) (ordinal 0) (authored-target "Case")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 34 45) (end 34 61)) (probe (position 34 45))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (kind featureTyping) (ordinal 0) (authored-target "RequirementCheck")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 86 19) (end 86 41)) (probe (position 86 19))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (kind featureTyping) (ordinal 0) (authored-target "VerificationMethodKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind")))))
  )
)
~~~
