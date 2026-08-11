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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "a_3_2_without_connectors.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 21))
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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "cff4749503e32cb5d023104e113a9eaa3787d08bf9475fc460970d0a1e6afae6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution"))) (kind "package") (name "WithoutConnectorsExecution") (declared-name "WithoutConnectorsExecution") (range (start (line 14) (character 0)) (end (line 14) (character 369))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 19) (character 1)) (end (line 19) (character 25))) (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 19) (character 16)) (end (line 19) (character 21))))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 20) (character 1)) (end (line 20) (character 54))) (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "WithoutConnectorsModelToBeExecuted::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 20) (character 16)) (end (line 20) (character 50))))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::MyBike"))) (kind "classifier decl") (name "MyBike") (declared-name "MyBike") (range (start (line 30) (character 1)) (end (line 30) (character 82))) (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::MyWheel"))) (kind "classifier decl") (name "MyWheel") (declared-name "MyWheel") (range (start (line 27) (character 1)) (end (line 27) (character 46))) (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::MyWheel1"))) (kind "classifier decl") (name "MyWheel1") (declared-name "MyWheel1") (range (start (line 23) (character 1)) (end (line 23) (character 39))) (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::MyWheel2"))) (kind "classifier decl") (name "MyWheel2") (declared-name "MyWheel2") (range (start (line 25) (character 1)) (end (line 25) (character 39))) (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 22) (character 1)) (end (line 22) (character 8))) (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 24) (character 1)) (end (line 24) (character 8))) (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsExecution::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 29) (character 1)) (end (line 29) (character 8))) (parent (node (document "d0") (qualified-name "WithoutConnectorsExecution"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted"))) (kind "package") (name "WithoutConnectorsModelToBeExecuted") (declared-name "WithoutConnectorsModelToBeExecuted") (range (start (line 1) (character 0)) (end (line 1) (character 196))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle"))) (kind "classifier decl") (name "Bicycle") (declared-name "Bicycle") (range (start (line 6) (character 1)) (end (line 6) (character 92))) (parent (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted::BikeFork"))) (kind "classifier decl") (name "BikeFork") (declared-name "BikeFork") (range (start (line 11) (character 1)) (end (line 11) (character 21))) (parent (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel"))) (kind "classifier decl") (name "Wheel") (declared-name "Wheel") (range (start (line 10) (character 1)) (end (line 10) (character 18))) (parent (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "WithoutConnectorsExecution::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Atoms::*") (range (start (line 19) (character 16)) (end (line 19) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "WithoutConnectorsExecution::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "WithoutConnectorsModelToBeExecuted::*") (range (start (line 20) (character 16)) (end (line 20) (character 50))) (outcome (status resolved) (target (node (document "d0") (qualified-name "WithoutConnectorsModelToBeExecuted")))))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
