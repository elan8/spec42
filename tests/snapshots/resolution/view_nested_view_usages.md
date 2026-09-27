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

    // The whole declaration lowers: every typing target, references and crosses, the prefix
    // modifiers and direction, an anonymous typed declaration, and a value.
    view def Appendix;
    view declarations {
        view twoTypes : DocumentType, Appendix;
        view referencing ::> document;
        view crossing => document;
        derived view derivedView;
        variation view variant;
        in view directed;
        view : Appendix;
        view valued = document;
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
      (diagnostic
        (severity information)
        (code "view_expose_empty")
        (source "semantic")
        (range (start 30 4) (end 39 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:85905ab1839fe42d396e1f0068352691796d28b038875ba4d2d150aa5d1dcb5b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Appendix"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::appendices"))) (kind view) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::introduction"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::requirements"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (anonymous (kind view) (ordinal 0))))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Appendix")))))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::crossing"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (crossSubsetting (reference "document")))))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::derivedView"))) (kind view) (membership (kind feature) (visibility default)) (facts (modifiers derived)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::directed"))) (kind view) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::referencing"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "document")))))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::twoTypes"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DocumentType")) (featureTyping (reference "Appendix")))))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::valued"))) (kind view) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (named (kind view) (name "valued")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (named (kind view) (name "valued")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (named (kind view) (name "valued")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (named (kind view) (name "valued")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (named (kind view) (name "valued")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::variant"))) (kind view) (membership (kind feature) (visibility default)) (facts (modifiers variation)))
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
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (anonymous (kind view) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Appendix")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Appendix")))))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::crossing"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "document")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document")))))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::referencing"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "document")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document")))))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::twoTypes"))) (kind featureTyping) (ordinal 0))
      (authored-target "DocumentType")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")))))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::twoTypes"))) (kind featureTyping) (ordinal 1))
      (authored-target "Appendix")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Appendix")))))
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
    (relationship (kind typing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (anonymous (kind view) (ordinal 0))))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Appendix"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (anonymous (kind view) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::crossing"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::crossing"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::referencing"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::referencing"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::twoTypes"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::twoTypes"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::twoTypes"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Appendix"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::twoTypes"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0))))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::requirements"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0)) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (anonymous (kind view) (ordinal 0)) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (named (kind view) (name "alpha")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "document")) (named (kind view) (name "alpha")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::vehicle"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::appendices"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::introduction"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType::requirements"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (anonymous (kind view) (ordinal 0))))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::crossing"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::derivedView"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::directed"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::referencing"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::twoTypes"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::valued"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::valued"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (named (kind view) (name "valued")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (named (kind view) (name "valued")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (named (kind view) (name "valued")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::variant"))) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Appendix")))
      (subtype (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (anonymous (kind view) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::twoTypes")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")))
      (subtype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::twoTypes")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (anonymous (kind view) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations")))
      (type (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Appendix")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Appendix")) (source direct))
      (supertype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Appendix")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::crossing")))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations")))
      (effective-type (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")) (source inherited) (from (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document"))))
      (supertype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")) (scopes any))
      (supertype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::derivedView")))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations")))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::directed")))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations")))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::referencing")))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations")))
      (effective-type (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")) (source inherited) (from (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document"))))
      (supertype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")) (scopes any))
      (supertype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::twoTypes")))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations")))
      (type (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Appendix")) (provenance authored))
      (type (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Appendix")) (source direct))
      (effective-type (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")) (source direct))
      (supertype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Appendix")) (scopes any))
      (supertype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::valued")))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations")))
      (supertype (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (named (kind view) (name "valued")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (named (kind view) (name "valued")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (named (kind view) (name "valued")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::valued")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::variant")))
      (featured-by (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations")))
    )
    (declaration (id (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document")))
      (type (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")) (source direct))
      (supertype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")) (scopes any))
      (subtype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::crossing")) (scopes any feature))
      (subtype (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::referencing")) (scopes any feature))
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
  (query (document "memory://snapshot/view_nested_view_usages.md") (range (start 37 15) (end 37 23)) (probe (position 37 15))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (path (named (kind package) (name "NestedViewCoverage")) (named (kind view) (name "declarations")) (anonymous (kind view) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Appendix")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Appendix")))))
    )
  )
  (query (document "memory://snapshot/view_nested_view_usages.md") (range (start 33 25) (end 33 33)) (probe (position 33 25))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::crossing"))) (kind crossSubsetting) (ordinal 0) (authored-target "document")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document")))))
    )
  )
  (query (document "memory://snapshot/view_nested_view_usages.md") (range (start 32 29) (end 32 37)) (probe (position 32 29))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::referencing"))) (kind referenceSubsetting) (ordinal 0) (authored-target "document")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::document")))))
    )
  )
  (query (document "memory://snapshot/view_nested_view_usages.md") (range (start 31 24) (end 31 36)) (probe (position 31 24))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::twoTypes"))) (kind featureTyping) (ordinal 0) (authored-target "DocumentType")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::DocumentType")))))
    )
  )
  (query (document "memory://snapshot/view_nested_view_usages.md") (range (start 31 38) (end 31 46)) (probe (position 31 38))
    (reference (id (source (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::declarations::twoTypes"))) (kind featureTyping) (ordinal 1) (authored-target "Appendix")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_nested_view_usages.md") (qualified-name "NestedViewCoverage::Appendix")))))
    )
  )
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
