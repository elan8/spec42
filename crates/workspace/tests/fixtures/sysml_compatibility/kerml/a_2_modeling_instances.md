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
    (element (kind "package") (id (node (document "d0") (qualified-name "ModelingInstances"))) (name "ModelingInstances") (declared-name "ModelingInstances")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ModelingInstances::Bicycle"))) (name "Bicycle") (declared-name "Bicycle"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ModelingInstances::MyBike"))) (name "MyBike") (declared-name "MyBike"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ModelingInstances::Vehicle"))) (name "Vehicle") (declared-name "Vehicle"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ModelingInstances::YourBike"))) (name "YourBike") (declared-name "YourBike"))
      )
    )
    (element (kind "package") (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))) (name "ModelingInstancesWithAtoms") (declared-name "ModelingInstancesWithAtoms")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::Bicycle"))) (name "Bicycle") (declared-name "Bicycle"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::Garage"))) (name "Garage") (declared-name "Garage"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::MyBike"))) (name "MyBike") (declared-name "MyBike"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::OurBicycle"))) (name "OurBicycle") (declared-name "OurBicycle"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::OurGarage"))) (name "OurGarage") (declared-name "OurGarage"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::Vehicle"))) (name "Vehicle") (declared-name "Vehicle"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::YourBike"))) (name "YourBike") (declared-name "YourBike"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::_atom"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::_atom#metadata_keyword"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::_atom#metadata_keyword2"))) (name "atom") (declared-name "atom"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::atom"))) (name "atom") (declared-name "atom"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::_atom"))) (to (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::_atom#metadata_keyword"))) (to (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::_atom#metadata_keyword2"))) (to (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
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
  (document "kerml/a_2_modeling_instances.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 27))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 21 1) (end 21 8))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 23 1) (end 23 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 23 1) (end 23 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 33 1) (end 33 8))
      )
    )
  )
)
~~~
