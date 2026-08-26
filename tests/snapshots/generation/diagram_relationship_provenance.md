# META
~~~ini
description=General view distinguishes authored and implied relationship provenance
type=generate
libraries=standard
plugin=repository:diagram
viewKind=general-view
viewDocument=diagram_relationship_provenance.md
viewQualifiedName=ProvenanceExample::selected
~~~
# SOURCE
~~~sysml
package ProvenanceExample {
    private import StandardViewDefinitions::*;
    part def Base { attribute mass : Real; }
    part def Child :> Base { attribute mass = 1200; }
    view selected : GeneralView { expose Child; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_relationship_provenance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 37) (end 2 41))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 3 29) (end 3 51))
        (related-information
          (related
            (uri "memory://snapshot/diagram_relationship_provenance.md")
            (range (start 2 20) (end 2 42))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:56ac1a41cff3d325151b7648ffcd7b27b65db0d0e4328768cadbb494f8c7cde4") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeneralView")))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Child")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")))))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Child")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass"))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass"))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass"))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 1200)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass")))
      (featured-by (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (subtype (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")))
      (supertype (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass")))
      (featured-by (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass")) (scopes any feature))
      (supertype (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (subtype (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (source direct))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 2 37) (end 2 41)) (probe (position 2 37))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 3 22) (end 3 26)) (probe (position 3 22))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")))))
    )
  )
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 4 20) (end 4 31)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 4 41) (end 4 46)) (probe (position 4 41))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Child")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:225f8fb0a35df4dab27ae9d24c9c08b0ff97cf3ab55fb34bbc840d26e1d591bb",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_relationship_provenance.md",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/base.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/parts.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/performances.md",
      "sourceDomain": "standard-library"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        3,
        13,
        3,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        22,
        3,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        39,
        3,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        46,
        3,
        50
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        9,
        4,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ProvenanceExample::Base"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ProvenanceExample::Base::mass"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ProvenanceExample::Child"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ProvenanceExample::Child::mass"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ProvenanceExample::Child::mass::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ProvenanceExample::Child::mass::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ProvenanceExample::selected"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Base::dataValues"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Base::things"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "Performances::evaluations"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Expression",
      "ownerQualifiedName": "ProvenanceExample::Child::mass",
      "source": 3,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "ProvenanceExample::Child::mass::",
      "source": 3,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "specializes",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "redefinition",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typeFeaturing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "typeFeaturing",
      "source": 5
    }
  ],
  "selectedView": {
    "reference": 6,
    "kind": "general-view",
    "name": "selected",
    "source": 4
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 2,
        "provenance": "authored",
        "reference": 13,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 3,
        "provenance": "authored",
        "reference": 16,
        "source": 1,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 3,
        "provenance": "authored",
        "reference": 21,
        "source": 2,
        "target": 3
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "general-view",
    "metadata": {
      "roots": [
        0
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              1
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "PartDefinition",
        "name": "Child",
        "notationRole": "definition",
        "owner": null,
        "reference": 2,
        "source": 0,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "mass",
        "notationRole": "usage",
        "owner": 0,
        "reference": 3,
        "source": 2,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "Expression",
        "name": null,
        "notationRole": "unsupported",
        "owner": 1,
        "reference": 11,
        "source": 3,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "Feature",
        "name": null,
        "notationRole": "unsupported",
        "owner": 2,
        "reference": 12,
        "source": 3,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "specializes",
        "navigation": 1,
        "provenance": "authored",
        "reference": 14,
        "source": 0,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 15,
        "source": 0,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 17,
        "source": 1,
        "target": {
          "reference": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 18,
        "source": 1,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 19,
        "source": 1,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 20,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 22,
        "source": 2,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 23,
        "source": 3,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 24,
        "source": 3,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "general"
    }
  }
}

~~~
