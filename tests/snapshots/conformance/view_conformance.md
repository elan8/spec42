# META
~~~ini
description=Viewpoint conformance, rendering targets and textual-representation languages
type=file
~~~
# SOURCE
~~~sysml
package Views {
    part def Structure;
    part structure : Structure;
    viewpoint def Concerns;
    viewpoint concerns : Concerns;
    rendering def Tree;

    view conforming {
        satisfy concerns;
    }

    view satisfiesSomethingThatIsNotAViewpoint {
        satisfy structure;
    }

    rendering conformingRendering : Tree;
    rendering rendersSomethingThatIsNotARendering : Structure;

    viewpoint def WithoutRepresentationLanguage {
        rep /* no language identifier */;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/view_conformance.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "view_expose_empty")
        (source "semantic")
        (range (start 7 4) (end 9 5))
      )
      (diagnostic
        (severity information)
        (code "view_expose_empty")
        (source "semantic")
        (range (start 11 4) (end 13 5))
      )
      (diagnostic
        (severity warning)
        (code "viewpoint_conformance_invalid_target_kind")
        (source "semantic")
        (range (start 12 16) (end 12 25))
        (related-information
          (related
            (uri "memory://snapshot/view_conformance.md")
            (range (start 2 4) (end 2 31))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "incompatible_type_kind")
        (source "semantic")
        (range (start 16 52) (end 16 61))
        (related-information
          (related
            (uri "memory://snapshot/view_conformance.md")
            (range (start 1 4) (end 1 23))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "view_rendering_invalid_target")
        (source "semantic")
        (range (start 16 52) (end 16 61))
        (related-information
          (related
            (uri "memory://snapshot/view_conformance.md")
            (range (start 1 4) (end 1 23))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "missing_rep_language")
        (source "parser")
        (range (start 19 8) (end 19 40))
      )
      (diagnostic
        (severity warning)
        (code "viewpoint_rep_language_unresolved")
        (source "semantic")
        (range (start 19 8) (end 19 40))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:6f1c573000816dbb1aa695927ab2bc4c664bdb4d76e83a7d93d733063eb34f63") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Concerns"))) (kind viewpoint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Structure"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Tree"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::WithoutRepresentationLanguage"))) (kind viewpoint-def) (membership (kind owning) (visibility default)) (documentation (rep (text " no language identifier "))))
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::concerns"))) (kind viewpoint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Concerns")))))
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::conforming"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (path (named (kind package) (name "Views")) (named (kind view) (name "conforming")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "concerns")))))
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::conformingRendering"))) (kind rendering) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tree")))))
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::rendersSomethingThatIsNotARendering"))) (kind rendering) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Structure")))))
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::satisfiesSomethingThatIsNotAViewpoint"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (path (named (kind package) (name "Views")) (named (kind view) (name "satisfiesSomethingThatIsNotAViewpoint")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "structure")))))
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::structure"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Structure")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::concerns"))) (kind featureTyping) (ordinal 0))
      (authored-target "Concerns")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Concerns")))))
    (reference (id (source (node (document "memory://snapshot/view_conformance.md") (path (named (kind package) (name "Views")) (named (kind view) (name "conforming")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "concerns")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::concerns")))))
    (reference (id (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::conformingRendering"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tree")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Tree")))))
    (reference (id (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::rendersSomethingThatIsNotARendering"))) (kind featureTyping) (ordinal 0))
      (authored-target "Structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Structure")))))
    (reference (id (source (node (document "memory://snapshot/view_conformance.md") (path (named (kind package) (name "Views")) (named (kind view) (name "satisfiesSomethingThatIsNotAViewpoint")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::structure")))))
    (reference (id (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::structure"))) (kind featureTyping) (ordinal 0))
      (authored-target "Structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Structure")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::concerns"))) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Concerns"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::concerns"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/view_conformance.md") (path (named (kind package) (name "Views")) (named (kind view) (name "conforming")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::concerns"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_conformance.md") (path (named (kind package) (name "Views")) (named (kind view) (name "conforming")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::conformingRendering"))) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Tree"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::conformingRendering"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::rendersSomethingThatIsNotARendering"))) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Structure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::rendersSomethingThatIsNotARendering"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/view_conformance.md") (path (named (kind package) (name "Views")) (named (kind view) (name "satisfiesSomethingThatIsNotAViewpoint")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::structure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_conformance.md") (path (named (kind package) (name "Views")) (named (kind view) (name "satisfiesSomethingThatIsNotAViewpoint")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::structure"))) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Structure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::structure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_conformance.md") (path (named (kind package) (name "Views")) (named (kind view) (name "conforming")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::conforming"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_conformance.md") (path (named (kind package) (name "Views")) (named (kind view) (name "satisfiesSomethingThatIsNotAViewpoint")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::satisfiesSomethingThatIsNotAViewpoint"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Concerns")))
      (subtype (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::concerns")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Structure")))
      (subtype (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::rendersSomethingThatIsNotARendering")) (scopes any))
      (subtype (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::structure")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Tree")))
      (subtype (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::conformingRendering")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::concerns")))
      (type (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Concerns")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Concerns")) (source direct))
      (supertype (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Concerns")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (path (named (kind package) (name "Views")) (named (kind view) (name "conforming")) (anonymous (kind satisfy) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::conforming")))
    )
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::conformingRendering")))
      (type (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Tree")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Tree")) (source direct))
      (supertype (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Tree")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::rendersSomethingThatIsNotARendering")))
      (type (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Structure")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Structure")) (source direct))
      (supertype (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Structure")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (path (named (kind package) (name "Views")) (named (kind view) (name "satisfiesSomethingThatIsNotAViewpoint")) (anonymous (kind satisfy) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::satisfiesSomethingThatIsNotAViewpoint")))
    )
    (declaration (id (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::structure")))
      (type (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Structure")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Structure")) (source direct))
      (supertype (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Structure")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/view_conformance.md") (range (start 4 25) (end 4 33)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::concerns"))) (kind featureTyping) (ordinal 0) (authored-target "Concerns")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Concerns")))))
    )
  )
  (query (document "memory://snapshot/view_conformance.md") (range (start 8 16) (end 8 24)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/view_conformance.md") (path (named (kind package) (name "Views")) (named (kind view) (name "conforming")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "concerns")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::concerns")))))
    )
  )
  (query (document "memory://snapshot/view_conformance.md") (range (start 15 36) (end 15 40)) (probe (position 15 36))
    (reference (id (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::conformingRendering"))) (kind featureTyping) (ordinal 0) (authored-target "Tree")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Tree")))))
    )
  )
  (query (document "memory://snapshot/view_conformance.md") (range (start 16 52) (end 16 61)) (probe (position 16 52))
    (reference (id (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::rendersSomethingThatIsNotARendering"))) (kind featureTyping) (ordinal 0) (authored-target "Structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Structure")))))
    )
  )
  (query (document "memory://snapshot/view_conformance.md") (range (start 12 16) (end 12 25)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/view_conformance.md") (path (named (kind package) (name "Views")) (named (kind view) (name "satisfiesSomethingThatIsNotAViewpoint")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::structure")))))
    )
  )
  (query (document "memory://snapshot/view_conformance.md") (range (start 2 21) (end 2 30)) (probe (position 2 21))
    (reference (id (source (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::structure"))) (kind featureTyping) (ordinal 0) (authored-target "Structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_conformance.md") (qualified-name "Views::Structure")))))
    )
  )
)
~~~
