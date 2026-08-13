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
        (code "unsupported_case_definition_member")
        (source "semantic")
        (range (start 21 2) (end 21 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_case_definition_member")
        (source "semantic")
        (range (start 21 6) (end 21 10))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_case_definition_member")
        (source "semantic")
        (range (start 21 11) (end 21 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_case_definition_member")
        (source "semantic")
        (range (start 21 16) (end 21 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_case_definition_member")
        (source "semantic")
        (range (start 23 2) (end 28 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_case_definition_member")
        (source "semantic")
        (range (start 30 2) (end 37 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_case_definition_member")
        (source "semantic")
        (range (start 39 2) (end 46 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_case_definition_member")
        (source "semantic")
        (range (start 48 2) (end 53 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_case_definition_member")
        (source "semantic")
        (range (start 55 2) (end 55 10))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_case_definition_member")
        (source "semantic")
        (range (start 55 11) (end 55 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_case_definition_member")
        (source "semantic")
        (range (start 55 16) (end 55 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_case_definition_member")
        (source "semantic")
        (range (start 55 25) (end 60 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 64 1) (end 69 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:fd3a88f3bd062fe1d73e2b30e1b8fb2cf3ab8767091a59849454b4fd8915d379") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Requirements::RequirementCheck") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Calculations::Calculation") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Calculations::calculations") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Parts::Part") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Parts::parts") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind case-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Calculation"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Requirements::RequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Calculations::Calculation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Calculations::calculations")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Parts::Part")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Parts::parts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind specialization) (ordinal 0))
      (authored-target "Calculation")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/cases.md") (range (start 7 16) (end 7 30)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cases.md") (range (start 8 16) (end 8 46)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Requirements::RequirementCheck")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cases.md") (range (start 9 16) (end 9 41)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Calculations::Calculation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cases.md") (range (start 10 16) (end 10 42)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Calculations::calculations")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cases.md") (range (start 11 16) (end 11 27)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Parts::Part")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cases.md") (range (start 12 16) (end 12 28)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/cases.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Parts::parts")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cases.md") (range (start 14 27) (end 14 38)) (probe (position 14 27))
    (reference (id (source (node (document "memory://snapshot/cases.md") (qualified-name "Cases::Case"))) (kind specialization) (ordinal 0) (authored-target "Calculation")
      (outcome (status unresolved)))
  )
)
~~~
