# META
~~~ini
description=General View draws a dependency with several clients from each drawn client to each drawn supplier, even when its first client is not drawn
specification=OMG SysML 2.0 (formal/2026-03-02), 8.2.2.4 Dependencies
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
type=generate
libraries=standard
plugin=native:diagram
viewKind=general-view
viewDocument=diagram_general_nary_dependency.md
viewQualifiedName=Trace::selected
~~~
# SOURCE
~~~sysml
package Trace {
    private import StandardViewDefinitions::*;
    part a;
    part b;
    part c;
    part d;
    dependency from a, b to c, d;
    view selected : GeneralView {
        expose b;
        expose c;
        expose d;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_general_nary_dependency.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:5023b7f3ad96e4df67c6b447c8b34f77ffa0152ceaf51e87c6c958588639eb9f") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "a")) (dependencyClient (reference "b")) (dependencySupplier (reference "c")) (dependencySupplier (reference "d")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::a"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::b"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::c"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::d"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeneralView")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "b")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "c")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "d")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::a")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::b")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::c")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 1))
      (authored-target "d")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::d")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::b")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0))
      (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::c")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0))
      (authored-target "d")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::d")))))
  )
  (relationships
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 1)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::c"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::d"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::c"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::d"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::a"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::b"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::c"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::d"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::selected"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::selected"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::a")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
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
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::b")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
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
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::c")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
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
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::d")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
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
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::selected")))
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
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::selected")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::selected")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_general_nary_dependency.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_nary_dependency.md") (range (start 6 20) (end 6 21)) (probe (position 6 20))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::a")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_nary_dependency.md") (range (start 6 23) (end 6 24)) (probe (position 6 23))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::b")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_nary_dependency.md") (range (start 6 28) (end 6 29)) (probe (position 6 28))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::c")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_nary_dependency.md") (range (start 6 31) (end 6 32)) (probe (position 6 31))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 1) (authored-target "d")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::d")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_nary_dependency.md") (range (start 7 20) (end 7 31)) (probe (position 7 20))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::selected"))) (kind featureTyping) (ordinal 0) (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_nary_dependency.md") (range (start 8 15) (end 8 16)) (probe (position 8 15))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::b")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_nary_dependency.md") (range (start 9 15) (end 9 16)) (probe (position 9 15))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0) (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::c")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_nary_dependency.md") (range (start 10 15) (end 10 16)) (probe (position 10 15))
    (reference (id (source (node (document "memory://snapshot/diagram_general_nary_dependency.md") (path (named (kind package) (name "Trace")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0) (authored-target "d")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_nary_dependency.md") (qualified-name "Trace::d")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:7594e75c323334cf9fbd28d9b32b15e707b1a38171864738f68f6f18f6509b07",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_general_nary_dependency.md",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        3,
        9,
        3,
        10
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        9,
        4,
        10
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        9,
        5,
        10
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        4,
        6,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        20,
        6,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        23,
        6,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        28,
        6,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        31,
        6,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        9,
        7,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Trace::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Trace::a"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Trace::b"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Trace::c"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Trace::d"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Trace::selected"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Dependency",
      "ownerQualifiedName": "Trace",
      "source": 3,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "dependencyClient",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "dependencyClient",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "dependencySupplier",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "dependencySupplier",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "dependency",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "dependency",
      "source": 2
    }
  ],
  "selectedView": {
    "reference": 5,
    "kind": "general-view",
    "name": "selected",
    "source": 8
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "dependency",
        "navigation": 4,
        "origin": 0,
        "provenance": "authored",
        "reference": 11,
        "source": 1,
        "target": 2
      },
      {
        "kind": "dependency",
        "navigation": 4,
        "origin": 0,
        "provenance": "authored",
        "reference": 12,
        "source": 1,
        "target": 3
      }
    ],
    "exposedRoots": [
      1,
      2,
      3
    ],
    "kind": "general-view",
    "metadata": {
      "roots": [
        1,
        2,
        3
      ]
    },
    "nodes": [
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "Dependency",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:7a5a6a9db687575366041f602c8389fe481a7c800540f7c85a53241831c4d958",
        "reference": 6,
        "source": 3,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "b",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:80b6d3728bf9c8317ef4ebbd90340b928ece020c577f60a95fa0d882e962c90d",
        "reference": 2,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "c",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:fabd4890c344de64415c32c9f14e4269917b927ee59248e62a1e23b88b64e44a",
        "reference": 3,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "d",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:4cb71c2b5fc21b8560cc1a66d0e6ccfaed5a280347c10f31eaf85842b855346d",
        "reference": 4,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "dependencyClient",
        "navigation": 4,
        "provenance": "authored",
        "reference": 7,
        "source": 0,
        "target": {
          "reference": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "dependencyClient",
        "navigation": 5,
        "provenance": "authored",
        "reference": 8,
        "source": 0,
        "target": {
          "node": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "dependencySupplier",
        "navigation": 6,
        "provenance": "authored",
        "reference": 9,
        "source": 0,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "dependencySupplier",
        "navigation": 7,
        "provenance": "authored",
        "reference": 10,
        "source": 0,
        "target": {
          "node": 3,
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
