# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureChainingFeatureConformance requires each chainingFeature after the first to be featured within the previous chainingFeature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureChainingFeatureConformance
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.4:validateFeatureChainingFeatureConformance
type=file
~~~
# SOURCE
~~~kerml
package Chains {
    classifier Thing {
        feature inner : Thing;
    }
    classifier Other;
    package Loose {
        feature unrelated : Thing featured by Other;
        feature free : Thing;
    }
    classifier Holder {
        feature outer : Thing {
            // Brings Loose's features into outer's visible memberships.
            public import Loose::*;
        }

        // Conforming: inner is featured by Thing, which outer conforms to.
        feature good chains outer.inner;

        // Conforming: free has no featuring type, so it is featured within anything.
        feature anywhere chains outer.free;

        // Invalid: unrelated resolves through outer's import, but it is featured by Other,
        // which outer does not conform to.
        feature bad chains outer.unrelated;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "feature_chaining_not_featured_within_previous")
        (source "semantic")
        (range (start 23 27) (end 23 42))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "feature_chaining_not_featured_within_previous")
        (source "semantic")
        (range (start 23 27) (end 23 42))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:81555965749da33ff6dbd3c994e4321353a47f6fb6bb2677b5e80cbab084e4ef"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::anywhere"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureChaining (reference "outer::free")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::bad"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureChaining (reference "outer::unrelated")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::good"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureChaining (reference "outer::inner")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (path (named (kind package) (name "Chains")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "outer")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Loose") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::free"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::unrelated"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (typeFeaturing (reference "Other")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Other"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::anywhere"))) (kind featureChaining) (ordinal 0))
      (authored-target "outer::free")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::free")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::bad"))) (kind featureChaining) (ordinal 0))
      (authored-target "outer::unrelated")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::unrelated")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::good"))) (kind featureChaining) (ordinal 0))
      (authored-target "outer::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (path (named (kind package) (name "Chains")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "outer")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Loose")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::free"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::unrelated"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::unrelated"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Other")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))))
  )
  (relationships
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::anywhere"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::free"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::anywhere"))) (kind featureChaining) (ordinal 0)))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::bad"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::unrelated"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::bad"))) (kind featureChaining) (ordinal 0)))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::good"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::good"))) (kind featureChaining) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::free"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::free"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::unrelated"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::unrelated"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::unrelated"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Other"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::unrelated"))) (kind typeFeaturing) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::anywhere"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::bad"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::good"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::anywhere")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::free")))
      (type (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::unrelated")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Other")))
      (type (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::free")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::unrelated")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))
      (type (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (range (start 19 32) (end 19 42)) (probe (position 19 32))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::anywhere"))) (kind featureChaining) (ordinal 0) (authored-target "outer::free")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::free")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (range (start 23 27) (end 23 42)) (probe (position 23 27))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::bad"))) (kind featureChaining) (ordinal 0) (authored-target "outer::unrelated")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::unrelated")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (range (start 16 28) (end 16 39)) (probe (position 16 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::good"))) (kind featureChaining) (ordinal 0) (authored-target "outer::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (range (start 10 24) (end 10 29)) (probe (position 10 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (range (start 12 26) (end 12 34)) (probe (position 12 26))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (path (named (kind package) (name "Chains")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "outer")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Loose")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (range (start 7 23) (end 7 28)) (probe (position 7 23))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::free"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (range (start 6 28) (end 6 33)) (probe (position 6 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::unrelated"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (range (start 6 46) (end 6 51)) (probe (position 6 46))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Loose::unrelated"))) (kind typeFeaturing) (ordinal 0) (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Other")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (range (start 2 24) (end 2 29)) (probe (position 2 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))))
    )
  )
)
~~~
