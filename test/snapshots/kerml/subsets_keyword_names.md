# META
~~~ini
description=Permissive QN parsing: keywords used as specialization target names
type=file
~~~
# SOURCE
~~~kerml
step s1 subsets step;
feature f1 redefines step;
feature f2 subsets do, step;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/subsets_keyword_names.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 0 16) (end 0 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 0) (end 1 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 2 0) (end 2 28))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:870cf7bf1d066adc36b27b6839d7bcafa39bd146446f9bf73557b640a5c403bc") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/subsets_keyword_names.md") (qualified-name "s1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "step"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/subsets_keyword_names.md") (qualified-name "s1"))) (kind subsetting) (ordinal 0))
      (authored-target "step")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/subsets_keyword_names.md") (range (start 0 16) (end 0 20)) (probe (position 0 16))
    (reference (id (source (node (document "memory://snapshot/subsets_keyword_names.md") (qualified-name "s1"))) (kind subsetting) (ordinal 0) (authored-target "step")
      (outcome (status unresolved)))
  )
)
~~~
