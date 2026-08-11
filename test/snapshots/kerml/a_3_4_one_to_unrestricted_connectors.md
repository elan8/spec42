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
  (document "a_3_4_one_to_unrestricted_connectors.md"
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
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e6c7304c008da73f3666c7775d3b842c07a155df5ef5e8cb0506c32aa7d2a252") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))) (kind "package") (name "OneToUnrestrictedConnectorsExecution") (declared-name "OneToUnrestrictedConnectorsExecution") (range (start (line 21) (character 0)) (end (line 21) (character 1209))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 26) (character 1)) (end (line 26) (character 25))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 26) (character 16)) (end (line 26) (character 21))))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 27) (character 1)) (end (line 27) (character 64))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "OneToUnrestrictedConnectorsModelToBeExecuted::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 27) (character 16)) (end (line 27) (character 60))))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBike"))) (kind "classifier decl") (name "MyBike") (declared-name "MyBike") (range (start (line 53) (character 1)) (end (line 53) (character 244))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket"))) (kind "classifier decl") (name "MyBikeBasket") (declared-name "MyBikeBasket") (range (start (line 37) (character 1)) (end (line 37) (character 61))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket1"))) (kind "classifier decl") (name "MyBikeBasket1") (declared-name "MyBikeBasket1") (range (start (line 33) (character 1)) (end (line 33) (character 49))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket1_Fork1_BBF_Link"))) (kind "kermlDecl") (name "MyBikeBasket1_Fork1_BBF_Link") (declared-name "MyBikeBasket1_Fork1_BBF_Link") (range (start (line 40) (character 1)) (end (line 40) (character 163))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket2"))) (kind "classifier decl") (name "MyBikeBasket2") (declared-name "MyBikeBasket2") (range (start (line 35) (character 1)) (end (line 35) (character 49))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket2_Fork1_BBF_Link"))) (kind "kermlDecl") (name "MyBikeBasket2_Fork1_BBF_Link") (declared-name "MyBikeBasket2_Fork1_BBF_Link") (range (start (line 45) (character 1)) (end (line 45) (character 163))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeBasket_Fork_BBF_Link"))) (kind "classifier decl") (name "MyBikeBasket_Fork_BBF_Link") (declared-name "MyBikeBasket_Fork_BBF_Link") (range (start (line 50) (character 1)) (end (line 50) (character 105))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork"))) (kind "import") (name "MyBikeFork") (declared-name "MyBikeFork") (range (start (line 30) (character 1)) (end (line 30) (character 56))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "OneToOneConnectorsExecution::MyBikeFork") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 30) (character 16)) (end (line 30) (character 55))))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork1"))) (kind "import") (name "MyBikeFork1") (declared-name "MyBikeFork1") (range (start (line 28) (character 1)) (end (line 28) (character 57))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "OneToOneConnectorsExecution::MyBikeFork1") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 28) (character 16)) (end (line 28) (character 56))))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork2"))) (kind "import") (name "MyBikeFork2") (declared-name "MyBikeFork2") (range (start (line 29) (character 1)) (end (line 29) (character 57))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "OneToOneConnectorsExecution::MyBikeFork2") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 29) (character 16)) (end (line 29) (character 56))))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 32) (character 1)) (end (line 32) (character 8))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 34) (character 1)) (end (line 34) (character 8))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 39) (character 1)) (end (line 39) (character 8))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword3"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 44) (character 1)) (end (line 44) (character 8))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::_atom#metadata_keyword4"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 52) (character 1)) (end (line 52) (character 8))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted"))) (kind "package") (name "OneToUnrestrictedConnectorsModelToBeExecuted") (declared-name "OneToUnrestrictedConnectorsModelToBeExecuted") (range (start (line 1) (character 0)) (end (line 1) (character 434))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::Bicycle"))) (kind "classifier decl") (name "Bicycle") (declared-name "Bicycle") (range (start (line 8) (character 1)) (end (line 8) (character 176))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasket"))) (kind "classifier decl") (name "BikeBasket") (declared-name "BikeBasket") (range (start (line 13) (character 1)) (end (line 13) (character 23))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::BikeBasketFixed"))) (kind "kermlDecl") (name "BikeBasketFixed") (declared-name "BikeBasketFixed") (range (start (line 15) (character 1)) (end (line 15) (character 96))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::BikeFork"))) (kind "import") (name "BikeFork") (declared-name "BikeFork") (range (start (line 6) (character 1)) (end (line 6) (character 61))) (parent (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted"))) (authored (membership (kind Import) (visibility "private") (import (reference "WithoutConnectorsModelToBeExecuted::BikeFork") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 60))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Atoms::*") (range (start (line 26) (character 16)) (end (line 26) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "OneToUnrestrictedConnectorsModelToBeExecuted::*") (range (start (line 27) (character 16)) (end (line 27) (character 60))) (outcome (status resolved) (target (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted")))))
    (reference (id (source (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork"))) (kind membershipImport) (ordinal 0)) (authored-target "OneToOneConnectorsExecution::MyBikeFork") (range (start (line 30) (character 16)) (end (line 30) (character 55))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork1"))) (kind membershipImport) (ordinal 0)) (authored-target "OneToOneConnectorsExecution::MyBikeFork1") (range (start (line 28) (character 16)) (end (line 28) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsExecution::MyBikeFork2"))) (kind membershipImport) (ordinal 0)) (authored-target "OneToOneConnectorsExecution::MyBikeFork2") (range (start (line 29) (character 16)) (end (line 29) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToUnrestrictedConnectorsModelToBeExecuted::BikeFork"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork") (range (start (line 6) (character 16)) (end (line 6) (character 60))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
