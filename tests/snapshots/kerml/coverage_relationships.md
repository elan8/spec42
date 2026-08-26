# META
~~~ini
description=Coverage: Standalone relationship declarations (disjoining, typing, subsetting, redefinition)
type=file
~~~
# SOURCE
~~~kerml
package RelationshipCoverage {
    type A;
    type B;
    type C;
    type D;
    feature f;
    feature g;
    feature parent;
    feature child;

    disjoining d1 disjoint A from B;
    disjoint C from D;

    typing t1 typing f typed by B;
    typing g : A;

    subset parent subsets f;

    redefinition child :>> parent;
    redefinition f redefines g;

    type UnionType unions A, B;
    type InterType intersects A, B;
    type DiffType differences A, B;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/coverage_relationships.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 4) (end 10 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 11 4) (end 11 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 4) (end 13 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 14 4) (end 14 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 16 4) (end 16 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 18 4) (end 18 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 19 4) (end 19 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:dc703b955cf7eb123eab106df1f01f499ff2c4cb83a66a167742c278ebe09a52") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::A"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::B"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::C"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::D"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::DiffType"))) (kind kerml-type) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (differencing (reference "A")) (differencing (reference "B")))))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::InterType"))) (kind kerml-type) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (intersecting (reference "A")) (intersecting (reference "B")))))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::UnionType"))) (kind kerml-type) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (unioning (reference "A")) (unioning (reference "B")))))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::child"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::f"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::g"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::parent"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::DiffType"))) (kind differencing) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::A")))))
    (reference (id (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::DiffType"))) (kind differencing) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::B")))))
    (reference (id (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::InterType"))) (kind intersecting) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::A")))))
    (reference (id (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::InterType"))) (kind intersecting) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::B")))))
    (reference (id (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::UnionType"))) (kind unioning) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::A")))))
    (reference (id (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::UnionType"))) (kind unioning) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::B")))))
  )
  (relationships
    (relationship (kind differencing) (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::DiffType"))) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::DiffType"))) (kind differencing) (ordinal 0)))
    (relationship (kind differencing) (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::DiffType"))) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::DiffType"))) (kind differencing) (ordinal 1)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::InterType"))) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::InterType"))) (kind intersecting) (ordinal 0)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::InterType"))) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::InterType"))) (kind intersecting) (ordinal 1)))
    (relationship (kind unioning) (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::UnionType"))) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::UnionType"))) (kind unioning) (ordinal 0)))
    (relationship (kind unioning) (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::UnionType"))) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::UnionType"))) (kind unioning) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::DiffType")))
      (set-operand (operator difference) (ordinal 0) (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::A")))
      (set-operand (operator difference) (ordinal 1) (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::B")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::InterType")))
      (set-operand (operator intersection) (ordinal 0) (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::A")))
      (set-operand (operator intersection) (ordinal 1) (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::B")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::UnionType")))
      (set-operand (operator union) (ordinal 0) (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::A")))
      (set-operand (operator union) (ordinal 1) (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::B")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_relationships.md") (range (start 23 30) (end 23 31)) (probe (position 23 30))
    (reference (id (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::DiffType"))) (kind differencing) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::A")))))
    )
  )
  (query (document "memory://snapshot/coverage_relationships.md") (range (start 23 33) (end 23 34)) (probe (position 23 33))
    (reference (id (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::DiffType"))) (kind differencing) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::B")))))
    )
  )
  (query (document "memory://snapshot/coverage_relationships.md") (range (start 22 30) (end 22 31)) (probe (position 22 30))
    (reference (id (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::InterType"))) (kind intersecting) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::A")))))
    )
  )
  (query (document "memory://snapshot/coverage_relationships.md") (range (start 22 33) (end 22 34)) (probe (position 22 33))
    (reference (id (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::InterType"))) (kind intersecting) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::B")))))
    )
  )
  (query (document "memory://snapshot/coverage_relationships.md") (range (start 21 26) (end 21 27)) (probe (position 21 26))
    (reference (id (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::UnionType"))) (kind unioning) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::A")))))
    )
  )
  (query (document "memory://snapshot/coverage_relationships.md") (range (start 21 29) (end 21 30)) (probe (position 21 29))
    (reference (id (source (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::UnionType"))) (kind unioning) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::B")))))
    )
  )
)
~~~
