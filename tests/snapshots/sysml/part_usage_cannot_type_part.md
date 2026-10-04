# META
~~~ini
description=Part usage typing requires a part definition
type=file
~~~
# SOURCE
~~~sysml
package Parts {
    part def Product { port spare; }
    part instance : Product;
    part invalid : instance;
    part valid : Product;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/part_usage_cannot_type_part.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "incompatible_type_kind")
        (source "semantic")
        (range (start 3 19) (end 3 27))
        (related-information
          (related
            (uri "memory://snapshot/part_usage_cannot_type_part.md")
            (range (start 2 4) (end 2 28))
          )
        )
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/part_usage_cannot_type_part.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "incompatible_type_kind")
        (source "semantic")
        (range (start 3 19) (end 3 27))
        (related-information
          (related
            (uri "memory://snapshot/part_usage_cannot_type_part.md")
            (range (start 2 4) (end 2 28))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d862541563d24c9501d0e3cc90781a8d1aae60f4b3289ff03f6892b0f7554880"))
  (declarations
    (declaration (id (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product::spare"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::instance"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
    (declaration (id (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::invalid"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "instance")))))
    (declaration (id (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::valid"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::instance"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product")))))
    (reference (id (source (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::invalid"))) (kind featureTyping) (ordinal 0))
      (authored-target "instance")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::instance")))))
    (reference (id (source (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::valid"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::instance"))) (target (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::instance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::invalid"))) (target (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::instance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::invalid"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::valid"))) (target (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::valid"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product::spare"))) (target (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product")))
      (subtype (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::instance")) (scopes any))
      (subtype (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::valid")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product::spare")))
      (featured-by (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product")))
    )
    (declaration (id (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::instance")))
      (type (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product")) (source direct))
      (supertype (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product")) (scopes any))
      (subtype (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::invalid")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::invalid")))
      (type (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::instance")) (provenance authored))
      (effective-type (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::instance")) (source direct))
      (supertype (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product")) (scopes any))
      (supertype (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::instance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::valid")))
      (type (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product")) (source direct))
      (supertype (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/part_usage_cannot_type_part.md") (range (start 2 20) (end 2 27)) (probe (position 2 20))
    (reference (id (source (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::instance"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product")))))
    )
  )
  (query (document "memory://snapshot/part_usage_cannot_type_part.md") (range (start 3 19) (end 3 27)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::invalid"))) (kind featureTyping) (ordinal 0) (authored-target "instance")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::instance")))))
    )
  )
  (query (document "memory://snapshot/part_usage_cannot_type_part.md") (range (start 4 17) (end 4 24)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::valid"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_usage_cannot_type_part.md") (qualified-name "Parts::Product")))))
    )
  )
)
~~~
