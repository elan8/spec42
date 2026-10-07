# META
~~~ini
description=SysML 8.3.26.6 checkRenderingUsageRedefinition requires a view rendering usage to redefine Views::View::viewRendering
specification=OMG SysML 2.0 (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.26.6:checkRenderingUsageRedefinition
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
// Every `render` member owns a RenderingUsage through a ViewRenderingMembership and redefines
// Views::View::viewRendering: the declared rendering form and the short form that subsets an
// existing rendering. A RenderingUsage outside a ViewRenderingMembership plays no such role.
package Renderings {
    rendering asTable;
    view def TableView {
        render asTable;
    }
    view def TreeView {
        render rendering asTree;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (redefinition-check (rule_id "sysml-2.0:8.3.26.6:checkRenderingUsageRedefinition") (outcome satisfied))
  (relationship (kind redefinition) (source (anonymous (owner "Renderings::TableView") (kind RenderingUsage) (ordinal 0))) (target "Views::View::viewRendering") (provenance implied) (outcome resolved))
  (relationship (kind redefinition) (source "Renderings::TreeView::asTree") (target "Views::View::viewRendering") (provenance implied) (outcome resolved))
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_rendering_usage_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:ace8298e18ca375497e130445742f4d71854564d7fd438734a2cc91ea5bd20a0") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TableView"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (path (named (kind package) (name "Renderings")) (named (kind view-def) (name "TableView")) (anonymous (kind rendering) (ordinal 0))))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)) (authored (membership (kind feature) (visibility default) (role view-rendering)) (relationships (subsetting (reference "asTable")))))
    (declaration (id (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TreeView"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TreeView::asTree"))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)))
    (declaration (id (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::asTable"))) (kind rendering) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (path (named (kind package) (name "Renderings")) (named (kind view-def) (name "TableView")) (anonymous (kind rendering) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "asTable")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::asTable")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (path (named (kind package) (name "Renderings")) (named (kind view-def) (name "TableView")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::asTable"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (path (named (kind package) (name "Renderings")) (named (kind view-def) (name "TableView")) (anonymous (kind rendering) (ordinal 0))))) (kind subsetting) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TableView"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (path (named (kind package) (name "Renderings")) (named (kind view-def) (name "TableView")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (path (named (kind package) (name "Renderings")) (named (kind view-def) (name "TableView")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TableView"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (path (named (kind package) (name "Renderings")) (named (kind view-def) (name "TableView")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View::viewRendering"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (path (named (kind package) (name "Renderings")) (named (kind view-def) (name "TableView")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TreeView"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TreeView::asTree"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TreeView::asTree"))) (target (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TreeView"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TreeView::asTree"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View::viewRendering"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TreeView::asTree"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::asTable"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TableView")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (path (named (kind package) (name "Renderings")) (named (kind view-def) (name "TableView")) (anonymous (kind rendering) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TableView")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View::viewRendering"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View::viewRendering")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::asTable")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TreeView")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TreeView::asTree")))
      (featured-by (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::TreeView")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View::viewRendering"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View::viewRendering")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::asTable")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings"))))
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
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings")) (scopes any feature))
      (subtype (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (path (named (kind package) (name "Renderings")) (named (kind view-def) (name "TableView")) (anonymous (kind rendering) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (range (start 6 15) (end 6 22)) (probe (position 6 15))
    (reference (id (source (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (path (named (kind package) (name "Renderings")) (named (kind view-def) (name "TableView")) (anonymous (kind rendering) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "asTable")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Renderings::asTable")))))
    )
  )
)
~~~
