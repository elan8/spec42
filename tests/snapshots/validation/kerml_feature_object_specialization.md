# META
~~~ini
description=KerML 8.3.3.3.4 feature Class and Structure typing implies the exact occurrence and object library specializations
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 checkFeatureObjectSpecialization and checkFeatureOccurrenceSpecialization
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureObjectSpecialization
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureOccurrenceSpecialization
libraries=standard
~~~
# SOURCE
~~~kerml
package Model {
    class OccurrenceType;
    struct ObjectType;
    class Owner {
        feature occurrence : OccurrenceType;
        feature object : ObjectType;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "Model::Owner::occurrence") (target "Occurrences::occurrences") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "Model::Owner::object") (target "Objects::objects") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_object_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:1981054e912b32c17658f69684e4f5e0b07d40a779f4035350dd7c32a385cb0b") (contract-version "constructor-expression-specialization-v9") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::ObjectType"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::OccurrenceType"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::object"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ObjectType")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::occurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OccurrenceType")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::object"))) (kind featureTyping) (ordinal 0))
      (authored-target "ObjectType")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::ObjectType")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::occurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "OccurrenceType")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::OccurrenceType")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::object"))) (target (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::ObjectType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::object"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::occurrence"))) (target (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::OccurrenceType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::occurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::ObjectType"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::OccurrenceType"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::object"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::object"))) (target (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::object"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::object"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::occurrence"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::occurrence"))) (target (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::occurrence"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::ObjectType")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::object")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::OccurrenceType")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::occurrence")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::object")))
      (featured-by (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner")))
      (type (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::ObjectType")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::ObjectType")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::ObjectType")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::occurrence")))
      (featured-by (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner")))
      (type (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::OccurrenceType")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::OccurrenceType")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::OccurrenceType")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_object_specialization.md") (range (start 5 25) (end 5 35)) (probe (position 5 25))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::object"))) (kind featureTyping) (ordinal 0) (authored-target "ObjectType")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::ObjectType")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_object_specialization.md") (range (start 4 29) (end 4 43)) (probe (position 4 29))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::Owner::occurrence"))) (kind featureTyping) (ordinal 0) (authored-target "OccurrenceType")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_object_specialization.md") (qualified-name "Model::OccurrenceType")))))
    )
  )
)
~~~
