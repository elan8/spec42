# META
~~~ini
description=KerML Mass Roll-up: MassRollup_1
type=file
~~~
# SOURCE
~~~kerml
package MassRollup_1 {
	private import NumericalFunctions::*;

	class MassedThing {
		feature mass : ScalarValues::Real;	
		composite subcomponents: MassedThing[0..*];

		feature totalMass : ScalarValues::Real = 
			mass + sum(subcomponents.totalMass);
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "mass_rollup_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5921eb11629ca2daccaacc5c585ac41fcaf59c110b0d81d55446501f9f58037b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MassRollup_1"))) (kind "package") (name "MassRollup_1") (declared-name "MassRollup_1"))
    (element (id (node (document "d0") (qualified-name "MassRollup_1::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "MassRollup_1"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MassRollup_1::MassedThing"))) (kind "classifier decl") (name "MassedThing") (declared-name "MassedThing") (parent (node (document "d0") (qualified-name "MassRollup_1"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MassRollup_1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "MassRollup_1::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions::*")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
