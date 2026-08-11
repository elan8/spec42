# META
~~~ini
description=KerML Simple Tests: Circular
type=file
~~~
# SOURCE
~~~kerml
package Circular {
	class A { }
	feature a: A;
	alias Circ for Circular;
	package P {
		public import Circular::*;
	}
	
	feature x :> z;
	feature y :> x;
	feature z :> y;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "circular.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Circular {
    class A { }
    feature a: A;
    alias Circ for Circular;
    package P {
        public import Circular::*;
    }

    feature x :> z;
    feature y :> x;
    feature z :> y;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "00aebb6ba004439ee30b63301928598e49736318f230bb516c5dbd7f3ceebae1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Circular"))) (kind "package") (name "Circular") (declared-name "Circular"))
    (element (id (node (document "d0") (qualified-name "Circular::A"))) (kind "classifier decl") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "Circular"))))
    (element (id (node (document "d0") (qualified-name "Circular::Circ"))) (kind "alias") (name "Circ") (declared-name "Circ") (parent (node (document "d0") (qualified-name "Circular"))))
    (element (id (node (document "d0") (qualified-name "Circular::P"))) (kind "package") (name "P") (declared-name "P") (parent (node (document "d0") (qualified-name "Circular"))))
    (element (id (node (document "d0") (qualified-name "Circular::P::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Circular::P"))) (authored (membership (kind Import) (visibility "public") (import (reference "Circular::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Circular::a"))) (kind "feature decl") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "Circular"))))
    (element (id (node (document "d0") (qualified-name "Circular::x"))) (kind "feature decl") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "Circular"))))
    (element (id (node (document "d0") (qualified-name "Circular::y"))) (kind "feature decl") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "Circular"))))
    (element (id (node (document "d0") (qualified-name "Circular::z"))) (kind "feature decl") (name "z") (declared-name "z") (parent (node (document "d0") (qualified-name "Circular"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Circular::P::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Circular::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "Circular")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
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
    (query (range (start 5 16) (end 5 24)) (probe (position 5 16))
      (reference
        (source (document "d0") (qualified-name "Circular::P::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Circular::*")
        (range (start 5 16) (end 5 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Circular") (range (start 0 0) (end 0 172)))
        )
      )
    )
  )
)
~~~
