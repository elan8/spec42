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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 7 16) (end 7 27))
      )
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
        (range (start 19 47) (end 19 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 19) (end 20 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 35) (end 21 41))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 45) (end 34 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 92) (end 41 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 50 80) (end 50 85))
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:384c8158e574ea87aca9b808965096c93cd9d2b821afefb35a2d06f83d3cf900") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package defines the base types for verification cases and related behavioral elements \n\t * in the SysML language.\n\t "))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Cases::Case") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Cases::cases") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Requirements::RequirementCheck") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf"))) (kind calc-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * PassIf returns a pass or fail VerdictKind depending on whether its argument is\n\t\t * true or false.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in)))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerdictKind")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "isPassing")) (expressionOperand (reference "VerdictKind::pass")) (expressionOperand (reference "VerdictKind::fail")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind"))) (kind enum-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * VerdictKind is an enumeration of the possible results of a VerificationCase.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::error"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::fail"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::inconclusive"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::pass"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind verification-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * VerificationCase is the most general class of performances of VerificationCaseDefinitions. \n\t\t * VericationCase is the base class of all VerificationCaseDefinitions.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Case")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::requirementVerifications"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t\t * A record of the evaluations of the RequirementChecks of requirements being verified.\n\t\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementCheck")) (subsetting (reference "subrequirements")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::subj"))) (kind subject) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind requirement) (name "obj")) (named (kind subject) (name "subj")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind requirement) (name "obj")) (named (kind subject) (name "subj")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind requirement) (name "obj")) (named (kind subject) (name "subj")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind requirement) (name "obj")) (named (kind subject) (name "subj")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind requirement) (name "obj")) (named (kind subject) (name "subj")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * Checks on whether the verifiedRequirements of the VerificationCase have been satisfied.\n\t\t\t "))) (feature-value (kind bind) (value (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind ref) (name "requirementVerifications")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind ref) (name "requirementVerifications")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementCheck")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind ref) (name "requirementVerifications")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind ref) (name "requirementVerifications")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind ref) (name "requirementVerifications")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::self"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerificationCase")) (redefinition (reference "Case::self")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases"))) (kind verification) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The subcases of this VerificationCase that are VerificationCaseUsages.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerificationCase")) (subsetting (reference "verificationCases")) (subsetting (reference "subcases")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subj"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Case::subj")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::verdict"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerdictKind")) (redefinition (reference "result")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod"))) (kind metadata-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * VerificationMethod can be used as metadata annotating a verification case or action.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerificationMethodKind")))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind"))) (kind enum-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * VerificationMethodKind is an enumeration of the standard methods by which verification\n\t\t * can be carried out.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::analyze"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::demo"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::inspect"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::test"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases"))) (kind verification) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * verificationCases is the base feature of all VerificationCaseUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerificationCase")) (subsetting (reference "cases")))))
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
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "isPassing")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "VerdictKind::pass")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::pass")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 2))
      (authored-target "VerdictKind::fail")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::fail")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind specialization) (ordinal 0))
      (authored-target "Case")
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
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerificationCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::self"))) (kind redefinition) (ordinal 0))
      (authored-target "Case::self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerificationCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases"))) (kind subsetting) (ordinal 0))
      (authored-target "verificationCases")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases"))) (kind subsetting) (ordinal 1))
      (authored-target "subcases")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subj"))) (kind redefinition) (ordinal 0))
      (authored-target "Case::subj")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::verdict"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerdictKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::verdict"))) (kind redefinition) (ordinal 0))
      (authored-target "result")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerificationMethodKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerificationCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases"))) (kind subsetting) (ordinal 0))
      (authored-target "cases")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::pass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::fail"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::self"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::verdict"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::verdict"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::error"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::fail"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::inconclusive"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::pass"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::requirementVerifications"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::subj"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::subj"))) (target (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind requirement) (name "obj")) (named (kind subject) (name "subj")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind requirement) (name "obj")) (named (kind subject) (name "subj")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind requirement) (name "obj")) (named (kind subject) (name "subj")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind ref) (name "requirementVerifications")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind ref) (name "requirementVerifications")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::self"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subj"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::verdict"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod::kind"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethod"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::analyze"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::demo"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::inspect"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind::test"))) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationMethodKind"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
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
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")))
      (subtype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::verdict")) (scopes any))
      (subtype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::verdict")) (scopes any))
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
      (subtype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::self")) (scopes any))
      (subtype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases")) (scopes any))
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
      (supertype (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind requirement) (name "obj")) (named (kind subject) (name "subj")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind requirement) (name "obj")) (named (kind subject) (name "subj")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind requirement) (name "obj")) (named (kind subject) (name "subj")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::subj")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::requirementVerifications")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind ref) (name "requirementVerifications")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind verification-def) (name "VerificationCase")) (named (kind ref) (name "requirementVerifications")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::self")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))
      (type (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")) (source direct))
      (supertype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))
      (type (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")) (source direct))
      (effective-type (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")) (source inherited) (from (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases"))))
      (supertype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")) (scopes any))
      (supertype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subj")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))
    )
    (declaration (id (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::verdict")))
      (featured-by (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))
      (type (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")) (provenance authored))
      (effective-type (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")) (source direct))
      (supertype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")) (scopes any))
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
      (subtype (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases")) (scopes any feature))
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
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "isPassing")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::PassIf::isPassing")))))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 77 57) (end 77 74)) (probe (position 77 57))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "VerdictKind::pass")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::pass")))))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 77 80) (end 77 97)) (probe (position 77 80))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (path (named (kind library-package) (name "VerificationCases")) (named (kind calc-def) (name "PassIf")) (named (kind parameter) (name "verdict")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 2) (authored-target "VerdictKind::fail")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind::fail")))))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 12 47) (end 12 51)) (probe (position 12 47))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (kind specialization) (ordinal 0) (authored-target "Case")
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
  (query (document "memory://snapshot/verification_cases.md") (range (start 19 26) (end 19 42)) (probe (position 19 26))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::self"))) (kind featureTyping) (ordinal 0) (authored-target "VerificationCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 19 47) (end 19 57)) (probe (position 19 47))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::self"))) (kind redefinition) (ordinal 0) (authored-target "Case::self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 41 47) (end 41 63)) (probe (position 41 47))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases"))) (kind featureTyping) (ordinal 0) (authored-target "VerificationCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")))))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 41 73) (end 41 90)) (probe (position 41 73))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases"))) (kind subsetting) (ordinal 0) (authored-target "verificationCases")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases")))))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 41 92) (end 41 100)) (probe (position 41 92))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subVerificationCases"))) (kind subsetting) (ordinal 1) (authored-target "subcases")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 20 19) (end 20 29)) (probe (position 20 19))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::subj"))) (kind redefinition) (ordinal 0) (authored-target "Case::subj")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 21 19) (end 21 30)) (probe (position 21 19))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::verdict"))) (kind featureTyping) (ordinal 0) (authored-target "VerdictKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerdictKind")))))
    )
  )
  (query (document "memory://snapshot/verification_cases.md") (range (start 21 35) (end 21 41)) (probe (position 21 35))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::verdict"))) (kind redefinition) (ordinal 0) (authored-target "result")
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
  (query (document "memory://snapshot/verification_cases.md") (range (start 50 80) (end 50 85)) (probe (position 50 80))
    (reference (id (source (node (document "memory://snapshot/verification_cases.md") (qualified-name "VerificationCases::verificationCases"))) (kind subsetting) (ordinal 0) (authored-target "cases")
      (outcome (status unresolved)))
    )
  )
)
~~~
