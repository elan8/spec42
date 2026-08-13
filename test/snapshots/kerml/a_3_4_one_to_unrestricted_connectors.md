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
  (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 60))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 8 1) (end 12 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 1) (end 12 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 13 1) (end 13 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 1) (end 13 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 15 1) (end 18 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 15 1) (end 18 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 26 16) (end 26 24))
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
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 32 1) (end 33 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 33 1) (end 33 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 33 1) (end 33 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 34 1) (end 35 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 35 1) (end 35 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 35 1) (end 35 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 37 1) (end 37 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 37 1) (end 37 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 39 1) (end 40 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 40 1) (end 43 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 40 1) (end 43 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 44 1) (end 45 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 45 1) (end 48 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 45 1) (end 48 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 50 1) (end 50 105))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 50 1) (end 50 105))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 52 1) (end 53 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 53 1) (end 57 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 53 1) (end 57 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:8e47dd9f44c10386e51751f0192eaa7c572b29cfb66ebde9a307e295b3c90c9c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (qualified-name "OneToUnrestrictedConnectorsExecution"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Atoms") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "OneToUnrestrictedConnectorsModelToBeExecuted") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "OneToOneConnectorsExecution::MyBikeFork1") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "OneToOneConnectorsExecution::MyBikeFork2") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "OneToOneConnectorsExecution::MyBikeFork") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "WithoutConnectorsModelToBeExecuted::BikeFork") (import (shape membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Atoms")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "OneToUnrestrictedConnectorsModelToBeExecuted")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted")))))
    (reference (id (source (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "OneToOneConnectorsExecution::MyBikeFork1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "OneToOneConnectorsExecution::MyBikeFork2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "OneToOneConnectorsExecution::MyBikeFork")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (range (start 26 16) (end 26 24)) (probe (position 26 16))
    (reference (id (source (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Atoms")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (range (start 27 16) (end 27 63)) (probe (position 27 16))
    (reference (id (source (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "OneToUnrestrictedConnectorsModelToBeExecuted")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted")))))
  )
  (query (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (range (start 28 16) (end 28 56)) (probe (position 28 16))
    (reference (id (source (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "OneToOneConnectorsExecution::MyBikeFork1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (range (start 29 16) (end 29 56)) (probe (position 29 16))
    (reference (id (source (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "OneToOneConnectorsExecution::MyBikeFork2")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (range (start 30 16) (end 30 55)) (probe (position 30 16))
    (reference (id (source (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "OneToOneConnectorsExecution::MyBikeFork")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (range (start 6 16) (end 6 60)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/a_3_4_one_to_unrestricted_connectors.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
      (outcome (status unresolved)))
  )
)
~~~
