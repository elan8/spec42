# META
~~~ini
description=KerML 8.3.3.3.4 checkFeatureCrossingSpecialization requires an end Feature's derived crossFeature to equal its authored ownedCrossFeature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 checkFeatureCrossingSpecialization
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureCrossingSpecialization
~~~
# SOURCE
~~~kerml
package Model {
    classifier Occurrence;
    assoc HappensDuring {
        feature timeCoincidentOccurrences : Occurrence;
        feature longerOccurrence : Occurrence;
        end happensDuring subsets timeCoincidentOccurrences
            feature thatOccurrence : Occurrence;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "kerml-1.0:8.3.3.3.4:checkFeatureCrossingSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_crossing_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d3432e1e6efdb5066fdbe725fab68cdb4b8c4c39d497485e4176ed4874be03ce") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::longerOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (cross-feature-projection (cross-feature (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence::happensDuring"))) (owned-cross-feature (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence::happensDuring"))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence::happensDuring"))) (kind kerml-end) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (subsetting (reference "timeCoincidentOccurrences")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::timeCoincidentOccurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::longerOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence::happensDuring"))) (kind subsetting) (ordinal 0))
      (authored-target "timeCoincidentOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::timeCoincidentOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::timeCoincidentOccurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::longerOccurrence"))) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::longerOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence"))) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence::happensDuring"))) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::timeCoincidentOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence::happensDuring"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::timeCoincidentOccurrences"))) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::timeCoincidentOccurrences"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::longerOccurrence"))) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence"))) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence::happensDuring"))) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::timeCoincidentOccurrences"))) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::longerOccurrence")))
      (featured-by (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring")))
      (type (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence")))
      (featured-by (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring")))
      (type (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence::happensDuring")))
      (type (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::timeCoincidentOccurrences"))))
      (supertype (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::timeCoincidentOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::timeCoincidentOccurrences")))
      (featured-by (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring")))
      (type (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence::happensDuring")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")))
      (subtype (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::longerOccurrence")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence::happensDuring")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::timeCoincidentOccurrences")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_crossing_specialization.md") (range (start 4 35) (end 4 45)) (probe (position 4 35))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::longerOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_crossing_specialization.md") (range (start 6 37) (end 6 47)) (probe (position 6 37))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_crossing_specialization.md") (range (start 5 34) (end 5 59)) (probe (position 5 34))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::thatOccurrence::happensDuring"))) (kind subsetting) (ordinal 0) (authored-target "timeCoincidentOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::timeCoincidentOccurrences")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_crossing_specialization.md") (range (start 3 44) (end 3 54)) (probe (position 3 44))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::HappensDuring::timeCoincidentOccurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_crossing_specialization.md") (qualified-name "Model::Occurrence")))))
    )
  )
)
~~~
