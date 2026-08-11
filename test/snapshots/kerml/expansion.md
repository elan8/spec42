# META
~~~ini
description=KerML Simple Tests: Expansion
type=file
~~~
# SOURCE
~~~kerml
package Expansion {
	private import ControlFunctions::select;
	feature x = x->select {in y; in w; in z; w+1}; 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "expansion.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 40))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 2 46) (end 2 49))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "6fcd1a00da42419198d6a99ff64a62349b3071a24702933587ea82bbb87177b7") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Expansion"))) (kind "package") (name "Expansion") (declared-name "Expansion"))
    (element (id (node (document "d0") (qualified-name "Expansion::select"))) (kind "import") (name "select") (declared-name "select") (parent (node (document "d0") (qualified-name "Expansion"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::select") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Expansion::x"))) (kind "feature decl") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "Expansion"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Expansion::select"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::select") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
  (document "d0"
    (query (range (start 1 16) (end 1 40)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Expansion::select"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::select")
        (range (start 1 16) (end 1 40))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
