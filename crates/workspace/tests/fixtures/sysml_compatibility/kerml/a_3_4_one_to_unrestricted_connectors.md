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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))) (name "OneToUnrestrictedConnectorsExecution") (declared-name "OneToUnrestrictedConnectorsExecution")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::*#import"))) (name "*") (declared-name "*"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBike"))) (name "MyBike") (declared-name "MyBike"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket"))) (name "MyBikeBasket") (declared-name "MyBikeBasket"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket1"))) (name "MyBikeBasket1") (declared-name "MyBikeBasket1"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket1_Fork1_BBF_Link"))) (name "MyBikeBasket1_Fork1_BBF_Link") (declared-name "MyBikeBasket1_Fork1_BBF_Link"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket2"))) (name "MyBikeBasket2") (declared-name "MyBikeBasket2"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket2_Fork1_BBF_Link"))) (name "MyBikeBasket2_Fork1_BBF_Link") (declared-name "MyBikeBasket2_Fork1_BBF_Link"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket_Fork_BBF_Link"))) (name "MyBikeBasket_Fork_BBF_Link") (declared-name "MyBikeBasket_Fork_BBF_Link"))
        (element (kind "import") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork"))) (name "MyBikeFork") (declared-name "MyBikeFork"))
        (element (kind "import") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork1"))) (name "MyBikeFork1") (declared-name "MyBikeFork1"))
        (element (kind "import") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork2"))) (name "MyBikeFork2") (declared-name "MyBikeFork2"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword2"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword3"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword4"))) (name "atom") (declared-name "atom"))
      )
    )
    (element (kind "package") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted"))) (name "OneToUnrestrictedConnectorsModelToBeExecuted") (declared-name "OneToUnrestrictedConnectorsModelToBeExecuted")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::Bicycle"))) (name "Bicycle") (declared-name "Bicycle"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasket"))) (name "BikeBasket") (declared-name "BikeBasket"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasketFixed"))) (name "BikeBasketFixed") (declared-name "BikeBasketFixed"))
        (element (kind "import") (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::BikeFork"))) (name "BikeFork") (declared-name "BikeFork"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom"))) (to (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword"))) (to (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword2"))) (to (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword3"))) (to (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword4"))) (to (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml/a_3_4_one_to_unrestricted_connectors.md"
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
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 32 1) (end 32 8))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 34 1) (end 34 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 34 1) (end 34 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 39 1) (end 39 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 44 1) (end 44 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 52 1) (end 52 8))
      )
    )
  )
)
~~~
