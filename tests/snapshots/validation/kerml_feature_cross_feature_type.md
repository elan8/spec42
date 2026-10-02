# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureCrossFeatureType requires the crossFeature of a Feature to have the same types as the Feature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureCrossFeatureType
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.4:validateFeatureCrossFeatureType
type=file
~~~
# SOURCE
~~~kerml
package Crossings {
    classifier C1 {
        feature a : C2;
    }
    classifier C2 {
        feature b : C1;
        feature wrong : C2;
    }
    assoc Conforming {
        // Conforming: each crossFeature has the same type as its end feature.
        end feature x : C1 crosses y.b;
        end feature y : C2 crosses x.a;
    }
    assoc Invalid {
        // Invalid: the crossFeature `wrong` is typed by C2, the end feature by C1 (and, through the
        // crossing, also C2, so it has more than one type).
        end feature x : C1 crosses y.wrong;
        end feature y : C2 crosses x.a;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_cross_feature_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "association_end_type_not_one")
        (source "semantic")
        (range (start 16 8) (end 16 43))
      )
      (diagnostic
        (severity warning)
        (code "cross_feature_type_mismatch")
        (source "semantic")
        (range (start 16 35) (end 16 42))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_cross_feature_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "association_end_type_not_one")
        (source "semantic")
        (range (start 16 8) (end 16 43))
      )
      (diagnostic
        (severity warning)
        (code "cross_feature_type_mismatch")
        (source "semantic")
        (range (start 16 35) (end 16 42))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:38ebf2dd96c18b84d80e00359cb991057f0d90f722c6a89d7a5422cdbad275e8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C2")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C1")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::wrong"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C2")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C1")) (crossSubsetting (reference "y::b")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C2")) (crossSubsetting (reference "x::a")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C1")) (crossSubsetting (reference "y::wrong")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C2")) (crossSubsetting (reference "x::a")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "C2")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::wrong"))) (kind featureTyping) (ordinal 0))
      (authored-target "C2")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::x"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "y::b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::b")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "C2")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::y"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "x::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::x"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "y::wrong")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::wrong")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "C2")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::y"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "x::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::b"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::wrong"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::wrong"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::x"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::x"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::x"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::y"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::y"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::y"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::x"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::x"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::wrong"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::x"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::y"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::y"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::y"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::b"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::wrong"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::x"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::y"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::x"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::y"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::b")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::x")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::x")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a")))
      (featured-by (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")))
      (type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::y")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::y")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::wrong")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::y")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::y")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::b")))
      (featured-by (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")))
      (type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::wrong")))
      (featured-by (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")))
      (type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::x")))
      (featured-by (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming")))
      (type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::b"))))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::b")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::y")))
      (featured-by (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming")))
      (type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a"))))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::x")))
      (featured-by (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid")))
      (type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::wrong"))))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::wrong")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::y")))
      (featured-by (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid")))
      (type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a"))))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 2 20) (end 2 22)) (probe (position 2 20))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a"))) (kind featureTyping) (ordinal 0) (authored-target "C2")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 5 20) (end 5 22)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::b"))) (kind featureTyping) (ordinal 0) (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 6 24) (end 6 26)) (probe (position 6 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::wrong"))) (kind featureTyping) (ordinal 0) (authored-target "C2")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 10 24) (end 10 26)) (probe (position 10 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::x"))) (kind featureTyping) (ordinal 0) (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 10 35) (end 10 38)) (probe (position 10 35))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::x"))) (kind crossSubsetting) (ordinal 0) (authored-target "y::b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::b")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 11 24) (end 11 26)) (probe (position 11 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::y"))) (kind featureTyping) (ordinal 0) (authored-target "C2")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 11 35) (end 11 38)) (probe (position 11 35))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Conforming::y"))) (kind crossSubsetting) (ordinal 0) (authored-target "x::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 16 24) (end 16 26)) (probe (position 16 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::x"))) (kind featureTyping) (ordinal 0) (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 16 35) (end 16 42)) (probe (position 16 35))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::x"))) (kind crossSubsetting) (ordinal 0) (authored-target "y::wrong")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2::wrong")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 17 24) (end 17 26)) (probe (position 17 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::y"))) (kind featureTyping) (ordinal 0) (authored-target "C2")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C2")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 17 35) (end 17 38)) (probe (position 17 35))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Invalid::y"))) (kind crossSubsetting) (ordinal 0) (authored-target "x::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::C1::a")))))
    )
  )
)
~~~
