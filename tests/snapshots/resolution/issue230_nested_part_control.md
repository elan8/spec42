# META
~~~ini
description=A nested part in an attribute definition lowers through the canonical part usage owner; a quoted frame name preserves its identity without treating a keyword as a name.
type=file
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package P { attribute def Show { part 'frame' : Frame; attribute def Frame; } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind feature_typing) (source "P::Show::frame")
    (target "P::Show::Frame") (provenance authored) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/issue230_nested_part_control.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "incompatible_type_kind")
        (source "semantic")
        (range (start 0 48) (end 0 53))
        (related-information
          (related
            (uri "memory://snapshot/issue230_nested_part_control.md")
            (range (start 0 55) (end 0 75))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3d2330d2187edbc482c2ce619bd900e3637c20107c909d239b3afbc96779767d"))
  (declarations
    (declaration (id (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::Frame"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::frame"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Frame")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::frame"))) (kind featureTyping) (ordinal 0))
      (authored-target "Frame")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::Frame")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::frame"))) (target (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::Frame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::frame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::frame"))) (target (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::Frame")))
      (subtype (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::frame")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::frame")))
      (featured-by (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show")))
      (type (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::Frame")) (provenance authored))
      (effective-type (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::Frame")) (source direct))
      (supertype (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::Frame")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/issue230_nested_part_control.md") (range (start 0 48) (end 0 53)) (probe (position 0 48))
    (reference (id (source (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::frame"))) (kind featureTyping) (ordinal 0) (authored-target "Frame")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_nested_part_control.md") (qualified-name "P::Show::Frame")))))
    )
  )
)
~~~
