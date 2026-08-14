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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 2) (end 19 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 6) (end 19 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 42) (end 26 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 68) (end 26 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 2) (end 41 10))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 11) (end 41 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 76 27) (end 76 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:384c8158e574ea87aca9b808965096c93cd9d2b821afefb35a2d06f83d3cf900") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package defines the base types for verification cases and related behavioral elements \n\t * in the SysML language.\n\t "))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Cases::Case") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Cases::cases") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Requirements::RequirementCheck") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf"))) (kind calc-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * PassIf returns a pass or fail VerdictKind depending on whether its argument is\n\t\t * true or false.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in)))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerdictKind")) (expressionOperand (reference "isPassing")) (expressionOperand (reference "VerdictKind::pass")) (expressionOperand (reference "VerdictKind::fail")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::error"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::fail"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::inconclusive"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::pass"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind verification-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * VerificationCase is the most general class of performances of VerificationCaseDefinitions. \n\t\t * VericationCase is the base class of all VerificationCaseDefinitions.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Case")) (expressionOperand (reference "ref")) (expressionOperand (reference "verification")) (expressionOperand (reference "self")) (expressionOperand (reference "ref")) (expressionOperand (reference "abstract")) (expressionOperand (reference "verification")) (expressionOperand (reference "subVerificationCases")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::requirementVerifications"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t\t * A record of the evaluations of the RequirementChecks of requirements being verified.\n\t\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementCheck")) (subsetting (reference "subrequirements")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::subj"))) (kind subject) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t * Checks on whether the verifiedRequirements of the VerificationCase have been satisfied.\n\t\t\t "))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementCheck")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod"))) (kind metadata-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * VerificationMethod can be used as metadata annotating a verification case or action.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerificationMethodKind")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::analyze"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::demo"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::inspect"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::test"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases"))) (kind verification) (membership (kind feature) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * verificationCases is the base feature of all VerificationCaseUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerificationCase")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Cases::Case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Cases::cases")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Requirements::RequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerdictKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind expressionOperand) (ordinal 0))
      (authored-target "isPassing")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind expressionOperand) (ordinal 1))
      (authored-target "VerdictKind::pass")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::pass")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind expressionOperand) (ordinal 2))
      (authored-target "VerdictKind::fail")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::fail")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind specialization) (ordinal 0))
      (authored-target "Case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind expressionOperand) (ordinal 0))
      (authored-target "ref")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind expressionOperand) (ordinal 1))
      (authored-target "verification")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind expressionOperand) (ordinal 2))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind expressionOperand) (ordinal 3))
      (authored-target "ref")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind expressionOperand) (ordinal 4))
      (authored-target "abstract")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind expressionOperand) (ordinal 5))
      (authored-target "verification")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind expressionOperand) (ordinal 6))
      (authored-target "subVerificationCases")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::requirementVerifications"))) (kind featureTyping) (ordinal 0))
      (authored-target "RequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::requirementVerifications"))) (kind subsetting) (ordinal 0))
      (authored-target "subrequirements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (kind featureTyping) (ordinal 0))
      (authored-target "RequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerificationMethodKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerificationCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::pass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::fail"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf")))
      (type (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")) (provenance authored))
      (effective-type (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")) (source direct))
      (supertype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")))
      (subtype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::error")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::fail")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::inconclusive")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::pass")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))
      (subtype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::requirementVerifications")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::subj")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::requirementVerifications")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod")))
      (type (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind")) (provenance authored))
      (effective-type (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind")) (source direct))
      (supertype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind")))
      (subtype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::analyze")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::demo")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::inspect")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::test")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases")))
      (type (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")) (source direct))
      (supertype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/verification_cases.md") (range (start 7 16) (end 7 27)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Cases::Case")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 8 16) (end 8 28)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Cases::cases")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 9 16) (end 9 46)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Requirements::RequirementCheck")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 10 16) (end 10 37)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 76 27) (end 76 34)) (probe (position 76 27))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 77 29) (end 77 40)) (probe (position 77 29))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind featureTyping) (ordinal 0) (authored-target "VerdictKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")))))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 77 46) (end 77 55)) (probe (position 77 46))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind expressionOperand) (ordinal 0) (authored-target "isPassing")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing")))))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 77 57) (end 77 74)) (probe (position 77 57))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind expressionOperand) (ordinal 1) (authored-target "VerdictKind::pass")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::pass")))))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 77 80) (end 77 97)) (probe (position 77 80))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind expressionOperand) (ordinal 2) (authored-target "VerdictKind::fail")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::fail")))))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 12 47) (end 12 51)) (probe (position 12 47))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind specialization) (ordinal 0) (authored-target "Case")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 19 2) (end 19 5)) (probe (position 19 2))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind expressionOperand) (ordinal 0) (authored-target "ref")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 19 6) (end 19 18)) (probe (position 19 6))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind expressionOperand) (ordinal 1) (authored-target "verification")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 19 19) (end 19 23)) (probe (position 19 19))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind expressionOperand) (ordinal 2) (authored-target "self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 34 2) (end 34 5)) (probe (position 34 2))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind expressionOperand) (ordinal 3) (authored-target "ref")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 41 2) (end 41 10)) (probe (position 41 2))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind expressionOperand) (ordinal 4) (authored-target "abstract")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 41 11) (end 41 23)) (probe (position 41 11))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind expressionOperand) (ordinal 5) (authored-target "verification")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 41 24) (end 41 44)) (probe (position 41 24))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind expressionOperand) (ordinal 6) (authored-target "subVerificationCases")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 26 42) (end 26 58)) (probe (position 26 42))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::requirementVerifications"))) (kind featureTyping) (ordinal 0) (authored-target "RequirementCheck")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 26 68) (end 26 83)) (probe (position 26 68))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::requirementVerifications"))) (kind subsetting) (ordinal 0) (authored-target "subrequirements")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 34 45) (end 34 61)) (probe (position 34 45))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (kind featureTyping) (ordinal 0) (authored-target "RequirementCheck")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 86 19) (end 86 41)) (probe (position 86 19))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (kind featureTyping) (ordinal 0) (authored-target "VerificationMethodKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind")))))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 50 43) (end 50 59)) (probe (position 50 43))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases"))) (kind featureTyping) (ordinal 0) (authored-target "VerificationCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))))
    )
  )
)
~~~
