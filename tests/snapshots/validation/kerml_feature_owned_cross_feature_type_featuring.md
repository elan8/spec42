# META
~~~ini
description=KerML 8.3.3.3.4 checkFeatureOwnedCrossFeatureTypeFeaturing requires an owned crossFeature to have featuringTypes consistent with the other ends
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureOwnedCrossFeatureTypeFeaturing
type=file
~~~
# SOURCE
~~~kerml
package Crossings {
    classifier Thing;
    classifier Other;
    assoc Link {
        end feature source : Thing;
        // The owned cross feature crossTarget is featured by the other end's type.
        end crossTarget [0..1] feature target : Other;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind type_featuring)
    (source "Crossings::Link::target::crossTarget")
    (target "Crossings::Thing")
    (provenance implied)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:fbe6327f2920cb60507118bed372ecc39f5cb8184295af4013b50de9034b7b32"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (cross-feature-projection (cross-feature (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target::crossTarget"))) (owned-cross-feature (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target::crossTarget"))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Other")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target::crossTarget"))) (kind kerml-end) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Other"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Other")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Other"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target::crossTarget"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Other"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target::crossTarget"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Other")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Other")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Other")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target::crossTarget")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Other")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Other")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Other")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (path (named (kind package) (name "Crossings")) (named (kind kerml-association) (name "Link")) (named (kind kerml-feature) (name "target")) (named (kind kerml-end) (name "crossTarget")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Other")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target::crossTarget")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (range (start 4 29) (end 4 34)) (probe (position 4 29))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (range (start 6 48) (end 6 53)) (probe (position 6 48))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0) (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Other")))))
    )
  )
)
~~~
