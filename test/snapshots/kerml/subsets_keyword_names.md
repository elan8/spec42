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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 0 0) (end 0 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 0 0) (end 0 21))
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:870cf7bf1d066adc36b27b6839d7bcafa39bd146446f9bf73557b640a5c403bc") (contract-version "parser-owned-resolution-v1"))
  (declarations
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
