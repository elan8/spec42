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
  (document "a_3_3_one_to_one_connectors.md"
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
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 25 16) (end 25 21))
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
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "337fa3fc1d3f1afb82edf88715da47886c3b1e49a6243d102e71cfb76ffc7f91") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))) (kind "package") (name "OneToOneConnectorsExecution") (declared-name "OneToOneConnectorsExecution") (range (start (line 20) (character 0)) (end (line 20) (character 1125))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 25) (character 1)) (end (line 25) (character 25))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 25) (character 16)) (end (line 25) (character 21))))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 26) (character 1)) (end (line 26) (character 54))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))) (authored (membership (kind Import) (visibility "public") (import (reference "OneToOneConnectorsModelToBeExecuted::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 26) (character 15)) (end (line 26) (character 50))))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBike"))) (kind "classifier decl") (name "MyBike") (declared-name "MyBike") (range (start (line 52) (character 1)) (end (line 52) (character 226))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeFork"))) (kind "classifier decl") (name "MyBikeFork") (declared-name "MyBikeFork") (range (start (line 36) (character 1)) (end (line 36) (character 55))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeFork1"))) (kind "classifier decl") (name "MyBikeFork1") (declared-name "MyBikeFork1") (range (start (line 32) (character 1)) (end (line 32) (character 45))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeFork2"))) (kind "classifier decl") (name "MyBikeFork2") (declared-name "MyBikeFork2") (range (start (line 34) (character 1)) (end (line 34) (character 45))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeWheel1_Fork1_BWF_Link"))) (kind "kermlDecl") (name "MyBikeWheel1_Fork1_BWF_Link") (declared-name "MyBikeWheel1_Fork1_BWF_Link") (range (start (line 39) (character 2)) (end (line 39) (character 156))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeWheel2_Fork2_BWF_Link"))) (kind "kermlDecl") (name "MyBikeWheel2_Fork2_BWF_Link") (declared-name "MyBikeWheel2_Fork2_BWF_Link") (range (start (line 44) (character 1)) (end (line 44) (character 155))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyBikeWheel_Fork_BWF_Link"))) (kind "classifier decl") (name "MyBikeWheel_Fork_BWF_Link") (declared-name "MyBikeWheel_Fork_BWF_Link") (range (start (line 49) (character 1)) (end (line 49) (character 102))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel"))) (kind "import") (name "MyWheel") (declared-name "MyWheel") (range (start (line 29) (character 1)) (end (line 29) (character 51))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))) (authored (membership (kind Import) (visibility "public") (import (reference "WithoutConnectorsExecution::MyWheel") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 29) (character 15)) (end (line 29) (character 50))))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel1"))) (kind "import") (name "MyWheel1") (declared-name "MyWheel1") (range (start (line 27) (character 1)) (end (line 27) (character 52))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))) (authored (membership (kind Import) (visibility "public") (import (reference "WithoutConnectorsExecution::MyWheel1") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 27) (character 15)) (end (line 27) (character 51))))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel2"))) (kind "import") (name "MyWheel2") (declared-name "MyWheel2") (range (start (line 28) (character 1)) (end (line 28) (character 52))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))) (authored (membership (kind Import) (visibility "public") (import (reference "WithoutConnectorsExecution::MyWheel2") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 28) (character 15)) (end (line 28) (character 51))))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 31) (character 1)) (end (line 31) (character 8))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 33) (character 1)) (end (line 33) (character 8))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 38) (character 1)) (end (line 38) (character 9))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword3"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 43) (character 1)) (end (line 43) (character 8))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsExecution::_atom#metadata_keyword4"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 51) (character 1)) (end (line 51) (character 8))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted"))) (kind "package") (name "OneToOneConnectorsModelToBeExecuted") (declared-name "OneToOneConnectorsModelToBeExecuted") (range (start (line 1) (character 0)) (end (line 1) (character 446))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::Bicycle"))) (kind "classifier decl") (name "Bicycle") (declared-name "Bicycle") (range (start (line 9) (character 1)) (end (line 9) (character 166))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::BikeFork"))) (kind "import") (name "BikeFork") (declared-name "BikeFork") (range (start (line 7) (character 4)) (end (line 7) (character 63))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted"))) (authored (membership (kind Import) (visibility "public") (import (reference "WithoutConnectorsModelToBeExecuted::BikeFork") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 18)) (end (line 7) (character 62))))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::BikeWheelFixed"))) (kind "kermlDecl") (name "BikeWheelFixed") (declared-name "BikeWheelFixed") (range (start (line 14) (character 1)) (end (line 14) (character 89))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::Wheel"))) (kind "import") (name "Wheel") (declared-name "Wheel") (range (start (line 6) (character 4)) (end (line 6) (character 60))) (parent (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted"))) (authored (membership (kind Import) (visibility "public") (import (reference "WithoutConnectorsModelToBeExecuted::Wheel") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 18)) (end (line 6) (character 59))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "OneToOneConnectorsExecution::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Atoms::*") (range (start (line 25) (character 16)) (end (line 25) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToOneConnectorsExecution::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "OneToOneConnectorsModelToBeExecuted::*") (range (start (line 26) (character 15)) (end (line 26) (character 50))) (outcome (status resolved) (target (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted")))))
    (reference (id (source (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsExecution::MyWheel") (range (start (line 29) (character 15)) (end (line 29) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel1"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsExecution::MyWheel1") (range (start (line 27) (character 15)) (end (line 27) (character 51))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToOneConnectorsExecution::MyWheel2"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsExecution::MyWheel2") (range (start (line 28) (character 15)) (end (line 28) (character 51))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::BikeFork"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::BikeFork") (range (start (line 7) (character 18)) (end (line 7) (character 62))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OneToOneConnectorsModelToBeExecuted::Wheel"))) (kind membershipImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::Wheel") (range (start (line 6) (character 18)) (end (line 6) (character 59))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
