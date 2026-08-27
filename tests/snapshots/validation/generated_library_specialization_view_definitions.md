# META
~~~ini
description=Generated library-specialization checks publish implied anchors for view and metadata definitions
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.26.5:checkRenderingDefinitionSpecialization
rule_id=sysml-2.0:8.3.26.7:checkViewDefinitionSpecialization
rule_id=sysml-2.0:8.3.27.2:checkMetadataDefinitionSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package GeneratedViewDefinitions {
    rendering def RenderingDefinition;
    view def ViewDefinition;
    metadata def MetadataDefinition;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "GeneratedViewDefinitions::RenderingDefinition") (target "Views::Rendering") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedViewDefinitions::ViewDefinition") (target "Views::View") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedViewDefinitions::MetadataDefinition") (target "Metadata::MetadataItem") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_library_specialization_view_definitions.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:ca8508e47cd9f607b214e9a367a0eb51e6deffbfb11126e9ae181cebd2a98ea8") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_view_definitions.md") (qualified-name "GeneratedViewDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_view_definitions.md") (qualified-name "GeneratedViewDefinitions::MetadataDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_view_definitions.md") (qualified-name "GeneratedViewDefinitions::RenderingDefinition"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_view_definitions.md") (qualified-name "GeneratedViewDefinitions::ViewDefinition"))) (kind view-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_view_definitions.md") (qualified-name "GeneratedViewDefinitions::MetadataDefinition"))) (target (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::MetadataItem"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_view_definitions.md") (qualified-name "GeneratedViewDefinitions::RenderingDefinition"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_view_definitions.md") (qualified-name "GeneratedViewDefinitions::ViewDefinition"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_view_definitions.md") (qualified-name "GeneratedViewDefinitions::MetadataDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::MetadataItem")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_view_definitions.md") (qualified-name "GeneratedViewDefinitions::RenderingDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_view_definitions.md") (qualified-name "GeneratedViewDefinitions::ViewDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
