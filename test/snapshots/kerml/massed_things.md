# META
~~~ini
description=KerML Massed Thing: MassedThings
type=file
~~~
# SOURCE
~~~kerml
private import ScalarValues::*;
package MassedThings {
	
	public class MassedThing {
		public name: String;
		public mass: Real = 0;
	}
	
	public assoc MassedThingAssembly {
		public end [0..1] feature assembly: MassedThing;
		public end [0..*] feature parts: MassedThing;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "massed_things.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 15) (end 0 27))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "bfd756aa831fb14bc04da32190ccba00be96a9ad77fa859085e757c5c628f754") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "*"))) (kind "import") (name "*") (declared-name "*") (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MassedThings"))) (kind "package") (name "MassedThings") (declared-name "MassedThings"))
    (element (id (node (document "d0") (qualified-name "MassedThings::MassedThing"))) (kind "classifier decl") (name "MassedThing") (declared-name "MassedThing") (parent (node (document "d0") (qualified-name "MassedThings"))))
    (element (id (node (document "d0") (qualified-name "MassedThings::MassedThingAssembly"))) (kind "kermlDecl") (name "MassedThingAssembly") (declared-name "MassedThingAssembly") (parent (node (document "d0") (qualified-name "MassedThings"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 0 15) (end 0 27)) (probe (position 0 15))
      (reference
        (source (document "d0") (qualified-name "*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 0 15) (end 0 27))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
