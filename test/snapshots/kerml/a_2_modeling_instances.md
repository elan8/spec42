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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "67d5b71f69b703c5155328bc785c8cba7493f57b826d4809c00711cd8f67dfdf") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ModelingInstances"))) (kind "package") (name "ModelingInstances") (declared-name "ModelingInstances"))
    (element (id (node (document "d0") (qualified-name "ModelingInstances::Bicycle"))) (kind "classifier decl") (name "Bicycle") (declared-name "Bicycle") (parent (node (document "d0") (qualified-name "ModelingInstances"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstances::MyBike"))) (kind "classifier decl") (name "MyBike") (declared-name "MyBike") (parent (node (document "d0") (qualified-name "ModelingInstances"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstances::Vehicle"))) (kind "classifier decl") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "ModelingInstances"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstances::YourBike"))) (kind "classifier decl") (name "YourBike") (declared-name "YourBike") (parent (node (document "d0") (qualified-name "ModelingInstances"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))) (kind "package") (name "ModelingInstancesWithAtoms") (declared-name "ModelingInstancesWithAtoms"))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::Bicycle"))) (kind "classifier decl") (name "Bicycle") (declared-name "Bicycle") (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::Garage"))) (kind "classifier decl") (name "Garage") (declared-name "Garage") (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::MyBike"))) (kind "classifier decl") (name "MyBike") (declared-name "MyBike") (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::OurBicycle"))) (kind "classifier decl") (name "OurBicycle") (declared-name "OurBicycle") (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::OurGarage"))) (kind "classifier decl") (name "OurGarage") (declared-name "OurGarage") (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::Vehicle"))) (kind "classifier decl") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::YourBike"))) (kind "classifier decl") (name "YourBike") (declared-name "YourBike") (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))))
    (element (id (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::atom"))) (kind "import") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "ModelingInstancesWithAtoms"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::atom") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ModelingInstancesWithAtoms::atom"))) (kind membershipImport) (ordinal 0)) (authored-target "Atoms::atom") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 16 16) (end 16 27)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "ModelingInstancesWithAtoms::atom"))
        (kind membershipImport) (ordinal 0) (authored-target "Atoms::atom")
        (range (start 16 16) (end 16 27))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
