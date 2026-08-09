# META
~~~ini
description=Standard Library: Systems Library/Cases
type=file
~~~
# SOURCE
~~~sysml
standard library package Cases {
	doc
	/*
	 * This package defines the base types for cases and related behavioral elements 
	 * in the SysML language.
	 */

	private import Base::Anything;
	private import Requirements::RequirementCheck;
	private import Calculations::Calculation;
	private import Calculations::calculations;
	private import Parts::Part;
	private import Parts::parts;
	
	abstract case def Case :> Calculation {
		doc
		/*
		 * Case is the most general class of performances of CaseDefinitions. 
		 * Case is the base class of all CaseDefinitions.
		 */
	
		ref case self : Case :>> Calculation::self;
		
		subject subj : Anything[1] {
			doc
			/*
			 * The subject that was investigated by this Case.
			 */
		}
		
		ref part actors : Part[0..*] :> parts {
			doc
			/*
			 * The Parts that fill the role of actors for this Case.
			 * (Note: This is not itself an actor parameter, because specific actor
			 * parameters will be added for specific Cases.)
			 */
		}
		
		objective obj : RequirementCheck[1] {
			doc
			/*
			 * A check of whether the objective RequirementUsage was satisfied for this Case.
			 */
		
			subject subj default Case::result;
		}
		
		return ref result[0..*] {
			doc
			/*
			 * The result determined by the case, which should satisfy the case objective.
			 */
		}
		
		abstract case subcases : Case[0..*] :> cases, subcalculations {
			doc
			/*
			 * Other Cases carried out as part of the performance of this Case.
			 */
		}
	
	}
	
	abstract case cases : Case[0..*] nonunique :> calculations {
		doc
		/*
		 * cases is the base Feature of all CaseUsages.
		 */
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Calculation'
semantic.unresolved_name 'Calculation::self'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'parts'
semantic.unresolved_name 'RequirementCheck'
semantic.unresolved_name 'subcalculations'
semantic.unresolved_name 'calculations'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Calculation'
semantic.unresolved_name 'Calculation::self'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'parts'
semantic.unresolved_name 'RequirementCheck'
semantic.unresolved_name 'subcalculations'
semantic.unresolved_name 'calculations'
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
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwCase,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwCase,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwSubject,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRef,KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwObjective,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
KwSubject,Ident,KwDefault,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwReturn,KwRef,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwCase,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwCase,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Cases'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'Requirements::RequirementCheck')
    (import_decl private 'Calculations::Calculation')
    (import_decl private 'Calculations::calculations')
    (import_decl private 'Parts::Part')
    (import_decl private 'Parts::parts')
    (case_def abstract 'Case' :> 'Calculation'
      (documentation)
      (sysml_decl ref 'self' : 'Case' :>> 'Calculation::self')
      (sysml_decl 'subj' : 'Anything' multiplicity
        (documentation))
      (part_usage ref 'actors' : 'Part' :> 'parts' multiplicity
        (documentation))
      (objective_member)
      (return_member)
      (sysml_decl abstract 'subcases' : 'Case' :> 'cases', 'subcalculations' multiplicity
        (documentation)))
    (sysml_decl abstract 'cases' : 'Case' :> 'calculations' multiplicity nonunique
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package Cases {
    doc /*
	 * This package defines the base types for cases and related behavioral elements 
	 * in the SysML language.
	 */

    private import Base::Anything;
    private import Requirements::RequirementCheck;
    private import Calculations::Calculation;
    private import Calculations::calculations;
    private import Parts::Part;
    private import Parts::parts;

    abstract case def Case :> Calculation {
        doc /*
		 * Case is the most general class of performances of CaseDefinitions. 
		 * Case is the base class of all CaseDefinitions.
		 */

        ref case self : Case :>> Calculation::self;

        subject subj : Anything [1] {
            doc /*
			 * The subject that was investigated by this Case.
			 */
        }

        ref part actors : Part :> parts [0..*] {
            doc /*
			 * The Parts that fill the role of actors for this Case.
			 * (Note: This is not itself an actor parameter, because specific actor
			 * parameters will be added for specific Cases.)
			 */
        }

        objective obj : RequirementCheck [1] {
            doc /*
			 * A check of whether the objective RequirementUsage was satisfied for this Case.
			 */

            subject subj default = Case::result;
        }

        return ref result[0..*] {
			doc
			/*
			 * The result determined by the case, which should satisfy the case objective.
			 */
		}

        abstract case subcases : Case :> cases, subcalculations [0..*] {
            doc /*
			 * Other Cases carried out as part of the performance of this Case.
			 */
        }
    }

    abstract case cases : Case :> calculations [0..*] nonunique {
        doc /*
		 * cases is the base Feature of all CaseUsages.
		 */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Cases'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (membership_import private -> 'Requirements::RequirementCheck'[unresolved])
      (membership_import private -> 'Calculations::Calculation'[unresolved])
      (membership_import private -> 'Calculations::calculations'[unresolved])
      (membership_import private -> 'Parts::Part'[unresolved])
      (membership_import private -> 'Parts::parts'[unresolved])
      (case_def abstract 'Case' :> 'Calculation'[unresolved]
        (documentation)
        (case_usage reference 'self' : 'Cases::Case'[case_def] :>> 'Calculation::self'[unresolved] :> 'Cases::cases'[case_usage][implied])
        (subject_membership in 'subj' : 'Anything'[unresolved]
          (multiplicity_range [1])
          (documentation))
        (part_usage reference 'actors' : 'Part'[unresolved] :> 'parts'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (objective_membership composite 'obj' : 'RequirementCheck'[unresolved]
          (documentation)
          (subject_membership in 'subj'
            (feature_value (default =))))
        (return_parameter_membership
          (reference_usage out reference 'result'
            (multiplicity_range [0..*])
            (documentation)))
        (case_usage abstract composite 'subcases' : 'Cases::Case'[case_def] :> 'Cases::cases'[case_usage] :> 'subcalculations'[unresolved]
          (multiplicity_range [0..*])
          (documentation)))
      (case_usage abstract 'cases' : 'Cases::Case'[case_def] :> 'calculations'[unresolved]
        (multiplicity_range [0..*])
        (documentation)))))
~~~
