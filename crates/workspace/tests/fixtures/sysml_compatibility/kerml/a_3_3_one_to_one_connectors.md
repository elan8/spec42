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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))) (name "OneToOneConnectorsExecution") (declared-name "OneToOneConnectorsExecution")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::*#import"))) (name "*") (declared-name "*"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBike"))) (name "MyBike") (declared-name "MyBike"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeFork"))) (name "MyBikeFork") (declared-name "MyBikeFork"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeFork1"))) (name "MyBikeFork1") (declared-name "MyBikeFork1"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeFork2"))) (name "MyBikeFork2") (declared-name "MyBikeFork2"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeWheel1_Fork1_BWF_Link"))) (name "MyBikeWheel1_Fork1_BWF_Link") (declared-name "MyBikeWheel1_Fork1_BWF_Link"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeWheel2_Fork2_BWF_Link"))) (name "MyBikeWheel2_Fork2_BWF_Link") (declared-name "MyBikeWheel2_Fork2_BWF_Link"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeWheel_Fork_BWF_Link"))) (name "MyBikeWheel_Fork_BWF_Link") (declared-name "MyBikeWheel_Fork_BWF_Link"))
        (element (kind "import") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel"))) (name "MyWheel") (declared-name "MyWheel"))
        (element (kind "import") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel1"))) (name "MyWheel1") (declared-name "MyWheel1"))
        (element (kind "import") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel2"))) (name "MyWheel2") (declared-name "MyWheel2"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword2"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword3"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword4"))) (name "atom") (declared-name "atom"))
      )
    )
    (element (kind "package") (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted"))) (name "OneToOneConnectorsModelToBeExecuted") (declared-name "OneToOneConnectorsModelToBeExecuted")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::Bicycle"))) (name "Bicycle") (declared-name "Bicycle"))
        (element (kind "import") (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::BikeFork"))) (name "BikeFork") (declared-name "BikeFork"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::BikeWheelFixed"))) (name "BikeWheelFixed") (declared-name "BikeWheelFixed"))
        (element (kind "import") (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::Wheel"))) (name "Wheel") (declared-name "Wheel"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom"))) (to (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword"))) (to (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword2"))) (to (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword3"))) (to (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword4"))) (to (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
