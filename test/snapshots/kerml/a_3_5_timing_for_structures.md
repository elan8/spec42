# META
~~~ini
description=KerML KerML Spec Annex A: A-3-5-TimingForStructures
type=file
~~~
# SOURCE
~~~kerml

package TimingForStructuresModelToBeExecuted1 {
	doc
	/* 
	 */

	private import WithoutConnectorsModelToBeExecuted::Wheel;
	private import WithoutConnectorsModelToBeExecuted::BikeFork;
	private import Occurrences::Occurrence;

	struct Bicycle {
		feature rollsOn : Wheel [2] subsets timeCoincidentOccurrences;
		feature holdsWheel : BikeFork [2] subsets timeCoincidentOccurrences;
	}
}

package TimingForStructuresExecution1 {
	doc
	/* 
	 */

	private import Atoms::*;
	private import TimingForStructuresModelToBeExecuted1::*;
	private import OneToOneConnectorsExecution::MyWheel;
	private import OneToOneConnectorsExecution::MyBikeFork;

	struct MyBikeTimeCoincident unions MyWheel, MyBikeFork, MyBike;

	#atom
	struct MyBike specializes Bicycle {
		feature redefines self : MyBike;
		feature redefines timeCoincidentOccurrences : MyBikeTimeCoincident [5];
		feature redefines rollsOn : MyWheel;
		feature redefines holdsWheel : MyBikeFork;
	}
}


package TimingForStructuresModelToBeExecuted2 {
	doc
	/* 
	 */

	private import WithoutConnectorsModelToBeExecuted::Wheel;
	private import WithoutConnectorsModelToBeExecuted::BikeFork;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensDuring;

	struct Bicycle {
		feature rollsOn : Wheel [2];
		feature holdsWheel : BikeFork [2];
		feature allParts : Occurrence unions rollsOn, holdsWheel;
		connector b_during_ap : HappensDuring from [1] self to [*] allParts;
	}
}

package TimingForStructuresExecution2 {
	doc
	/* 
	 */

	private import Atoms::*;
	private import TimingForStructuresModelToBeExecuted2::*;
	private import Occurrences::HappensDuring;
	private import OneToOneConnectorsExecution::MyWheel;
	private import OneToOneConnectorsExecution::MyBikeFork;
	
	struct MyWheel1 specializes OneToOneConnectorsExecution::MyWheel1;
	struct MyWheel2 specializes OneToOneConnectorsExecution::MyWheel2;
    struct MyBikeFork1 specializes OneToOneConnectorsExecution::MyBikeFork1;
    struct MyBikeFork2 specializes OneToOneConnectorsExecution::MyBikeFork2;

	#atom
	assoc MyBike_During_Wheel1_Link specializes HappensDuring {
		end feature redefines shorterOccurrence : MyBike;
		end feature redefines longerOccurrence : MyWheel1;
	}
	#atom
	assoc MyBike_During_Wheel2_Link specializes HappensDuring {
		end feature redefines shorterOccurrence : MyBike;
		end feature redefines longerOccurrence : MyWheel2;
	}
	#atom
	assoc MyBike_During_Fork1_Link specializes HappensDuring {
		end feature redefines shorterOccurrence : MyBike;
		end feature redefines longerOccurrence : MyBikeFork1;
	}
	#atom
	assoc MyBike_During_Fork2_Link specializes HappensDuring {
		end feature redefines shorterOccurrence : MyBike;
		end feature redefines longerOccurrence : MyBikeFork2;
	}

	assoc MyBike_During_Parts_Link specializes HappensDuring
		unions MyBike_During_Wheel1_Link, MyBike_During_Fork1_Link,
		       MyBike_During_Wheel2_Link, MyBike_During_Fork2_Link;

	struct MyBikeParts unions MyWheel, MyBikeFork;

	#atom
	struct MyBike specializes Bicycle {
		feature redefines rollsOn : MyWheel;
		feature redefines holdsWheel : MyBikeFork;
		feature redefines allParts : MyBikeParts [4];

		feature redefines self : MyBike;
		connector redefines b_during_ap : MyBike_During_Parts_Link [4]
			from [1] self to [*] allParts;
	}
}

package TimingForStructuresModelToBeExecuted3 {
	doc
	/* 
	 */

	private import WithoutConnectorsModelToBeExecuted::Wheel;
	private import WithoutConnectorsModelToBeExecuted::BikeFork;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensWhile;

	struct Bicycle {
		feature rollsOn : Wheel [2];
		feature holdsWheel : BikeFork [2];
		feature allParts : Occurrence unions rollsOn, holdsWheel;
		feature redefines endShot : Bicycle;
		connector be_while_pe : HappensWhile from [1] endShot to [*] endShot.allParts.endShot;
	}
}

package TimingForStructuresExecution3 {
	doc
	/* 
	 */

	private import Atoms::*;
	private import TimingForStructuresModelToBeExecuted3::*;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensWhile;
	private import WithoutConnectorsModelToBeExecuted::Wheel;
	private import WithoutConnectorsModelToBeExecuted::BikeFork;

	  /* End atoms */
	#atom
	struct MyWheel1End specializes Wheel;
	#atom
	struct MyWheel1 specializes Wheel {
		feature redefines endShot : MyWheel1End;
	}
	#atom
	struct MyWheel2End specializes Wheel;
	#atom
	struct MyWheel2 specializes Wheel {
		feature redefines endShot : MyWheel2End;
	}
	struct MyBikeFork1End specializes BikeFork;
	#atom
	struct MyBikeFork1 specializes BikeFork {
		feature redefines endShot : MyBikeFork1End;
	}
	struct MyBikeFork2End specializes BikeFork;
	#atom
	struct MyBikeFork2 specializes BikeFork {
		feature redefines endShot : MyBikeFork2End;
	}
	#atom
	struct MyBikeEnd specializes Bicycle;

	  /* HappensWhile atoms */
	#atom
	assoc MyBikeEnd_While_Wheel1End_Link specializes HappensWhile {
		end feature redefines thisOccurrence : MyBikeEnd;
		end feature redefines thatOccurrence : MyWheel1End;
	}
	#atom
	assoc MyBikeEnd_While_Wheel2End_Link specializes HappensWhile {
		end feature redefines thisOccurrence : MyBikeEnd;
		end feature redefines thatOccurrence : MyWheel2End;
	}
	#atom
	assoc MyBikeEnd_While_Fork1End_Link specializes HappensWhile {
		end feature redefines thisOccurrence : MyBikeEnd;
		end feature redefines thatOccurrence : MyBikeFork1End;
	}
	#atom
	assoc MyBikeEnd_While_Fork2End_Link specializes HappensWhile {
		end feature redefines thisOccurrence : MyBikeEnd;
		end feature redefines thatOccurrence : MyBikeFork2End;
	}

	assoc MyBikeEnd_While_PartsEnd_Link specializes HappensWhile
		unions MyBikeEnd_While_Wheel1End_Link, MyBikeEnd_While_Fork1End_Link,
		       MyBikeEnd_While_Wheel2End_Link, MyBikeEnd_While_Fork2End_Link;

	#atom
	struct MyBike specializes Bicycle {
		feature redefines endShot : MyBikeEnd;
		connector redefines be_while_pe : MyBikeEnd_While_PartsEnd_Link [4]
			from [1] endShot to [*] endShot.allParts.endShot;  
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "a_3_5_timing_for_structures.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 16) (end 21 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 23 16) (end 23 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 24 16) (end 24 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 43 16) (end 43 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 44 16) (end 44 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 45 16) (end 45 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 46 16) (end 46 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 61 16) (end 61 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 63 16) (end 63 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 64 16) (end 64 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 65 16) (end 65 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 116 16) (end 116 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 117 16) (end 117 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 118 16) (end 118 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 119 16) (end 119 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 135 16) (end 135 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 137 16) (end 137 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 138 16) (end 138 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 139 16) (end 139 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 140 16) (end 140 60))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml

package TimingForStructuresModelToBeExecuted1 {
	doc
	/* 
	 */

	private import WithoutConnectorsModelToBeExecuted::Wheel;
	private import WithoutConnectorsModelToBeExecuted::BikeFork;
	private import Occurrences::Occurrence;

	struct Bicycle {
		feature rollsOn : Wheel [2] subsets timeCoincidentOccurrences;
		feature holdsWheel : BikeFork [2] subsets timeCoincidentOccurrences;
	}
}

package TimingForStructuresExecution1 {
	doc
	/* 
	 */

	private import Atoms::*;
	private import TimingForStructuresModelToBeExecuted1::*;
	private import OneToOneConnectorsExecution::MyWheel;
	private import OneToOneConnectorsExecution::MyBikeFork;

	struct MyBikeTimeCoincident unions MyWheel, MyBikeFork, MyBike;

	#atom
	struct MyBike specializes Bicycle {
		feature redefines self : MyBike;
		feature redefines timeCoincidentOccurrences : MyBikeTimeCoincident [5];
		feature redefines rollsOn : MyWheel;
		feature redefines holdsWheel : MyBikeFork;
	}
}


package TimingForStructuresModelToBeExecuted2 {
	doc
	/* 
	 */

	private import WithoutConnectorsModelToBeExecuted::Wheel;
	private import WithoutConnectorsModelToBeExecuted::BikeFork;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensDuring;

	struct Bicycle {
		feature rollsOn : Wheel [2];
		feature holdsWheel : BikeFork [2];
		feature allParts : Occurrence unions rollsOn, holdsWheel;
		connector b_during_ap : HappensDuring from [1] self to [*] allParts;
	}
}

package TimingForStructuresExecution2 {
	doc
	/* 
	 */

	private import Atoms::*;
	private import TimingForStructuresModelToBeExecuted2::*;
	private import Occurrences::HappensDuring;
	private import OneToOneConnectorsExecution::MyWheel;
	private import OneToOneConnectorsExecution::MyBikeFork;
	
	struct MyWheel1 specializes OneToOneConnectorsExecution::MyWheel1;
	struct MyWheel2 specializes OneToOneConnectorsExecution::MyWheel2;
    struct MyBikeFork1 specializes OneToOneConnectorsExecution::MyBikeFork1;
    struct MyBikeFork2 specializes OneToOneConnectorsExecution::MyBikeFork2;

	#atom
	assoc MyBike_During_Wheel1_Link specializes HappensDuring {
		end feature redefines shorterOccurrence : MyBike;
		end feature redefines longerOccurrence : MyWheel1;
	}
	#atom
	assoc MyBike_During_Wheel2_Link specializes HappensDuring {
		end feature redefines shorterOccurrence : MyBike;
		end feature redefines longerOccurrence : MyWheel2;
	}
	#atom
	assoc MyBike_During_Fork1_Link specializes HappensDuring {
		end feature redefines shorterOccurrence : MyBike;
		end feature redefines longerOccurrence : MyBikeFork1;
	}
	#atom
	assoc MyBike_During_Fork2_Link specializes HappensDuring {
		end feature redefines shorterOccurrence : MyBike;
		end feature redefines longerOccurrence : MyBikeFork2;
	}

	assoc MyBike_During_Parts_Link specializes HappensDuring
		unions MyBike_During_Wheel1_Link, MyBike_During_Fork1_Link,
		       MyBike_During_Wheel2_Link, MyBike_During_Fork2_Link;

	struct MyBikeParts unions MyWheel, MyBikeFork;

	#atom
	struct MyBike specializes Bicycle {
		feature redefines rollsOn : MyWheel;
		feature redefines holdsWheel : MyBikeFork;
		feature redefines allParts : MyBikeParts [4];

		feature redefines self : MyBike;
		connector redefines b_during_ap : MyBike_During_Parts_Link [4]
			from [1] self to [*] allParts;
	}
}

package TimingForStructuresModelToBeExecuted3 {
	doc
	/* 
	 */

	private import WithoutConnectorsModelToBeExecuted::Wheel;
	private import WithoutConnectorsModelToBeExecuted::BikeFork;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensWhile;

	struct Bicycle {
		feature rollsOn : Wheel [2];
		feature holdsWheel : BikeFork [2];
		feature allParts : Occurrence unions rollsOn, holdsWheel;
		feature redefines endShot : Bicycle;
		connector be_while_pe : HappensWhile from [1] endShot to [*] endShot.allParts.endShot;
	}
}

package TimingForStructuresExecution3 {
	doc
	/* 
	 */

	private import Atoms::*;
	private import TimingForStructuresModelToBeExecuted3::*;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensWhile;
	private import WithoutConnectorsModelToBeExecuted::Wheel;
	private import WithoutConnectorsModelToBeExecuted::BikeFork;

	  /* End atoms */
	#atom
	struct MyWheel1End specializes Wheel;
	#atom
	struct MyWheel1 specializes Wheel {
		feature redefines endShot : MyWheel1End;
	}
	#atom
	struct MyWheel2End specializes Wheel;
	#atom
	struct MyWheel2 specializes Wheel {
		feature redefines endShot : MyWheel2End;
	}
	struct MyBikeFork1End specializes BikeFork;
	#atom
	struct MyBikeFork1 specializes BikeFork {
		feature redefines endShot : MyBikeFork1End;
	}
	struct MyBikeFork2End specializes BikeFork;
	#atom
	struct MyBikeFork2 specializes BikeFork {
		feature redefines endShot : MyBikeFork2End;
	}
	#atom
	struct MyBikeEnd specializes Bicycle;

	  /* HappensWhile atoms */
	#atom
	assoc MyBikeEnd_While_Wheel1End_Link specializes HappensWhile {
		end feature redefines thisOccurrence : MyBikeEnd;
		end feature redefines thatOccurrence : MyWheel1End;
	}
	#atom
	assoc MyBikeEnd_While_Wheel2End_Link specializes HappensWhile {
		end feature redefines thisOccurrence : MyBikeEnd;
		end feature redefines thatOccurrence : MyWheel2End;
	}
	#atom
	assoc MyBikeEnd_While_Fork1End_Link specializes HappensWhile {
		end feature redefines thisOccurrence : MyBikeEnd;
		end feature redefines thatOccurrence : MyBikeFork1End;
	}
	#atom
	assoc MyBikeEnd_While_Fork2End_Link specializes HappensWhile {
		end feature redefines thisOccurrence : MyBikeEnd;
		end feature redefines thatOccurrence : MyBikeFork2End;
	}

	assoc MyBikeEnd_While_PartsEnd_Link specializes HappensWhile
		unions MyBikeEnd_While_Wheel1End_Link, MyBikeEnd_While_Fork1End_Link,
		       MyBikeEnd_While_Wheel2End_Link, MyBikeEnd_While_Fork2End_Link;

	#atom
	struct MyBike specializes Bicycle {
		feature redefines endShot : MyBikeEnd;
		connector redefines be_while_pe : MyBikeEnd_While_PartsEnd_Link [4]
			from [1] endShot to [*] endShot.allParts.endShot;  
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "641bd72b64e7916cbca6a3fbc6952c6f2e2b423494e42d32dd86e0f340504afc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution1"))) (kind "package") (name "TimingForStructuresExecution1") (declared-name "TimingForStructuresExecution1"))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution1::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution1::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution1"))) (authored (membership (kind Import) (visibility "private") (import (reference "TimingForStructuresModelToBeExecuted1::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution1::MyBike"))) (kind "classifier decl") (name "MyBike") (declared-name "MyBike") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution1"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution1::MyBikeFork"))) (kind "import") (name "MyBikeFork") (declared-name "MyBikeFork") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution1"))) (authored (membership (kind Import) (visibility "private") (import (reference "OneToOneConnectorsExecution::MyBikeFork") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution1::MyBikeTimeCoincident"))) (kind "classifier decl") (name "MyBikeTimeCoincident") (declared-name "MyBikeTimeCoincident") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution1"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution1::MyWheel"))) (kind "import") (name "MyWheel") (declared-name "MyWheel") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution1"))) (authored (membership (kind Import) (visibility "private") (import (reference "OneToOneConnectorsExecution::MyWheel") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution1::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution1"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2"))) (kind "package") (name "TimingForStructuresExecution2") (declared-name "TimingForStructuresExecution2"))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))) (authored (membership (kind Import) (visibility "private") (import (reference "TimingForStructuresModelToBeExecuted2::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::HappensDuring"))) (kind "import") (name "HappensDuring") (declared-name "HappensDuring") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensDuring") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBike"))) (kind "classifier decl") (name "MyBike") (declared-name "MyBike") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBikeFork"))) (kind "import") (name "MyBikeFork") (declared-name "MyBikeFork") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))) (authored (membership (kind Import) (visibility "private") (import (reference "OneToOneConnectorsExecution::MyBikeFork") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBikeFork1"))) (kind "classifier decl") (name "MyBikeFork1") (declared-name "MyBikeFork1") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBikeFork2"))) (kind "classifier decl") (name "MyBikeFork2") (declared-name "MyBikeFork2") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBikeParts"))) (kind "classifier decl") (name "MyBikeParts") (declared-name "MyBikeParts") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBike_During_Fork1_Link"))) (kind "kermlDecl") (name "MyBike_During_Fork1_Link") (declared-name "MyBike_During_Fork1_Link") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBike_During_Fork2_Link"))) (kind "kermlDecl") (name "MyBike_During_Fork2_Link") (declared-name "MyBike_During_Fork2_Link") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBike_During_Parts_Link"))) (kind "kermlDecl") (name "MyBike_During_Parts_Link") (declared-name "MyBike_During_Parts_Link") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBike_During_Wheel1_Link"))) (kind "kermlDecl") (name "MyBike_During_Wheel1_Link") (declared-name "MyBike_During_Wheel1_Link") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBike_During_Wheel2_Link"))) (kind "kermlDecl") (name "MyBike_During_Wheel2_Link") (declared-name "MyBike_During_Wheel2_Link") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyWheel"))) (kind "import") (name "MyWheel") (declared-name "MyWheel") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))) (authored (membership (kind Import) (visibility "private") (import (reference "OneToOneConnectorsExecution::MyWheel") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyWheel1"))) (kind "classifier decl") (name "MyWheel1") (declared-name "MyWheel1") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyWheel2"))) (kind "classifier decl") (name "MyWheel2") (declared-name "MyWheel2") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom#metadata_keyword3"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom#metadata_keyword4"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3"))) (kind "package") (name "TimingForStructuresExecution3") (declared-name "TimingForStructuresExecution3"))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))) (authored (membership (kind Import) (visibility "private") (import (reference "TimingForStructuresModelToBeExecuted3::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::BikeFork"))) (kind "import") (name "BikeFork") (declared-name "BikeFork") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))) (authored (membership (kind Import) (visibility "private") (import (reference "WithoutConnectorsModelToBeExecuted::BikeFork") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::HappensWhile"))) (kind "import") (name "HappensWhile") (declared-name "HappensWhile") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensWhile") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBike"))) (kind "classifier decl") (name "MyBike") (declared-name "MyBike") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeEnd"))) (kind "classifier decl") (name "MyBikeEnd") (declared-name "MyBikeEnd") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeEnd_While_Fork1End_Link"))) (kind "kermlDecl") (name "MyBikeEnd_While_Fork1End_Link") (declared-name "MyBikeEnd_While_Fork1End_Link") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeEnd_While_Fork2End_Link"))) (kind "kermlDecl") (name "MyBikeEnd_While_Fork2End_Link") (declared-name "MyBikeEnd_While_Fork2End_Link") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeEnd_While_PartsEnd_Link"))) (kind "kermlDecl") (name "MyBikeEnd_While_PartsEnd_Link") (declared-name "MyBikeEnd_While_PartsEnd_Link") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeEnd_While_Wheel1End_Link"))) (kind "kermlDecl") (name "MyBikeEnd_While_Wheel1End_Link") (declared-name "MyBikeEnd_While_Wheel1End_Link") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeEnd_While_Wheel2End_Link"))) (kind "kermlDecl") (name "MyBikeEnd_While_Wheel2End_Link") (declared-name "MyBikeEnd_While_Wheel2End_Link") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeFork1"))) (kind "classifier decl") (name "MyBikeFork1") (declared-name "MyBikeFork1") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeFork1End"))) (kind "classifier decl") (name "MyBikeFork1End") (declared-name "MyBikeFork1End") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeFork2"))) (kind "classifier decl") (name "MyBikeFork2") (declared-name "MyBikeFork2") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeFork2End"))) (kind "classifier decl") (name "MyBikeFork2End") (declared-name "MyBikeFork2End") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyWheel1"))) (kind "classifier decl") (name "MyWheel1") (declared-name "MyWheel1") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyWheel1End"))) (kind "classifier decl") (name "MyWheel1End") (declared-name "MyWheel1End") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyWheel2"))) (kind "classifier decl") (name "MyWheel2") (declared-name "MyWheel2") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyWheel2End"))) (kind "classifier decl") (name "MyWheel2End") (declared-name "MyWheel2End") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::Wheel"))) (kind "import") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))) (authored (membership (kind Import) (visibility "private") (import (reference "WithoutConnectorsModelToBeExecuted::Wheel") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword10"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword11"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword3"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword4"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword5"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword6"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword7"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword8"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword9"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1"))) (kind "package") (name "TimingForStructuresModelToBeExecuted1") (declared-name "TimingForStructuresModelToBeExecuted1"))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1::Bicycle"))) (kind "classifier decl") (name "Bicycle") (declared-name "Bicycle") (parent (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1::BikeFork"))) (kind "import") (name "BikeFork") (declared-name "BikeFork") (parent (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1"))) (authored (membership (kind Import) (visibility "private") (import (reference "WithoutConnectorsModelToBeExecuted::BikeFork") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1::Wheel"))) (kind "import") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1"))) (authored (membership (kind Import) (visibility "private") (import (reference "WithoutConnectorsModelToBeExecuted::Wheel") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2"))) (kind "package") (name "TimingForStructuresModelToBeExecuted2") (declared-name "TimingForStructuresModelToBeExecuted2"))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::Bicycle"))) (kind "classifier decl") (name "Bicycle") (declared-name "Bicycle") (parent (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::BikeFork"))) (kind "import") (name "BikeFork") (declared-name "BikeFork") (parent (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2"))) (authored (membership (kind Import) (visibility "private") (import (reference "WithoutConnectorsModelToBeExecuted::BikeFork") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::HappensDuring"))) (kind "import") (name "HappensDuring") (declared-name "HappensDuring") (parent (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensDuring") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::Wheel"))) (kind "import") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2"))) (authored (membership (kind Import) (visibility "private") (import (reference "WithoutConnectorsModelToBeExecuted::Wheel") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3"))) (kind "package") (name "TimingForStructuresModelToBeExecuted3") (declared-name "TimingForStructuresModelToBeExecuted3"))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::Bicycle"))) (kind "classifier decl") (name "Bicycle") (declared-name "Bicycle") (parent (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3"))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::BikeFork"))) (kind "import") (name "BikeFork") (declared-name "BikeFork") (parent (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3"))) (authored (membership (kind Import) (visibility "private") (import (reference "WithoutConnectorsModelToBeExecuted::BikeFork") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::HappensWhile"))) (kind "import") (name "HappensWhile") (declared-name "HappensWhile") (parent (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensWhile") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::Wheel"))) (kind "import") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3"))) (authored (membership (kind Import) (visibility "private") (import (reference "WithoutConnectorsModelToBeExecuted::Wheel") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Atoms::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "TimingForStructuresModelToBeExecuted1::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution1::MyBikeFork"))) (kind membershipImport) (ordinal 0)) (authored-target "OneToOneConnectorsExecution::MyBikeFork") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution1::MyWheel"))) (kind membershipImport) (ordinal 0)) (authored-target "OneToOneConnectorsExecution::MyWheel") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Atoms::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "TimingForStructuresModelToBeExecuted2::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution2::HappensDuring"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensDuring") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBikeFork"))) (kind membershipImport) (ordinal 0)) (authored-target "OneToOneConnectorsExecution::MyBikeFork") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyWheel"))) (kind membershipImport) (ordinal 0)) (authored-target "OneToOneConnectorsExecution::MyWheel") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution3::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Atoms::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution3::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "TimingForStructuresModelToBeExecuted3::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution3::BikeFork"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution3::HappensWhile"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensWhile") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution3::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresExecution3::Wheel"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1::BikeFork"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1::Wheel"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::BikeFork"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::HappensDuring"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensDuring") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::Wheel"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::BikeFork"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::HappensWhile"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensWhile") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::Wheel"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 21 16) (end 21 21)) (probe (position 21 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution1::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Atoms::*")
        (range (start 21 16) (end 21 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 61 16) (end 61 21)) (probe (position 61 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Atoms::*")
        (range (start 61 16) (end 61 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 135 16) (end 135 21)) (probe (position 135 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution3::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Atoms::*")
        (range (start 135 16) (end 135 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 39)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 8 16) (end 8 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 45 16) (end 45 39)) (probe (position 45 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 45 16) (end 45 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 118 16) (end 118 39)) (probe (position 118 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 118 16) (end 118 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 137 16) (end 137 39)) (probe (position 137 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution3::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 137 16) (end 137 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 119 16) (end 119 41)) (probe (position 119 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::HappensWhile"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensWhile")
        (range (start 119 16) (end 119 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 138 16) (end 138 41)) (probe (position 138 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution3::HappensWhile"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensWhile")
        (range (start 138 16) (end 138 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 46 16) (end 46 42)) (probe (position 46 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::HappensDuring"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensDuring")
        (range (start 46 16) (end 46 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 63 16) (end 63 42)) (probe (position 63 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution2::HappensDuring"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensDuring")
        (range (start 63 16) (end 63 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 23 16) (end 23 52)) (probe (position 23 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution1::MyWheel"))
        (kind membershipImport) (ordinal 0) (authored-target "OneToOneConnectorsExecution::MyWheel")
        (range (start 23 16) (end 23 52))
        (outcome (status unresolved))
      )
    )
    (query (range (start 64 16) (end 64 52)) (probe (position 64 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution2::MyWheel"))
        (kind membershipImport) (ordinal 0) (authored-target "OneToOneConnectorsExecution::MyWheel")
        (range (start 64 16) (end 64 52))
        (outcome (status unresolved))
      )
    )
    (query (range (start 22 16) (end 22 53)) (probe (position 22 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution1::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "TimingForStructuresModelToBeExecuted1::*")
        (range (start 22 16) (end 22 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1") (range (start 1 0) (end 1 385)))
        )
      )
    )
    (query (range (start 62 16) (end 62 53)) (probe (position 62 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution2::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "TimingForStructuresModelToBeExecuted2::*")
        (range (start 62 16) (end 62 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2") (range (start 38 0) (end 38 492)))
        )
      )
    )
    (query (range (start 136 16) (end 136 53)) (probe (position 136 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution3::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "TimingForStructuresModelToBeExecuted3::*")
        (range (start 136 16) (end 136 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3") (range (start 111 0) (end 111 548)))
        )
      )
    )
    (query (range (start 24 16) (end 24 55)) (probe (position 24 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution1::MyBikeFork"))
        (kind membershipImport) (ordinal 0) (authored-target "OneToOneConnectorsExecution::MyBikeFork")
        (range (start 24 16) (end 24 55))
        (outcome (status unresolved))
      )
    )
    (query (range (start 65 16) (end 65 55)) (probe (position 65 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution2::MyBikeFork"))
        (kind membershipImport) (ordinal 0) (authored-target "OneToOneConnectorsExecution::MyBikeFork")
        (range (start 65 16) (end 65 55))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 57)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1::Wheel"))
        (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
        (range (start 6 16) (end 6 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 43 16) (end 43 57)) (probe (position 43 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::Wheel"))
        (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
        (range (start 43 16) (end 43 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 116 16) (end 116 57)) (probe (position 116 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::Wheel"))
        (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
        (range (start 116 16) (end 116 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 139 16) (end 139 57)) (probe (position 139 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution3::Wheel"))
        (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
        (range (start 139 16) (end 139 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 60)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1::BikeFork"))
        (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
        (range (start 7 16) (end 7 60))
        (outcome (status unresolved))
      )
    )
    (query (range (start 44 16) (end 44 60)) (probe (position 44 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::BikeFork"))
        (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
        (range (start 44 16) (end 44 60))
        (outcome (status unresolved))
      )
    )
    (query (range (start 117 16) (end 117 60)) (probe (position 117 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::BikeFork"))
        (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
        (range (start 117 16) (end 117 60))
        (outcome (status unresolved))
      )
    )
    (query (range (start 140 16) (end 140 60)) (probe (position 140 16))
      (reference
        (source (document "d0") (qualified-name "TimingForStructuresExecution3::BikeFork"))
        (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
        (range (start 140 16) (end 140 60))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
