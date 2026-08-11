# META
~~~ini
description=Empty member (bare semicolon) inside package body
type=file
~~~
# SOURCE
~~~sysml
package MyPkg {;}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "empty_member_in_package.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 0 15) (end 0 16))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "11a983feba5a56728927d8fab9766a65e7c413be19a1d78ca2f4560df1370cf3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MyPkg"))) (kind "package") (name "MyPkg") (declared-name "MyPkg"))
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
