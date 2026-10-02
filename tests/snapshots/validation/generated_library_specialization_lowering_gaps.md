# META
~~~ini
description=Generated library-specialization rules remain explicit until authored forms publish their exact semantic metaclass and query identity
specification=OMG SysML 2.0 and KerML 1.0 (formal/26-03)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
blocked_by=lowering-gap-library-specialization-forms
rule_id=kerml-1.0:8.3.4.12.3:checkMetadataFeatureSpecialization
rule_id=sysml-2.0:8.3.13.4:checkConnectionUsageSpecialization
rule_id=sysml-2.0:8.3.14.3:checkInterfaceUsageSpecialization
rule_id=sysml-2.0:8.3.19.3:checkCalculationUsageSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package GeneratedSpecializationLoweringGaps {
    item def Thing;
    metaclass Marker;
    metadata MetadataFeature : Marker about Thing;
    connection ConnectionUsage;
    interface InterfaceUsage;
    calc CalculationUsage;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source "GeneratedSpecializationLoweringGaps::MetadataFeature") (target "Metaobjects::metaobjects") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source "GeneratedSpecializationLoweringGaps::ConnectionUsage") (target "Connections::connections") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source "GeneratedSpecializationLoweringGaps::InterfaceUsage") (target "Interfaces::interfaces") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source "GeneratedSpecializationLoweringGaps::CalculationUsage") (target "Calculations::calculations") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_library_specialization_lowering_gaps.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:7c9c0442db0fed6122c713d9e88abdee00e2b91f6b3933d7375ae1ae2521739b") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::CalculationUsage"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::ConnectionUsage"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::InterfaceUsage"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Marker"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::MetadataFeature"))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Marker")) (metadataAnnotationAbout (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Thing"))) (kind item-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::MetadataFeature"))) (kind featureTyping) (ordinal 0))
      (authored-target "Marker")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Marker")))))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::MetadataFeature"))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::MetadataFeature"))) (target (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Marker"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::MetadataFeature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::MetadataFeature"))) (target (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::MetadataFeature"))) (kind metadataAnnotationAbout) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::CalculationUsage"))) (target (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::ConnectionUsage"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::InterfaceUsage"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Marker"))) (target (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::MetadataFeature"))) (target (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::metadataItems"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Thing"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::CalculationUsage")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::ConnectionUsage")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::InterfaceUsage")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Marker")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::MetadataFeature")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::MetadataFeature")))
      (type (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Marker")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Marker")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::MetadataItem")) (source inherited) (from (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::metadataItems"))))
      (effective-type (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Marker")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::MetadataItem")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::metadataItems")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::metaobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Thing")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Thing"))) (form usage) (definition (resolved (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Marker")))) (about (resolved (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Thing")))))
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::ConnectionUsage"))) (kind connection))
  (connector (id (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::InterfaceUsage"))) (kind interface))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (range (start 3 31) (end 3 37)) (probe (position 3 31))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::MetadataFeature"))) (kind featureTyping) (ordinal 0) (authored-target "Marker")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Marker")))))
    )
  )
  (query (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (range (start 3 44) (end 3 49)) (probe (position 3 44))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::MetadataFeature"))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_lowering_gaps.md") (qualified-name "GeneratedSpecializationLoweringGaps::Thing")))))
    )
  )
)
~~~
