# META
~~~ini
description=A typed feature's implicitly redefined member publishes inherited scope for its nested body
type=file
~~~
# SOURCE
~~~sysml
package TypedFeatureScope {
    item def Payload {
        item nested;
    }
    connection def Carrier {
        end item source : Payload;
        end item target;
    }
    connection invocation : Carrier {
        end item source {
            item selected :>> nested;
        }
        end item target;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:69f38172612593652b65702de4ff7661156a895a49eee6661690ca6f670c69e9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::source"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Payload")))))
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::target"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers end)))
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload::nested"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Carrier")))))
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers end)))
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source::selected"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "nested")))))
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::target"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers end)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Payload")
      (outcome (status resolved) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload")))))
    (reference (id (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation"))) (kind featureTyping) (ordinal 0))
      (authored-target "Carrier")
      (outcome (status resolved) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier")))))
    (reference (id (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source::selected"))) (kind redefinition) (ordinal 0))
      (authored-target "nested")
      (outcome (status resolved) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload::nested")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::source"))) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation"))) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source::selected"))) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload::nested"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source::selected"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::source"))) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::target"))) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload::nested"))) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source"))) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::source"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source"))) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source::selected"))) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::target"))) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::target"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::target"))) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier")))
      (subtype (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::source")))
      (featured-by (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier")))
      (type (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload")) (provenance authored))
      (effective-type (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload")) (source direct))
      (supertype (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload")) (scopes any))
      (subtype (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::target")))
      (featured-by (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier")))
      (subtype (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::target")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload")))
      (subtype (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::source")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload::nested")))
      (featured-by (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload")))
      (subtype (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source::selected")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation")))
      (type (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier")) (provenance authored))
      (effective-type (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier")) (source direct))
      (supertype (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source")))
      (featured-by (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation")))
      (effective-type (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload")) (source inherited) (from (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::source"))))
      (supertype (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source::selected")))
      (featured-by (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source")))
      (supertype (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload::nested")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::target")))
      (featured-by (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation")))
      (supertype (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::target")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (range (start 5 26) (end 5 33)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier::source"))) (kind featureTyping) (ordinal 0) (authored-target "Payload")
      (outcome (status resolved) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload")))))
    )
  )
  (query (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (range (start 8 28) (end 8 35)) (probe (position 8 28))
    (reference (id (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation"))) (kind featureTyping) (ordinal 0) (authored-target "Carrier")
      (outcome (status resolved) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Carrier")))))
    )
  )
  (query (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (range (start 10 30) (end 10 36)) (probe (position 10 30))
    (reference (id (source (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::invocation::source::selected"))) (kind redefinition) (ordinal 0) (authored-target "nested")
      (outcome (status resolved) (target (node (document "memory://snapshot/typed_feature_implicit_redefinition_scope.md") (qualified-name "TypedFeatureScope::Payload::nested")))))
    )
  )
)
~~~
