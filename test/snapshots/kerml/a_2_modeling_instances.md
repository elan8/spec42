# META
~~~ini
description=KerML KerML Spec Annex A: A-2-ModelingInstances
type=file
~~~
# SOURCE
~~~kerml
package ModelingInstances {
	doc
	/* 
	 */

	classifier Vehicle;
	classifier Bicycle specializes Vehicle;
	classifier MyBike [1] specializes Bicycle;
	classifier YourBike [1] specializes Bicycle disjoint from MyBike;
}

package ModelingInstancesWithAtoms {
	doc
	/* 
	 */

	private import Atoms::atom;

	classifier Vehicle;
	classifier Bicycle specializes Vehicle;

	#atom
	classifier MyBike specializes Bicycle;
	#atom
	classifier YourBike specializes Bicycle;

	/* Assigning feature values. */

	classifier Garage {
		feature stores : Bicycle [*];
	}
	classifier OurBicycle unions MyBike, YourBike;

	#atom
	classifier OurGarage specializes Garage {
		feature redefines stores : OurBicycle [2];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "a_2_modeling_instances.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 27))
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
KwClassifier,Ident,Semicolon,
KwClassifier,Ident,KwSpecializes,Ident,Semicolon,
KwClassifier,Ident,OpenSquare,DecimalValue,CloseSquare,KwSpecializes,Ident,Semicolon,
KwClassifier,Ident,OpenSquare,DecimalValue,CloseSquare,KwSpecializes,Ident,KwDisjoint,KwFrom,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwClassifier,Ident,Semicolon,
KwClassifier,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwClassifier,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwClassifier,Ident,KwSpecializes,Ident,Semicolon,
RegularComment,
KwClassifier,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
CloseCurly,
KwClassifier,Ident,KwUnions,Ident,Comma,Ident,Semicolon,
Hash,Ident,
KwClassifier,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ModelingInstances'
    (documentation)
    (classifier_def 'Vehicle')
    (classifier_def 'Bicycle' :> 'Vehicle')
    (classifier_def 'MyBike' multiplicity     (multiplicity_range) :> 'Bicycle')
    (classifier_def 'YourBike' multiplicity     (multiplicity_range) :> 'Bicycle' disjoint from 'MyBike'))
  (package_def 'ModelingInstancesWithAtoms'
    (documentation)
    (import_decl private 'Atoms::atom')
    (classifier_def 'Vehicle')
    (classifier_def 'Bicycle' :> 'Vehicle')
    (classifier_def #'atom' 'MyBike' :> 'Bicycle')
    (classifier_def #'atom' 'YourBike' :> 'Bicycle')
    (comment)
    (classifier_def 'Garage'
      (feature_def 'stores' : 'Bicycle' multiplicity))
    (classifier_def 'OurBicycle' unions 'MyBike', 'YourBike')
    (classifier_def #'atom' 'OurGarage' :> 'Garage'
      (feature_def :>> 'stores' : 'OurBicycle' multiplicity))))
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
package ModelingInstances {
	doc
	/* 
	 */

	classifier Vehicle;
	classifier Bicycle specializes Vehicle;
	classifier MyBike [1] specializes Bicycle;
	classifier YourBike [1] specializes Bicycle disjoint from MyBike;
}

package ModelingInstancesWithAtoms {
	doc
	/* 
	 */

	private import Atoms::atom;

	classifier Vehicle;
	classifier Bicycle specializes Vehicle;

	#atom
	classifier MyBike specializes Bicycle;
	#atom
	classifier YourBike specializes Bicycle;

	/* Assigning feature values. */

	classifier Garage {
		feature stores : Bicycle [*];
	}
	classifier OurBicycle unions MyBike, YourBike;

	#atom
	classifier OurGarage specializes Garage {
		feature redefines stores : OurBicycle [2];
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c063a9ae14a7fc3b2301e61b50687d20e1a16966b75fa5762268d2474a6ba0ff") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ModelingInstances"))) (kind "package") (name "ModelingInstances") (declared-name "ModelingInstances") (range (start (line 0) (character 0)) (end (line 0) (character 218))))
    (element (id (node (document "d0") (qualified-name "ModelingInstances::Bicycle"))) (kind "classifier decl") (name "Bicycle") (declared-name "Bicycle") (range (start (line 6) (character 1)) (end (line 6) (character 40))) (parent (node (document "d0") (qualified-name "ModelingInstances"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstances::MyBike"))) (kind "classifier decl") (name "MyBike") (declared-name "MyBike") (range (start (line 7) (character 1)) (end (line 7) (character 43))) (parent (node (document "d0") (qualified-name "ModelingInstances"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstances::Vehicle"))) (kind "classifier decl") (name "Vehicle") (declared-name "Vehicle") (range (start (line 5) (character 1)) (end (line 5) (character 20))) (parent (node (document "d0") (qualified-name "ModelingInstances"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstances::YourBike"))) (kind "classifier decl") (name "YourBike") (declared-name "YourBike") (range (start (line 8) (character 1)) (end (line 8) (character 66))) (parent (node (document "d0") (qualified-name "ModelingInstances"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))) (kind "package") (name "ModelingInstancesWithAtoms") (declared-name "ModelingInstancesWithAtoms") (range (start (line 11) (character 0)) (end (line 11) (character 481))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::Bicycle"))) (kind "classifier decl") (name "Bicycle") (declared-name "Bicycle") (range (start (line 19) (character 1)) (end (line 19) (character 40))) (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::Garage"))) (kind "classifier decl") (name "Garage") (declared-name "Garage") (range (start (line 28) (character 1)) (end (line 28) (character 55))) (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::MyBike"))) (kind "classifier decl") (name "MyBike") (declared-name "MyBike") (range (start (line 22) (character 1)) (end (line 22) (character 39))) (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::OurBicycle"))) (kind "classifier decl") (name "OurBicycle") (declared-name "OurBicycle") (range (start (line 31) (character 1)) (end (line 31) (character 47))) (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::OurGarage"))) (kind "classifier decl") (name "OurGarage") (declared-name "OurGarage") (range (start (line 34) (character 1)) (end (line 34) (character 90))) (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::Vehicle"))) (kind "classifier decl") (name "Vehicle") (declared-name "Vehicle") (range (start (line 18) (character 1)) (end (line 18) (character 20))) (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::YourBike"))) (kind "classifier decl") (name "YourBike") (declared-name "YourBike") (range (start (line 24) (character 1)) (end (line 24) (character 41))) (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 21) (character 1)) (end (line 21) (character 8))) (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 23) (character 1)) (end (line 23) (character 8))) (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 33) (character 1)) (end (line 33) (character 8))) (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::atom"))) (kind "import") (name "atom") (declared-name "atom") (range (start (line 16) (character 1)) (end (line 16) (character 28))) (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::atom") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 27))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::atom"))) (kind membershipImport) (ordinal 0)) (authored-target "Atoms::atom") (range (start (line 16) (character 16)) (end (line 16) (character 27))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
