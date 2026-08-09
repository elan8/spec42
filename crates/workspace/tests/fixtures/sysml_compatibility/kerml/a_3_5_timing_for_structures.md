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
(model
  (namespace
    (package 'TimingForStructuresModelToBeExecuted1'
      (documentation)
      (membership_import private -> 'WithoutConnectorsModelToBeExecuted::Wheel'[unresolved])
      (membership_import private -> 'WithoutConnectorsModelToBeExecuted::BikeFork'[unresolved])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (structure_def 'Bicycle'
        (feature_def 'rollsOn' : 'Wheel'[unresolved] :> 'timeCoincidentOccurrences'[unresolved]
          (multiplicity_range [2]))
        (feature_def 'holdsWheel' : 'BikeFork'[unresolved] :> 'timeCoincidentOccurrences'[unresolved]
          (multiplicity_range [2]))))
    (package 'TimingForStructuresExecution1'
      (documentation)
      (namespace_import private -> 'Atoms'[unresolved])
      (namespace_import private -> 'TimingForStructuresModelToBeExecuted1'[package])
      (membership_import private -> 'OneToOneConnectorsExecution::MyWheel'[unresolved])
      (membership_import private -> 'OneToOneConnectorsExecution::MyBikeFork'[unresolved])
      (structure_def 'MyBikeTimeCoincident'
        (unioning)
        (unioning)
        (unioning))
      (structure_def 'MyBike' :> 'TimingForStructuresModelToBeExecuted1::Bicycle'[structure_def]
        (feature_def :>> 'self'[unresolved] : 'TimingForStructuresExecution1::MyBike'[structure_def])
        (feature_def :>> 'timeCoincidentOccurrences'[unresolved] : 'TimingForStructuresExecution1::MyBikeTimeCoincident'[structure_def]
          (multiplicity_range [5]))
        (feature_def :>> 'TimingForStructuresModelToBeExecuted1::Bicycle::rollsOn'[feature_def] : 'MyWheel'[unresolved])
        (feature_def :>> 'TimingForStructuresModelToBeExecuted1::Bicycle::holdsWheel'[feature_def] : 'MyBikeFork'[unresolved])))
    (package 'TimingForStructuresModelToBeExecuted2'
      (documentation)
      (membership_import private -> 'WithoutConnectorsModelToBeExecuted::Wheel'[unresolved])
      (membership_import private -> 'WithoutConnectorsModelToBeExecuted::BikeFork'[unresolved])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::HappensDuring'[unresolved])
      (structure_def 'Bicycle'
        (feature_def 'rollsOn' : 'Wheel'[unresolved]
          (multiplicity_range [2]))
        (feature_def 'holdsWheel' : 'BikeFork'[unresolved]
          (multiplicity_range [2]))
        (feature_def 'allParts' : 'Occurrence'[unresolved])
        (connector_def 'b_during_ap' : 'HappensDuring'[unresolved]
          (connector_end 'self')
          (connector_end 'allParts'))))
    (package 'TimingForStructuresExecution2'
      (documentation)
      (namespace_import private -> 'Atoms'[unresolved])
      (namespace_import private -> 'TimingForStructuresModelToBeExecuted2'[package])
      (membership_import private -> 'Occurrences::HappensDuring'[unresolved])
      (membership_import private -> 'OneToOneConnectorsExecution::MyWheel'[unresolved])
      (membership_import private -> 'OneToOneConnectorsExecution::MyBikeFork'[unresolved])
      (structure_def 'MyWheel1' :> 'OneToOneConnectorsExecution::MyWheel1'[unresolved])
      (structure_def 'MyWheel2' :> 'OneToOneConnectorsExecution::MyWheel2'[unresolved])
      (structure_def 'MyBikeFork1' :> 'OneToOneConnectorsExecution::MyBikeFork1'[unresolved])
      (structure_def 'MyBikeFork2' :> 'OneToOneConnectorsExecution::MyBikeFork2'[unresolved])
      (association_def 'MyBike_During_Wheel1_Link' :> 'HappensDuring'[unresolved]
        (feature_def end :>> 'shorterOccurrence'[unresolved] : 'TimingForStructuresExecution2::MyBike'[structure_def])
        (feature_def end :>> 'longerOccurrence'[unresolved] : 'TimingForStructuresExecution2::MyWheel1'[structure_def]))
      (association_def 'MyBike_During_Wheel2_Link' :> 'HappensDuring'[unresolved]
        (feature_def end :>> 'shorterOccurrence'[unresolved] : 'TimingForStructuresExecution2::MyBike'[structure_def])
        (feature_def end :>> 'longerOccurrence'[unresolved] : 'TimingForStructuresExecution2::MyWheel2'[structure_def]))
      (association_def 'MyBike_During_Fork1_Link' :> 'HappensDuring'[unresolved]
        (feature_def end :>> 'shorterOccurrence'[unresolved] : 'TimingForStructuresExecution2::MyBike'[structure_def])
        (feature_def end :>> 'longerOccurrence'[unresolved] : 'TimingForStructuresExecution2::MyBikeFork1'[structure_def]))
      (association_def 'MyBike_During_Fork2_Link' :> 'HappensDuring'[unresolved]
        (feature_def end :>> 'shorterOccurrence'[unresolved] : 'TimingForStructuresExecution2::MyBike'[structure_def])
        (feature_def end :>> 'longerOccurrence'[unresolved] : 'TimingForStructuresExecution2::MyBikeFork2'[structure_def]))
      (association_def 'MyBike_During_Parts_Link' :> 'HappensDuring'[unresolved]
        (unioning)
        (unioning)
        (unioning)
        (unioning))
      (structure_def 'MyBikeParts'
        (unioning)
        (unioning))
      (structure_def 'MyBike' :> 'TimingForStructuresModelToBeExecuted2::Bicycle'[structure_def]
        (feature_def :>> 'TimingForStructuresModelToBeExecuted2::Bicycle::rollsOn'[feature_def] : 'MyWheel'[unresolved])
        (feature_def :>> 'TimingForStructuresModelToBeExecuted2::Bicycle::holdsWheel'[feature_def] : 'MyBikeFork'[unresolved])
        (feature_def :>> 'TimingForStructuresModelToBeExecuted2::Bicycle::allParts'[feature_def] : 'TimingForStructuresExecution2::MyBikeParts'[structure_def]
          (multiplicity_range [4]))
        (feature_def :>> 'self'[unresolved] : 'TimingForStructuresExecution2::MyBike'[structure_def])
        (connector_def :>> 'TimingForStructuresModelToBeExecuted2::Bicycle::b_during_ap'[connector_def] : 'TimingForStructuresExecution2::MyBike_During_Parts_Link'[association_def]
          (multiplicity_range [4])
          (connector_end 'self')
          (connector_end 'allParts'))))
    (package 'TimingForStructuresModelToBeExecuted3'
      (documentation)
      (membership_import private -> 'WithoutConnectorsModelToBeExecuted::Wheel'[unresolved])
      (membership_import private -> 'WithoutConnectorsModelToBeExecuted::BikeFork'[unresolved])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::HappensWhile'[unresolved])
      (structure_def 'Bicycle'
        (feature_def 'rollsOn' : 'Wheel'[unresolved]
          (multiplicity_range [2]))
        (feature_def 'holdsWheel' : 'BikeFork'[unresolved]
          (multiplicity_range [2]))
        (feature_def 'allParts' : 'Occurrence'[unresolved])
        (feature_def :>> 'endShot'[unresolved] : 'TimingForStructuresModelToBeExecuted3::Bicycle'[structure_def])
        (connector_def 'be_while_pe' : 'HappensWhile'[unresolved]
          (connector_end 'endShot')
          (connector_end 'endShot.allParts.endShot'))))
    (package 'TimingForStructuresExecution3'
      (documentation)
      (namespace_import private -> 'Atoms'[unresolved])
      (namespace_import private -> 'TimingForStructuresModelToBeExecuted3'[package])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::HappensWhile'[unresolved])
      (membership_import private -> 'WithoutConnectorsModelToBeExecuted::Wheel'[unresolved])
      (membership_import private -> 'WithoutConnectorsModelToBeExecuted::BikeFork'[unresolved])
      (structure_def 'MyWheel1End' :> 'Wheel'[unresolved])
      (structure_def 'MyWheel1' :> 'Wheel'[unresolved]
        (feature_def :>> 'endShot'[unresolved] : 'TimingForStructuresExecution3::MyWheel1End'[structure_def]))
      (structure_def 'MyWheel2End' :> 'Wheel'[unresolved])
      (structure_def 'MyWheel2' :> 'Wheel'[unresolved]
        (feature_def :>> 'endShot'[unresolved] : 'TimingForStructuresExecution3::MyWheel2End'[structure_def]))
      (structure_def 'MyBikeFork1End' :> 'BikeFork'[unresolved])
      (structure_def 'MyBikeFork1' :> 'BikeFork'[unresolved]
        (feature_def :>> 'endShot'[unresolved] : 'TimingForStructuresExecution3::MyBikeFork1End'[structure_def]))
      (structure_def 'MyBikeFork2End' :> 'BikeFork'[unresolved])
      (structure_def 'MyBikeFork2' :> 'BikeFork'[unresolved]
        (feature_def :>> 'endShot'[unresolved] : 'TimingForStructuresExecution3::MyBikeFork2End'[structure_def]))
      (structure_def 'MyBikeEnd' :> 'TimingForStructuresModelToBeExecuted3::Bicycle'[structure_def])
      (association_def 'MyBikeEnd_While_Wheel1End_Link' :> 'HappensWhile'[unresolved]
        (feature_def end :>> 'thisOccurrence'[unresolved] : 'TimingForStructuresExecution3::MyBikeEnd'[structure_def])
        (feature_def end :>> 'thatOccurrence'[unresolved] : 'TimingForStructuresExecution3::MyWheel1End'[structure_def]))
      (association_def 'MyBikeEnd_While_Wheel2End_Link' :> 'HappensWhile'[unresolved]
        (feature_def end :>> 'thisOccurrence'[unresolved] : 'TimingForStructuresExecution3::MyBikeEnd'[structure_def])
        (feature_def end :>> 'thatOccurrence'[unresolved] : 'TimingForStructuresExecution3::MyWheel2End'[structure_def]))
      (association_def 'MyBikeEnd_While_Fork1End_Link' :> 'HappensWhile'[unresolved]
        (feature_def end :>> 'thisOccurrence'[unresolved] : 'TimingForStructuresExecution3::MyBikeEnd'[structure_def])
        (feature_def end :>> 'thatOccurrence'[unresolved] : 'TimingForStructuresExecution3::MyBikeFork1End'[structure_def]))
      (association_def 'MyBikeEnd_While_Fork2End_Link' :> 'HappensWhile'[unresolved]
        (feature_def end :>> 'thisOccurrence'[unresolved] : 'TimingForStructuresExecution3::MyBikeEnd'[structure_def])
        (feature_def end :>> 'thatOccurrence'[unresolved] : 'TimingForStructuresExecution3::MyBikeFork2End'[structure_def]))
      (association_def 'MyBikeEnd_While_PartsEnd_Link' :> 'HappensWhile'[unresolved]
        (unioning)
        (unioning)
        (unioning)
        (unioning))
      (structure_def 'MyBike' :> 'TimingForStructuresModelToBeExecuted3::Bicycle'[structure_def]
        (feature_def :>> 'endShot'[unresolved] : 'TimingForStructuresExecution3::MyBikeEnd'[structure_def])
        (connector_def :>> 'TimingForStructuresModelToBeExecuted3::Bicycle::be_while_pe'[connector_def] : 'TimingForStructuresExecution3::MyBikeEnd_While_PartsEnd_Link'[association_def]
          (multiplicity_range [4])
          (connector_end 'endShot')
          (connector_end 'endShot.allParts.endShot'))))))
~~~
