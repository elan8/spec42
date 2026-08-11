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
  (document "calculations.md"
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
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "2784623ca7d31d17862567f20e3eef437646a1c3c3622b31436c64eac39de589") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Calculations"))) (kind "package") (name "Calculations") (declared-name "Calculations"))
    (element (id (node (document "d0") (qualified-name "Calculations::Action"))) (kind "import") (name "Action") (declared-name "Action") (parent (node (document "d0") (qualified-name "Calculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::Action") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Calculations::Calculation"))) (kind "calc def") (name "Calculation") (declared-name "Calculation") (parent (node (document "d0") (qualified-name "Calculations"))))
    (element (id (node (document "d0") (qualified-name "Calculations::Calculation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Calculations::Calculation"))))
    (element (id (node (document "d0") (qualified-name "Calculations::Calculation::self"))) (kind "calc") (name "self") (declared-name "self") (parent (node (document "d0") (qualified-name "Calculations::Calculation"))) (authored (membership (kind Feature)) (relationships (typing (reference "Calculation")))))
    (element (id (node (document "d0") (qualified-name "Calculations::Calculation::subcalculations"))) (kind "calc") (name "subcalculations") (declared-name "subcalculations") (parent (node (document "d0") (qualified-name "Calculations::Calculation"))) (authored (membership (kind Feature)) (relationships (typing (reference "Calculation")))))
    (element (id (node (document "d0") (qualified-name "Calculations::Calculation::subcalculations::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Calculations::Calculation::subcalculations"))))
    (element (id (node (document "d0") (qualified-name "Calculations::Evaluation"))) (kind "import") (name "Evaluation") (declared-name "Evaluation") (parent (node (document "d0") (qualified-name "Calculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Evaluation") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Calculations::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Calculations"))))
    (element (id (node (document "d0") (qualified-name "Calculations::actions"))) (kind "import") (name "actions") (declared-name "actions") (parent (node (document "d0") (qualified-name "Calculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::actions") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Calculations::calculations"))) (kind "calc def") (name "calculations") (declared-name "calculations") (parent (node (document "d0") (qualified-name "Calculations"))))
    (element (id (node (document "d0") (qualified-name "Calculations::calculations::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Calculations::calculations"))))
    (element (id (node (document "d0") (qualified-name "Calculations::evaluations"))) (kind "import") (name "evaluations") (declared-name "evaluations") (parent (node (document "d0") (qualified-name "Calculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::evaluations") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Calculations::Action"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::Action") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculations::Calculation::self"))) (kind featureTyping) (ordinal 0)) (authored-target "Calculation") (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculations::Calculation")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculations::Calculation::subcalculations"))) (kind featureTyping) (ordinal 0)) (authored-target "Calculation") (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculations::Calculation")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculations::Evaluation"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Evaluation") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculations::actions"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::actions") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculations::evaluations"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::evaluations") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculations::Calculation::self"))) (target (node (document "d0") (qualified-name "Calculations::Calculation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculations::Calculation::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculations::Calculation::subcalculations"))) (target (node (document "d0") (qualified-name "Calculations::Calculation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculations::Calculation::subcalculations"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Calculations::Calculation")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 9 16) (end 9 31)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Calculations::Action"))
        (kind membershipImport) (ordinal 0) (authored-target "Actions::Action")
        (range (start 9 16) (end 9 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 32)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Calculations::actions"))
        (kind membershipImport) (ordinal 0) (authored-target "Actions::actions")
        (range (start 10 16) (end 10 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 40)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Calculations::Evaluation"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::Evaluation")
        (range (start 7 16) (end 7 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 41)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Calculations::evaluations"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::evaluations")
        (range (start 8 16) (end 8 41))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
