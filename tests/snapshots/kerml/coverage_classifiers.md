# META
~~~ini
description=Coverage: KerML classifier definition keywords (datatype, class, struct, assoc, metaclass)
type=file
~~~
# SOURCE
~~~kerml
package ClassifierCoverage {
    datatype Scalar;
    class Entity;
    struct Vector;
    assoc Relationship;
    assoc struct AssocStruct;
    metaclass MetaEntity;

    datatype D specializes Scalar;
    class C :> Entity;
    struct S specializes Vector;
    assoc A :> Relationship;
    metaclass M specializes MetaEntity;

    abstract datatype AbstractScalar;
    abstract struct AbstractVector;
    abstract assoc AbstractRel;
    abstract metaclass AbstractMeta;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/coverage_classifiers.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:a409938f8c5980767aa1917a6332c2122d4da7f70e11e69d18fdb0a20f2c3502") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::A"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Relationship")))))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::AbstractMeta"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::AbstractRel"))) (kind kerml-association) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::AbstractScalar"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::AbstractVector"))) (kind kerml-structure) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::AssocStruct"))) (kind kerml-association-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::C"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Entity")))))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::D"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Scalar")))))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Entity"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::M"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MetaEntity")))))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::MetaEntity"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Relationship"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::S"))) (kind kerml-structure) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vector")))))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Scalar"))) (kind kerml-datatype) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Vector"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::A"))) (kind specialization) (ordinal 0))
      (authored-target "Relationship")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Relationship")))))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::C"))) (kind specialization) (ordinal 0))
      (authored-target "Entity")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Entity")))))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::D"))) (kind specialization) (ordinal 0))
      (authored-target "Scalar")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Scalar")))))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::M"))) (kind specialization) (ordinal 0))
      (authored-target "MetaEntity")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::MetaEntity")))))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::S"))) (kind specialization) (ordinal 0))
      (authored-target "Vector")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Vector")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::A"))) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Relationship"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::A"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::C"))) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Entity"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::C"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::D"))) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Scalar"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::D"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::M"))) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::MetaEntity"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::M"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::S"))) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Vector"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::S"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::A")))
      (supertype (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Relationship")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::C")))
      (supertype (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Entity")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::D")))
      (supertype (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Scalar")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Entity")))
      (subtype (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::C")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::M")))
      (supertype (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::MetaEntity")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::MetaEntity")))
      (subtype (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::M")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Relationship")))
      (subtype (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::A")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::S")))
      (supertype (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Vector")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Scalar")))
      (subtype (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::D")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Vector")))
      (subtype (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::S")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_classifiers.md") (range (start 11 15) (end 11 27)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::A"))) (kind specialization) (ordinal 0) (authored-target "Relationship")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Relationship")))))
    )
  )
  (query (document "memory://snapshot/coverage_classifiers.md") (range (start 9 15) (end 9 21)) (probe (position 9 15))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::C"))) (kind specialization) (ordinal 0) (authored-target "Entity")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Entity")))))
    )
  )
  (query (document "memory://snapshot/coverage_classifiers.md") (range (start 8 27) (end 8 33)) (probe (position 8 27))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::D"))) (kind specialization) (ordinal 0) (authored-target "Scalar")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Scalar")))))
    )
  )
  (query (document "memory://snapshot/coverage_classifiers.md") (range (start 12 28) (end 12 38)) (probe (position 12 28))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::M"))) (kind specialization) (ordinal 0) (authored-target "MetaEntity")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::MetaEntity")))))
    )
  )
  (query (document "memory://snapshot/coverage_classifiers.md") (range (start 10 25) (end 10 31)) (probe (position 10 25))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::S"))) (kind specialization) (ordinal 0) (authored-target "Vector")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Vector")))))
    )
  )
)
~~~
