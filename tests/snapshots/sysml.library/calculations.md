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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 7 16) (end 7 40))
      )
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 33) (end 19 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 47) (end 19 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 62) (end 21 72))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:76edc93a07e743414784c68e1469af2a857523f9f84b6ed941875b990b53c295") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package defines the base types for calculations and related behavioral elements in the\n\t * SysML language.\n\t "))))
    (declaration (id (node (document "memory://snapshot/calculations.md") (path (named (kind library-package) (name "Calculations")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::Evaluation") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/calculations.md") (path (named (kind library-package) (name "Calculations")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::evaluations") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/calculations.md") (path (named (kind library-package) (name "Calculations")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::Action") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/calculations.md") (path (named (kind library-package) (name "Calculations")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::actions") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation"))) (kind calc-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Calculation is the most general class of evaluations of CalculationDefinitions in a\n\t\t * system or part of a system. Calculation is the base class of all CalculationDefinitions.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")) (specialization (reference "Evaluation")))))
    (declaration (id (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::self"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Calculation")) (redefinition (reference "Action::self")) (redefinition (reference "Evaluation::self")))))
    (declaration (id (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations"))) (kind calc) (membership (kind feature) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t\t * The subactions of this Calculation that are Calculations.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Calculation")) (subsetting (reference "calculations")) (subsetting (reference "subactions")))))
    (declaration (id (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations"))) (kind calc-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * calculations is the base Feature for all CalculationUsages.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "actions")) (specialization (reference "evaluations")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/calculations.md") (path (named (kind library-package) (name "Calculations")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::Evaluation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (path (named (kind library-package) (name "Calculations")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::evaluations")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (path (named (kind library-package) (name "Calculations")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::Action")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (path (named (kind library-package) (name "Calculations")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::actions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation"))) (kind specialization) (ordinal 1))
      (authored-target "Evaluation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "Calculation")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation")))))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::self"))) (kind redefinition) (ordinal 0))
      (authored-target "Action::self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::self"))) (kind redefinition) (ordinal 1))
      (authored-target "Evaluation::self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations"))) (kind featureTyping) (ordinal 0))
      (authored-target "Calculation")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation")))))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations"))) (kind subsetting) (ordinal 0))
      (authored-target "calculations")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations")))))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations"))) (kind subsetting) (ordinal 1))
      (authored-target "subactions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations"))) (kind specialization) (ordinal 0))
      (authored-target "actions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations"))) (kind specialization) (ordinal 1))
      (authored-target "evaluations")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::self"))) (target (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations"))) (target (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations"))) (target (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::self"))) (target (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations"))) (target (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation")))
      (subtype (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::self")) (scopes any))
      (subtype (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::self")))
      (featured-by (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation")))
      (type (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation")) (provenance authored))
      (effective-type (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation")) (source direct))
      (supertype (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations")))
      (featured-by (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation")))
      (type (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation")) (provenance authored))
      (effective-type (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation")) (source direct))
      (supertype (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any))
      (supertype (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations")))
      (subtype (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/calculations.md") (range (start 7 16) (end 7 40)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (path (named (kind library-package) (name "Calculations")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::Evaluation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/calculations.md") (range (start 8 16) (end 8 41)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (path (named (kind library-package) (name "Calculations")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::evaluations")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/calculations.md") (range (start 9 16) (end 9 31)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (path (named (kind library-package) (name "Calculations")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::Action")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/calculations.md") (range (start 10 16) (end 10 32)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (path (named (kind library-package) (name "Calculations")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::actions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/calculations.md") (range (start 12 34) (end 12 40)) (probe (position 12 34))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/calculations.md") (range (start 12 42) (end 12 52)) (probe (position 12 42))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation"))) (kind specialization) (ordinal 1) (authored-target "Evaluation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/calculations.md") (range (start 19 17) (end 19 28)) (probe (position 19 17))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::self"))) (kind featureTyping) (ordinal 0) (authored-target "Calculation")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation")))))
    )
  )
  (query (document "memory://snapshot/calculations.md") (range (start 19 33) (end 19 45)) (probe (position 19 33))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::self"))) (kind redefinition) (ordinal 0) (authored-target "Action::self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/calculations.md") (range (start 19 47) (end 19 63)) (probe (position 19 47))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::self"))) (kind redefinition) (ordinal 1) (authored-target "Evaluation::self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/calculations.md") (range (start 21 33) (end 21 44)) (probe (position 21 33))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations"))) (kind featureTyping) (ordinal 0) (authored-target "Calculation")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation")))))
    )
  )
  (query (document "memory://snapshot/calculations.md") (range (start 21 48) (end 21 60)) (probe (position 21 48))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations"))) (kind subsetting) (ordinal 0) (authored-target "calculations")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations")))))
    )
  )
  (query (document "memory://snapshot/calculations.md") (range (start 21 62) (end 21 72)) (probe (position 21 62))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::Calculation::subcalculations"))) (kind subsetting) (ordinal 1) (authored-target "subactions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/calculations.md") (range (start 30 60) (end 30 67)) (probe (position 30 60))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations"))) (kind specialization) (ordinal 0) (authored-target "actions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/calculations.md") (range (start 30 69) (end 30 80)) (probe (position 30 69))
    (reference (id (source (node (document "memory://snapshot/calculations.md") (qualified-name "Calculations::calculations"))) (kind specialization) (ordinal 1) (authored-target "evaluations")
      (outcome (status unresolved)))
    )
  )
)
~~~
