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
  (document "verification_cases.md"
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
    )
  )
)
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwVerification,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwVerification,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwSubject,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
KwObjective,Ident,ColonGtGt,Ident,ColonColon,Ident,OpenCurly,
KwSubject,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwRequirement,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwRef,KwRequirement,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Eq,Ident,Dot,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwVerification,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwVerification,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwEnum,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
Ident,Semicolon,
Ident,Semicolon,
Ident,Semicolon,
Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwReturn,KwAttribute,Ident,Colon,Ident,Eq,KwIf,Ident,Question,Ident,ColonColon,Ident,KwElse,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwEnum,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
Ident,Semicolon,
Ident,Semicolon,
Ident,Semicolon,
Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'VerificationCases'
    (documentation)
    (import_decl private 'Cases::Case')
    (import_decl private 'Cases::cases')
    (import_decl private 'Requirements::RequirementCheck')
    (import_decl private 'ScalarValues::Boolean')
    (verification_case_def abstract 'VerificationCase' :> 'Case'
      (documentation)
      (sysml_decl ref 'self' : 'VerificationCase' :>> 'Case::self')
      (sysml_decl 'subj' :>> 'Case::subj')
      (return_member)
      (objective_member)
      (requirement_usage ref 'requirementVerifications' : 'RequirementCheck' multiplicity value
        (documentation))
      (sysml_decl abstract 'subVerificationCases' : 'VerificationCase' :> 'verificationCases', 'subcases' multiplicity
        (documentation)))
    (sysml_decl abstract 'verificationCases' : 'VerificationCase' :> 'cases' multiplicity nonunique
      (documentation))
    (enum_def 'VerdictKind'
      (documentation)
      (enum_value 'pass')
      (enum_value 'fail')
      (enum_value 'inconclusive')
      (enum_value 'error'))
    (calc_def 'PassIf'
      (documentation)
      (attribute_usage in 'isPassing' : 'Boolean')
      (return_member))
    (metadata_def 'VerificationMethod'
      (documentation)
      (attribute_usage 'kind' : 'VerificationMethodKind' multiplicity))
    (enum_def 'VerificationMethodKind'
      (documentation)
      (enum_value 'inspect')
      (enum_value 'analyze')
      (enum_value 'demo')
      (enum_value 'test'))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Case'
semantic.unresolved_name 'Case::self'
semantic.unresolved_name 'Case::subj'
semantic.unresolved_name 'result'
semantic.unresolved_name 'Case::obj'
semantic.unresolved_name 'RequirementCheck'
semantic.unresolved_name 'subrequirements'
semantic.unresolved_name 'RequirementCheck'
semantic.unresolved_name 'subcases'
semantic.unresolved_name 'cases'
semantic.unresolved_name 'Boolean'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Case'
semantic.unresolved_name 'Case::self'
semantic.unresolved_name 'Case::subj'
semantic.unresolved_name 'result'
semantic.unresolved_name 'Case::obj'
semantic.unresolved_name 'RequirementCheck'
semantic.unresolved_name 'subrequirements'
semantic.unresolved_name 'RequirementCheck'
semantic.unresolved_name 'subcases'
semantic.unresolved_name 'cases'
semantic.unresolved_name 'Boolean'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c15a96da9fdc69f36668f313a467dc39b0e653c52fd1f99a2be6310a4c788951") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VerificationCases"))) (kind "package") (name "VerificationCases") (declared-name "VerificationCases") (range (start (line 0) (character 0)) (end (line 0) (character 2488))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 10) (character 1)) (end (line 10) (character 38))) (parent (node (document "d0") (qualified-name "VerificationCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 37))))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::Case"))) (kind "import") (name "Case") (declared-name "Case") (range (start (line 7) (character 1)) (end (line 7) (character 28))) (parent (node (document "d0") (qualified-name "VerificationCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Cases::Case") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 27))))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::PassIf"))) (kind "calc def") (name "PassIf") (declared-name "PassIf") (range (start (line 69) (character 1)) (end (line 69) (character 279))) (parent (node (document "d0") (qualified-name "VerificationCases"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::PassIf::_documentation"))) (kind "documentation") (name "") (range (start (line 69) (character 1)) (end (line 69) (character 279))) (parent (node (document "d0") (qualified-name "VerificationCases::PassIf"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::PassIf::isPassing"))) (kind "in out parameter") (name "isPassing") (declared-name "isPassing") (range (start (line 76) (character 2)) (end (line 76) (character 35))) (parent (node (document "d0") (qualified-name "VerificationCases::PassIf"))) (authored (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::RequirementCheck"))) (kind "import") (name "RequirementCheck") (declared-name "RequirementCheck") (range (start (line 9) (character 1)) (end (line 9) (character 47))) (parent (node (document "d0") (qualified-name "VerificationCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Requirements::RequirementCheck") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 46))))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerdictKind"))) (kind "enum def") (name "VerdictKind") (declared-name "VerdictKind") (range (start (line 57) (character 1)) (end (line 57) (character 168))) (parent (node (document "d0") (qualified-name "VerificationCases"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerdictKind::error"))) (kind "enumerated value") (name "error") (declared-name "error") (range (start (line 66) (character 2)) (end (line 66) (character 7))) (parent (node (document "d0") (qualified-name "VerificationCases::VerdictKind"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerdictKind::fail"))) (kind "enumerated value") (name "fail") (declared-name "fail") (range (start (line 64) (character 2)) (end (line 64) (character 6))) (parent (node (document "d0") (qualified-name "VerificationCases::VerdictKind"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerdictKind::inconclusive"))) (kind "enumerated value") (name "inconclusive") (declared-name "inconclusive") (range (start (line 65) (character 2)) (end (line 65) (character 14))) (parent (node (document "d0") (qualified-name "VerificationCases::VerdictKind"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerdictKind::pass"))) (kind "enumerated value") (name "pass") (declared-name "pass") (range (start (line 63) (character 2)) (end (line 63) (character 6))) (parent (node (document "d0") (qualified-name "VerificationCases::VerdictKind"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerificationCase"))) (kind "verification def") (name "VerificationCase") (declared-name "VerificationCase") (range (start (line 12) (character 1)) (end (line 12) (character 1104))) (parent (node (document "d0") (qualified-name "VerificationCases"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Case") (range (start (line 12) (character 47)) (end (line 12) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerificationCase::_documentation"))) (kind "documentation") (name "") (range (start (line 12) (character 1)) (end (line 12) (character 1104))) (parent (node (document "d0") (qualified-name "VerificationCases::VerificationCase"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerificationCase::obj"))) (kind "objective") (name "obj") (declared-name "obj") (range (start (line 23) (character 2)) (end (line 23) (character 287))) (parent (node (document "d0") (qualified-name "VerificationCases::VerificationCase"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (kind "requirement") (name "requirementVerifications") (declared-name "requirementVerifications") (range (start (line 34) (character 6)) (end (line 34) (character 218))) (parent (node (document "d0") (qualified-name "VerificationCases::VerificationCase"))) (authored (membership (kind Feature)) (relationships (typing (reference "RequirementCheck") (range none)))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerificationCase::requirementVerifications::_documentation"))) (kind "documentation") (name "") (range (start (line 34) (character 6)) (end (line 34) (character 218))) (parent (node (document "d0") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethod"))) (kind "metadata def") (name "VerificationMethod") (declared-name "VerificationMethod") (range (start (line 80) (character 1)) (end (line 80) (character 195))) (parent (node (document "d0") (qualified-name "VerificationCases"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethod::_documentation"))) (kind "documentation") (name "") (range (start (line 80) (character 1)) (end (line 80) (character 195))) (parent (node (document "d0") (qualified-name "VerificationCases::VerificationMethod"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethod::kind"))) (kind "attribute") (name "kind") (declared-name "kind") (range (start (line 86) (character 2)) (end (line 86) (character 48))) (parent (node (document "d0") (qualified-name "VerificationCases::VerificationMethod"))) (authored (membership (kind Feature)) (relationships (typing (reference "VerificationMethodKind") (range none)))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind"))) (kind "enum def") (name "VerificationMethodKind") (declared-name "VerificationMethodKind") (range (start (line 89) (character 1)) (end (line 89) (character 211))) (parent (node (document "d0") (qualified-name "VerificationCases"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind::analyze"))) (kind "enumerated value") (name "analyze") (declared-name "analyze") (range (start (line 97) (character 2)) (end (line 97) (character 9))) (parent (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind::demo"))) (kind "enumerated value") (name "demo") (declared-name "demo") (range (start (line 98) (character 2)) (end (line 98) (character 6))) (parent (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind::inspect"))) (kind "enumerated value") (name "inspect") (declared-name "inspect") (range (start (line 96) (character 2)) (end (line 96) (character 9))) (parent (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind::test"))) (kind "enumerated value") (name "test") (declared-name "test") (range (start (line 99) (character 2)) (end (line 99) (character 6))) (parent (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2488))) (parent (node (document "d0") (qualified-name "VerificationCases"))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::cases"))) (kind "import") (name "cases") (declared-name "cases") (range (start (line 8) (character 1)) (end (line 8) (character 29))) (parent (node (document "d0") (qualified-name "VerificationCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Cases::cases") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 28))))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::verificationCases"))) (kind "verification") (name "verificationCases") (declared-name "verificationCases") (range (start (line 50) (character 1)) (end (line 50) (character 181))) (parent (node (document "d0") (qualified-name "VerificationCases"))) (authored (membership (kind Feature)) (relationships (typing (reference "VerificationCase") (range none)))))
    (element (id (node (document "d0") (qualified-name "VerificationCases::verificationCases::_documentation"))) (kind "documentation") (name "") (range (start (line 50) (character 1)) (end (line 50) (character 181))) (parent (node (document "d0") (qualified-name "VerificationCases::verificationCases"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VerificationCases::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 10) (character 16)) (end (line 10) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VerificationCases::Case"))) (kind membershipImport) (ordinal 0)) (authored-target "Cases::Case") (range (start (line 7) (character 16)) (end (line 7) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VerificationCases::PassIf::isPassing"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationCases::Boolean")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationCases::RequirementCheck"))) (kind membershipImport) (ordinal 0)) (authored-target "Requirements::RequirementCheck") (range (start (line 9) (character 16)) (end (line 9) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VerificationCases::VerificationCase"))) (kind specialization) (ordinal 0)) (authored-target "Case") (range (start (line 12) (character 47)) (end (line 12) (character 51))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationCases::Case")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (kind featureTyping) (ordinal 0)) (authored-target "RequirementCheck") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationCases::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationCases::VerificationMethod::kind"))) (kind featureTyping) (ordinal 0)) (authored-target "VerificationMethodKind") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind")))))
    (reference (id (source (node (document "d0") (qualified-name "VerificationCases::cases"))) (kind membershipImport) (ordinal 0)) (authored-target "Cases::cases") (range (start (line 8) (character 16)) (end (line 8) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VerificationCases::verificationCases"))) (kind featureTyping) (ordinal 0)) (authored-target "VerificationCase") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationCases::VerificationCase")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VerificationCases::PassIf::isPassing"))) (target (node (document "d0") (qualified-name "VerificationCases::Boolean"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationCases::PassIf::isPassing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "VerificationCases::VerificationCase"))) (target (node (document "d0") (qualified-name "VerificationCases::Case"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationCases::VerificationCase"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (target (node (document "d0") (qualified-name "VerificationCases::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VerificationCases::VerificationMethod::kind"))) (target (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationCases::VerificationMethod::kind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VerificationCases::verificationCases"))) (target (node (document "d0") (qualified-name "VerificationCases::VerificationCase"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationCases::verificationCases"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "VerificationCases::PassIf")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
