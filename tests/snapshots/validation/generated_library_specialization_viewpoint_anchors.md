# META
~~~ini
description=The viewpoint specialization anchors resolve through the SYSML21-301 correction (Views::ViewpointCheck / Views::viewpointChecks); the join-node anchor stays blocked until the canonical library publishes Actions::Action::join
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
blocked_by=library-gap-join-node-specialization-anchor
rule_id=sysml-2.0:8.3.26.8:checkViewpointDefinitionSpecialization
rule_id=sysml-2.0:8.3.26.9:checkViewpointUsageSpecialization
rule_id=sysml-2.0:8.3.17.11:checkJoinNodeSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package GeneratedViewpointAnchors {
    viewpoint def ViewpointDefinition;
    viewpoint ViewpointUsage;
    action def Act { join Join; }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "GeneratedViewpointAnchors::ViewpointDefinition") (target "Views::ViewpointCheck") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source "GeneratedViewpointAnchors::ViewpointUsage") (target "Views::viewpointChecks") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedViewpointAnchors::Act::Join") (target "Actions::Action::join") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 3 21) (end 3 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:ca83b54dcdbca20e47cb051fb914861abfd0bcbbcfc73746f0f12aa0e77f9b8b") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors::Act::Join"))) (kind join) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors::ViewpointDefinition"))) (kind viewpoint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors::ViewpointUsage"))) (kind viewpoint) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors::Act"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors::Act::Join"))) (target (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors::Act"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors::ViewpointDefinition"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::ViewpointCheck"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors::ViewpointUsage"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::viewpointChecks"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors::Act")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors::Act::Join")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors::Act")))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors::ViewpointDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::ViewpointCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_viewpoint_anchors.md") (qualified-name "GeneratedViewpointAnchors::ViewpointUsage")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::ViewpointCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::viewpointChecks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::ViewpointCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::viewpointChecks")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
