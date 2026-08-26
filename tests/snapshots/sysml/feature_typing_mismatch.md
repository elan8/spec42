# META
~~~ini
description=SysML Feature Typing Kind Mismatch (SC-4)
type=file
~~~
# SOURCE
~~~sysml
attribute def Foo {}
part p : Foo;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/feature_typing_mismatch.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "incompatible_type_kind")
        (source "semantic")
        (range (start 1 9) (end 1 12))
        (related-information
          (related
            (uri "memory://snapshot/feature_typing_mismatch.md")
            (range (start 0 0) (end 0 20))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:a0a856788cf1ca3fd0dda5bb3371ffcf886813008ca1718209210cbd802e6668") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "Foo"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Foo")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "p"))) (kind featureTyping) (ordinal 0))
      (authored-target "Foo")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "Foo")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "p"))) (target (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "Foo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "p"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "Foo")))
      (subtype (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "p")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "p")))
      (type (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "Foo")) (provenance authored))
      (effective-type (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "Foo")) (source direct))
      (supertype (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "Foo")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/feature_typing_mismatch.md") (range (start 1 9) (end 1 12)) (probe (position 1 9))
    (reference (id (source (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "p"))) (kind featureTyping) (ordinal 0) (authored-target "Foo")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_typing_mismatch.md") (qualified-name "Foo")))))
    )
  )
)
~~~
