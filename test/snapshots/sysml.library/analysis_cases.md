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
  (document "memory://snapshot/analysis_cases.md"
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
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 13 39) (end 13 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 2) (end 20 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 39) (end 20 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 21 2) (end 21 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:c11a3c112b83e4c61ac39761582e70e13eb004e653100913dd189b04f87cfb16") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package defines the base types for analysis cases and related behavioral elements \n\t * in the SysML language.\n\t "))))
    (declaration (id (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::Evaluation") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::evaluations") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Calculations::Calculation") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Cases::Case") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Cases::cases") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase"))) (kind analysis-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * AnalysisCase is the most general class of performances of AnalysisCaseDefinitions. \n\t\t * AnalysisCase is the base class of all AnalysisCaseDefinitions.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Case")) (expressionOperand (reference "ref")))))
    (declaration (id (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::self"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AnalysisCase")) (redefinition (reference "Case::self")))))
    (declaration (id (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))) (kind analysis) (membership (kind feature) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t\t * Other AnalysisCases carried out as part of the performance of this AnalysisCase.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AnalysisCase")))))
    (declaration (id (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::analysisCases"))) (kind analysis) (membership (kind feature) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * analysisCases is the base feature of all AnalysisCaseUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AnalysisCase")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::Evaluation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::evaluations")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Calculations::Calculation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Cases::Case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Cases::cases")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase"))) (kind specialization) (ordinal 0))
      (authored-target "Case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase"))) (kind expressionOperand) (ordinal 0))
      (authored-target "ref")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "AnalysisCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")))))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::self"))) (kind redefinition) (ordinal 0))
      (authored-target "Case::self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))) (kind featureTyping) (ordinal 0))
      (authored-target "AnalysisCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")))))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::analysisCases"))) (kind featureTyping) (ordinal 0))
      (authored-target "AnalysisCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::self"))) (target (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))) (target (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::analysisCases"))) (target (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::analysisCases"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase"))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")))
      (subtype (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::self")) (scopes any))
      (subtype (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases")) (scopes any))
      (subtype (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::analysisCases")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::self")))
      (featured-by (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")))
      (type (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")) (source direct))
      (supertype (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases")))
      (featured-by (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")))
      (type (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")) (source direct))
      (supertype (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::analysisCases")))
      (type (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")) (source direct))
      (supertype (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/analysis_cases.md") (range (start 7 16) (end 7 40)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::Evaluation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_cases.md") (range (start 8 16) (end 8 41)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::evaluations")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_cases.md") (range (start 9 16) (end 9 41)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Calculations::Calculation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_cases.md") (range (start 10 16) (end 10 27)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Cases::Case")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_cases.md") (range (start 11 16) (end 11 28)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (path (named (kind library-package) (name "AnalysisCases")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Cases::cases")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_cases.md") (range (start 13 39) (end 13 43)) (probe (position 13 39))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase"))) (kind specialization) (ordinal 0) (authored-target "Case")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_cases.md") (range (start 20 2) (end 20 5)) (probe (position 20 2))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase"))) (kind expressionOperand) (ordinal 0) (authored-target "ref")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_cases.md") (range (start 20 22) (end 20 34)) (probe (position 20 22))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::self"))) (kind featureTyping) (ordinal 0) (authored-target "AnalysisCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")))))
    )
  )
  (query (document "memory://snapshot/analysis_cases.md") (range (start 20 39) (end 20 49)) (probe (position 20 39))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::self"))) (kind redefinition) (ordinal 0) (authored-target "Case::self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_cases.md") (range (start 23 39) (end 23 51)) (probe (position 23 39))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::subAnalysisCases"))) (kind featureTyping) (ordinal 0) (authored-target "AnalysisCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")))))
    )
  )
  (query (document "memory://snapshot/analysis_cases.md") (range (start 31 35) (end 31 47)) (probe (position 31 35))
    (reference (id (source (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::analysisCases"))) (kind featureTyping) (ordinal 0) (authored-target "AnalysisCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")))))
    )
  )
)
~~~
