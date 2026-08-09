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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwClassifier,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwConnector,Ident,Colon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
KwAssoc,Ident,OpenCurly,
KwEnd,KwFeature,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
Hash,Ident,
KwClassifier,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwClassifier,Ident,KwSpecializes,Ident,Semicolon,
KwClassifier,Ident,KwUnions,Ident,Comma,Ident,Semicolon,
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
KwClassifier,Ident,KwUnions,Ident,Comma,Ident,Semicolon,
Hash,Ident,
KwClassifier,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwConnector,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'OneToOneConnectorsModelToBeExecuted'
    (documentation)
    (import_decl public 'WithoutConnectorsModelToBeExecuted::Wheel')
    (import_decl public 'WithoutConnectorsModelToBeExecuted::BikeFork')
    (classifier_def 'Bicycle'
      (feature_def 'rollsOn' : 'Wheel' multiplicity)
      (feature_def 'holdsWheel' : 'BikeFork' multiplicity)
      (connector_def 'fixWheel' : 'BikeWheelFixed'
        (connector_end)
        (connector_end)))
    (association_def 'BikeWheelFixed'
      (feature_def end 'wheel' : 'Wheel')
      (feature_def end 'fixedTo' : 'BikeFork')))
  (package_def 'OneToOneConnectorsExecution'
    (documentation)
    (import_decl private 'Atoms::*')
    (import_decl public 'OneToOneConnectorsModelToBeExecuted::*')
    (import_decl public 'WithoutConnectorsExecution::MyWheel1')
    (import_decl public 'WithoutConnectorsExecution::MyWheel2')
    (import_decl public 'WithoutConnectorsExecution::MyWheel')
    (classifier_def #'atom' 'MyBikeFork1' :> 'BikeFork')
    (classifier_def #'atom' 'MyBikeFork2' :> 'BikeFork')
    (classifier_def 'MyBikeFork' unions 'MyBikeFork1', 'MyBikeFork2')
    (association_def #'atom' 'MyBikeWheel1_Fork1_BWF_Link' :> 'BikeWheelFixed'
      (feature_def end :>> 'wheel' : 'MyWheel1')
      (feature_def end :>> 'fixedTo' : 'MyBikeFork1'))
    (association_def #'atom' 'MyBikeWheel2_Fork2_BWF_Link' :> 'BikeWheelFixed'
      (feature_def end :>> 'wheel' : 'MyWheel2')
      (feature_def end :>> 'fixedTo' : 'MyBikeFork2'))
    (classifier_def 'MyBikeWheel_Fork_BWF_Link' unions 'MyBikeWheel1_Fork1_BWF_Link', 'MyBikeWheel2_Fork2_BWF_Link')
    (classifier_def #'atom' 'MyBike' :> 'Bicycle'
      (feature_def :>> 'rollsOn' : 'MyWheel')
      (feature_def :>> 'holdsWheel' : 'MyBikeFork')
      (connector_def redefines 'fixWheel' : 'MyBikeWheel_Fork_BWF_Link' multiplicity
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package OneToOneConnectorsModelToBeExecuted {
    doc /* 
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
    doc /* 
	 */

    private import Atoms::*;
    public import OneToOneConnectorsModelToBeExecuted::*;
    public import WithoutConnectorsExecution::MyWheel1;
    public import WithoutConnectorsExecution::MyWheel2;
    public import WithoutConnectorsExecution::MyWheel;

    #atom classifier MyBikeFork1 specializes BikeFork;
    #atom classifier MyBikeFork2 specializes BikeFork;

    classifier MyBikeFork unions MyBikeFork1, MyBikeFork2;

    #atom assoc MyBikeWheel1_Fork1_BWF_Link specializes BikeWheelFixed {
        end feature redefines wheel : MyWheel1;
        end feature redefines fixedTo : MyBikeFork1;
    }
    #atom assoc MyBikeWheel2_Fork2_BWF_Link specializes BikeWheelFixed {
        end feature redefines wheel : MyWheel2;
        end feature redefines fixedTo : MyBikeFork2;
    }

    classifier MyBikeWheel_Fork_BWF_Link unions MyBikeWheel1_Fork1_BWF_Link, MyBikeWheel2_Fork2_BWF_Link;

    #atom classifier MyBike specializes Bicycle {
        feature redefines rollsOn : MyWheel;
        feature redefines holdsWheel : MyBikeFork;
        connector redefines fixWheel : MyBikeWheel_Fork_BWF_Link [2] from [1] rollsOn to [1] holdsWheel;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'MyWheel1'
semantic.unresolved_name 'MyWheel2'
semantic.unresolved_name 'MyWheel'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'MyWheel1'
semantic.unresolved_name 'MyWheel2'
semantic.unresolved_name 'MyWheel'
~~~
# SMG
~~~
(model
  (namespace
    (package 'OneToOneConnectorsModelToBeExecuted'
      (documentation)
      (membership_import public -> 'WithoutConnectorsModelToBeExecuted::Wheel'[unresolved])
      (membership_import public -> 'WithoutConnectorsModelToBeExecuted::BikeFork'[unresolved])
      (classifier_def 'Bicycle'
        (feature_def 'rollsOn' : 'Wheel'[unresolved]
          (multiplicity_range [2]))
        (feature_def 'holdsWheel' : 'BikeFork'[unresolved]
          (multiplicity_range [*]))
        (connector_def 'fixWheel' : 'OneToOneConnectorsModelToBeExecuted::BikeWheelFixed'[association_def]
          (connector_end 'rollsOn')
          (connector_end 'holdsWheel')))
      (association_def 'BikeWheelFixed'
        (feature_def end 'wheel' : 'Wheel'[unresolved])
        (feature_def end 'fixedTo' : 'BikeFork'[unresolved])))
    (package 'OneToOneConnectorsExecution'
      (documentation)
      (namespace_import private -> 'Atoms'[unresolved])
      (namespace_import public -> 'OneToOneConnectorsModelToBeExecuted'[package])
      (membership_import public -> 'WithoutConnectorsExecution::MyWheel1'[unresolved])
      (membership_import public -> 'WithoutConnectorsExecution::MyWheel2'[unresolved])
      (membership_import public -> 'WithoutConnectorsExecution::MyWheel'[unresolved])
      (classifier_def 'MyBikeFork1' :> 'BikeFork'[unresolved])
      (classifier_def 'MyBikeFork2' :> 'BikeFork'[unresolved])
      (classifier_def 'MyBikeFork'
        (unioning)
        (unioning))
      (association_def 'MyBikeWheel1_Fork1_BWF_Link' :> 'OneToOneConnectorsModelToBeExecuted::BikeWheelFixed'[association_def]
        (feature_def end :>> 'OneToOneConnectorsModelToBeExecuted::BikeWheelFixed::wheel'[feature_def] : 'MyWheel1'[unresolved])
        (feature_def end :>> 'OneToOneConnectorsModelToBeExecuted::BikeWheelFixed::fixedTo'[feature_def] : 'OneToOneConnectorsExecution::MyBikeFork1'[classifier_def]))
      (association_def 'MyBikeWheel2_Fork2_BWF_Link' :> 'OneToOneConnectorsModelToBeExecuted::BikeWheelFixed'[association_def]
        (feature_def end :>> 'OneToOneConnectorsModelToBeExecuted::BikeWheelFixed::wheel'[feature_def] : 'MyWheel2'[unresolved])
        (feature_def end :>> 'OneToOneConnectorsModelToBeExecuted::BikeWheelFixed::fixedTo'[feature_def] : 'OneToOneConnectorsExecution::MyBikeFork2'[classifier_def]))
      (classifier_def 'MyBikeWheel_Fork_BWF_Link'
        (unioning)
        (unioning))
      (classifier_def 'MyBike' :> 'OneToOneConnectorsModelToBeExecuted::Bicycle'[classifier_def]
        (feature_def :>> 'OneToOneConnectorsModelToBeExecuted::Bicycle::rollsOn'[feature_def] : 'MyWheel'[unresolved])
        (feature_def :>> 'OneToOneConnectorsModelToBeExecuted::Bicycle::holdsWheel'[feature_def] : 'OneToOneConnectorsExecution::MyBikeFork'[classifier_def])
        (connector_def :>> 'OneToOneConnectorsModelToBeExecuted::Bicycle::fixWheel'[connector_def] : 'OneToOneConnectorsExecution::MyBikeWheel_Fork_BWF_Link'[classifier_def]
          (multiplicity_range [2])
          (connector_end 'rollsOn')
          (connector_end 'holdsWheel'))))))
~~~
