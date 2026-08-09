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
# FORMAT
~~~sysml
package ModelingInstances {
    doc /* 
	 */

    classifier Vehicle;
    classifier Bicycle specializes Vehicle;
    classifier MyBike[1] specializes Bicycle;
    classifier YourBike[1] specializes Bicycle disjoint from MyBike;
}

package ModelingInstancesWithAtoms {
    doc /* 
	 */

    private import Atoms::atom;

    classifier Vehicle;
    classifier Bicycle specializes Vehicle;

    #atom classifier MyBike specializes Bicycle;
    #atom classifier YourBike specializes Bicycle;

    /* Assigning feature values. */

    classifier Garage {
        feature stores : Bicycle [*];
    }
    classifier OurBicycle unions MyBike, YourBike;

    #atom classifier OurGarage specializes Garage {
        feature redefines stores : OurBicycle [2];
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
    (package 'ModelingInstances'
      (documentation)
      (classifier_def 'Vehicle')
      (classifier_def 'Bicycle' :> 'ModelingInstances::Vehicle'[classifier_def])
      (classifier_def 'MyBike' :> 'ModelingInstances::Bicycle'[classifier_def]
        (multiplicity_range [1]))
      (classifier_def 'YourBike' :> 'ModelingInstances::Bicycle'[classifier_def]
        (multiplicity_range [1])
        (disjoining_decl)))
    (package 'ModelingInstancesWithAtoms'
      (documentation)
      (membership_import private -> 'Atoms::atom'[unresolved])
      (classifier_def 'Vehicle')
      (classifier_def 'Bicycle' :> 'ModelingInstancesWithAtoms::Vehicle'[classifier_def])
      (classifier_def 'MyBike' :> 'ModelingInstancesWithAtoms::Bicycle'[classifier_def])
      (classifier_def 'YourBike' :> 'ModelingInstancesWithAtoms::Bicycle'[classifier_def])
      (classifier_def 'Garage'
        (feature_def 'stores' : 'ModelingInstancesWithAtoms::Bicycle'[classifier_def]
          (multiplicity_range [*])))
      (classifier_def 'OurBicycle'
        (unioning)
        (unioning))
      (classifier_def 'OurGarage' :> 'ModelingInstancesWithAtoms::Garage'[classifier_def]
        (feature_def :>> 'ModelingInstancesWithAtoms::Garage::stores'[feature_def] : 'ModelingInstancesWithAtoms::OurBicycle'[classifier_def]
          (multiplicity_range [2]))))))
~~~
