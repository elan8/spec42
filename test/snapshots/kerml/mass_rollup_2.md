# META
~~~ini
description=KerML Mass Roll-up: MassRollup_2
type=file
~~~
# SOURCE
~~~kerml
package MassRollup_2 {
	private import NumericalFunctions::*;
	private import ISQ::*;
	
	class MassedThing {
		feature mass : ScalarValues::Real; 
		feature totalMass : ScalarValues::Real =
			mass + sum(subcomponents.totalMass);
			
		feature subcomponents redefines massedThings;	
	}
	
	feature massedThings: MassedThing[0..*];

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "mass_rollup_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 19))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package MassRollup_2 {
	private import NumericalFunctions::*;
	private import ISQ::*;
	
	class MassedThing {
		feature mass : ScalarValues::Real; 
		feature totalMass : ScalarValues::Real =
			mass + sum(subcomponents.totalMass);
			
		feature subcomponents redefines massedThings;	
	}
	
	feature massedThings: MassedThing[0..*];

}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "eec7fd779b5b00169d1c01104c3687f91086c63c10ea6f2168061f2ee162baaa") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MassRollup_2"))) (kind "package") (name "MassRollup_2") (declared-name "MassRollup_2") (range (start (line 0) (character 0)) (end (line 0) (character 332))))
    (element (id (node (document "d0") (qualified-name "MassRollup_2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 38))) (parent (node (document "d0") (qualified-name "MassRollup_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "MassRollup_2::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 23))) (parent (node (document "d0") (qualified-name "MassRollup_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 19))))))
    (element (id (node (document "d0") (qualified-name "MassRollup_2::MassedThing"))) (kind "classifier decl") (name "MassedThing") (declared-name "MassedThing") (range (start (line 4) (character 1)) (end (line 4) (character 197))) (parent (node (document "d0") (qualified-name "MassRollup_2"))))
    (element (id (node (document "d0") (qualified-name "MassRollup_2::massedThings"))) (kind "feature decl") (name "massedThings") (declared-name "massedThings") (range (start (line 12) (character 1)) (end (line 12) (character 41))) (parent (node (document "d0") (qualified-name "MassRollup_2"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MassRollup_2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup_2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 2) (character 16)) (end (line 2) (character 19))) (outcome (status unresolved)))
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
    (query (range (start 2 16) (end 2 19)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "MassRollup_2::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 2 16) (end 2 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "MassRollup_2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions::*")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
