# META
~~~ini
description=KerML KerML Spec Annex A: A-3-4-OneToUnrestrictedConnectors
type=file
~~~
# SOURCE
~~~kerml

package OneToUnrestrictedConnectorsModelToBeExecuted {
	doc
	/* 
	 */

	private import WithoutConnectorsModelToBeExecuted::BikeFork;

	classifier Bicycle {
		feature carrier : BikeBasket [*];
		feature holdsWheel : BikeFork [*];
		connector carrierFixed : BikeBasketFixed from [*] carrier to [1] holdsWheel;
	}
	classifier BikeBasket;

	assoc BikeBasketFixed {
		end feature basket : BikeBasket;
		end feature fixedTo : BikeFork;
	}
}

package OneToUnrestrictedConnectorsExecution {
	doc
	/* 
	 */

	private import Atoms::*;
	private import OneToUnrestrictedConnectorsModelToBeExecuted::*;
	private import OneToOneConnectorsExecution::MyBikeFork1;
	private import OneToOneConnectorsExecution::MyBikeFork2;
	private import OneToOneConnectorsExecution::MyBikeFork;

	#atom
	classifier MyBikeBasket1 specializes BikeBasket;
	#atom
	classifier MyBikeBasket2 specializes BikeBasket;

	classifier MyBikeBasket unions MyBikeBasket1, MyBikeBasket2;

	#atom
	assoc MyBikeBasket1_Fork1_BBF_Link specializes BikeBasketFixed {
		end feature redefines basket : MyBikeBasket1;
		end feature redefines fixedTo : MyBikeFork1;
	}
	#atom
	assoc MyBikeBasket2_Fork1_BBF_Link specializes BikeBasketFixed {
		end feature redefines basket : MyBikeBasket2;
		end feature redefines fixedTo : MyBikeFork1;
	}

	classifier MyBikeBasket_Fork_BBF_Link unions MyBikeBasket1_Fork1_BBF_Link, MyBikeBasket2_Fork1_BBF_Link;

	#atom
	classifier MyBike specializes Bicycle {
		feature redefines carrier : MyBikeBasket [2];
		feature redefines holdsWheel : MyBikeFork [2];
		connector redefines carrierFixed : MyBikeBasket_Fork_BBF_Link [2] from [*] carrier to [1] holdsWheel;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "a_3_4_one_to_unrestricted_connectors.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 26 16) (end 26 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 28 16) (end 28 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 29 16) (end 29 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 30 16) (end 30 55))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "aa182489e3f08016d4b522af5263fbfa2a83695f518482c4a0bc9a655d9d25b1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))) (kind "package") (name "OneToUnrestrictedConnectorsExecution") (declared-name "OneToUnrestrictedConnectorsExecution"))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "OneToUnrestrictedConnectorsModelToBeExecuted::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBike"))) (kind "classifier decl") (name "MyBike") (declared-name "MyBike") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket"))) (kind "classifier decl") (name "MyBikeBasket") (declared-name "MyBikeBasket") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket1"))) (kind "classifier decl") (name "MyBikeBasket1") (declared-name "MyBikeBasket1") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket1_Fork1_BBF_Link"))) (kind "kermlDecl") (name "MyBikeBasket1_Fork1_BBF_Link") (declared-name "MyBikeBasket1_Fork1_BBF_Link") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket2"))) (kind "classifier decl") (name "MyBikeBasket2") (declared-name "MyBikeBasket2") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket2_Fork1_BBF_Link"))) (kind "kermlDecl") (name "MyBikeBasket2_Fork1_BBF_Link") (declared-name "MyBikeBasket2_Fork1_BBF_Link") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket_Fork_BBF_Link"))) (kind "classifier decl") (name "MyBikeBasket_Fork_BBF_Link") (declared-name "MyBikeBasket_Fork_BBF_Link") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork"))) (kind "import") (name "MyBikeFork") (declared-name "MyBikeFork") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "OneToOneConnectorsExecution::MyBikeFork") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork1"))) (kind "import") (name "MyBikeFork1") (declared-name "MyBikeFork1") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "OneToOneConnectorsExecution::MyBikeFork1") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork2"))) (kind "import") (name "MyBikeFork2") (declared-name "MyBikeFork2") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "OneToOneConnectorsExecution::MyBikeFork2") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword3"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword4"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted"))) (kind "package") (name "OneToUnrestrictedConnectorsModelToBeExecuted") (declared-name "OneToUnrestrictedConnectorsModelToBeExecuted"))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::Bicycle"))) (kind "classifier decl") (name "Bicycle") (declared-name "Bicycle") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasket"))) (kind "classifier decl") (name "BikeBasket") (declared-name "BikeBasket") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasketFixed"))) (kind "kermlDecl") (name "BikeBasketFixed") (declared-name "BikeBasketFixed") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::BikeFork"))) (kind "import") (name "BikeFork") (declared-name "BikeFork") (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted"))) (authored (membership (kind Import) (visibility "private") (import (reference "WithoutConnectorsModelToBeExecuted::BikeFork") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Atoms::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "OneToUnrestrictedConnectorsModelToBeExecuted::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork"))) (kind membershipImport) (ordinal 0)) (authored-target "OneToOneConnectorsExecution::MyBikeFork") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork1"))) (kind membershipImport) (ordinal 0)) (authored-target "OneToOneConnectorsExecution::MyBikeFork1") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork2"))) (kind membershipImport) (ordinal 0)) (authored-target "OneToOneConnectorsExecution::MyBikeFork2") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::BikeFork"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 26 16) (end 26 21)) (probe (position 26 16))
      (reference
        (source (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Atoms::*")
        (range (start 26 16) (end 26 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 30 16) (end 30 55)) (probe (position 30 16))
      (reference
        (source (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork"))
        (kind membershipImport) (ordinal 0) (authored-target "OneToOneConnectorsExecution::MyBikeFork")
        (range (start 30 16) (end 30 55))
        (outcome (status unresolved))
      )
    )
    (query (range (start 28 16) (end 28 56)) (probe (position 28 16))
      (reference
        (source (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork1"))
        (kind membershipImport) (ordinal 0) (authored-target "OneToOneConnectorsExecution::MyBikeFork1")
        (range (start 28 16) (end 28 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 29 16) (end 29 56)) (probe (position 29 16))
      (reference
        (source (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork2"))
        (kind membershipImport) (ordinal 0) (authored-target "OneToOneConnectorsExecution::MyBikeFork2")
        (range (start 29 16) (end 29 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 60)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::BikeFork"))
        (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
        (range (start 6 16) (end 6 60))
        (outcome (status unresolved))
      )
    )
    (query (range (start 27 16) (end 27 60)) (probe (position 27 16))
      (reference
        (source (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "OneToUnrestrictedConnectorsModelToBeExecuted::*")
        (range (start 27 16) (end 27 60))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted") (range (start 1 0) (end 1 434)))
        )
      )
    )
  )
)
~~~
