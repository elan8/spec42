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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "AnalysisCases"))) (name "AnalysisCases") (declared-name "AnalysisCases")
      (contains
        (element (kind "analysis def") (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))) (name "AnalysisCase") (declared-name "AnalysisCase")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase")))))
            (element (kind "analysis") (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::self"))) (name "self") (declared-name "self") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase")))))
            (element (kind "analysis") (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))) (name "subAnalysisCases") (declared-name "subAnalysisCases") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisCases::Calculation"))) (name "Calculation") (declared-name "Calculation"))
        (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisCases::Case"))) (name "Case") (declared-name "Case"))
        (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisCases::Evaluation"))) (name "Evaluation") (declared-name "Evaluation"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "AnalysisCases::_documentation"))) (name ""))
        (element (kind "analysis") (id (node (document "d0") (qualified-name "AnalysisCases::analysisCases"))) (name "analysisCases") (declared-name "analysisCases")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "AnalysisCases::analysisCases::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisCases::cases"))) (name "cases") (declared-name "cases"))
        (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisCases::evaluations"))) (name "evaluations") (declared-name "evaluations"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::_documentation"))) (to (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases::_documentation"))) (to (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AnalysisCases::_documentation"))) (to (node (document "d0") (qualified-name "AnalysisCases"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AnalysisCases::analysisCases::_documentation"))) (to (node (document "d0") (qualified-name "AnalysisCases::analysisCases"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::self"))) (to (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))) (to (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisCases::analysisCases"))) (to (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/analysis_cases.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 1) (end 7 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 1) (end 8 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 1) (end 9 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 1) (end 10 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 1) (end 11 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 13 1) (end 13 509))
      )
    )
  )
)
~~~
