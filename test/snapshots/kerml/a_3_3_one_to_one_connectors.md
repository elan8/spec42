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
  (document "memory://snapshot/a_3_3_one_to_one_connectors.md"
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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 9 1) (end 13 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 9 1) (end 13 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 14 1) (end 17 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 14 1) (end 17 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 25 16) (end 25 24))
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
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 31 1) (end 32 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 32 1) (end 32 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 32 1) (end 32 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 33 1) (end 34 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 34 1) (end 34 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 34 1) (end 34 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 36 1) (end 36 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 36 1) (end 36 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 38 1) (end 39 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 39 2) (end 42 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 39 2) (end 42 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 43 1) (end 44 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 44 1) (end 47 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 44 1) (end 47 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 49 1) (end 49 102))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 49 1) (end 49 102))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 51 1) (end 52 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 52 1) (end 56 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 52 1) (end 56 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:e660cbaa83368787ab4cad1049b2d0c3f65b39b537f37a5e2aacc15378455b07") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (qualified-name "OneToOneConnectorsExecution"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Atoms") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "OneToOneConnectorsModelToBeExecuted") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "WithoutConnectorsExecution::MyWheel1") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "WithoutConnectorsExecution::MyWheel2") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "WithoutConnectorsExecution::MyWheel") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (qualified-name "OneToOneConnectorsModelToBeExecuted"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "WithoutConnectorsModelToBeExecuted::Wheel") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "WithoutConnectorsModelToBeExecuted::BikeFork") (import (shape membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Atoms")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "OneToOneConnectorsModelToBeExecuted")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (qualified-name "OneToOneConnectorsModelToBeExecuted")))))
    (reference (id (source (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "WithoutConnectorsExecution::MyWheel1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "WithoutConnectorsExecution::MyWheel2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "WithoutConnectorsExecution::MyWheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
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
  (query (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (range (start 25 16) (end 25 24)) (probe (position 25 16))
    (reference (id (source (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Atoms")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (range (start 26 15) (end 26 53)) (probe (position 26 15))
    (reference (id (source (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "OneToOneConnectorsModelToBeExecuted")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (qualified-name "OneToOneConnectorsModelToBeExecuted")))))
  )
  (query (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (range (start 27 15) (end 27 51)) (probe (position 27 15))
    (reference (id (source (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsExecution::MyWheel1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (range (start 28 15) (end 28 51)) (probe (position 28 15))
    (reference (id (source (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsExecution::MyWheel2")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (range (start 29 15) (end 29 50)) (probe (position 29 15))
    (reference (id (source (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsExecution::MyWheel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (range (start 6 18) (end 6 59)) (probe (position 6 18))
    (reference (id (source (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (range (start 7 18) (end 7 62)) (probe (position 7 18))
    (reference (id (source (node (document "memory://snapshot/a_3_3_one_to_one_connectors.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
      (outcome (status unresolved)))
  )
)
~~~
