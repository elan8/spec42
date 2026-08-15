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
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:dc703b955cf7eb123eab106df1f01f499ff2c4cb83a66a167742c278ebe09a52") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::A"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::B"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::C"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::D"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::DiffType"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::InterType"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::UnionType"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::child"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::f"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::g"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::parent"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
