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
    doc /* 
	 */

    classifier Bicycle {
        feature rollsOn : Wheel [2];
        feature holdsWheel : BikeFork [*];
    }
    classifier Wheel;
    classifier BikeFork;
}

package WithoutConnectorsExecution {
    doc /* 
	 */

    private import Atoms::*;
    private import WithoutConnectorsModelToBeExecuted::*;

    #atom classifier MyWheel1 specializes Wheel;
    #atom classifier MyWheel2 specializes Wheel;

    classifier MyWheel unions MyWheel1, MyWheel2;

    #atom classifier MyBike specializes Bicycle {
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
(model
  (namespace
    (package 'WithoutConnectorsModelToBeExecuted'
      (documentation)
      (classifier_def 'Bicycle'
        (feature_def 'rollsOn' : 'WithoutConnectorsModelToBeExecuted::Wheel'[classifier_def]
          (multiplicity_range [2]))
        (feature_def 'holdsWheel' : 'WithoutConnectorsModelToBeExecuted::BikeFork'[classifier_def]
          (multiplicity_range [*])))
      (classifier_def 'Wheel')
      (classifier_def 'BikeFork'))
    (package 'WithoutConnectorsExecution'
      (documentation)
      (namespace_import private -> 'Atoms'[unresolved])
      (namespace_import private -> 'WithoutConnectorsModelToBeExecuted'[package])
      (classifier_def 'MyWheel1' :> 'WithoutConnectorsModelToBeExecuted::Wheel'[classifier_def])
      (classifier_def 'MyWheel2' :> 'WithoutConnectorsModelToBeExecuted::Wheel'[classifier_def])
      (classifier_def 'MyWheel'
        (unioning)
        (unioning))
      (classifier_def 'MyBike' :> 'WithoutConnectorsModelToBeExecuted::Bicycle'[classifier_def]
        (feature_def :>> 'WithoutConnectorsModelToBeExecuted::Bicycle::rollsOn'[feature_def] : 'WithoutConnectorsExecution::MyWheel'[classifier_def])))))
~~~
