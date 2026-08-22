# META
~~~ini
description=Qualified references resolve to opaque identities with explicit scope, kind, and ambiguity
type=file
libraries=standard
~~~
# SOURCE
## a.sysml
~~~sysml
package Shared {
    view selected;
    part wrong;
}
~~~
## b.sysml
~~~sysml
package Shared {
    view selected;
}
~~~
# QUALIFIED REFERENCE QUERIES
~~~text
resolve a.sysml Shared::selected ViewUsage
resolve * Shared::selected ViewUsage
resolve a.sysml Shared::wrong ViewUsage
resolve a.sysml Shared::missing ViewUsage
resolve * StandardViewDefinitions::GeneralView ViewDefinition
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/a.sysml"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 4) (end 2 15))
      )
    )
  )
  (document "memory://snapshot/b.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:08705b01ea1da3f617bc63af56d143a3a47cf251043022b5ecb31e029805c7b6") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Shared"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Shared::selected"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Shared::wrong"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/b.sysml") (qualified-name "Shared"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/b.sysml") (qualified-name "Shared::selected"))) (kind view) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/a.sysml") (qualified-name "Shared::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/b.sysml") (qualified-name "Shared::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/a.sysml") (qualified-name "Shared::wrong"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Shared::selected")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Shared::wrong")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/b.sysml") (qualified-name "Shared::selected")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
# QUALIFIED REFERENCE RESULTS
~~~sexpr
(qualified-reference-queries
  (reference (document "memory://snapshot/a.sysml") (qualified-name "Shared::selected") (expected-kind "ViewUsage")
    (outcome (status recovery) (candidate (identity "element/v125:memory://snapshot/a.sysml7:packagen6:Shared1:04:viewn8:selected1:0") (kind "ViewUsage") (qualified-name "Shared::selected") (location (document "memory://snapshot/a.sysml") (range (start 1 9) (end 1 17)) (role Declaration))))
  )
  (reference (document any) (qualified-name "Shared::selected") (expected-kind "ViewUsage")
    (outcome (status ambiguous) (candidates (candidate (identity "element/v125:memory://snapshot/a.sysml7:packagen6:Shared1:04:viewn8:selected1:0") (kind "ViewUsage") (qualified-name "Shared::selected") (location (document "memory://snapshot/a.sysml") (range (start 1 9) (end 1 17)) (role Declaration))) (candidate (identity "element/v125:memory://snapshot/b.sysml7:packagen6:Shared1:04:viewn8:selected1:0") (kind "ViewUsage") (qualified-name "Shared::selected") (location (document "memory://snapshot/b.sysml") (range (start 1 9) (end 1 17)) (role Declaration)))))
  )
  (reference (document "memory://snapshot/a.sysml") (qualified-name "Shared::wrong") (expected-kind "ViewUsage")
    (outcome (status wrong-kind) (candidates (candidate (identity "element/v125:memory://snapshot/a.sysml7:packagen6:Shared1:04:partn5:wrong1:0") (kind "PartUsage") (qualified-name "Shared::wrong") (location (document "memory://snapshot/a.sysml") (range (start 2 9) (end 2 14)) (role Declaration)))))
  )
  (reference (document "memory://snapshot/a.sysml") (qualified-name "Shared::missing") (expected-kind "ViewUsage")
    (outcome (status recovery))
  )
  (reference (document any) (qualified-name "StandardViewDefinitions::GeneralView") (expected-kind "ViewDefinition")
    (outcome (status recovery) (candidate (identity "element/v160:memory://snapshot/sysml.library/standard_view_definitions.md15:library-packagen23:StandardViewDefinitions1:08:view-defn11:GeneralView1:0") (kind "ViewDefinition") (qualified-name "StandardViewDefinitions::GeneralView") (location (document "memory://snapshot/sysml.library/standard_view_definitions.md") (range (start 6 18) (end 6 29)) (role Declaration))))
  )
)
~~~
