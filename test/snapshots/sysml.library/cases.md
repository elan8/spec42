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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/cases.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 46))
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
        (range (start 10 16) (end 10 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 14 27) (end 14 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 2) (end 21 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 6) (end 21 10))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 11) (end 21 15))
      )
      (diagnostic
        (severity error)
        (code "recovered_use_case_body_element")
        (source "parser")
        (range (start 21 16) (end 21 45))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 21 16) (end 21 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 17) (end 23 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 20) (end 30 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 34) (end 30 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 18) (end 39 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_case_definition_member")
        (source "semantic")
        (range (start 48 2) (end 53 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 2) (end 55 10))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 11) (end 55 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 16) (end 55 24))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:fd3a88f3bd062fe1d73e2b30e1b8fb2cf3ab8767091a59849454b4fd8915d379") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package defines the base types for cases and related behavioral elements \n\t * in the SysML language.\n\t "))))
    (declaration (id (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Requirements::RequirementCheck") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Calculations::Calculation") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Calculations::calculations") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Parts::Part") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Parts::parts") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind case-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * Case is the most general class of performances of CaseDefinitions. \n\t\t * Case is the base class of all CaseDefinitions.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Calculation")) (expressionOperand (reference "ref")) (expressionOperand (reference "case")) (expressionOperand (reference "self")) (expressionOperand (reference "abstract")) (expressionOperand (reference "case")) (expressionOperand (reference "subcases")))))
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::actors"))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The Parts that fill the role of actors for this Case.\n\t\t\t * (Note: This is not itself an actor parameter, because specific actor\n\t\t\t * parameters will be added for specific Cases.)\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Part")) (subsetting (reference "parts")))))
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::obj"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t * A check of whether the objective RequirementUsage was satisfied for this Case.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementCheck")))))
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::obj::subj"))) (kind subject) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::subj"))) (kind subject) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")))))
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases::cases"))) (kind case) (membership (kind feature) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * cases is the base Feature of all CaseUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Case")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Requirements::RequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Calculations::Calculation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Calculations::calculations")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Parts::Part")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Parts::parts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind specialization) (ordinal 0))
      (authored-target "Calculation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind expressionOperand) (ordinal 0))
      (authored-target "ref")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind expressionOperand) (ordinal 1))
      (authored-target "case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind expressionOperand) (ordinal 2))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind expressionOperand) (ordinal 3))
      (authored-target "abstract")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind expressionOperand) (ordinal 4))
      (authored-target "case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind expressionOperand) (ordinal 5))
      (authored-target "subcases")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::actors"))) (kind featureTyping) (ordinal 0))
      (authored-target "Part")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::actors"))) (kind subsetting) (ordinal 0))
      (authored-target "parts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::obj"))) (kind featureTyping) (ordinal 0))
      (authored-target "RequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::subj"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::cases"))) (kind featureTyping) (ordinal 0))
      (authored-target "Case")
      (outcome (status resolved) (target (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::cases"))) (target (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::cases"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case")))
      (subtype (node (document "memory://snapshot/cases.md") (qualified-name "Cases::cases")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::actors")))
      (featured-by (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case")))
    )
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::obj")))
      (featured-by (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case")))
    )
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::obj::subj")))
      (featured-by (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::obj")))
    )
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::subj")))
      (featured-by (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case")))
    )
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases::cases")))
      (type (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case")) (provenance authored))
      (effective-type (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case")) (source direct))
      (supertype (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/cases.md") (range (start 7 16) (end 7 30)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 8 16) (end 8 46)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Requirements::RequirementCheck")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 9 16) (end 9 41)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Calculations::Calculation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 10 16) (end 10 42)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Calculations::calculations")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 11 16) (end 11 27)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Parts::Part")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 12 16) (end 12 28)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/cases.md") (path (named (kind library-package) (name "Cases")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Parts::parts")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 14 27) (end 14 38)) (probe (position 14 27))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind specialization) (ordinal 0) (authored-target "Calculation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 21 2) (end 21 5)) (probe (position 21 2))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind expressionOperand) (ordinal 0) (authored-target "ref")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 21 6) (end 21 10)) (probe (position 21 6))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind expressionOperand) (ordinal 1) (authored-target "case")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 21 11) (end 21 15)) (probe (position 21 11))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind expressionOperand) (ordinal 2) (authored-target "self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 55 2) (end 55 10)) (probe (position 55 2))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind expressionOperand) (ordinal 3) (authored-target "abstract")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 55 11) (end 55 15)) (probe (position 55 11))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind expressionOperand) (ordinal 4) (authored-target "case")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 55 16) (end 55 24)) (probe (position 55 16))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind expressionOperand) (ordinal 5) (authored-target "subcases")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 30 20) (end 30 24)) (probe (position 30 20))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::actors"))) (kind featureTyping) (ordinal 0) (authored-target "Part")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 30 34) (end 30 39)) (probe (position 30 34))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::actors"))) (kind subsetting) (ordinal 0) (authored-target "parts")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 39 18) (end 39 34)) (probe (position 39 18))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::obj"))) (kind featureTyping) (ordinal 0) (authored-target "RequirementCheck")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 23 17) (end 23 25)) (probe (position 23 17))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case::subj"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cases.md") (range (start 64 23) (end 64 27)) (probe (position 64 23))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::cases"))) (kind featureTyping) (ordinal 0) (authored-target "Case")
      (outcome (status resolved) (target (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case")))))
    )
  )
)
~~~
