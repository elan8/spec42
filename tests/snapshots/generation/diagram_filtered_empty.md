# META
~~~ini
description=Filtering out the only explicitly exposed definition produces a valid complete empty view; rendering explains the empty selection
type=generate
libraries=standard
require_complete_publication=true
plugin=native:diagram
viewKind=interconnection-view
viewDocument=model.sysml
viewQualifiedName=EmptySelection::selected
~~~
# SOURCE
## model.sysml
~~~sysml
package EmptySelection {
    private import StandardViewDefinitions::*;
    part def Component;
    part def Assembly { part child : Component; }
    view selected : InterconnectionView { expose Assembly; filter @SysML::PartUsage; }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:41fa1be64a9a8efaaa3e5fb3db49fb6bea50ec2131153e406065e5cf9c2d2a81") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "EmptySelection")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly::child"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")) (filterMetadataTest (reference "SysML::PartUsage")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "EmptySelection")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Assembly")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "EmptySelection")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly::child"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Component")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::selected"))) (kind filterMetadataTest) (ordinal 0))
      (authored-target "SysML::PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "EmptySelection")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly::child"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly::child"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::selected"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::selected"))) (kind filterMetadataTest) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "EmptySelection")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "EmptySelection")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly::child"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly::child"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly::child"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Component"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "EmptySelection")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::selected"))) (provenance implied))
  )
  (evaluation
    (filter (owner (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::selected"))) (form view) (state unsupported) (start 4 66) (end 4 83))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly::child")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Component")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Component")) (scopes any))
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
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Component")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly::child")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source direct))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "EmptySelection")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "EmptySelection")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 3 37) (end 3 46)) (probe (position 3 37))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly::child"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Component")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 20) (end 4 39)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::selected"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 67) (end 4 83)) (probe (position 4 67))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::selected"))) (kind filterMetadataTest) (ordinal 0) (authored-target "SysML::PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 49) (end 4 57)) (probe (position 4 49))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "EmptySelection")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "EmptySelection::Assembly")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:cc9185f9670ba992e159242d0a4bc833a5678cb05bb8dedb2046a815ce19fff5",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
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
      "qualifiedName": "EmptySelection::selected"
    }
  ],
  "selectedView": {
    "reference": 0,
    "kind": "interconnection-view",
    "name": "selected",
    "source": 0
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [],
    "exposedRoots": [],
    "kind": "interconnection-view",
    "metadata": {
      "connectors": [],
      "parts": [],
      "portFamilies": [],
      "ports": []
    },
    "nodes": [],
    "relationships": [],
    "scene": {
      "kind": "interconnection"
    }
  }
}

~~~
