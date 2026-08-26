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
type=file
~~~
# SOURCE
~~~sysml
package Ends {
    part def Thing;
    connection def Restrictions {
        // Conforming: an end feature carrying none of the forbidden restrictions.
        end plain : Thing;

        // Invalid: `derived` and `abstract` are forbidden on an end feature. KerML's own
        // `EndFeaturePrefix` spells only `const? end`, so the spelling that authors a
        // restriction beside `end` is SysML's `DefaultReferenceUsage` (`( 'end' )? RefPrefix
        // UsageDeclaration`), whose `RefPrefix` has no `composite` or `portion` slot: those two
        // restrictions have no textual spelling in either language.
        end derived derivedEnd : Thing;
        end abstract abstractEnd : Thing;
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
        (range (start 11 8) (end 11 39))
      )
      (diagnostic
        (severity warning)
        (code "end_feature_invalid_restrictions")
        (source "semantic")
        (range (start 12 8) (end 12 41))
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
        (range (start 11 8) (end 11 39))
      )
      (diagnostic
        (severity warning)
        (code "end_feature_invalid_restrictions")
        (source "semantic")
        (range (start 12 8) (end 12 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:7adff2768985094be47ecb59e3117f6d452c0c72f50f65aaf3ca1d7ceff3617b") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd"))) (kind connection) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (positional-end 2)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::derivedEnd"))) (kind connection) (membership (kind feature) (visibility default)) (facts (modifiers derived) (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::derivedEnd"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd"))) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::derivedEnd"))) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::derivedEnd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd"))) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::derivedEnd"))) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions")))
      (positional-ends (authored 3) (effective 3))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd")))
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
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::derivedEnd")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_end_restrictions.md") (range (start 12 35) (end 12 40)) (probe (position 12 35))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::abstractEnd"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_end_restrictions.md") (range (start 11 33) (end 11 38)) (probe (position 11 33))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::derivedEnd"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_end_restrictions.md") (range (start 4 20) (end 4 25)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Restrictions::plain"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_restrictions.md") (qualified-name "Ends::Thing")))))
    )
  )
)
~~~
