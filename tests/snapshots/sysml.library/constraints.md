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
  (document "memory://snapshot/constraints.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 7 16) (end 7 47))
      )
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
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 12 44) (end 12 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 43) (end 19 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 74) (end 22 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 29 67) (end 29 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 66) (end 36 82))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:a2662fc9318e275b4298e4f830ae8f2f2576fca8b2566b7bb92fca80941a2e57") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package defines the base types for constraints and related elements in the\n\t * SysML language.\n\t "))))
    (declaration (id (node (document "memory://snapshot/constraints.md") (path (named (kind library-package) (name "Constraints")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::BooleanEvaluation") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/constraints.md") (path (named (kind library-package) (name "Constraints")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::booleanEvaluations") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/constraints.md") (path (named (kind library-package) (name "Constraints")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::trueEvaluations") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/constraints.md") (path (named (kind library-package) (name "Constraints")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::falseEvaluations") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck"))) (kind constraint-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * ConstraintCheck is the most general class for constraint checking. ConstraintCheck is the base\n\t\t * type of all ConstraintDefinitions.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BooleanEvaluation")))))
    (declaration (id (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck::self"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ConstraintCheck")) (redefinition (reference "BooleanEvaluation::self")))))
    (declaration (id (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::assertedConstraintChecks"))) (kind constraint) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t * assertedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be true.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "constraintChecks")) (subsetting (reference "trueEvaluations")))))
    (declaration (id (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks"))) (kind constraint) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * constraintChecks is the base feature of all ConstraintUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ConstraintCheck")) (subsetting (reference "booleanEvaluations")))))
    (declaration (id (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::negatedConstraintChecks"))) (kind constraint) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t * negatedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be false.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "constraintChecks")) (subsetting (reference "falseEvaluations")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/constraints.md") (path (named (kind library-package) (name "Constraints")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::BooleanEvaluation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (path (named (kind library-package) (name "Constraints")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::booleanEvaluations")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (path (named (kind library-package) (name "Constraints")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::trueEvaluations")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (path (named (kind library-package) (name "Constraints")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::falseEvaluations")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck"))) (kind specialization) (ordinal 0))
      (authored-target "BooleanEvaluation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "ConstraintCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")))))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck::self"))) (kind redefinition) (ordinal 0))
      (authored-target "BooleanEvaluation::self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::assertedConstraintChecks"))) (kind subsetting) (ordinal 0))
      (authored-target "constraintChecks")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks")))))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::assertedConstraintChecks"))) (kind subsetting) (ordinal 1))
      (authored-target "trueEvaluations")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks"))) (kind featureTyping) (ordinal 0))
      (authored-target "ConstraintCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")))))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks"))) (kind subsetting) (ordinal 0))
      (authored-target "booleanEvaluations")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::negatedConstraintChecks"))) (kind subsetting) (ordinal 0))
      (authored-target "constraintChecks")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks")))))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::negatedConstraintChecks"))) (kind subsetting) (ordinal 1))
      (authored-target "falseEvaluations")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck::self"))) (target (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::assertedConstraintChecks"))) (target (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::assertedConstraintChecks"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks"))) (target (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::negatedConstraintChecks"))) (target (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::negatedConstraintChecks"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck::self"))) (target (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")))
      (subtype (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck::self")) (scopes any))
      (subtype (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck::self")))
      (featured-by (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")))
      (type (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (provenance authored))
      (effective-type (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source direct))
      (supertype (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::assertedConstraintChecks")))
      (effective-type (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source inherited) (from (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks"))))
      (supertype (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks")))
      (type (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (provenance authored))
      (effective-type (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source direct))
      (supertype (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (subtype (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::assertedConstraintChecks")) (scopes any feature))
      (subtype (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::negatedConstraintChecks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::negatedConstraintChecks")))
      (effective-type (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source inherited) (from (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks"))))
      (supertype (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/constraints.md") (range (start 7 16) (end 7 47)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (path (named (kind library-package) (name "Constraints")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::BooleanEvaluation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/constraints.md") (range (start 8 16) (end 8 48)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (path (named (kind library-package) (name "Constraints")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::booleanEvaluations")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/constraints.md") (range (start 9 16) (end 9 45)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (path (named (kind library-package) (name "Constraints")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::trueEvaluations")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/constraints.md") (range (start 10 16) (end 10 46)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (path (named (kind library-package) (name "Constraints")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::falseEvaluations")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/constraints.md") (range (start 12 44) (end 12 61)) (probe (position 12 44))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck"))) (kind specialization) (ordinal 0) (authored-target "BooleanEvaluation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/constraints.md") (range (start 19 23) (end 19 38)) (probe (position 19 23))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck::self"))) (kind featureTyping) (ordinal 0) (authored-target "ConstraintCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")))))
    )
  )
  (query (document "memory://snapshot/constraints.md") (range (start 19 43) (end 19 66)) (probe (position 19 43))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck::self"))) (kind redefinition) (ordinal 0) (authored-target "BooleanEvaluation::self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/constraints.md") (range (start 29 49) (end 29 65)) (probe (position 29 49))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::assertedConstraintChecks"))) (kind subsetting) (ordinal 0) (authored-target "constraintChecks")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks")))))
    )
  )
  (query (document "memory://snapshot/constraints.md") (range (start 29 67) (end 29 82)) (probe (position 29 67))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::assertedConstraintChecks"))) (kind subsetting) (ordinal 1) (authored-target "trueEvaluations")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/constraints.md") (range (start 22 39) (end 22 54)) (probe (position 22 39))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks"))) (kind featureTyping) (ordinal 0) (authored-target "ConstraintCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::ConstraintCheck")))))
    )
  )
  (query (document "memory://snapshot/constraints.md") (range (start 22 74) (end 22 92)) (probe (position 22 74))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks"))) (kind subsetting) (ordinal 0) (authored-target "booleanEvaluations")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/constraints.md") (range (start 36 48) (end 36 64)) (probe (position 36 48))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::negatedConstraintChecks"))) (kind subsetting) (ordinal 0) (authored-target "constraintChecks")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::constraintChecks")))))
    )
  )
  (query (document "memory://snapshot/constraints.md") (range (start 36 66) (end 36 82)) (probe (position 36 66))
    (reference (id (source (node (document "memory://snapshot/constraints.md") (qualified-name "Constraints::negatedConstraintChecks"))) (kind subsetting) (ordinal 1) (authored-target "falseEvaluations")
      (outcome (status unresolved)))
    )
  )
)
~~~
