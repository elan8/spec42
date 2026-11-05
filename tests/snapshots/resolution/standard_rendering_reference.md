# META
~~~ini
description=Rendering reference identities survive document assembly with the standard library (S10)
type=file
libraries=standard
require_complete_publication=true
require_no_diagnostics=true
~~~
# SOURCE
~~~sysml
package RenderingReferences {
    part def Subject;
    view table { expose Subject; render Views::asElementTable; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/standard_rendering_reference.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:ffb6ab2cdd8296eda5bd9c9430715d5287fed62d1988aba1879c9a757de3d1bb") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/standard_rendering_reference.md") (qualified-name "RenderingReferences"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/standard_rendering_reference.md") (qualified-name "RenderingReferences::Subject"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/standard_rendering_reference.md") (qualified-name "RenderingReferences::table"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Subject")))))
    (declaration (id (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind rendering) (ordinal 0))))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)) (authored (membership (kind feature) (visibility default) (role view-rendering)) (relationships (subsetting (reference "Views::asElementTable")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind rendering) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "Views::asElementTable")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::asElementTable")))))
    (reference (id (source (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Subject")
      (outcome (status resolved) (target (node (document "memory://snapshot/standard_rendering_reference.md") (qualified-name "RenderingReferences::Subject")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::asElementTable"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind rendering) (ordinal 0))))) (kind subsetting) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/standard_rendering_reference.md") (qualified-name "RenderingReferences::Subject"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/standard_rendering_reference.md") (qualified-name "RenderingReferences::Subject"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/standard_rendering_reference.md") (qualified-name "RenderingReferences::table"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/standard_rendering_reference.md") (qualified-name "RenderingReferences::table"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/standard_rendering_reference.md") (qualified-name "RenderingReferences::table"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View::viewRendering"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/standard_rendering_reference.md") (qualified-name "RenderingReferences::Subject")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/standard_rendering_reference.md") (qualified-name "RenderingReferences::table")))
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
    (declaration (id (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/standard_rendering_reference.md") (qualified-name "RenderingReferences::table")))
    )
    (declaration (id (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind rendering) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/standard_rendering_reference.md") (qualified-name "RenderingReferences::table")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View::viewRendering"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::TabularRendering")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::asElementTable"))))
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
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::TabularRendering")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View::viewRendering")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::asElementTable")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/standard_rendering_reference.md") (range (start 2 40) (end 2 61)) (probe (position 2 40))
    (reference (id (source (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind rendering) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "Views::asElementTable")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::asElementTable")))))
    )
  )
  (query (document "memory://snapshot/standard_rendering_reference.md") (range (start 2 24) (end 2 31)) (probe (position 2 24))
    (reference (id (source (node (document "memory://snapshot/standard_rendering_reference.md") (path (named (kind package) (name "RenderingReferences")) (named (kind view) (name "table")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Subject")
      (outcome (status resolved) (target (node (document "memory://snapshot/standard_rendering_reference.md") (qualified-name "RenderingReferences::Subject")))))
    )
  )
)
~~~
