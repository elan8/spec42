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
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 1 4) (end 5 4))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 10 4) (end 24 0))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:dc703b955cf7eb123eab106df1f01f499ff2c4cb83a66a167742c278ebe09a52") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::child"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::f"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::g"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_relationships.md") (qualified-name "RelationshipCoverage::parent"))) (kind default-reference) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
