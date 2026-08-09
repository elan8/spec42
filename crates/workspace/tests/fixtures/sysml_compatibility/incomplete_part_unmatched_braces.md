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
# FORMAT
~~~sysml
package AyPkpowerTrain {
    part engine {
        g { }
    }
}
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
# SMG
~~~
(semantic-graph
  (containment
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
