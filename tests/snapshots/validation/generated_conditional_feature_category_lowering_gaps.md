# META
~~~ini
description=Feature category specialization expectations remain explicit until canonical metaclass category facts are published
specification=OMG KerML 1.0
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureOccurrenceSpecialization
rule_id=kerml-1.0:8.3.3.3.4:checkFeaturePortionSpecialization
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureSubobjectSpecialization
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureSuboccurrenceSpecialization
blocked_by=lowering-gap-feature-category-metaclass-predicates
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package FeatureCategoryLoweringGaps {
    class OccurrenceType;
    struct ObjectType;

    class Owner {
        feature occurrence : OccurrenceType;
        portion feature portion : OccurrenceType;
        composite feature suboccurrence : OccurrenceType;
    }

    struct StructureOwner {
        composite feature subobject : ObjectType;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "FeatureCategoryLoweringGaps::Owner::occurrence") (target "Occurrences::occurrences") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "FeatureCategoryLoweringGaps::Owner::portion") (target "Occurrence::Occurrence::portions") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "FeatureCategoryLoweringGaps::Owner::suboccurrence") (target "Occurrence::Occurrence::suboccurrences") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "FeatureCategoryLoweringGaps::StructureOwner::subobject") (target "Occurrence::Occurrence::suboccurrences") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:3a58ba4d60e3713380f4830acb0239794d339d226a13d7167422d8e369f1d6c4") (contract-version "parser-owned-resolution-v2") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::ObjectType"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::occurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OccurrenceType")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::portion"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OccurrenceType")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::suboccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OccurrenceType")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::StructureOwner"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::StructureOwner::subobject"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ObjectType")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::occurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "OccurrenceType")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::portion"))) (kind featureTyping) (ordinal 0))
      (authored-target "OccurrenceType")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::suboccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "OccurrenceType")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::StructureOwner::subobject"))) (kind featureTyping) (ordinal 0))
      (authored-target "ObjectType")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::ObjectType")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::occurrence"))) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::occurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::portion"))) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::portion"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::suboccurrence"))) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::suboccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::StructureOwner::subobject"))) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::ObjectType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::StructureOwner::subobject"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::ObjectType"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::occurrence"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::occurrence"))) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::portion"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::portion"))) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::suboccurrence"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::suboccurrence"))) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::StructureOwner"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::StructureOwner::subobject"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::StructureOwner::subobject"))) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::StructureOwner"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::ObjectType")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::StructureOwner::subobject")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::occurrence")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::portion")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::suboccurrence")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::occurrence")))
      (featured-by (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner")))
      (type (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::portion")))
      (featured-by (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner")))
      (type (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::suboccurrence")))
      (featured-by (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner")))
      (type (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::StructureOwner")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::StructureOwner::subobject")))
      (featured-by (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::StructureOwner")))
      (type (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::ObjectType")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::ObjectType")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::ObjectType")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (range (start 5 29) (end 5 43)) (probe (position 5 29))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::occurrence"))) (kind featureTyping) (ordinal 0) (authored-target "OccurrenceType")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (range (start 6 34) (end 6 48)) (probe (position 6 34))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::portion"))) (kind featureTyping) (ordinal 0) (authored-target "OccurrenceType")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (range (start 7 42) (end 7 56)) (probe (position 7 42))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::Owner::suboccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "OccurrenceType")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::OccurrenceType")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (range (start 11 38) (end 11 48)) (probe (position 11 38))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::StructureOwner::subobject"))) (kind featureTyping) (ordinal 0) (authored-target "ObjectType")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_feature_category_lowering_gaps.md") (qualified-name "FeatureCategoryLoweringGaps::ObjectType")))))
    )
  )
)
~~~
