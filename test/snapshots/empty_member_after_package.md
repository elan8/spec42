# META
~~~ini
description=Empty member (bare semicolon) at file level after package
type=file
~~~
# SOURCE
~~~sysml
package MyPkg { }; in newX : Real;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "empty_member_after_package.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "sysml")
        (range (start 0 17) (end 0 34))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package MyPkg { }; in newX : Real;

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "4dcfb3745080896bd90a864c32b726a97a6a3f1cfd477732c79f67c29cb4febb") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MyPkg"))) (kind "package") (name "MyPkg") (declared-name "MyPkg") (range (start (line 0) (character 0)) (end (line 0) (character 17))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
