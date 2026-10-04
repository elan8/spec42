# META
~~~ini
description=Part definition and usage parse with and without a body, including nested part definitions
type=file
~~~
# SOURCE
~~~sysml
package P {
    part def Bare;
    part def Braced { }
    part def Vehicle {
        part engine : Bare;
        part wheel : Bare { }
        part def Nested;
    }
    part instance : Bare;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/part_def_with_and_without_body.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8d528f74439f4d6984457d7128ccd5e7b5d45c763c697802f724a0ea37918a8a"))
  (declarations
    (declaration (id (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Braced"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::Nested"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bare")))))
    (declaration (id (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::wheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bare")))))
    (declaration (id (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::instance"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bare")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bare")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")))))
    (reference (id (source (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::wheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bare")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")))))
    (reference (id (source (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::instance"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bare")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::engine"))) (target (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::wheel"))) (target (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::wheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::instance"))) (target (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::instance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::engine"))) (target (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::wheel"))) (target (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")))
      (subtype (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::engine")) (scopes any))
      (subtype (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::wheel")) (scopes any))
      (subtype (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::instance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::engine")))
      (featured-by (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle")))
      (type (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")) (provenance authored))
      (effective-type (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")) (source direct))
      (supertype (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::wheel")))
      (featured-by (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle")))
      (type (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")) (provenance authored))
      (effective-type (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")) (source direct))
      (supertype (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::instance")))
      (type (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")) (provenance authored))
      (effective-type (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")) (source direct))
      (supertype (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/part_def_with_and_without_body.md") (range (start 4 22) (end 4 26)) (probe (position 4 22))
    (reference (id (source (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Bare")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")))))
    )
  )
  (query (document "memory://snapshot/part_def_with_and_without_body.md") (range (start 5 21) (end 5 25)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Vehicle::wheel"))) (kind featureTyping) (ordinal 0) (authored-target "Bare")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")))))
    )
  )
  (query (document "memory://snapshot/part_def_with_and_without_body.md") (range (start 8 20) (end 8 24)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::instance"))) (kind featureTyping) (ordinal 0) (authored-target "Bare")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_def_with_and_without_body.md") (qualified-name "P::Bare")))))
    )
  )
)
~~~
