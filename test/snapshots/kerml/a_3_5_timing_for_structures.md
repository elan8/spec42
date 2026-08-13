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
  (document "memory://snapshot/a_3_5_timing_for_structures.md"
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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 10 1) (end 13 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 1) (end 13 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 16) (end 21 24))
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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 26 1) (end 26 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 26 1) (end 26 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 28 1) (end 29 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 29 1) (end 34 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 29 1) (end 34 2))
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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 48 1) (end 53 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 48 1) (end 53 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 61 16) (end 61 24))
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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 67 1) (end 67 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 67 1) (end 67 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 68 1) (end 68 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 68 1) (end 68 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 69 4) (end 69 76))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 69 4) (end 69 76))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 70 4) (end 70 76))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 70 4) (end 70 76))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 72 1) (end 73 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 73 1) (end 76 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 73 1) (end 76 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 77 1) (end 78 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 78 1) (end 81 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 78 1) (end 81 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 82 1) (end 83 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 83 1) (end 86 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 83 1) (end 86 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 87 1) (end 88 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 88 1) (end 91 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 88 1) (end 91 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 93 1) (end 95 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 93 1) (end 95 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 97 1) (end 97 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 97 1) (end 97 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 99 1) (end 100 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 100 1) (end 108 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 100 1) (end 108 2))
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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 121 1) (end 127 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 121 1) (end 127 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 135 16) (end 135 24))
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
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 143 1) (end 144 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 144 1) (end 144 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 144 1) (end 144 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 145 1) (end 146 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 146 1) (end 148 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 146 1) (end 148 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 149 1) (end 150 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 150 1) (end 150 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 150 1) (end 150 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 151 1) (end 152 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 152 1) (end 154 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 152 1) (end 154 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 155 1) (end 155 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 155 1) (end 155 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 156 1) (end 157 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 157 1) (end 159 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 157 1) (end 159 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 160 1) (end 160 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 160 1) (end 160 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 161 1) (end 162 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 162 1) (end 164 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 162 1) (end 164 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 165 1) (end 166 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 166 1) (end 166 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 166 1) (end 166 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 169 1) (end 170 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 170 1) (end 173 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 170 1) (end 173 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 174 1) (end 175 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 175 1) (end 178 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 175 1) (end 178 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 179 1) (end 180 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 180 1) (end 183 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 180 1) (end 183 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 184 1) (end 185 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 185 1) (end 188 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 185 1) (end 188 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 190 1) (end 192 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 190 1) (end 192 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 194 1) (end 195 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 195 1) (end 199 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 195 1) (end 199 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:3065ea3747b613e2e189122705d88ac5d79c4bd4b216a270beeedd8ab5e731b2") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (qualified-name "TimingForStructuresExecution1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Atoms") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "TimingForStructuresModelToBeExecuted1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "OneToOneConnectorsExecution::MyWheel") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "OneToOneConnectorsExecution::MyBikeFork") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (qualified-name "TimingForStructuresExecution2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Atoms") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "TimingForStructuresModelToBeExecuted2") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensDuring") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "OneToOneConnectorsExecution::MyWheel") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "OneToOneConnectorsExecution::MyBikeFork") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (qualified-name "TimingForStructuresExecution3"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Atoms") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "TimingForStructuresModelToBeExecuted3") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensWhile") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "WithoutConnectorsModelToBeExecuted::Wheel") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "WithoutConnectorsModelToBeExecuted::BikeFork") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (qualified-name "TimingForStructuresModelToBeExecuted1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "WithoutConnectorsModelToBeExecuted::Wheel") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "WithoutConnectorsModelToBeExecuted::BikeFork") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (qualified-name "TimingForStructuresModelToBeExecuted2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "WithoutConnectorsModelToBeExecuted::Wheel") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "WithoutConnectorsModelToBeExecuted::BikeFork") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensDuring") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (qualified-name "TimingForStructuresModelToBeExecuted3"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "WithoutConnectorsModelToBeExecuted::Wheel") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "WithoutConnectorsModelToBeExecuted::BikeFork") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensWhile") (import (shape membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Atoms")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "TimingForStructuresModelToBeExecuted1")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (qualified-name "TimingForStructuresModelToBeExecuted1")))))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "OneToOneConnectorsExecution::MyWheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "OneToOneConnectorsExecution::MyBikeFork")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Atoms")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "TimingForStructuresModelToBeExecuted2")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (qualified-name "TimingForStructuresModelToBeExecuted2")))))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensDuring")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "OneToOneConnectorsExecution::MyWheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "OneToOneConnectorsExecution::MyBikeFork")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Atoms")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "TimingForStructuresModelToBeExecuted3")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (qualified-name "TimingForStructuresModelToBeExecuted3")))))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensWhile")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensDuring")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensWhile")
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
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 21 16) (end 21 24)) (probe (position 21 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Atoms")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 22 16) (end 22 56)) (probe (position 22 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "TimingForStructuresModelToBeExecuted1")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (qualified-name "TimingForStructuresModelToBeExecuted1")))))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 23 16) (end 23 52)) (probe (position 23 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "OneToOneConnectorsExecution::MyWheel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 24 16) (end 24 55)) (probe (position 24 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "OneToOneConnectorsExecution::MyBikeFork")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 61 16) (end 61 24)) (probe (position 61 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Atoms")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 62 16) (end 62 56)) (probe (position 62 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "TimingForStructuresModelToBeExecuted2")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (qualified-name "TimingForStructuresModelToBeExecuted2")))))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 63 16) (end 63 42)) (probe (position 63 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensDuring")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 64 16) (end 64 52)) (probe (position 64 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "OneToOneConnectorsExecution::MyWheel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 65 16) (end 65 55)) (probe (position 65 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "OneToOneConnectorsExecution::MyBikeFork")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 135 16) (end 135 24)) (probe (position 135 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Atoms")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 136 16) (end 136 56)) (probe (position 136 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "TimingForStructuresModelToBeExecuted3")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (qualified-name "TimingForStructuresModelToBeExecuted3")))))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 137 16) (end 137 39)) (probe (position 137 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 138 16) (end 138 41)) (probe (position 138 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensWhile")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 139 16) (end 139 57)) (probe (position 139 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 140 16) (end 140 60)) (probe (position 140 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 6 16) (end 6 57)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 7 16) (end 7 60)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 8 16) (end 8 39)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 43 16) (end 43 57)) (probe (position 43 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 44 16) (end 44 60)) (probe (position 44 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 45 16) (end 45 39)) (probe (position 45 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 46 16) (end 46 42)) (probe (position 46 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensDuring")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 116 16) (end 116 57)) (probe (position 116 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 117 16) (end 117 60)) (probe (position 117 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 118 16) (end 118 39)) (probe (position 118 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_5_timing_for_structures.md") (range (start 119 16) (end 119 41)) (probe (position 119 16))
    (reference (id (source (node (document "memory://snapshot/a_3_5_timing_for_structures.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensWhile")
      (outcome (status unresolved)))
  )
)
~~~
