# META
~~~ini
description=Diagram node typing is authored FeatureTyping, not the implied library effective-type closure
type=generate
libraries=standard
plugin=repository:diagram
viewKind=general-view
viewDocument=diagram_direct_typing.md
viewQualifiedName=DirectTypingExample::selected
~~~
# SOURCE
~~~sysml
package DirectTypingExample {
    private import StandardViewDefinitions::*;
    part def Board;
    part def Assembly {
        part typedChild : Board;
        part untypedChild;
    }
    part root : Assembly;
    view selected : GeneralView { expose root; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_direct_typing.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:60187702aff36b2605cec05ffff861fc828e788ee8b90dc107e511239d2e9a5f") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (path (named (kind package) (name "DirectTypingExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly::typedChild"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Board")))))
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly::untypedChild"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Board"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::root"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Assembly")))))
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeneralView")))))
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (path (named (kind package) (name "DirectTypingExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "root")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_direct_typing.md") (path (named (kind package) (name "DirectTypingExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly::typedChild"))) (kind featureTyping) (ordinal 0))
      (authored-target "Board")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Board")))))
    (reference (id (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::root"))) (kind featureTyping) (ordinal 0))
      (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly")))))
    (reference (id (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_direct_typing.md") (path (named (kind package) (name "DirectTypingExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "root")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::root")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly::typedChild"))) (target (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Board"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly::typedChild"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::root"))) (target (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::root"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_direct_typing.md") (path (named (kind package) (name "DirectTypingExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::root"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_direct_typing.md") (path (named (kind package) (name "DirectTypingExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly::typedChild"))) (target (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly::typedChild"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly::untypedChild"))) (target (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly::untypedChild"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Board"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::root"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_direct_typing.md") (path (named (kind package) (name "DirectTypingExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::root")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly::typedChild")))
      (featured-by (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly")))
      (type (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Board")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Board")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Board")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly::untypedChild")))
      (featured-by (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly")))
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
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Board")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly::typedChild")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::root")))
      (type (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::selected")))
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
    (declaration (id (node (document "memory://snapshot/diagram_direct_typing.md") (path (named (kind package) (name "DirectTypingExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_direct_typing.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_direct_typing.md") (path (named (kind package) (name "DirectTypingExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_direct_typing.md") (range (start 4 26) (end 4 31)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly::typedChild"))) (kind featureTyping) (ordinal 0) (authored-target "Board")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Board")))))
    )
  )
  (query (document "memory://snapshot/diagram_direct_typing.md") (range (start 7 16) (end 7 24)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::root"))) (kind featureTyping) (ordinal 0) (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::Assembly")))))
    )
  )
  (query (document "memory://snapshot/diagram_direct_typing.md") (range (start 8 20) (end 8 31)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/diagram_direct_typing.md") (range (start 8 41) (end 8 45)) (probe (position 8 41))
    (reference (id (source (node (document "memory://snapshot/diagram_direct_typing.md") (path (named (kind package) (name "DirectTypingExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "root")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_direct_typing.md") (qualified-name "DirectTypingExample::root")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:3872951ad8c86edc7cd8c4bd82ae4b402741c73db34eb2c971c553f78afdf005",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_direct_typing.md",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/parts.md",
      "sourceDomain": "standard-library"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        4,
        13,
        4,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        26,
        4,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        13,
        5,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        9,
        7,
        13
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        16,
        7,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        9,
        8,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "DirectTypingExample::Assembly"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "DirectTypingExample::Assembly::typedChild"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "DirectTypingExample::Assembly::untypedChild"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "DirectTypingExample::Board"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "DirectTypingExample::root"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "DirectTypingExample::selected"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Parts::parts"
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "typing",
      "source": 4
    }
  ],
  "selectedView": {
    "reference": 5,
    "kind": "general-view",
    "name": "selected",
    "source": 5
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 0,
        "origin": 1,
        "provenance": "implied",
        "reference": 12,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 2,
        "provenance": "implied",
        "reference": 13,
        "source": 0,
        "target": 2
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
            "kind": "parts",
            "members": [
              1,
              2
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PartUsage",
        "name": "root",
        "notationRole": "usage",
        "owner": null,
        "reference": 4,
        "source": 3,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Assembly",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "PartUsage",
        "name": "typedChild",
        "notationRole": "usage",
        "owner": 0,
        "reference": 1,
        "source": 0,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Board",
              "reference": 3
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "PartUsage",
        "name": "untypedChild",
        "notationRole": "usage",
        "owner": 0,
        "reference": 2,
        "source": 2,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 4,
        "provenance": "authored",
        "reference": 15,
        "source": 0,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 14,
        "source": 0,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 9,
        "source": 1,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 7,
        "source": 1,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 8,
        "source": 1,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 10,
        "source": 2,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 11,
        "source": 2,
        "target": {
          "reference": 0,
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
