# META
~~~ini
description=Nested view usages in view definition and view usage bodies lower as owned view declarations, recursively, with their typing, redefinition, abstract modifier, and expose members
type=file
~~~
# SOURCE
~~~sysml
package NestedViewCoverage {
    part def Vehicle;
    part vehicle : Vehicle;

    // A view definition decomposed into subviews. Each nested view is a feature of the
    // definition, implicitly subsetting the library's View::subviews.
    view def DocumentType {
        view introduction;
        view requirements;
        abstract view appendices[0..*];
    }

    // Nested views in a view usage body, recursively and in authored (not alphabetical) order.
    view document : DocumentType {
        view zeta {
            view background;
        }
        view alpha {
            expose vehicle;
        }

        // The anonymous redefinition form resolves its target through the definition.
        view :>> requirements {
            expose NestedViewCoverage::*;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/view_nested_view_usages.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "view_expose_empty")
        (source "semantic")
        (range (start 13 4) (end 25 5))
      )
      (diagnostic
        (severity information)
        (code "view_expose_empty")
        (source "semantic")
        (range (start 14 8) (end 16 9))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e996988fc750aa0a54c6148eb877c1e8e33b982a88bf7be2e7f5eed38d67737f"))
  (declarations
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::appendices"))) (kind view) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::introduction"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::requirements"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DocumentType")))))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0))))) (kind view) (membership (kind feature) (visibility default)) (effective-identification (name "requirements") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "requirements")))))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0)) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "NestedViewCoverage")))))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document::alpha"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (named (kind view) (name "alpha")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document::zeta"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document::zeta::background"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document"))) (kind featureTyping) (ordinal 0))
      (authored-target "DocumentType")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")))))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "requirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::requirements")))))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0)) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "NestedViewCoverage")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage")))))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (named (kind view) (name "alpha")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0))))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::requirements"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0)) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0)) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (named (kind view) (name "alpha")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (named (kind view) (name "alpha")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::vehicle"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::appendices"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::introduction"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::requirements"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0))))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0)) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document::alpha"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (named (kind view) (name "alpha")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document::alpha"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document::zeta"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document::zeta::background"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document::zeta"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")))
      (subtype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::appendices")))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::introduction")))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::requirements")))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")))
      (subtype (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Vehicle")))
      (subtype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document")))
      (type (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")) (source direct))
      (supertype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document")))
      (supertype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::requirements")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0)) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document::alpha")))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document")))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (named (kind view) (name "alpha")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document::alpha")))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document::zeta")))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document")))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document::zeta::background")))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document::zeta")))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::vehicle")))
      (type (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Vehicle")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/view_nested_view_usages.md") (range (start 13 20) (end 13 32)) (probe (position 13 20))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document"))) (kind featureTyping) (ordinal 0) (authored-target "DocumentType")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")))))
    )
  )
  (query (document "memory://snapshot/view_nested_view_usages.md") (range (start 22 17) (end 22 29)) (probe (position 22 17))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "requirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::requirements")))))
    )
  )
  (query (document "memory://snapshot/view_nested_view_usages.md") (range (start 23 19) (end 23 40)) (probe (position 23 19))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0)) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "NestedViewCoverage")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage")))))
    )
  )
  (query (document "memory://snapshot/view_nested_view_usages.md") (range (start 18 19) (end 18 26)) (probe (position 18 19))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (named (kind view) (name "alpha")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::vehicle")))))
    )
  )
  (query (document "memory://snapshot/view_nested_view_usages.md") (range (start 2 19) (end 2 26)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Vehicle")))))
    )
  )
)
~~~
