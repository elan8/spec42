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
    (element (id (node (document "d0") (qualified-name "AnalysisCases"))) (kind "package") (name "AnalysisCases") (declared-name "AnalysisCases"))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))) (kind "analysis def") (name "AnalysisCase") (declared-name "AnalysisCase") (parent (node (document "d0") (qualified-name "AnalysisCases"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Case")))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::self"))) (kind "analysis") (name "self") (declared-name "self") (parent (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))) (authored (membership (kind Feature)) (relationships (typing (reference "AnalysisCase")))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))) (kind "analysis") (name "subAnalysisCases") (declared-name "subAnalysisCases") (parent (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))) (authored (membership (kind Feature)) (relationships (typing (reference "AnalysisCase")))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::Calculation"))) (kind "import") (name "Calculation") (declared-name "Calculation") (parent (node (document "d0") (qualified-name "AnalysisCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Calculations::Calculation") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::Case"))) (kind "import") (name "Case") (declared-name "Case") (parent (node (document "d0") (qualified-name "AnalysisCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Cases::Case") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::Evaluation"))) (kind "import") (name "Evaluation") (declared-name "Evaluation") (parent (node (document "d0") (qualified-name "AnalysisCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Evaluation") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AnalysisCases"))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::analysisCases"))) (kind "analysis") (name "analysisCases") (declared-name "analysisCases") (parent (node (document "d0") (qualified-name "AnalysisCases"))) (authored (membership (kind Feature)) (relationships (typing (reference "AnalysisCase")))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::analysisCases::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AnalysisCases::analysisCases"))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::cases"))) (kind "import") (name "cases") (declared-name "cases") (parent (node (document "d0") (qualified-name "AnalysisCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Cases::cases") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisCases::evaluations"))) (kind "import") (name "evaluations") (declared-name "evaluations") (parent (node (document "d0") (qualified-name "AnalysisCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::evaluations") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))) (kind specialization) (ordinal 0)) (authored-target "Case") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisCases::Case")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::self"))) (kind featureTyping) (ordinal 0)) (authored-target "AnalysisCase") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))) (kind featureTyping) (ordinal 0)) (authored-target "AnalysisCase") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::Calculation"))) (kind membershipImport) (ordinal 0)) (authored-target "Calculations::Calculation") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::Case"))) (kind membershipImport) (ordinal 0)) (authored-target "Cases::Case") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::Evaluation"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Evaluation") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::analysisCases"))) (kind featureTyping) (ordinal 0)) (authored-target "AnalysisCase") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisCases::AnalysisCase")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::cases"))) (kind membershipImport) (ordinal 0)) (authored-target "Cases::cases") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisCases::evaluations"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::evaluations") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 13 39) (end 13 43)) (probe (position 13 39))
      (reference
        (source (document "d0") (qualified-name "AnalysisCases::AnalysisCase"))
        (kind specialization) (ordinal 0) (authored-target "Case")
        (range (start 13 39) (end 13 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AnalysisCases::Case") (range (start 10 1) (end 10 28)))
        )
      )
    )
    (query (range (start 10 16) (end 10 27)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "AnalysisCases::Case"))
        (kind membershipImport) (ordinal 0) (authored-target "Cases::Case")
        (range (start 10 16) (end 10 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 28)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "AnalysisCases::cases"))
        (kind membershipImport) (ordinal 0) (authored-target "Cases::cases")
        (range (start 11 16) (end 11 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 40)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "AnalysisCases::Evaluation"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::Evaluation")
        (range (start 7 16) (end 7 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 41)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "AnalysisCases::evaluations"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::evaluations")
        (range (start 8 16) (end 8 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 41)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "AnalysisCases::Calculation"))
        (kind membershipImport) (ordinal 0) (authored-target "Calculations::Calculation")
        (range (start 9 16) (end 9 41))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
