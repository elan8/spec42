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
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 4) (end 1 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 4) (end 4 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 6 4) (end 6 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 8 27) (end 8 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 11 15) (end 11 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 12 28) (end 12 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:a409938f8c5980767aa1917a6332c2122d4da7f70e11e69d18fdb0a20f2c3502") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::A"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Relationship"))))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::AbstractMeta"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::AbstractRel"))) (kind kerml-association) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::AbstractScalar"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::AbstractVector"))) (kind kerml-structure) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::AssocStruct"))) (kind kerml-association-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::C"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Entity"))))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::D"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Scalar"))))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Entity"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::M"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MetaEntity"))))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::S"))) (kind kerml-structure) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vector"))))
    (declaration (id (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Vector"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::A"))) (kind specialization) (ordinal 0))
      (authored-target "Relationship")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::C"))) (kind specialization) (ordinal 0))
      (authored-target "Entity")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Entity")))))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::D"))) (kind specialization) (ordinal 0))
      (authored-target "Scalar")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::M"))) (kind specialization) (ordinal 0))
      (authored-target "MetaEntity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::S"))) (kind specialization) (ordinal 0))
      (authored-target "Vector")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Vector")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::C"))) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Entity"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::C"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::S"))) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Vector"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::S"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_classifiers.md") (range (start 11 15) (end 11 27)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::A"))) (kind specialization) (ordinal 0) (authored-target "Relationship")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/coverage_classifiers.md") (range (start 9 15) (end 9 21)) (probe (position 9 15))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::C"))) (kind specialization) (ordinal 0) (authored-target "Entity")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Entity")))))
  )
  (query (document "memory://snapshot/coverage_classifiers.md") (range (start 8 27) (end 8 33)) (probe (position 8 27))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::D"))) (kind specialization) (ordinal 0) (authored-target "Scalar")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/coverage_classifiers.md") (range (start 12 28) (end 12 38)) (probe (position 12 28))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::M"))) (kind specialization) (ordinal 0) (authored-target "MetaEntity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/coverage_classifiers.md") (range (start 10 25) (end 10 31)) (probe (position 10 25))
    (reference (id (source (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::S"))) (kind specialization) (ordinal 0) (authored-target "Vector")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_classifiers.md") (qualified-name "ClassifierCoverage::Vector")))))
  )
)
~~~
