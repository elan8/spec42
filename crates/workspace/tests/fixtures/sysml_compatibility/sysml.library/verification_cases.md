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
    doc /*
	 * This package defines the base types for verification cases and related behavioral elements 
	 * in the SysML language.
	 */

    private import Cases::Case;
    private import Cases::cases;
    private import Requirements::RequirementCheck;
    private import ScalarValues::Boolean;

    abstract verification def VerificationCase :> Case {
        doc /*
		 * VerificationCase is the most general class of performances of VerificationCaseDefinitions. 
		 * VericationCase is the base class of all VerificationCaseDefinitions.
		 */

        ref verification self : VerificationCase :>> Case::self;
        subject subj :>> Case::subj;
        return verdict : VerdictKind :>> result;

        objective obj :>> Case::obj {
            subject subj = VerificationCase::subj;

            requirement requirementVerifications : RequirementCheck :> subrequirements [0..*] {
                doc /*
				 * A record of the evaluations of the RequirementChecks of requirements being verified.
				 */
            }
        }

        ref requirement requirementVerifications : RequirementCheck [0..*] = obj.requirementVerifications {
            doc /*
			 * Checks on whether the verifiedRequirements of the VerificationCase have been satisfied.
			 */
        }

        abstract verification subVerificationCases : VerificationCase :> verificationCases, subcases [0..*] {
            doc /*
			 * The subcases of this VerificationCase that are VerificationCaseUsages.
			 */
        }
    }

    abstract verification verificationCases : VerificationCase :> cases [0..*] nonunique {
        doc /*
		 * verificationCases is the base feature of all VerificationCaseUsages.
		 */
    }

    enum def VerdictKind {
        doc /*
		 * VerdictKind is an enumeration of the possible results of a VerificationCase.
		 */

        enum pass;
        enum fail;
        enum inconclusive;
        enum error;
    }

    calc def PassIf {
        doc /*
		 * PassIf returns a pass or fail VerdictKind depending on whether its argument is
		 * true or false.
		 */

        in attribute isPassing : Boolean;
        return attribute verdict : VerdictKind = if isPassing? VerdictKind::pass else VerdictKind::fail;
    }

    metadata def VerificationMethod {
        doc /*
		 * VerificationMethod can be used as metadata annotating a verification case or action.
		 */

        attribute kind : VerificationMethodKind [1..*];
    }

    enum def VerificationMethodKind {
        doc /*
		 * VerificationMethodKind is an enumeration of the standard methods by which verification
		 * can be carried out.
		 */

        enum inspect;
        enum analyze;
        enum demo;
        enum test;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'VerificationCases'
      (documentation)
      (membership_import private -> 'Cases::Case'[unresolved])
      (membership_import private -> 'Cases::cases'[unresolved])
      (membership_import private -> 'Requirements::RequirementCheck'[unresolved])
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (verification_case_def abstract 'VerificationCase' :> 'Case'[unresolved]
        (documentation)
        (verification_case_usage reference 'self' : 'VerificationCases::VerificationCase'[verification_case_def] :>> 'Case::self'[unresolved])
        (subject_membership in 'subj' :>> 'Case::subj'[unresolved])
        (return_parameter_membership
          (feature_def out 'verdict' : 'VerificationCases::VerdictKind'[enum_def] :>> 'result'[unresolved]))
        (objective_membership composite 'obj' :>> 'Case::obj'[unresolved]
          (subject_membership in 'subj'
            (feature_value (=)))
          (requirement_usage composite 'requirementVerifications' : 'RequirementCheck'[unresolved] :> 'subrequirements'[unresolved]
            (multiplicity_range [0..*])
            (documentation)))
        (requirement_usage reference 'requirementVerifications' : 'RequirementCheck'[unresolved]
          (multiplicity_range [0..*])
          (feature_value (=))
          (documentation))
        (verification_case_usage abstract composite 'subVerificationCases' : 'VerificationCases::VerificationCase'[verification_case_def] :> 'VerificationCases::verificationCases'[verification_case_usage] :> 'subcases'[unresolved]
          (multiplicity_range [0..*])
          (documentation)))
      (verification_case_usage abstract 'verificationCases' : 'VerificationCases::VerificationCase'[verification_case_def] :> 'cases'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (enum_def 'VerdictKind'
        (documentation)
        (enum_usage composite 'pass')
        (enum_usage composite 'fail')
        (enum_usage composite 'inconclusive')
        (enum_usage composite 'error'))
      (calculation_def 'PassIf'
        (documentation)
        (attribute_usage in 'isPassing' : 'Boolean'[unresolved])
        (return_parameter_membership
          (attribute_usage out 'verdict' : 'VerificationCases::VerdictKind'[enum_def]
            (feature_value (=)))))
      (metadata_def 'VerificationMethod'
        (documentation)
        (attribute_usage composite 'kind' : 'VerificationCases::VerificationMethodKind'[enum_def]
          (multiplicity_range [1..*])))
      (enum_def 'VerificationMethodKind'
        (documentation)
        (enum_usage composite 'inspect')
        (enum_usage composite 'analyze')
        (enum_usage composite 'demo')
        (enum_usage composite 'test')))))
~~~
