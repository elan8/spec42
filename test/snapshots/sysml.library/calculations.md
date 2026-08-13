# META
~~~ini
description=Standard Library: Systems Library/Calculations
type=file
~~~
# SOURCE
~~~sysml
standard library package Calculations {
	doc
	/*
	 * This package defines the base types for calculations and related behavioral elements in the
	 * SysML language.
	 */

	private import Performances::Evaluation;
	private import Performances::evaluations;
	private import Actions::Action;
	private import Actions::actions;
	
	abstract calc def Calculation :> Action, Evaluation {
		doc
		/*
		 * Calculation is the most general class of evaluations of CalculationDefinitions in a
		 * system or part of a system. Calculation is the base class of all CalculationDefinitions.
		 */
	
		ref calc self: Calculation :>> Action::self, Evaluation::self;
		
		abstract calc subcalculations: Calculation :> calculations, subactions {
			doc
			/*
			 * The subactions of this Calculation that are Calculations.
			 */
		}
		
	}
	
	abstract calc calculations: Calculation[0..*] nonunique :> actions, evaluations {
		doc
		/*
		 * calculations is the base Feature for all CalculationUsages.
		 */
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/calculations.md"
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
        (range (start 9 16) (end 9 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 12 34) (end 12 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 12 42) (end 12 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 19 2) (end 19 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 19 6) (end 19 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 21 2) (end 26 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 30 60) (end 30 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 30 69) (end 30 80))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:76edc93a07e743414784c68e1469af2a857523f9f84b6ed941875b990b53c295") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculations.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::Evaluation") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/calculations.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::evaluations") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/calculations.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::Action") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/calculations.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::actions") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")) (specialization (reference "Evaluation"))))
    (declaration (id (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "actions")) (specialization (reference "evaluations"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/calculations.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::Evaluation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::evaluations")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::Action")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::actions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation"))) (kind specialization) (ordinal 1))
      (authored-target "Evaluation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations"))) (kind specialization) (ordinal 0))
      (authored-target "actions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations"))) (kind specialization) (ordinal 1))
      (authored-target "evaluations")
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
  (query (document "memory://snapshot/calculations.md") (range (start 7 16) (end 7 40)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::Evaluation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/calculations.md") (range (start 8 16) (end 8 41)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::evaluations")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/calculations.md") (range (start 9 16) (end 9 31)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::Action")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/calculations.md") (range (start 10 16) (end 10 32)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::actions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/calculations.md") (range (start 12 34) (end 12 40)) (probe (position 12 34))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/calculations.md") (range (start 12 42) (end 12 52)) (probe (position 12 42))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation"))) (kind specialization) (ordinal 1) (authored-target "Evaluation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/calculations.md") (range (start 30 60) (end 30 67)) (probe (position 30 60))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations"))) (kind specialization) (ordinal 0) (authored-target "actions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/calculations.md") (range (start 30 69) (end 30 80)) (probe (position 30 69))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations"))) (kind specialization) (ordinal 1) (authored-target "evaluations")
      (outcome (status unresolved)))
  )
)
~~~
