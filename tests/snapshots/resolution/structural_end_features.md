# META
~~~ini
description=KerML end-feature restrictions and redefinition of an end
type=file
~~~
# SOURCE
~~~kerml
package Ends {
    classifier Thing;

    assoc Association {
        // An end feature is neither derived, abstract nor composite. A plain one is fine.
        end feature plain : Thing;
        derived end feature computed : Thing;
        abstract end feature vague : Thing;
        composite end feature owned : Thing;
    }

    // Redefining an end requires an end. `kept` redefines `plain` and is one; `lost` redefines it
    // implicitly by name and is not.
    assoc Keeps specializes Association {
        end feature plain : Thing;
    }
    assoc Loses specializes Association {
        feature plain : Thing;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/structural_end_features.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "end_feature_invalid_prefix")
        (source "parser")
        (range (start 6 8) (end 7 8))
      )
      (diagnostic
        (severity error)
        (code "end_feature_invalid_prefix")
        (source "parser")
        (range (start 7 8) (end 8 8))
      )
      (diagnostic
        (severity error)
        (code "end_feature_invalid_prefix")
        (source "parser")
        (range (start 8 8) (end 9 4))
      )
      (diagnostic
        (severity warning)
        (code "redefinition_end_mismatch")
        (source "semantic")
        (range (start 17 8) (end 17 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:2ac66c3d094088d695b65a3898361493342d9c54b20d40fcdddd7a368074812c") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association::plain"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Association")))))
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps::plain"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Association")))))
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses::plain"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association::plain"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")))))
    (reference (id (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps"))) (kind specialization) (ordinal 0))
      (authored-target "Association")
      (outcome (status resolved) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association")))))
    (reference (id (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps::plain"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")))))
    (reference (id (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses"))) (kind specialization) (ordinal 0))
      (authored-target "Association")
      (outcome (status resolved) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association")))))
    (reference (id (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses::plain"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association::plain"))) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association::plain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps"))) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps::plain"))) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps::plain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses"))) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses::plain"))) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses::plain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association::plain"))) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps::plain"))) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association::plain"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps::plain"))) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses::plain"))) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association::plain"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses::plain"))) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association")))
      (subtype (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association::plain")))
      (featured-by (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association")))
      (type (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps::plain")) (scopes any feature))
      (subtype (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses::plain")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps")))
      (supertype (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps::plain")))
      (featured-by (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps")))
      (type (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")) (source inherited) (from (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association::plain"))))
      (supertype (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association::plain")) (scopes any feature))
      (supertype (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses")))
      (supertype (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses::plain")))
      (featured-by (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses")))
      (type (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")) (source inherited) (from (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association::plain"))))
      (supertype (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association::plain")) (scopes any feature))
      (supertype (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")))
      (subtype (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association::plain")) (scopes any))
      (subtype (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps::plain")) (scopes any))
      (subtype (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses::plain")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/structural_end_features.md") (range (start 5 28) (end 5 33)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association::plain"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")))))
    )
  )
  (query (document "memory://snapshot/structural_end_features.md") (range (start 13 28) (end 13 39)) (probe (position 13 28))
    (reference (id (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps"))) (kind specialization) (ordinal 0) (authored-target "Association")
      (outcome (status resolved) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association")))))
    )
  )
  (query (document "memory://snapshot/structural_end_features.md") (range (start 14 28) (end 14 33)) (probe (position 14 28))
    (reference (id (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Keeps::plain"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")))))
    )
  )
  (query (document "memory://snapshot/structural_end_features.md") (range (start 16 28) (end 16 39)) (probe (position 16 28))
    (reference (id (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses"))) (kind specialization) (ordinal 0) (authored-target "Association")
      (outcome (status resolved) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Association")))))
    )
  )
  (query (document "memory://snapshot/structural_end_features.md") (range (start 17 24) (end 17 29)) (probe (position 17 24))
    (reference (id (source (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Loses::plain"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/structural_end_features.md") (qualified-name "Ends::Thing")))))
    )
  )
)
~~~
