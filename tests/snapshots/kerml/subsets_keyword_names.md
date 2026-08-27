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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 1 21) (end 1 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 19) (end 2 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 23) (end 2 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:870cf7bf1d066adc36b27b6839d7bcafa39bd146446f9bf73557b640a5c403bc"))
  (declarations
    (declaration (id (node (document "memory://snapshot/subsets_keyword_names.md") (qualified-name "f1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "step")))))
    (declaration (id (node (document "memory://snapshot/subsets_keyword_names.md") (qualified-name "f2"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "do")) (subsetting (reference "step")))))
    (declaration (id (node (document "memory://snapshot/subsets_keyword_names.md") (qualified-name "s1"))) (kind kerml-step) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "step")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/subsets_keyword_names.md") (qualified-name "f1"))) (kind redefinition) (ordinal 0))
      (authored-target "step")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/subsets_keyword_names.md") (qualified-name "f2"))) (kind subsetting) (ordinal 0))
      (authored-target "do")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/subsets_keyword_names.md") (qualified-name "f2"))) (kind subsetting) (ordinal 1))
      (authored-target "step")
      (outcome (status unresolved)))
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
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/subsets_keyword_names.md") (range (start 1 21) (end 1 25)) (probe (position 1 21))
    (reference (id (source (node (document "memory://snapshot/subsets_keyword_names.md") (qualified-name "f1"))) (kind redefinition) (ordinal 0) (authored-target "step")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/subsets_keyword_names.md") (range (start 2 19) (end 2 21)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/subsets_keyword_names.md") (qualified-name "f2"))) (kind subsetting) (ordinal 0) (authored-target "do")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/subsets_keyword_names.md") (range (start 2 23) (end 2 27)) (probe (position 2 23))
    (reference (id (source (node (document "memory://snapshot/subsets_keyword_names.md") (qualified-name "f2"))) (kind subsetting) (ordinal 1) (authored-target "step")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/subsets_keyword_names.md") (range (start 0 16) (end 0 20)) (probe (position 0 16))
    (reference (id (source (node (document "memory://snapshot/subsets_keyword_names.md") (qualified-name "s1"))) (kind subsetting) (ordinal 0) (authored-target "step")
      (outcome (status unresolved)))
    )
  )
)
~~~
