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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "analysis_cases.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 28))
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "2cd6d0163e28cf17fed402e3e7f3c11dc033944b2a3c9ee76671fe51cdb14825") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AnalysisCases"))) (kind "package") (name "AnalysisCases") (declared-name "AnalysisCases") (range (start (line 0) (character 0)) (end (line 0) (character 1038))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))) (kind "analysis def") (name "AnalysisCase") (declared-name "AnalysisCase") (range (start (line 13) (character 1)) (end (line 13) (character 509))) (parent (node (document "d0") (qualified-name "AnalysisCases"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Case") (range (start (line 13) (character 39)) (end (line 13) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::_documentation"))) (kind "documentation") (name "") (range (start (line 13) (character 1)) (end (line 13) (character 509))) (parent (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::self"))) (kind "analysis") (name "self") (declared-name "self") (range (start (line 20) (character 6)) (end (line 20) (character 50))) (parent (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))) (authored (membership (kind Feature)) (relationships (typing (reference "AnalysisCase") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))) (kind "analysis") (name "subAnalysisCases") (declared-name "subAnalysisCases") (range (start (line 23) (character 2)) (end (line 23) (character 197))) (parent (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))) (authored (membership (kind Feature)) (relationships (typing (reference "AnalysisCase") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases::_documentation"))) (kind "documentation") (name "") (range (start (line 23) (character 2)) (end (line 23) (character 197))) (parent (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::Calculation"))) (kind "import") (name "Calculation") (declared-name "Calculation") (range (start (line 9) (character 1)) (end (line 9) (character 42))) (parent (node (document "d0") (qualified-name "AnalysisCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Calculations::Calculation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 41))))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::Case"))) (kind "import") (name "Case") (declared-name "Case") (range (start (line 10) (character 1)) (end (line 10) (character 28))) (parent (node (document "d0") (qualified-name "AnalysisCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Cases::Case") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 27))))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::Evaluation"))) (kind "import") (name "Evaluation") (declared-name "Evaluation") (range (start (line 7) (character 1)) (end (line 7) (character 41))) (parent (node (document "d0") (qualified-name "AnalysisCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Evaluation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 40))))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1038))) (parent (node (document "d0") (qualified-name "AnalysisCases"))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::analysisCases"))) (kind "analysis") (name "analysisCases") (declared-name "analysisCases") (range (start (line 31) (character 1)) (end (line 31) (character 160))) (parent (node (document "d0") (qualified-name "AnalysisCases"))) (authored (membership (kind Feature)) (relationships (typing (reference "AnalysisCase") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::analysisCases::_documentation"))) (kind "documentation") (name "") (range (start (line 31) (character 1)) (end (line 31) (character 160))) (parent (node (document "d0") (qualified-name "AnalysisCases::analysisCases"))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::cases"))) (kind "import") (name "cases") (declared-name "cases") (range (start (line 11) (character 1)) (end (line 11) (character 29))) (parent (node (document "d0") (qualified-name "AnalysisCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Cases::cases") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 28))))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::evaluations"))) (kind "import") (name "evaluations") (declared-name "evaluations") (range (start (line 8) (character 1)) (end (line 8) (character 42))) (parent (node (document "d0") (qualified-name "AnalysisCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::evaluations") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 41))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))) (kind specialization) (ordinal 0)) (authored-target "Case") (range (start (line 13) (character 39)) (end (line 13) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisCases::Case")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::self"))) (kind featureTyping) (ordinal 0)) (authored-target "AnalysisCase") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))) (kind featureTyping) (ordinal 0)) (authored-target "AnalysisCase") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::Calculation"))) (kind membershipImport) (ordinal 0)) (authored-target "Calculations::Calculation") (range (start (line 9) (character 16)) (end (line 9) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::Case"))) (kind membershipImport) (ordinal 0)) (authored-target "Cases::Case") (range (start (line 10) (character 16)) (end (line 10) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::Evaluation"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Evaluation") (range (start (line 7) (character 16)) (end (line 7) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::analysisCases"))) (kind featureTyping) (ordinal 0)) (authored-target "AnalysisCase") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::cases"))) (kind membershipImport) (ordinal 0)) (authored-target "Cases::cases") (range (start (line 11) (character 16)) (end (line 11) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::evaluations"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::evaluations") (range (start (line 8) (character 16)) (end (line 8) (character 41))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))) (target (node (document "d0") (qualified-name "AnalysisCases::Case"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::self"))) (target (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))) (target (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisCases::analysisCases"))) (target (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisCases::analysisCases"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
