# META
~~~ini
description=KerML 8.3.3.3.4 checkFeatureOwnedCrossFeatureRedefinitionSpecialization requires a cross feature to subset matching redefined-end cross features
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureOwnedCrossFeatureRedefinitionSpecialization
type=file
~~~
# SOURCE
~~~kerml
package Redefinition {
    classifier A;
    classifier B;
    assoc Parent { end crossing [1] feature a : A; end feature b : B; }
    // Child's ends redefine Parent's positionally, so Child's owned cross feature subsets the
    // cross feature of the end it redefines.
    assoc Child :> Parent { end narrowed [1] feature x : A; end feature y : B; }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (redefinition-check (rule_id "kerml-1.0:8.3.3.3.4:checkFeatureOwnedCrossFeatureRedefinitionSpecialization") (outcome satisfied))
  (relationship (kind subsetting) (source "Redefinition::Child::x::narrowed") (target "Redefinition::Parent::a::crossing") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e5f44a6e62435f33a89becb23e9dcb5d9f689ae473f0f303e72282dbdb59cb75"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Parent")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (cross-feature-projection (cross-feature (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x::narrowed"))) (owned-cross-feature (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x::narrowed"))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x::narrowed"))) (kind kerml-end) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (cross-feature-projection (cross-feature (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a::crossing"))) (owned-cross-feature (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a::crossing"))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a::crossing"))) (kind kerml-end) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Parent")) (named (kind kerml-feature) (name "a")) (named (kind kerml-end) (name "crossing")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Parent")) (named (kind kerml-feature) (name "a")) (named (kind kerml-end) (name "crossing")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Parent")) (named (kind kerml-feature) (name "a")) (named (kind kerml-end) (name "crossing")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Parent")) (named (kind kerml-feature) (name "a")) (named (kind kerml-end) (name "crossing")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::y"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::b"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x::narrowed"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x::narrowed"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a::crossing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::y"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::y"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::b"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a::crossing"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Parent")) (named (kind kerml-feature) (name "a")) (named (kind kerml-end) (name "crossing")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Parent")) (named (kind kerml-feature) (name "a")) (named (kind kerml-end) (name "crossing")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::b"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x::narrowed")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a::crossing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::y")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::b")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child")))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a"))))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x::narrowed")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a::crossing"))))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a::crossing")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Child")) (named (kind kerml-feature) (name "x")) (named (kind kerml-end) (name "narrowed")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::y")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::b"))))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::b")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a::crossing")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x::narrowed")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Parent")) (named (kind kerml-feature) (name "a")) (named (kind kerml-end) (name "crossing")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-association) (name "Parent")) (named (kind kerml-feature) (name "a")) (named (kind kerml-end) (name "crossing")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::b")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::y")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (range (start 6 19) (end 6 25)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child"))) (kind specialization) (ordinal 0) (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (range (start 6 57) (end 6 58)) (probe (position 6 57))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::x"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (range (start 6 76) (end 6 77)) (probe (position 6 76))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Child::y"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (range (start 3 48) (end 3 49)) (probe (position 3 48))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (range (start 3 67) (end 3 68)) (probe (position 3 67))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::Parent::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_redefinition_specialization.md") (qualified-name "Redefinition::B")))))
    )
  )
)
~~~
