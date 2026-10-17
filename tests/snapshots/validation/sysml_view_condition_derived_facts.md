# META
~~~ini
description=SysML 8.3.26.7 and 8.3.26.11 view definitions and usages derive their filter conditions in authored order
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.26.7:deriveViewDefinitionViewCondition
rule_id=sysml-2.0:8.3.26.11:deriveViewUsageViewCondition
libraries=standard
type=file
~~~
# SOURCE
~~~sysml
package ViewFacts {
    view def Parts {
        filter @SysML::PartUsage;
    }
    view parts : Parts {
        filter @SysML::PartUsage;
        filter @SysML::PortUsage;
    }
    view plain;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "sysml-2.0:8.3.26.7:deriveViewDefinitionViewCondition") (source "ViewFacts::Parts") (position 1) (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.26.11:deriveViewUsageViewCondition") (source "ViewFacts::parts") (position 2) (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.26.11:deriveViewUsageViewCondition") (source "ViewFacts::plain") (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_view_condition_derived_facts.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:f1106c8299c02cdc254a67f4f1062e59a8e3a29583f4fc76a5e6ba660af6e811") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::Parts"))) (kind view-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (filterMetadataTest (reference "SysML::PartUsage")))))
    (declaration (id (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Parts")) (filterMetadataTest (reference "SysML::PartUsage")) (filterMetadataTest (reference "SysML::PortUsage")))))
    (declaration (id (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::plain"))) (kind view) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::Parts"))) (kind filterMetadataTest) (ordinal 0))
      (authored-target "SysML::PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    (reference (id (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (kind featureTyping) (ordinal 0))
      (authored-target "Parts")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::Parts")))))
    (reference (id (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (kind filterMetadataTest) (ordinal 0))
      (authored-target "SysML::PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    (reference (id (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (kind filterMetadataTest) (ordinal 1))
      (authored-target "SysML::PortUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PortUsage")))))
  )
  (relationships
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::Parts"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::Parts"))) (kind filterMetadataTest) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (target (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::Parts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (kind filterMetadataTest) (ordinal 0)))
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PortUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (kind filterMetadataTest) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::Parts"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::plain"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
  )
  (evaluation
    (filter (owner (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::Parts"))) (form view) (state unsupported) (start 2 15) (end 2 32))
    (filter (owner (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (form view) (state unsupported) (start 5 15) (end 5 32))
    (filter (owner (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (form view) (state unsupported) (start 6 15) (end 6 32))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::Parts")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts")))
      (type (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::Parts")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (effective-type (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::Parts")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::Parts")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::plain")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_view_condition_derived_facts.md") (range (start 2 16) (end 2 32)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::Parts"))) (kind filterMetadataTest) (ordinal 0) (authored-target "SysML::PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    )
  )
  (query (document "memory://snapshot/sysml_view_condition_derived_facts.md") (range (start 4 17) (end 4 22)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (kind featureTyping) (ordinal 0) (authored-target "Parts")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::Parts")))))
    )
  )
  (query (document "memory://snapshot/sysml_view_condition_derived_facts.md") (range (start 5 16) (end 5 32)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (kind filterMetadataTest) (ordinal 0) (authored-target "SysML::PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    )
  )
  (query (document "memory://snapshot/sysml_view_condition_derived_facts.md") (range (start 6 16) (end 6 32)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/sysml_view_condition_derived_facts.md") (qualified-name "ViewFacts::parts"))) (kind filterMetadataTest) (ordinal 1) (authored-target "SysML::PortUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PortUsage")))))
    )
  )
)
~~~
