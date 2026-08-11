# META
~~~ini
description=Incomplete part definition with unmatched braces - formatter adds compensating braces
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
package AyPkpowerTrain {
    part engine {
        g { }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "incomplete_part_unmatched_braces.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "sysml")
        (range (start 2 13) (end 2 14))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
Ident,OpenCurly,CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'AyPkpowerTrain'
    (part_usage 'engine'
      (default_ref_usage 'g'))))
~~~
# EXPECTED
~~~
parse.expected_close_curly
parse.expected_close_curly
~~~
# PROBLEMS
~~~
parse.expected_close_curly
parse.expected_close_curly
~~~
# FORMAT
~~~sysml
package AyPkpowerTrain {
    part engine {
        g { }

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "8433b897766392db984a54288bbf7143b364edb565325e63c3e63cb7531afc30") (contract-version "canonical-resolution-v1"))
  (structure
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
