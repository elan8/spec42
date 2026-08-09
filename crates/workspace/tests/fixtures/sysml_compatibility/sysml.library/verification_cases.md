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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "VerificationCases"))) (name "VerificationCases") (declared-name "VerificationCases")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "VerificationCases::Boolean"))) (name "Boolean") (declared-name "Boolean"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VerificationCases::Case"))) (name "Case") (declared-name "Case"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VerificationCases::PassIf"))) (name "PassIf") (declared-name "PassIf")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "VerificationCases::PassIf::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::PassIf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VerificationCases::PassIf::isPassing"))) (name "isPassing") (declared-name "isPassing") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::PassIf")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "VerificationCases::RequirementCheck"))) (name "RequirementCheck") (declared-name "RequirementCheck"))
        (element (kind "enum def") (id (node (document "d0") (qualified-name "VerificationCases::VerdictKind"))) (name "VerdictKind") (declared-name "VerdictKind")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "VerificationCases::VerdictKind::error"))) (name "error") (declared-name "error") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerdictKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "VerificationCases::VerdictKind::fail"))) (name "fail") (declared-name "fail") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerdictKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "VerificationCases::VerdictKind::inconclusive"))) (name "inconclusive") (declared-name "inconclusive") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerdictKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "VerificationCases::VerdictKind::pass"))) (name "pass") (declared-name "pass") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerdictKind")))))
          )
        )
        (element (kind "verification def") (id (node (document "d0") (qualified-name "VerificationCases::VerificationCase"))) (name "VerificationCase") (declared-name "VerificationCase")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "VerificationCases::VerificationCase::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerificationCase")))))
            (element (kind "objective") (id (node (document "d0") (qualified-name "VerificationCases::VerificationCase::obj"))) (name "obj") (declared-name "obj") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerificationCase")))))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (name "requirementVerifications") (declared-name "requirementVerifications") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerificationCase"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "VerificationCases::VerificationCase::requirementVerifications::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerificationCase")))))
              )
            )
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethod"))) (name "VerificationMethod") (declared-name "VerificationMethod")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethod::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerificationMethod")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethod::kind"))) (name "kind") (declared-name "kind") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerificationMethod")))))
          )
        )
        (element (kind "enum def") (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind"))) (name "VerificationMethodKind") (declared-name "VerificationMethodKind")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind::analyze"))) (name "analyze") (declared-name "analyze") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind::demo"))) (name "demo") (declared-name "demo") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind::inspect"))) (name "inspect") (declared-name "inspect") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind::test"))) (name "test") (declared-name "test") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "VerificationCases::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "VerificationCases::cases"))) (name "cases") (declared-name "cases"))
        (element (kind "verification") (id (node (document "d0") (qualified-name "VerificationCases::verificationCases"))) (name "verificationCases") (declared-name "verificationCases")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "VerificationCases::verificationCases::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VerificationCases::VerificationCase")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VerificationCases::PassIf::_documentation"))) (to (node (document "d0") (qualified-name "VerificationCases::PassIf"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VerificationCases::VerificationCase::_documentation"))) (to (node (document "d0") (qualified-name "VerificationCases::VerificationCase"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VerificationCases::VerificationCase::requirementVerifications::_documentation"))) (to (node (document "d0") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VerificationCases::VerificationMethod::_documentation"))) (to (node (document "d0") (qualified-name "VerificationCases::VerificationMethod"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VerificationCases::_documentation"))) (to (node (document "d0") (qualified-name "VerificationCases"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VerificationCases::verificationCases::_documentation"))) (to (node (document "d0") (qualified-name "VerificationCases::verificationCases"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VerificationCases::VerificationMethod::kind"))) (to (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VerificationCases::verificationCases"))) (to (node (document "d0") (qualified-name "VerificationCases::VerificationCase"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::PassIf"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerdictKind"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerdictKind::error"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerdictKind::fail"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerdictKind::inconclusive"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerdictKind::pass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerificationCase"))) (status missing-prerequisite) (target "VerificationCases::VerificationCase"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerificationCase::obj"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerificationCase::requirementVerifications"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerificationMethod"))) (status missing-prerequisite) (target "Metadata::MetadataItem"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerificationMethod::kind"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind::analyze"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind::demo"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind::inspect"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::VerificationMethodKind::test"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VerificationCases::verificationCases"))) (status missing-prerequisite) (target "VerificationCases::verificationCases"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/verification_cases.md"
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
        (range (start 12 1) (end 12 1104))
      )
      (diagnostic
        (severity warning)
        (code "objective_binding_unresolved")
        (source "semantic")
        (range (start 23 2) (end 23 287))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 6) (end 34 218))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 76 2) (end 76 35))
      )
    )
  )
)
~~~
