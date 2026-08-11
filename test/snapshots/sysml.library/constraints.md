# META
~~~ini
description=Standard Library: Systems Library/Constraints
type=file
~~~
# SOURCE
~~~sysml
standard library package Constraints {
	doc
	/*
	 * This package defines the base types for constraints and related elements in the
	 * SysML language.
	 */

	private import Performances::BooleanEvaluation;
	private import Performances::booleanEvaluations;
	private import Performances::trueEvaluations;
	private import Performances::falseEvaluations;
	
	abstract constraint def ConstraintCheck :> BooleanEvaluation {
		doc
		/*
		 * ConstraintCheck is the most general class for constraint checking. ConstraintCheck is the base
		 * type of all ConstraintDefinitions.
		 */
	
		ref constraint self: ConstraintCheck :>> BooleanEvaluation::self;
	}
	
	abstract constraint constraintChecks: ConstraintCheck[0..*] nonunique :> booleanEvaluations {
		doc
		/*
		 * constraintChecks is the base feature of all ConstraintUsages.
		 */
	}
	
	abstract constraint assertedConstraintChecks :> constraintChecks, trueEvaluations {
		doc
		/*
		 * assertedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be true.
		 */
	}
		
	abstract constraint negatedConstraintChecks :> constraintChecks, falseEvaluations {
		doc
		/*
		 * negatedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be false.
		 */
	}
		
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "constraints.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 46))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Constraints {
    doc
    /*
	 * This package defines the base types for constraints and related elements in the
	 * SysML language.
	 */

    private import Performances::BooleanEvaluation;
    private import Performances::booleanEvaluations;
    private import Performances::trueEvaluations;
    private import Performances::falseEvaluations;

    abstract constraint def ConstraintCheck :> BooleanEvaluation {
        doc
        /*
		 * ConstraintCheck is the most general class for constraint checking. ConstraintCheck is the base
		 * type of all ConstraintDefinitions.
		 */

        ref constraint self: ConstraintCheck :>> BooleanEvaluation::self;
    }

    abstract constraint constraintChecks: ConstraintCheck[0..*] nonunique :> booleanEvaluations {
        doc
        /*
		 * constraintChecks is the base feature of all ConstraintUsages.
		 */
    }

    abstract constraint assertedConstraintChecks :> constraintChecks, trueEvaluations {
        doc
        /*
		 * assertedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be true.
		 */
    }

    abstract constraint negatedConstraintChecks :> constraintChecks, falseEvaluations {
        doc
        /*
		 * negatedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be false.
		 */
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e6a5af70747a8758f782dce8c528980734d11ff20a07939911c26cc754292e20") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Constraints"))) (kind "package") (name "Constraints") (declared-name "Constraints"))
    (element (id (node (document "d0") (qualified-name "Constraints::BooleanEvaluation"))) (kind "import") (name "BooleanEvaluation") (declared-name "BooleanEvaluation") (parent (node (document "d0") (qualified-name "Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::BooleanEvaluation") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))) (kind "constraint def") (name "ConstraintCheck") (declared-name "ConstraintCheck") (parent (node (document "d0") (qualified-name "Constraints"))) (authored (membership (kind Owning)) (relationships (specializes (reference "BooleanEvaluation")))))
    (element (id (node (document "d0") (qualified-name "Constraints::ConstraintCheck::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))))
    (element (id (node (document "d0") (qualified-name "Constraints::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Constraints"))))
    (element (id (node (document "d0") (qualified-name "Constraints::assertedConstraintChecks"))) (kind "constraint") (name "assertedConstraintChecks") (declared-name "assertedConstraintChecks") (parent (node (document "d0") (qualified-name "Constraints"))))
    (element (id (node (document "d0") (qualified-name "Constraints::assertedConstraintChecks::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Constraints::assertedConstraintChecks"))))
    (element (id (node (document "d0") (qualified-name "Constraints::booleanEvaluations"))) (kind "import") (name "booleanEvaluations") (declared-name "booleanEvaluations") (parent (node (document "d0") (qualified-name "Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::booleanEvaluations") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Constraints::constraintChecks"))) (kind "constraint") (name "constraintChecks") (declared-name "constraintChecks") (parent (node (document "d0") (qualified-name "Constraints"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConstraintCheck")))))
    (element (id (node (document "d0") (qualified-name "Constraints::constraintChecks::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Constraints::constraintChecks"))))
    (element (id (node (document "d0") (qualified-name "Constraints::falseEvaluations"))) (kind "import") (name "falseEvaluations") (declared-name "falseEvaluations") (parent (node (document "d0") (qualified-name "Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::falseEvaluations") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Constraints::negatedConstraintChecks"))) (kind "constraint") (name "negatedConstraintChecks") (declared-name "negatedConstraintChecks") (parent (node (document "d0") (qualified-name "Constraints"))))
    (element (id (node (document "d0") (qualified-name "Constraints::negatedConstraintChecks::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Constraints::negatedConstraintChecks"))))
    (element (id (node (document "d0") (qualified-name "Constraints::trueEvaluations"))) (kind "import") (name "trueEvaluations") (declared-name "trueEvaluations") (parent (node (document "d0") (qualified-name "Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::trueEvaluations") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Constraints::BooleanEvaluation"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::BooleanEvaluation") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))) (kind specialization) (ordinal 0)) (authored-target "BooleanEvaluation") (outcome (status resolved) (target (node (document "d0") (qualified-name "Constraints::BooleanEvaluation")))))
    (reference (id (source (node (document "d0") (qualified-name "Constraints::booleanEvaluations"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::booleanEvaluations") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraints::constraintChecks"))) (kind featureTyping) (ordinal 0)) (authored-target "ConstraintCheck") (outcome (status resolved) (target (node (document "d0") (qualified-name "Constraints::ConstraintCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Constraints::falseEvaluations"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::falseEvaluations") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraints::trueEvaluations"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::trueEvaluations") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))) (target (node (document "d0") (qualified-name "Constraints::BooleanEvaluation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Constraints::constraintChecks"))) (target (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Constraints::constraintChecks"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Constraints::ConstraintCheck")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 12 44) (end 12 61)) (probe (position 12 44))
      (reference
        (source (document "d0") (qualified-name "Constraints::ConstraintCheck"))
        (kind specialization) (ordinal 0) (authored-target "BooleanEvaluation")
        (range (start 12 44) (end 12 61))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Constraints::BooleanEvaluation") (range (start 7 1) (end 7 48)))
        )
      )
    )
    (query (range (start 9 16) (end 9 45)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Constraints::trueEvaluations"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::trueEvaluations")
        (range (start 9 16) (end 9 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 46)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Constraints::falseEvaluations"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::falseEvaluations")
        (range (start 10 16) (end 10 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 47)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Constraints::BooleanEvaluation"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::BooleanEvaluation")
        (range (start 7 16) (end 7 47))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 48)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Constraints::booleanEvaluations"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::booleanEvaluations")
        (range (start 8 16) (end 8 48))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
