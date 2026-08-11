# META
~~~ini
description=KerML KerML Spec Annex A: A-3-2-WithoutConnectors
type=file
~~~
# SOURCE
~~~kerml

package WithoutConnectorsModelToBeExecuted {
	doc
	/* 
	 */

	classifier Bicycle {
		feature rollsOn : Wheel [2];
		feature holdsWheel : BikeFork [*];
	}
	classifier Wheel;
	classifier BikeFork;
}

package WithoutConnectorsExecution {
	doc
	/* 
	 */

	private import Atoms::*;
	private import WithoutConnectorsModelToBeExecuted::*;

	#atom
	classifier MyWheel1 specializes Wheel;
	#atom
	classifier MyWheel2 specializes Wheel;

	classifier MyWheel unions MyWheel1, MyWheel2;

	#atom
	classifier MyBike specializes Bicycle {
		feature redefines rollsOn : MyWheel;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "a_3_2_without_connectors.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 21))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "801c0d29f3ae17c21169956b0a8c190f280a6e9f9669732d5dd25caec800517f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution"))) (kind "package") (name "WithoutConnectorsExecution") (declared-name "WithoutConnectorsExecution"))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "WithoutConnectorsModelToBeExecuted::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::MyBike"))) (kind "classifier decl") (name "MyBike") (declared-name "MyBike") (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::MyWheel"))) (kind "classifier decl") (name "MyWheel") (declared-name "MyWheel") (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::MyWheel1"))) (kind "classifier decl") (name "MyWheel1") (declared-name "MyWheel1") (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::MyWheel2"))) (kind "classifier decl") (name "MyWheel2") (declared-name "MyWheel2") (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted"))) (kind "package") (name "WithoutConnectorsModelToBeExecuted") (declared-name "WithoutConnectorsModelToBeExecuted"))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle"))) (kind "classifier decl") (name "Bicycle") (declared-name "Bicycle") (parent (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted::BikeFork"))) (kind "classifier decl") (name "BikeFork") (declared-name "BikeFork") (parent (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel"))) (kind "classifier decl") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "WithoutConnectorsExecution::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Atoms::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "WithoutConnectorsExecution::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
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
    (query (range (start 19 16) (end 19 21)) (probe (position 19 16))
      (reference
        (source (document "d0") (qualified-name "WithoutConnectorsExecution::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Atoms::*")
        (range (start 19 16) (end 19 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 16) (end 20 50)) (probe (position 20 16))
      (reference
        (source (document "d0") (qualified-name "WithoutConnectorsExecution::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::*")
        (range (start 20 16) (end 20 50))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted") (range (start 1 0) (end 1 196)))
        )
      )
    )
  )
)
~~~
