# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureEndNotDerivedAbstractCompositeOrPortion requires a Feature with isEnd = true to have isDerived, isAbstract, isComposite and isPortion all false
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureEndNotDerivedAbstractCompositeOrPortion
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.4:validateFeatureEndNotDerivedAbstractCompositeOrPortion
blocked_by=parser-gap-67-end-restriction-modifiers
type=file
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
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 7 8) (end 8 8))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 8 8) (end 9 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 9 8) (end 10 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 10 8) (end 11 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:388c34b8c2533c1897a3bb14d435522341fa783d7150e41218719c7b745c0fd1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions")))
      (type (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_end_restrictions.md") (range (start 4 28) (end 4 33)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    )
  )
)
~~~
