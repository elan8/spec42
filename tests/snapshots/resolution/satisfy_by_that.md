# META
~~~ini
description=satisfy requirement … by that resolves that as the featuring instance (things::that)
specification=OMG SysML 2.0 (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Beta2/PDF
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package Remaining {
    view def Box {
        satisfy requirement viewpointConformance by that;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/satisfy_by_that.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/satisfy_by_that.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:67cf84e90bc3f6d9bc889ab2c8c736f358a4e51fc0b00b04d2455b4179881a69") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining::Box"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining::Box::viewpointConformance"))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfyTarget (reference "that")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining::Box::viewpointConformance"))) (kind satisfyTarget) (ordinal 0))
      (authored-target "that")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that")))))
  )
  (relationships
    (relationship (kind satisfyTarget) (source (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining::Box::viewpointConformance"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining::Box::viewpointConformance"))) (kind satisfyTarget) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining::Box"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining::Box::viewpointConformance"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining::Box::viewpointConformance"))) (target (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining::Box"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining::Box::viewpointConformance"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining::Box")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining::Box::viewpointConformance")))
      (featured-by (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining::Box")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::assertedConstraintChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::trueEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/satisfy_by_that.md") (range (start 2 52) (end 2 56)) (probe (position 2 52))
    (reference (id (source (node (document "memory://snapshot/satisfy_by_that.md") (qualified-name "Remaining::Box::viewpointConformance"))) (kind satisfyTarget) (ordinal 0) (authored-target "that")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that")))))
    )
  )
)
~~~
