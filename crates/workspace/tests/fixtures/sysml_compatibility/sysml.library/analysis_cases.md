# META
~~~ini
description=Standard Library: Systems Library/AnalysisCases
type=file
~~~
# SOURCE
~~~sysml
standard library package AnalysisCases {
	doc
	/*
	 * This package defines the base types for analysis cases and related behavioral elements 
	 * in the SysML language.
	 */

	private import Performances::Evaluation;
	private import Performances::evaluations;
	private import Calculations::Calculation;
	private import Cases::Case;
	private import Cases::cases;
	
	abstract analysis def AnalysisCase :> Case {
		doc
		/*
		 * AnalysisCase is the most general class of performances of AnalysisCaseDefinitions. 
		 * AnalysisCase is the base class of all AnalysisCaseDefinitions.
		 */
	
		ref analysis self : AnalysisCase :>> Case::self;		
		subject subj :>> Case::subj;
		
		abstract analysis subAnalysisCases : AnalysisCase[0..*] :> analysisCases, subcases {
			doc
			/*
			 * Other AnalysisCases carried out as part of the performance of this AnalysisCase.
			 */
		}
	}
	
	abstract analysis analysisCases : AnalysisCase[0..*] nonunique :> cases {
		doc
		/*
		 * analysisCases is the base feature of all AnalysisCaseUsages.
		 */
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Case'
semantic.unresolved_name 'Case::self'
semantic.unresolved_name 'Case::subj'
semantic.unresolved_name 'subcases'
semantic.unresolved_name 'cases'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Case'
semantic.unresolved_name 'Case::self'
semantic.unresolved_name 'Case::subj'
semantic.unresolved_name 'subcases'
semantic.unresolved_name 'cases'
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
KwAbstract,KwAnalysis,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwAnalysis,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwSubject,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwAnalysis,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwAnalysis,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'AnalysisCases'
    (documentation)
    (import_decl private 'Performances::Evaluation')
    (import_decl private 'Performances::evaluations')
    (import_decl private 'Calculations::Calculation')
    (import_decl private 'Cases::Case')
    (import_decl private 'Cases::cases')
    (analysis_case_def abstract 'AnalysisCase' :> 'Case'
      (documentation)
      (sysml_decl ref 'self' : 'AnalysisCase' :>> 'Case::self')
      (sysml_decl 'subj' :>> 'Case::subj')
      (sysml_decl abstract 'subAnalysisCases' : 'AnalysisCase' :> 'analysisCases', 'subcases' multiplicity
        (documentation)))
    (sysml_decl abstract 'analysisCases' : 'AnalysisCase' :> 'cases' multiplicity nonunique
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package AnalysisCases {
    doc /*
	 * This package defines the base types for analysis cases and related behavioral elements 
	 * in the SysML language.
	 */

    private import Performances::Evaluation;
    private import Performances::evaluations;
    private import Calculations::Calculation;
    private import Cases::Case;
    private import Cases::cases;

    abstract analysis def AnalysisCase :> Case {
        doc /*
		 * AnalysisCase is the most general class of performances of AnalysisCaseDefinitions. 
		 * AnalysisCase is the base class of all AnalysisCaseDefinitions.
		 */

        ref analysis self : AnalysisCase :>> Case::self;
        subject subj :>> Case::subj;

        abstract analysis subAnalysisCases : AnalysisCase :> analysisCases, subcases [0..*] {
            doc /*
			 * Other AnalysisCases carried out as part of the performance of this AnalysisCase.
			 */
        }
    }

    abstract analysis analysisCases : AnalysisCase :> cases [0..*] nonunique {
        doc /*
		 * analysisCases is the base feature of all AnalysisCaseUsages.
		 */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'AnalysisCases'
      (documentation)
      (membership_import private -> 'Performances::Evaluation'[unresolved])
      (membership_import private -> 'Performances::evaluations'[unresolved])
      (membership_import private -> 'Calculations::Calculation'[unresolved])
      (membership_import private -> 'Cases::Case'[unresolved])
      (membership_import private -> 'Cases::cases'[unresolved])
      (analysis_case_def abstract 'AnalysisCase' :> 'Case'[unresolved]
        (documentation)
        (analysis_case_usage reference 'self' : 'AnalysisCases::AnalysisCase'[analysis_case_def] :>> 'Case::self'[unresolved])
        (subject_membership in 'subj' :>> 'Case::subj'[unresolved])
        (analysis_case_usage abstract composite 'subAnalysisCases' : 'AnalysisCases::AnalysisCase'[analysis_case_def] :> 'AnalysisCases::analysisCases'[analysis_case_usage] :> 'subcases'[unresolved]
          (multiplicity_range [0..*])
          (documentation)))
      (analysis_case_usage abstract 'analysisCases' : 'AnalysisCases::AnalysisCase'[analysis_case_def] :> 'cases'[unresolved]
        (multiplicity_range [0..*])
        (documentation)))))
~~~
