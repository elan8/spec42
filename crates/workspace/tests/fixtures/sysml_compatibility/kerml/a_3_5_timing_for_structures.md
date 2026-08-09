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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwStruct,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwStruct,Ident,KwUnions,Ident,Comma,Ident,Comma,Ident,Semicolon,
Hash,Ident,
KwStruct,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwStruct,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,KwUnions,Ident,Comma,Ident,Semicolon,
KwConnector,Ident,Colon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,Star,CloseSquare,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwStruct,Ident,KwSpecializes,Ident,ColonColon,Ident,Semicolon,
KwStruct,Ident,KwSpecializes,Ident,ColonColon,Ident,Semicolon,
KwStruct,Ident,KwSpecializes,Ident,ColonColon,Ident,Semicolon,
KwStruct,Ident,KwSpecializes,Ident,ColonColon,Ident,Semicolon,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,
KwUnions,Ident,Comma,Ident,Comma,
Ident,Comma,Ident,Semicolon,
KwStruct,Ident,KwUnions,Ident,Comma,Ident,Semicolon,
Hash,Ident,
KwStruct,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwConnector,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,
KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,Star,CloseSquare,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwStruct,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,KwUnions,Ident,Comma,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwConnector,Ident,Colon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,Star,CloseSquare,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
RegularComment,
Hash,Ident,
KwStruct,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwStruct,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
Hash,Ident,
KwStruct,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwStruct,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwStruct,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwStruct,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwStruct,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwStruct,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
Hash,Ident,
KwStruct,Ident,KwSpecializes,Ident,Semicolon,
RegularComment,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,
KwUnions,Ident,Comma,Ident,Comma,
Ident,Comma,Ident,Semicolon,
Hash,Ident,
KwStruct,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwConnector,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,
KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,Star,CloseSquare,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'TimingForStructuresModelToBeExecuted1'
    (documentation)
    (import_decl private 'WithoutConnectorsModelToBeExecuted::Wheel')
    (import_decl private 'WithoutConnectorsModelToBeExecuted::BikeFork')
    (import_decl private 'Occurrences::Occurrence')
    (structure_def 'Bicycle'
      (feature_def 'rollsOn' : 'Wheel' multiplicity :> 'timeCoincidentOccurrences')
      (feature_def 'holdsWheel' : 'BikeFork' multiplicity :> 'timeCoincidentOccurrences')))
  (package_def 'TimingForStructuresExecution1'
    (documentation)
    (import_decl private 'Atoms::*')
    (import_decl private 'TimingForStructuresModelToBeExecuted1::*')
    (import_decl private 'OneToOneConnectorsExecution::MyWheel')
    (import_decl private 'OneToOneConnectorsExecution::MyBikeFork')
    (structure_def 'MyBikeTimeCoincident' unions 'MyWheel', 'MyBikeFork', 'MyBike')
    (structure_def #'atom' 'MyBike' :> 'Bicycle'
      (feature_def :>> 'self' : 'MyBike')
      (feature_def :>> 'timeCoincidentOccurrences' : 'MyBikeTimeCoincident' multiplicity)
      (feature_def :>> 'rollsOn' : 'MyWheel')
      (feature_def :>> 'holdsWheel' : 'MyBikeFork')))
  (package_def 'TimingForStructuresModelToBeExecuted2'
    (documentation)
    (import_decl private 'WithoutConnectorsModelToBeExecuted::Wheel')
    (import_decl private 'WithoutConnectorsModelToBeExecuted::BikeFork')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensDuring')
    (structure_def 'Bicycle'
      (feature_def 'rollsOn' : 'Wheel' multiplicity)
      (feature_def 'holdsWheel' : 'BikeFork' multiplicity)
      (feature_def 'allParts' : 'Occurrence' unions 'rollsOn', 'holdsWheel')
      (connector_def 'b_during_ap' : 'HappensDuring'
        (connector_end)
        (connector_end))))
  (package_def 'TimingForStructuresExecution2'
    (documentation)
    (import_decl private 'Atoms::*')
    (import_decl private 'TimingForStructuresModelToBeExecuted2::*')
    (import_decl private 'Occurrences::HappensDuring')
    (import_decl private 'OneToOneConnectorsExecution::MyWheel')
    (import_decl private 'OneToOneConnectorsExecution::MyBikeFork')
    (structure_def 'MyWheel1' :> 'OneToOneConnectorsExecution::MyWheel1')
    (structure_def 'MyWheel2' :> 'OneToOneConnectorsExecution::MyWheel2')
    (structure_def 'MyBikeFork1' :> 'OneToOneConnectorsExecution::MyBikeFork1')
    (structure_def 'MyBikeFork2' :> 'OneToOneConnectorsExecution::MyBikeFork2')
    (association_def #'atom' 'MyBike_During_Wheel1_Link' :> 'HappensDuring'
      (feature_def end :>> 'shorterOccurrence' : 'MyBike')
      (feature_def end :>> 'longerOccurrence' : 'MyWheel1'))
    (association_def #'atom' 'MyBike_During_Wheel2_Link' :> 'HappensDuring'
      (feature_def end :>> 'shorterOccurrence' : 'MyBike')
      (feature_def end :>> 'longerOccurrence' : 'MyWheel2'))
    (association_def #'atom' 'MyBike_During_Fork1_Link' :> 'HappensDuring'
      (feature_def end :>> 'shorterOccurrence' : 'MyBike')
      (feature_def end :>> 'longerOccurrence' : 'MyBikeFork1'))
    (association_def #'atom' 'MyBike_During_Fork2_Link' :> 'HappensDuring'
      (feature_def end :>> 'shorterOccurrence' : 'MyBike')
      (feature_def end :>> 'longerOccurrence' : 'MyBikeFork2'))
    (association_def 'MyBike_During_Parts_Link' :> 'HappensDuring' unions 'MyBike_During_Wheel1_Link', 'MyBike_During_Fork1_Link', 'MyBike_During_Wheel2_Link', 'MyBike_During_Fork2_Link')
    (structure_def 'MyBikeParts' unions 'MyWheel', 'MyBikeFork')
    (structure_def #'atom' 'MyBike' :> 'Bicycle'
      (feature_def :>> 'rollsOn' : 'MyWheel')
      (feature_def :>> 'holdsWheel' : 'MyBikeFork')
      (feature_def :>> 'allParts' : 'MyBikeParts' multiplicity)
      (feature_def :>> 'self' : 'MyBike')
      (connector_def redefines 'b_during_ap' : 'MyBike_During_Parts_Link' multiplicity
        (connector_end)
        (connector_end))))
  (package_def 'TimingForStructuresModelToBeExecuted3'
    (documentation)
    (import_decl private 'WithoutConnectorsModelToBeExecuted::Wheel')
    (import_decl private 'WithoutConnectorsModelToBeExecuted::BikeFork')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensWhile')
    (structure_def 'Bicycle'
      (feature_def 'rollsOn' : 'Wheel' multiplicity)
      (feature_def 'holdsWheel' : 'BikeFork' multiplicity)
      (feature_def 'allParts' : 'Occurrence' unions 'rollsOn', 'holdsWheel')
      (feature_def :>> 'endShot' : 'Bicycle')
      (connector_def 'be_while_pe' : 'HappensWhile'
        (connector_end)
        (connector_end))))
  (package_def 'TimingForStructuresExecution3'
    (documentation)
    (import_decl private 'Atoms::*')
    (import_decl private 'TimingForStructuresModelToBeExecuted3::*')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensWhile')
    (import_decl private 'WithoutConnectorsModelToBeExecuted::Wheel')
    (import_decl private 'WithoutConnectorsModelToBeExecuted::BikeFork')
    (comment)
    (structure_def #'atom' 'MyWheel1End' :> 'Wheel')
    (structure_def #'atom' 'MyWheel1' :> 'Wheel'
      (feature_def :>> 'endShot' : 'MyWheel1End'))
    (structure_def #'atom' 'MyWheel2End' :> 'Wheel')
    (structure_def #'atom' 'MyWheel2' :> 'Wheel'
      (feature_def :>> 'endShot' : 'MyWheel2End'))
    (structure_def 'MyBikeFork1End' :> 'BikeFork')
    (structure_def #'atom' 'MyBikeFork1' :> 'BikeFork'
      (feature_def :>> 'endShot' : 'MyBikeFork1End'))
    (structure_def 'MyBikeFork2End' :> 'BikeFork')
    (structure_def #'atom' 'MyBikeFork2' :> 'BikeFork'
      (feature_def :>> 'endShot' : 'MyBikeFork2End'))
    (structure_def #'atom' 'MyBikeEnd' :> 'Bicycle')
    (comment)
    (association_def #'atom' 'MyBikeEnd_While_Wheel1End_Link' :> 'HappensWhile'
      (feature_def end :>> 'thisOccurrence' : 'MyBikeEnd')
      (feature_def end :>> 'thatOccurrence' : 'MyWheel1End'))
    (association_def #'atom' 'MyBikeEnd_While_Wheel2End_Link' :> 'HappensWhile'
      (feature_def end :>> 'thisOccurrence' : 'MyBikeEnd')
      (feature_def end :>> 'thatOccurrence' : 'MyWheel2End'))
    (association_def #'atom' 'MyBikeEnd_While_Fork1End_Link' :> 'HappensWhile'
      (feature_def end :>> 'thisOccurrence' : 'MyBikeEnd')
      (feature_def end :>> 'thatOccurrence' : 'MyBikeFork1End'))
    (association_def #'atom' 'MyBikeEnd_While_Fork2End_Link' :> 'HappensWhile'
      (feature_def end :>> 'thisOccurrence' : 'MyBikeEnd')
      (feature_def end :>> 'thatOccurrence' : 'MyBikeFork2End'))
    (association_def 'MyBikeEnd_While_PartsEnd_Link' :> 'HappensWhile' unions 'MyBikeEnd_While_Wheel1End_Link', 'MyBikeEnd_While_Fork1End_Link', 'MyBikeEnd_While_Wheel2End_Link', 'MyBikeEnd_While_Fork2End_Link')
    (structure_def #'atom' 'MyBike' :> 'Bicycle'
      (feature_def :>> 'endShot' : 'MyBikeEnd')
      (connector_def redefines 'be_while_pe' : 'MyBikeEnd_While_PartsEnd_Link' multiplicity
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package TimingForStructuresModelToBeExecuted1 {
    doc /* 
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
    doc /* 
	 */

    private import Atoms::*;
    private import TimingForStructuresModelToBeExecuted1::*;
    private import OneToOneConnectorsExecution::MyWheel;
    private import OneToOneConnectorsExecution::MyBikeFork;

    struct MyBikeTimeCoincident unions MyWheel, MyBikeFork, MyBike;

    #atom struct MyBike specializes Bicycle {
        feature redefines self : MyBike;
        feature redefines timeCoincidentOccurrences : MyBikeTimeCoincident [5];
        feature redefines rollsOn : MyWheel;
        feature redefines holdsWheel : MyBikeFork;
    }
}

package TimingForStructuresModelToBeExecuted2 {
    doc /* 
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
    doc /* 
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

    #atom assoc MyBike_During_Wheel1_Link specializes HappensDuring {
        end feature redefines shorterOccurrence : MyBike;
        end feature redefines longerOccurrence : MyWheel1;
    }
    #atom assoc MyBike_During_Wheel2_Link specializes HappensDuring {
        end feature redefines shorterOccurrence : MyBike;
        end feature redefines longerOccurrence : MyWheel2;
    }
    #atom assoc MyBike_During_Fork1_Link specializes HappensDuring {
        end feature redefines shorterOccurrence : MyBike;
        end feature redefines longerOccurrence : MyBikeFork1;
    }
    #atom assoc MyBike_During_Fork2_Link specializes HappensDuring {
        end feature redefines shorterOccurrence : MyBike;
        end feature redefines longerOccurrence : MyBikeFork2;
    }

    assoc MyBike_During_Parts_Link specializes HappensDuring unions MyBike_During_Wheel1_Link, MyBike_During_Fork1_Link, MyBike_During_Wheel2_Link, MyBike_During_Fork2_Link;

    struct MyBikeParts unions MyWheel, MyBikeFork;

    #atom struct MyBike specializes Bicycle {
        feature redefines rollsOn : MyWheel;
        feature redefines holdsWheel : MyBikeFork;
        feature redefines allParts : MyBikeParts [4];

        feature redefines self : MyBike;
        connector redefines b_during_ap : MyBike_During_Parts_Link [4] from [1] self to [*] allParts;
    }
}

package TimingForStructuresModelToBeExecuted3 {
    doc /* 
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
    doc /* 
	 */

    private import Atoms::*;
    private import TimingForStructuresModelToBeExecuted3::*;
    private import Occurrences::Occurrence;
    private import Occurrences::HappensWhile;
    private import WithoutConnectorsModelToBeExecuted::Wheel;
    private import WithoutConnectorsModelToBeExecuted::BikeFork;

    /* End atoms */
    #atom struct MyWheel1End specializes Wheel;
    #atom struct MyWheel1 specializes Wheel {
        feature redefines endShot : MyWheel1End;
    }
    #atom struct MyWheel2End specializes Wheel;
    #atom struct MyWheel2 specializes Wheel {
        feature redefines endShot : MyWheel2End;
    }
    struct MyBikeFork1End specializes BikeFork;
    #atom struct MyBikeFork1 specializes BikeFork {
        feature redefines endShot : MyBikeFork1End;
    }
    struct MyBikeFork2End specializes BikeFork;
    #atom struct MyBikeFork2 specializes BikeFork {
        feature redefines endShot : MyBikeFork2End;
    }
    #atom struct MyBikeEnd specializes Bicycle;

    /* HappensWhile atoms */
    #atom assoc MyBikeEnd_While_Wheel1End_Link specializes HappensWhile {
        end feature redefines thisOccurrence : MyBikeEnd;
        end feature redefines thatOccurrence : MyWheel1End;
    }
    #atom assoc MyBikeEnd_While_Wheel2End_Link specializes HappensWhile {
        end feature redefines thisOccurrence : MyBikeEnd;
        end feature redefines thatOccurrence : MyWheel2End;
    }
    #atom assoc MyBikeEnd_While_Fork1End_Link specializes HappensWhile {
        end feature redefines thisOccurrence : MyBikeEnd;
        end feature redefines thatOccurrence : MyBikeFork1End;
    }
    #atom assoc MyBikeEnd_While_Fork2End_Link specializes HappensWhile {
        end feature redefines thisOccurrence : MyBikeEnd;
        end feature redefines thatOccurrence : MyBikeFork2End;
    }

    assoc MyBikeEnd_While_PartsEnd_Link specializes HappensWhile unions MyBikeEnd_While_Wheel1End_Link, MyBikeEnd_While_Fork1End_Link, MyBikeEnd_While_Wheel2End_Link, MyBikeEnd_While_Fork2End_Link;

    #atom struct MyBike specializes Bicycle {
        feature redefines endShot : MyBikeEnd;
        connector redefines be_while_pe : MyBikeEnd_While_PartsEnd_Link [4] from [1] endShot to [*] endShot.allParts.endShot;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'timeCoincidentOccurrences'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'timeCoincidentOccurrences'
semantic.unresolved_name 'self'
semantic.unresolved_name 'timeCoincidentOccurrences'
semantic.unresolved_name 'MyWheel'
semantic.unresolved_name 'MyBikeFork'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'OneToOneConnectorsExecution::MyWheel1'
semantic.unresolved_name 'OneToOneConnectorsExecution::MyWheel2'
semantic.unresolved_name 'OneToOneConnectorsExecution::MyBikeFork1'
semantic.unresolved_name 'OneToOneConnectorsExecution::MyBikeFork2'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'shorterOccurrence'
semantic.unresolved_name 'longerOccurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'shorterOccurrence'
semantic.unresolved_name 'longerOccurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'shorterOccurrence'
semantic.unresolved_name 'longerOccurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'shorterOccurrence'
semantic.unresolved_name 'longerOccurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'MyWheel'
semantic.unresolved_name 'MyBikeFork'
semantic.unresolved_name 'self'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'thisOccurrence'
semantic.unresolved_name 'thatOccurrence'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'thisOccurrence'
semantic.unresolved_name 'thatOccurrence'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'thisOccurrence'
semantic.unresolved_name 'thatOccurrence'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'thisOccurrence'
semantic.unresolved_name 'thatOccurrence'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'endShot'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'timeCoincidentOccurrences'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'timeCoincidentOccurrences'
semantic.unresolved_name 'self'
semantic.unresolved_name 'timeCoincidentOccurrences'
semantic.unresolved_name 'MyWheel'
semantic.unresolved_name 'MyBikeFork'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'OneToOneConnectorsExecution::MyWheel1'
semantic.unresolved_name 'OneToOneConnectorsExecution::MyWheel2'
semantic.unresolved_name 'OneToOneConnectorsExecution::MyBikeFork1'
semantic.unresolved_name 'OneToOneConnectorsExecution::MyBikeFork2'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'shorterOccurrence'
semantic.unresolved_name 'longerOccurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'shorterOccurrence'
semantic.unresolved_name 'longerOccurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'shorterOccurrence'
semantic.unresolved_name 'longerOccurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'shorterOccurrence'
semantic.unresolved_name 'longerOccurrence'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'MyWheel'
semantic.unresolved_name 'MyBikeFork'
semantic.unresolved_name 'self'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'thisOccurrence'
semantic.unresolved_name 'thatOccurrence'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'thisOccurrence'
semantic.unresolved_name 'thatOccurrence'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'thisOccurrence'
semantic.unresolved_name 'thatOccurrence'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'thisOccurrence'
semantic.unresolved_name 'thatOccurrence'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'endShot'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "TimingForStructuresExecution1"))) (name "TimingForStructuresExecution1") (declared-name "TimingForStructuresExecution1")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution1::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution1::*#import"))) (name "*") (declared-name "*"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution1::MyBike"))) (name "MyBike") (declared-name "MyBike"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution1::MyBikeFork"))) (name "MyBikeFork") (declared-name "MyBikeFork"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution1::MyBikeTimeCoincident"))) (name "MyBikeTimeCoincident") (declared-name "MyBikeTimeCoincident"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution1::MyWheel"))) (name "MyWheel") (declared-name "MyWheel"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution1::_atom"))) (name "atom") (declared-name "atom"))
      )
    )
    (element (kind "package") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2"))) (name "TimingForStructuresExecution2") (declared-name "TimingForStructuresExecution2")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::HappensDuring"))) (name "HappensDuring") (declared-name "HappensDuring"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBike"))) (name "MyBike") (declared-name "MyBike"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBikeFork"))) (name "MyBikeFork") (declared-name "MyBikeFork"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBikeFork1"))) (name "MyBikeFork1") (declared-name "MyBikeFork1"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBikeFork2"))) (name "MyBikeFork2") (declared-name "MyBikeFork2"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBikeParts"))) (name "MyBikeParts") (declared-name "MyBikeParts"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBike_During_Fork1_Link"))) (name "MyBike_During_Fork1_Link") (declared-name "MyBike_During_Fork1_Link"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBike_During_Fork2_Link"))) (name "MyBike_During_Fork2_Link") (declared-name "MyBike_During_Fork2_Link"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBike_During_Parts_Link"))) (name "MyBike_During_Parts_Link") (declared-name "MyBike_During_Parts_Link"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBike_During_Wheel1_Link"))) (name "MyBike_During_Wheel1_Link") (declared-name "MyBike_During_Wheel1_Link"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyBike_During_Wheel2_Link"))) (name "MyBike_During_Wheel2_Link") (declared-name "MyBike_During_Wheel2_Link"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyWheel"))) (name "MyWheel") (declared-name "MyWheel"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyWheel1"))) (name "MyWheel1") (declared-name "MyWheel1"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::MyWheel2"))) (name "MyWheel2") (declared-name "MyWheel2"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom#metadata_keyword"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom#metadata_keyword2"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom#metadata_keyword3"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom#metadata_keyword4"))) (name "atom") (declared-name "atom"))
      )
    )
    (element (kind "package") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3"))) (name "TimingForStructuresExecution3") (declared-name "TimingForStructuresExecution3")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::BikeFork"))) (name "BikeFork") (declared-name "BikeFork"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::HappensWhile"))) (name "HappensWhile") (declared-name "HappensWhile"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBike"))) (name "MyBike") (declared-name "MyBike"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeEnd"))) (name "MyBikeEnd") (declared-name "MyBikeEnd"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeEnd_While_Fork1End_Link"))) (name "MyBikeEnd_While_Fork1End_Link") (declared-name "MyBikeEnd_While_Fork1End_Link"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeEnd_While_Fork2End_Link"))) (name "MyBikeEnd_While_Fork2End_Link") (declared-name "MyBikeEnd_While_Fork2End_Link"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeEnd_While_PartsEnd_Link"))) (name "MyBikeEnd_While_PartsEnd_Link") (declared-name "MyBikeEnd_While_PartsEnd_Link"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeEnd_While_Wheel1End_Link"))) (name "MyBikeEnd_While_Wheel1End_Link") (declared-name "MyBikeEnd_While_Wheel1End_Link"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeEnd_While_Wheel2End_Link"))) (name "MyBikeEnd_While_Wheel2End_Link") (declared-name "MyBikeEnd_While_Wheel2End_Link"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeFork1"))) (name "MyBikeFork1") (declared-name "MyBikeFork1"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeFork1End"))) (name "MyBikeFork1End") (declared-name "MyBikeFork1End"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeFork2"))) (name "MyBikeFork2") (declared-name "MyBikeFork2"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyBikeFork2End"))) (name "MyBikeFork2End") (declared-name "MyBikeFork2End"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyWheel1"))) (name "MyWheel1") (declared-name "MyWheel1"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyWheel1End"))) (name "MyWheel1End") (declared-name "MyWheel1End"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyWheel2"))) (name "MyWheel2") (declared-name "MyWheel2"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::MyWheel2End"))) (name "MyWheel2End") (declared-name "MyWheel2End"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::Wheel"))) (name "Wheel") (declared-name "Wheel"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword10"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword11"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword2"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword3"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword4"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword5"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword6"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword7"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword8"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword9"))) (name "atom") (declared-name "atom"))
      )
    )
    (element (kind "package") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1"))) (name "TimingForStructuresModelToBeExecuted1") (declared-name "TimingForStructuresModelToBeExecuted1")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1::Bicycle"))) (name "Bicycle") (declared-name "Bicycle"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1::BikeFork"))) (name "BikeFork") (declared-name "BikeFork"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted1::Wheel"))) (name "Wheel") (declared-name "Wheel"))
      )
    )
    (element (kind "package") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2"))) (name "TimingForStructuresModelToBeExecuted2") (declared-name "TimingForStructuresModelToBeExecuted2")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::Bicycle"))) (name "Bicycle") (declared-name "Bicycle"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::BikeFork"))) (name "BikeFork") (declared-name "BikeFork"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::HappensDuring"))) (name "HappensDuring") (declared-name "HappensDuring"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted2::Wheel"))) (name "Wheel") (declared-name "Wheel"))
      )
    )
    (element (kind "package") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3"))) (name "TimingForStructuresModelToBeExecuted3") (declared-name "TimingForStructuresModelToBeExecuted3")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::Bicycle"))) (name "Bicycle") (declared-name "Bicycle"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::BikeFork"))) (name "BikeFork") (declared-name "BikeFork"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::HappensWhile"))) (name "HappensWhile") (declared-name "HappensWhile"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TimingForStructuresModelToBeExecuted3::Wheel"))) (name "Wheel") (declared-name "Wheel"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution1::_atom"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution1"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom#metadata_keyword"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom#metadata_keyword2"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom#metadata_keyword3"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution2::_atom#metadata_keyword4"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution2"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword10"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword11"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword2"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword3"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword4"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword5"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword6"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword7"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword8"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TimingForStructuresExecution3::_atom#metadata_keyword9"))) (to (node (document "d0") (qualified-name "TimingForStructuresExecution3"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
