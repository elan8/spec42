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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwClassifier,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwConnector,Ident,Colon,Ident,KwFrom,OpenSquare,Star,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
KwClassifier,Ident,Semicolon,
KwAssoc,Ident,OpenCurly,
KwEnd,KwFeature,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,Semicolon,
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
KwFeature,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwConnector,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFrom,OpenSquare,Star,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'OneToUnrestrictedConnectorsModelToBeExecuted'
    (documentation)
    (import_decl private 'WithoutConnectorsModelToBeExecuted::BikeFork')
    (classifier_def 'Bicycle'
      (feature_def 'carrier' : 'BikeBasket' multiplicity)
      (feature_def 'holdsWheel' : 'BikeFork' multiplicity)
      (connector_def 'carrierFixed' : 'BikeBasketFixed'
        (connector_end)
        (connector_end)))
    (classifier_def 'BikeBasket')
    (association_def 'BikeBasketFixed'
      (feature_def end 'basket' : 'BikeBasket')
      (feature_def end 'fixedTo' : 'BikeFork')))
  (package_def 'OneToUnrestrictedConnectorsExecution'
    (documentation)
    (import_decl private 'Atoms::*')
    (import_decl private 'OneToUnrestrictedConnectorsModelToBeExecuted::*')
    (import_decl private 'OneToOneConnectorsExecution::MyBikeFork1')
    (import_decl private 'OneToOneConnectorsExecution::MyBikeFork2')
    (import_decl private 'OneToOneConnectorsExecution::MyBikeFork')
    (classifier_def #'atom' 'MyBikeBasket1' :> 'BikeBasket')
    (classifier_def #'atom' 'MyBikeBasket2' :> 'BikeBasket')
    (classifier_def 'MyBikeBasket' unions 'MyBikeBasket1', 'MyBikeBasket2')
    (association_def #'atom' 'MyBikeBasket1_Fork1_BBF_Link' :> 'BikeBasketFixed'
      (feature_def end :>> 'basket' : 'MyBikeBasket1')
      (feature_def end :>> 'fixedTo' : 'MyBikeFork1'))
    (association_def #'atom' 'MyBikeBasket2_Fork1_BBF_Link' :> 'BikeBasketFixed'
      (feature_def end :>> 'basket' : 'MyBikeBasket2')
      (feature_def end :>> 'fixedTo' : 'MyBikeFork1'))
    (classifier_def 'MyBikeBasket_Fork_BBF_Link' unions 'MyBikeBasket1_Fork1_BBF_Link', 'MyBikeBasket2_Fork1_BBF_Link')
    (classifier_def #'atom' 'MyBike' :> 'Bicycle'
      (feature_def :>> 'carrier' : 'MyBikeBasket' multiplicity)
      (feature_def :>> 'holdsWheel' : 'MyBikeFork' multiplicity)
      (connector_def redefines 'carrierFixed' : 'MyBikeBasket_Fork_BBF_Link' multiplicity
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package OneToUnrestrictedConnectorsModelToBeExecuted {
    doc /* 
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
    doc /* 
	 */

    private import Atoms::*;
    private import OneToUnrestrictedConnectorsModelToBeExecuted::*;
    private import OneToOneConnectorsExecution::MyBikeFork1;
    private import OneToOneConnectorsExecution::MyBikeFork2;
    private import OneToOneConnectorsExecution::MyBikeFork;

    #atom classifier MyBikeBasket1 specializes BikeBasket;
    #atom classifier MyBikeBasket2 specializes BikeBasket;

    classifier MyBikeBasket unions MyBikeBasket1, MyBikeBasket2;

    #atom assoc MyBikeBasket1_Fork1_BBF_Link specializes BikeBasketFixed {
        end feature redefines basket : MyBikeBasket1;
        end feature redefines fixedTo : MyBikeFork1;
    }
    #atom assoc MyBikeBasket2_Fork1_BBF_Link specializes BikeBasketFixed {
        end feature redefines basket : MyBikeBasket2;
        end feature redefines fixedTo : MyBikeFork1;
    }

    classifier MyBikeBasket_Fork_BBF_Link unions MyBikeBasket1_Fork1_BBF_Link, MyBikeBasket2_Fork1_BBF_Link;

    #atom classifier MyBike specializes Bicycle {
        feature redefines carrier : MyBikeBasket [2];
        feature redefines holdsWheel : MyBikeFork [2];
        connector redefines carrierFixed : MyBikeBasket_Fork_BBF_Link [2] from [*] carrier to [1] holdsWheel;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'MyBikeFork1'
semantic.unresolved_name 'MyBikeFork1'
semantic.unresolved_name 'MyBikeFork'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'BikeFork'
semantic.unresolved_name 'MyBikeFork1'
semantic.unresolved_name 'MyBikeFork1'
semantic.unresolved_name 'MyBikeFork'
~~~
# SMG
~~~
(model
  (namespace
    (package 'OneToUnrestrictedConnectorsModelToBeExecuted'
      (documentation)
      (membership_import private -> 'WithoutConnectorsModelToBeExecuted::BikeFork'[unresolved])
      (classifier_def 'Bicycle'
        (feature_def 'carrier' : 'OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasket'[classifier_def]
          (multiplicity_range [*]))
        (feature_def 'holdsWheel' : 'BikeFork'[unresolved]
          (multiplicity_range [*]))
        (connector_def 'carrierFixed' : 'OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasketFixed'[association_def]
          (connector_end 'carrier')
          (connector_end 'holdsWheel')))
      (classifier_def 'BikeBasket')
      (association_def 'BikeBasketFixed'
        (feature_def end 'basket' : 'OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasket'[classifier_def])
        (feature_def end 'fixedTo' : 'BikeFork'[unresolved])))
    (package 'OneToUnrestrictedConnectorsExecution'
      (documentation)
      (namespace_import private -> 'Atoms'[unresolved])
      (namespace_import private -> 'OneToUnrestrictedConnectorsModelToBeExecuted'[package])
      (membership_import private -> 'OneToOneConnectorsExecution::MyBikeFork1'[unresolved])
      (membership_import private -> 'OneToOneConnectorsExecution::MyBikeFork2'[unresolved])
      (membership_import private -> 'OneToOneConnectorsExecution::MyBikeFork'[unresolved])
      (classifier_def 'MyBikeBasket1' :> 'OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasket'[classifier_def])
      (classifier_def 'MyBikeBasket2' :> 'OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasket'[classifier_def])
      (classifier_def 'MyBikeBasket'
        (unioning)
        (unioning))
      (association_def 'MyBikeBasket1_Fork1_BBF_Link' :> 'OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasketFixed'[association_def]
        (feature_def end :>> 'OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasketFixed::basket'[feature_def] : 'OneToUnrestrictedConnectorsExecution::MyBikeBasket1'[classifier_def])
        (feature_def end :>> 'OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasketFixed::fixedTo'[feature_def] : 'MyBikeFork1'[unresolved]))
      (association_def 'MyBikeBasket2_Fork1_BBF_Link' :> 'OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasketFixed'[association_def]
        (feature_def end :>> 'OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasketFixed::basket'[feature_def] : 'OneToUnrestrictedConnectorsExecution::MyBikeBasket2'[classifier_def])
        (feature_def end :>> 'OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasketFixed::fixedTo'[feature_def] : 'MyBikeFork1'[unresolved]))
      (classifier_def 'MyBikeBasket_Fork_BBF_Link'
        (unioning)
        (unioning))
      (classifier_def 'MyBike' :> 'OneToUnrestrictedConnectorsModelToBeExecuted::Bicycle'[classifier_def]
        (feature_def :>> 'OneToUnrestrictedConnectorsModelToBeExecuted::Bicycle::carrier'[feature_def] : 'OneToUnrestrictedConnectorsExecution::MyBikeBasket'[classifier_def]
          (multiplicity_range [2]))
        (feature_def :>> 'OneToUnrestrictedConnectorsModelToBeExecuted::Bicycle::holdsWheel'[feature_def] : 'MyBikeFork'[unresolved]
          (multiplicity_range [2]))
        (connector_def :>> 'OneToUnrestrictedConnectorsModelToBeExecuted::Bicycle::carrierFixed'[connector_def] : 'OneToUnrestrictedConnectorsExecution::MyBikeBasket_Fork_BBF_Link'[classifier_def]
          (multiplicity_range [2])
          (connector_end 'carrier')
          (connector_end 'holdsWheel'))))))
~~~
