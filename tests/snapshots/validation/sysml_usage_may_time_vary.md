# META
~~~ini
description=Usage mayTimeVary retains its required effective occurrence, library-specialization, and portion fact boundary
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.6.4:deriveUsageMayTimeVary
libraries=standard
~~~
# SOURCE
~~~sysml
package Model {
    part def Owner specializes Occurrences::Occurrence {
        part ordinary;
        snapshot occurrence slice;
        action behavior : Actions::Action;
        ref selfLink : Links::SelfLink;
        ref happensLink : Occurrences::HappensLink;
    }
    part packageOwned;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageMayTimeVary") (source "Model::Owner::ordinary") (outcome true))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageMayTimeVary") (source "Model::Owner::slice") (outcome false))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageMayTimeVary") (source "Model::Owner::behavior") (outcome false))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageMayTimeVary") (source "Model::Owner::selfLink") (outcome false))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageMayTimeVary") (source "Model::Owner::happensLink") (outcome false))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageMayTimeVary") (source "Model::packageOwned") (outcome false)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_may_time_vary.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 8) (end 2 22))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 8 4) (end 8 22))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:0b471401af4dcdc4355382d3360178d4b7e9d829833b3b1225b8c69062adf8f9") (contract-version "lossless-publication-completeness-v3") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Occurrences::Occurrence")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::behavior"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Actions::Action")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::happensLink"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrences::HappensLink")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::ordinary"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::selfLink"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Links::SelfLink")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::slice"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::packageOwned"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner"))) (kind specialization) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::behavior"))) (kind featureTyping) (ordinal 0))
      (authored-target "Actions::Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::happensLink"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrences::HappensLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::selfLink"))) (kind featureTyping) (ordinal 0))
      (authored-target "Links::SelfLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::SelfLink")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::behavior"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::behavior"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::happensLink"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::happensLink"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::selfLink"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::SelfLink"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::selfLink"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::behavior"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::behavior"))) (target (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::behavior"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::happensLink"))) (target (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::ordinary"))) (target (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::ordinary"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::selfLink"))) (target (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::slice"))) (target (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::slice"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::slice"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::packageOwned"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::behavior")))
      (featured-by (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner")))
      (type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::involvingPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::ownedPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedActions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::happensLink")))
      (featured-by (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner")))
      (type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::ordinary")))
      (featured-by (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::selfLink")))
      (featured-by (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner")))
      (type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::SelfLink")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::SelfLink")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::SelfLink")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::slice")))
      (featured-by (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portions")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::packageOwned")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_usage_may_time_vary.md") (range (start 1 31) (end 1 54)) (probe (position 1 31))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner"))) (kind specialization) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_may_time_vary.md") (range (start 4 26) (end 4 41)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::behavior"))) (kind featureTyping) (ordinal 0) (authored-target "Actions::Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_may_time_vary.md") (range (start 6 26) (end 6 50)) (probe (position 6 26))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::happensLink"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrences::HappensLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_may_time_vary.md") (range (start 5 23) (end 5 38)) (probe (position 5 23))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::Owner::selfLink"))) (kind featureTyping) (ordinal 0) (authored-target "Links::SelfLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::SelfLink")))))
    )
  )
)
~~~
