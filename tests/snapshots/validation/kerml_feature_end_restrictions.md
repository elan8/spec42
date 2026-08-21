# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureEndNotDerivedAbstractCompositeOrPortion requires a Feature with isEnd = true to have isDerived, isAbstract, isComposite and isPortion all false
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureEndNotDerivedAbstractCompositeOrPortion
type=file
skip_validation=end_feature_invalid_restrictions is reported for derived, abstract and composite end features but not for a portion end feature, whose (modifiers end portion) fact is published without a diagnostic
~~~
# SOURCE
~~~kerml
package Ends {
    classifier Thing;
    assoc Restrictions {
        // Conforming: an end feature carrying none of the forbidden restrictions.
        end feature plain : Thing;

        // Invalid: each of the four restrictions is forbidden on an end feature.
        derived end feature derivedEnd : Thing;
        abstract end feature abstractEnd : Thing;
        composite end feature compositeEnd : Thing;
        portion end feature portionEnd : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_end_restrictions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "end_feature_invalid_restrictions")
        (source "semantic")
        (range (start 7 8) (end 7 47))
      )
      (diagnostic
        (severity warning)
        (code "end_feature_invalid_restrictions")
        (source "semantic")
        (range (start 8 8) (end 8 49))
      )
      (diagnostic
        (severity warning)
        (code "end_feature_invalid_restrictions")
        (source "semantic")
        (range (start 9 8) (end 9 51))
      )
      (diagnostic
        (severity warning)
        (code "end_feature_invalid_restrictions")
        (source "semantic")
        (range (start 10 8) (end 10 47))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_end_restrictions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "end_feature_invalid_restrictions")
        (source "semantic")
        (range (start 7 8) (end 7 47))
      )
      (diagnostic
        (severity warning)
        (code "end_feature_invalid_restrictions")
        (source "semantic")
        (range (start 8 8) (end 8 49))
      )
      (diagnostic
        (severity warning)
        (code "end_feature_invalid_restrictions")
        (source "semantic")
        (range (start 9 8) (end 9 51))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:388c34b8c2533c1897a3bb14d435522341fa783d7150e41218719c7b745c0fd1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers abstract end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::compositeEnd"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::derivedEnd"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers derived end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::portionEnd"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end portion)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::compositeEnd"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::derivedEnd"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::portionEnd"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd"))) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::compositeEnd"))) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::compositeEnd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::derivedEnd"))) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::derivedEnd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::portionEnd"))) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::portionEnd"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions")))
      (type (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::compositeEnd")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions")))
      (type (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::derivedEnd")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions")))
      (type (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions")))
      (type (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::portionEnd")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions")))
      (type (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::compositeEnd")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::derivedEnd")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::portionEnd")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_end_restrictions.md") (range (start 8 43) (end 8 48)) (probe (position 8 43))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_end_restrictions.md") (range (start 9 45) (end 9 50)) (probe (position 9 45))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::compositeEnd"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_end_restrictions.md") (range (start 7 41) (end 7 46)) (probe (position 7 41))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::derivedEnd"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_end_restrictions.md") (range (start 4 28) (end 4 33)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_end_restrictions.md") (range (start 10 41) (end 10 46)) (probe (position 10 41))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::portionEnd"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    )
  )
)
~~~
