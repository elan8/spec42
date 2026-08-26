# META
~~~ini
description=Generated viewpoint specialization rules remain explicitly blocked until canonical standard-library anchors are published
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
blocked_by=library-gap-viewpoint-specialization-anchors
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
  (relationship (kind specialization) (source "GeneratedViewpointAnchors::ViewpointDefinition") (target "Views::Viewpoint") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedViewpointAnchors::ViewpointUsage") (target "Views::viewpoints") (provenance implied) (outcome resolved))
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
        (range (start 1 4) (end 1 38))
      )
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 2 4) (end 2 29))
      )
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
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:ca83b54dcdbca20e47cb051fb914861abfd0bcbbcfc73746f0f12aa0e77f9b8b") (contract-version "constructor-expression-specialization-v9") (admitted (standard-library 94)))
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
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
