# META
~~~ini
description=KerML KerML Spec Annex A: A-3-3-OneToOneConnectors
type=file
~~~
# SOURCE
~~~kerml

package OneToOneConnectorsModelToBeExecuted {
	doc
	/* 
	 */

    public import WithoutConnectorsModelToBeExecuted::Wheel;
    public import WithoutConnectorsModelToBeExecuted::BikeFork;

	classifier Bicycle {
		feature rollsOn : Wheel [2];
		feature holdsWheel : BikeFork [*];
		connector fixWheel : BikeWheelFixed from [1] rollsOn to [1] holdsWheel;
	}
	assoc BikeWheelFixed {
		end feature wheel : Wheel;
		end feature fixedTo : BikeFork;
	}
}

package OneToOneConnectorsExecution {
	doc
	/* 
	 */

	private import Atoms::*;
	public import OneToOneConnectorsModelToBeExecuted::*;
	public import WithoutConnectorsExecution::MyWheel1;
	public import WithoutConnectorsExecution::MyWheel2;
	public import WithoutConnectorsExecution::MyWheel;

	#atom
	classifier MyBikeFork1 specializes BikeFork;
	#atom
	classifier MyBikeFork2 specializes BikeFork;

	classifier MyBikeFork unions MyBikeFork1, MyBikeFork2;

	#atom
 	assoc MyBikeWheel1_Fork1_BWF_Link specializes BikeWheelFixed {
		end feature redefines wheel : MyWheel1;
		end feature redefines fixedTo : MyBikeFork1;
	}
	#atom
	assoc MyBikeWheel2_Fork2_BWF_Link specializes BikeWheelFixed {
		end feature redefines wheel : MyWheel2;
		end feature redefines fixedTo : MyBikeFork2;
	}

	classifier MyBikeWheel_Fork_BWF_Link unions MyBikeWheel1_Fork1_BWF_Link, MyBikeWheel2_Fork2_BWF_Link;

	#atom
	classifier MyBike specializes Bicycle {
		feature redefines rollsOn : MyWheel;
		feature redefines holdsWheel : MyBikeFork;
		connector redefines fixWheel : MyBikeWheel_Fork_BWF_Link [2] from [1] rollsOn to [1] holdsWheel;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "a_3_3_one_to_one_connectors.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 18) (end 6 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 18) (end 7 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 25 16) (end 25 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 27 15) (end 27 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 28 15) (end 28 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 29 15) (end 29 50))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml

package OneToOneConnectorsModelToBeExecuted {
	doc
	/* 
	 */

    public import WithoutConnectorsModelToBeExecuted::Wheel;
    public import WithoutConnectorsModelToBeExecuted::BikeFork;

	classifier Bicycle {
		feature rollsOn : Wheel [2];
		feature holdsWheel : BikeFork [*];
		connector fixWheel : BikeWheelFixed from [1] rollsOn to [1] holdsWheel;
	}
	assoc BikeWheelFixed {
		end feature wheel : Wheel;
		end feature fixedTo : BikeFork;
	}
}

package OneToOneConnectorsExecution {
	doc
	/* 
	 */

	private import Atoms::*;
	public import OneToOneConnectorsModelToBeExecuted::*;
	public import WithoutConnectorsExecution::MyWheel1;
	public import WithoutConnectorsExecution::MyWheel2;
	public import WithoutConnectorsExecution::MyWheel;

	#atom
	classifier MyBikeFork1 specializes BikeFork;
	#atom
	classifier MyBikeFork2 specializes BikeFork;

	classifier MyBikeFork unions MyBikeFork1, MyBikeFork2;

	#atom
 	assoc MyBikeWheel1_Fork1_BWF_Link specializes BikeWheelFixed {
		end feature redefines wheel : MyWheel1;
		end feature redefines fixedTo : MyBikeFork1;
	}
	#atom
	assoc MyBikeWheel2_Fork2_BWF_Link specializes BikeWheelFixed {
		end feature redefines wheel : MyWheel2;
		end feature redefines fixedTo : MyBikeFork2;
	}

	classifier MyBikeWheel_Fork_BWF_Link unions MyBikeWheel1_Fork1_BWF_Link, MyBikeWheel2_Fork2_BWF_Link;

	#atom
	classifier MyBike specializes Bicycle {
		feature redefines rollsOn : MyWheel;
		feature redefines holdsWheel : MyBikeFork;
		connector redefines fixWheel : MyBikeWheel_Fork_BWF_Link [2] from [1] rollsOn to [1] holdsWheel;
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b74c699f2eabf08b665b68fcfab80f6e4fb18fcd4a8d95945c3de516148e67d1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))) (kind "package") (name "OneToOneConnectorsExecution") (declared-name "OneToOneConnectorsExecution"))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))) (authored (membership (kind Import) (visibility "public") (import (reference "OneToOneConnectorsModelToBeExecuted::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBike"))) (kind "classifier decl") (name "MyBike") (declared-name "MyBike") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeFork"))) (kind "classifier decl") (name "MyBikeFork") (declared-name "MyBikeFork") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeFork1"))) (kind "classifier decl") (name "MyBikeFork1") (declared-name "MyBikeFork1") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeFork2"))) (kind "classifier decl") (name "MyBikeFork2") (declared-name "MyBikeFork2") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeWheel1_Fork1_BWF_Link"))) (kind "kermlDecl") (name "MyBikeWheel1_Fork1_BWF_Link") (declared-name "MyBikeWheel1_Fork1_BWF_Link") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeWheel2_Fork2_BWF_Link"))) (kind "kermlDecl") (name "MyBikeWheel2_Fork2_BWF_Link") (declared-name "MyBikeWheel2_Fork2_BWF_Link") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeWheel_Fork_BWF_Link"))) (kind "classifier decl") (name "MyBikeWheel_Fork_BWF_Link") (declared-name "MyBikeWheel_Fork_BWF_Link") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel"))) (kind "import") (name "MyWheel") (declared-name "MyWheel") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))) (authored (membership (kind Import) (visibility "public") (import (reference "WithoutConnectorsExecution::MyWheel") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel1"))) (kind "import") (name "MyWheel1") (declared-name "MyWheel1") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))) (authored (membership (kind Import) (visibility "public") (import (reference "WithoutConnectorsExecution::MyWheel1") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel2"))) (kind "import") (name "MyWheel2") (declared-name "MyWheel2") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))) (authored (membership (kind Import) (visibility "public") (import (reference "WithoutConnectorsExecution::MyWheel2") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword3"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword4"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted"))) (kind "package") (name "OneToOneConnectorsModelToBeExecuted") (declared-name "OneToOneConnectorsModelToBeExecuted"))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::Bicycle"))) (kind "classifier decl") (name "Bicycle") (declared-name "Bicycle") (parent (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::BikeFork"))) (kind "import") (name "BikeFork") (declared-name "BikeFork") (parent (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted"))) (authored (membership (kind Import) (visibility "public") (import (reference "WithoutConnectorsModelToBeExecuted::BikeFork") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::BikeWheelFixed"))) (kind "kermlDecl") (name "BikeWheelFixed") (declared-name "BikeWheelFixed") (parent (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::Wheel"))) (kind "import") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted"))) (authored (membership (kind Import) (visibility "public") (import (reference "WithoutConnectorsModelToBeExecuted::Wheel") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "OneToOneConnectorsExecution::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Atoms::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToOneConnectorsExecution::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "OneToOneConnectorsModelToBeExecuted::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsExecution::MyWheel") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel1"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsExecution::MyWheel1") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel2"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsExecution::MyWheel2") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::BikeFork"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::Wheel"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 25 16) (end 25 21)) (probe (position 25 16))
      (reference
        (source (document "d0") (qualified-name "OneToOneConnectorsExecution::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Atoms::*")
        (range (start 25 16) (end 25 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 26 15) (end 26 50)) (probe (position 26 15))
      (reference
        (source (document "d0") (qualified-name "OneToOneConnectorsExecution::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "OneToOneConnectorsModelToBeExecuted::*")
        (range (start 26 15) (end 26 50))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted") (range (start 1 0) (end 1 446)))
        )
      )
    )
    (query (range (start 29 15) (end 29 50)) (probe (position 29 15))
      (reference
        (source (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel"))
        (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsExecution::MyWheel")
        (range (start 29 15) (end 29 50))
        (outcome (status unresolved))
      )
    )
    (query (range (start 27 15) (end 27 51)) (probe (position 27 15))
      (reference
        (source (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel1"))
        (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsExecution::MyWheel1")
        (range (start 27 15) (end 27 51))
        (outcome (status unresolved))
      )
    )
    (query (range (start 28 15) (end 28 51)) (probe (position 28 15))
      (reference
        (source (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel2"))
        (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsExecution::MyWheel2")
        (range (start 28 15) (end 28 51))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 18) (end 6 59)) (probe (position 6 18))
      (reference
        (source (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::Wheel"))
        (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
        (range (start 6 18) (end 6 59))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 18) (end 7 62)) (probe (position 7 18))
      (reference
        (source (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::BikeFork"))
        (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
        (range (start 7 18) (end 7 62))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
