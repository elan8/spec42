# META
~~~ini
description=KerML 8.3.3.3.4 checkFeatureOwnedCrossFeatureRedefinitionSpecialization is satisfied when a redefined end's cross feature is the second chaining feature of its authored CrossSubsetting: the owned cross feature is implied to subset it
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureOwnedCrossFeatureRedefinitionSpecialization
coverage_role=secondary
type=file
~~~
# SOURCE
~~~kerml
package Redefinition {
    classifier A;
    classifier B { feature inner : A; }
    assoc Parent { end feature a : A crosses b.inner; end feature b : B; }
    assoc Child :> Parent { end narrowed [1] feature x : A; end feature y : B; }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (redefinition-check (rule_id "kerml-1.0:8.3.3.3.4:checkFeatureOwnedCrossFeatureRedefinitionSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9a22892273d3fea29b293a6a03988c2bbd90be50263168f80346b26be5946968"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Parent")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (cross-feature-projection (cross-feature (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x::narrowed"))) (owned-cross-feature (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x::narrowed"))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x::narrowed"))) (kind kerml-end) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")) (crossSubsetting (reference "b::inner")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "b::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::y"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::b"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x::narrowed"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x::narrowed"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x::narrowed"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::y"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::y"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::b"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::b"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x::narrowed")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::y")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::b")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x::narrowed")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child")))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner"))))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a"))))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x::narrowed")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner"))))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::y")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::b"))))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::b")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner"))))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::b")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::y")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (range (start 2 35) (end 2 36)) (probe (position 2 35))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (range (start 4 19) (end 4 25)) (probe (position 4 19))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child"))) (kind specialization) (ordinal 0) (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (range (start 4 57) (end 4 58)) (probe (position 4 57))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::x"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (range (start 4 76) (end 4 77)) (probe (position 4 76))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Child::y"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (range (start 3 35) (end 3 36)) (probe (position 3 35))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (range (start 3 45) (end 3 52)) (probe (position 3 45))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::a"))) (kind crossSubsetting) (ordinal 0) (authored-target "b::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B::inner")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (range (start 3 70) (end 3 71)) (probe (position 3 70))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::Parent::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization_authored_crossing.md") (qualified-name "Redefinition::B")))))
    )
  )
)
~~~
