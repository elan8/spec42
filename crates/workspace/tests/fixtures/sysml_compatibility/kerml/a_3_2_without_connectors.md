# META
~~~ini
description=KerML KerML Spec Annex A: A-3-2-WithoutConnectors
type=file
~~~
# SOURCE
~~~kerml

package WithoutConnectorsModelToBeExecuted {
	doc
	/* 
	 */

	classifier Bicycle {
		feature rollsOn : Wheel [2];
		feature holdsWheel : BikeFork [*];
	}
	classifier Wheel;
	classifier BikeFork;
}

package WithoutConnectorsExecution {
	doc
	/* 
	 */

	private import Atoms::*;
	private import WithoutConnectorsModelToBeExecuted::*;

	#atom
	classifier MyWheel1 specializes Wheel;
	#atom
	classifier MyWheel2 specializes Wheel;

	classifier MyWheel unions MyWheel1, MyWheel2;

	#atom
	classifier MyBike specializes Bicycle {
		feature redefines rollsOn : MyWheel;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwClassifier,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
CloseCurly,
KwClassifier,Ident,Semicolon,
KwClassifier,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
Hash,Ident,
KwClassifier,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwClassifier,Ident,KwSpecializes,Ident,Semicolon,
KwClassifier,Ident,KwUnions,Ident,Comma,Ident,Semicolon,
Hash,Ident,
KwClassifier,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'WithoutConnectorsModelToBeExecuted'
    (documentation)
    (classifier_def 'Bicycle'
      (feature_def 'rollsOn' : 'Wheel' multiplicity)
      (feature_def 'holdsWheel' : 'BikeFork' multiplicity))
    (classifier_def 'Wheel')
    (classifier_def 'BikeFork'))
  (package_def 'WithoutConnectorsExecution'
    (documentation)
    (import_decl private 'Atoms::*')
    (import_decl private 'WithoutConnectorsModelToBeExecuted::*')
    (classifier_def #'atom' 'MyWheel1' :> 'Wheel')
    (classifier_def #'atom' 'MyWheel2' :> 'Wheel')
    (classifier_def 'MyWheel' unions 'MyWheel1', 'MyWheel2')
    (classifier_def #'atom' 'MyBike' :> 'Bicycle'
      (feature_def :>> 'rollsOn' : 'MyWheel'))))
~~~
# FORMAT
~~~sysml

package WithoutConnectorsModelToBeExecuted {
	doc
	/* 
	 */

	classifier Bicycle {
		feature rollsOn : Wheel [2];
		feature holdsWheel : BikeFork [*];
	}
	classifier Wheel;
	classifier BikeFork;
}

package WithoutConnectorsExecution {
	doc
	/* 
	 */

	private import Atoms::*;
	private import WithoutConnectorsModelToBeExecuted::*;

	#atom
	classifier MyWheel1 specializes Wheel;
	#atom
	classifier MyWheel2 specializes Wheel;

	classifier MyWheel unions MyWheel1, MyWheel2;

	#atom
	classifier MyBike specializes Bicycle {
		feature redefines rollsOn : MyWheel;
	}
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "WithoutConnectorsExecution"))) (name "WithoutConnectorsExecution") (declared-name "WithoutConnectorsExecution")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::*#import"))) (name "*") (declared-name "*"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::MyBike"))) (name "MyBike") (declared-name "MyBike"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::MyWheel"))) (name "MyWheel") (declared-name "MyWheel"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::MyWheel1"))) (name "MyWheel1") (declared-name "MyWheel1"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::MyWheel2"))) (name "MyWheel2") (declared-name "MyWheel2"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::_atom"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::_atom#metadata_keyword"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::_atom#metadata_keyword2"))) (name "atom") (declared-name "atom"))
      )
    )
    (element (kind "package") (id (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted"))) (name "WithoutConnectorsModelToBeExecuted") (declared-name "WithoutConnectorsModelToBeExecuted")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle"))) (name "Bicycle") (declared-name "Bicycle"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted::BikeFork"))) (name "BikeFork") (declared-name "BikeFork"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel"))) (name "Wheel") (declared-name "Wheel"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "WithoutConnectorsExecution::_atom"))) (to (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "WithoutConnectorsExecution::_atom#metadata_keyword"))) (to (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "WithoutConnectorsExecution::_atom#metadata_keyword2"))) (to (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
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
  (document "kerml/a_3_2_without_connectors.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 21))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 22 1) (end 22 8))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 24 1) (end 24 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 24 1) (end 24 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 29 1) (end 29 8))
      )
    )
  )
)
~~~
