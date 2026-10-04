# META
~~~ini
description=SysML 8.3.11.3 validatePartUsagePartDefinition requires at least one of the itemDefinitions of a PartUsage to be a PartDefinition
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.11.3 validatePartUsagePartDefinition
source_expectation=accepted
rule_family=validate
expectation=by_construction
evidence_reference=file:tests/snapshots/validation/generated_library_specialization_usages.md
rule_id=sysml-2.0:8.3.11.3:validatePartUsagePartDefinition
libraries=standard
type=file
~~~
# SOURCE
~~~sysml
// The violating side has no model: checkPartUsageSpecialization implies that every part usage
// subsets Parts::parts, so Parts::Part (a part definition) is always among its types
// (Feature::type includes the types of subsetted features), whatever else it is typed by.
// generated_library_specialization_usages.md pins that implied subsetting and the inherited
// Parts::Part effective type.
package Parts {
    part def Component;
    item def Material;
    part def Holder {
        // Conforming: the part usage is typed by a part definition.
        part good : Component;

        // Also conforming: typed by an item definition, yet Parts::Part is still a type.
        part bad : Material;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_part_usage_part_definition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:8ad92e82fcf81854ec819c80c425b9c0fcda3e29be02e392d1b54f186efd90c0") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Material")))))
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material"))) (kind item-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad"))) (kind featureTyping) (ordinal 0))
      (authored-target "Material")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material")))))
    (reference (id (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad"))) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good"))) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad"))) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good"))) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad")))
      (featured-by (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder")))
      (type (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good")))
      (featured-by (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder")))
      (type (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_part_usage_part_definition.md") (range (start 13 19) (end 13 27)) (probe (position 13 19))
    (reference (id (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad"))) (kind featureTyping) (ordinal 0) (authored-target "Material")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material")))))
    )
  )
  (query (document "memory://snapshot/sysml_part_usage_part_definition.md") (range (start 10 20) (end 10 29)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component")))))
    )
  )
)
~~~
