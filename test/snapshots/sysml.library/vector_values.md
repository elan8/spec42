# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Data Type Library/VectorValues
type=file
~~~
# SOURCE
~~~kerml
standard library package VectorValues {
    doc /*
	 * This package provides a basic model of abstract vectors as well as concrete vectors
	 * whose components are numerical values. The package VectorFunctions defines the 
	 * corresponding vector-space functions.
	 */

    private import ScalarValues::NumericalValue;
    private import ScalarValues::Real;
    private import Collections::Array;

    abstract datatype VectorValue {
        doc /*
		 * A VectorValue is an abstract data type whose values may be operated on using
		 * VectorFunctions.
		 */
    }

    datatype NumericalVectorValue :> VectorValue, Array intersects VectorValue, Array {
        doc /*
		 * A NumericalVectorValue is a kind of VectorValue that is specifically represented
		 * as a one-dimensional array of NumericalValues. The dimension is allowed to be empty,
		 * permitting a NumericalVectorValue of rank 0, which is essentially isomorphic to a
		 * scalar NumericalValue.
		 */

        feature dimension[0..1] :>> dimensions;
        feature :>> elements : NumericalValue;
    }

    datatype CartesianVectorValue :> NumericalVectorValue {
        doc /*
		 * CartesianVectorValue is a specialization Numerical VectorValue for which there are 
		 * specific implementations in VectorFunctions of the abstract vector-space functions.
		 * 
		 * Note: The restriction of the element type to Real is to facilitate
		 * the complete definition of these functions.
		 */

        feature :>> elements : Real;
    }

    datatype ThreeVectorValue :> NumericalVectorValue {
        doc /*
		 * A ThreeVectorValue is a NumericalVectorValue that has dimension 3.
		 */

        feature :>> dimension = 3;
    }

    datatype CartesianThreeVectorValue :> CartesianVectorValue, ThreeVectorValue intersects CartesianVectorValue, ThreeVectorValue {
        doc /*
		 * A CartesianThreeVectorValue is a NumericalVectorValue that is both Cartesian and
		 * has dimension 3.
		 */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vector_values.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 37))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package VectorValues {
    doc /*
	 * This package provides a basic model of abstract vectors as well as concrete vectors
	 * whose components are numerical values. The package VectorFunctions defines the 
	 * corresponding vector-space functions.
	 */

    private import ScalarValues::NumericalValue;
    private import ScalarValues::Real;
    private import Collections::Array;

    abstract datatype VectorValue {
        doc /*
		 * A VectorValue is an abstract data type whose values may be operated on using
		 * VectorFunctions.
		 */
    }

    datatype NumericalVectorValue :> VectorValue, Array intersects VectorValue, Array {
        doc /*
		 * A NumericalVectorValue is a kind of VectorValue that is specifically represented
		 * as a one-dimensional array of NumericalValues. The dimension is allowed to be empty,
		 * permitting a NumericalVectorValue of rank 0, which is essentially isomorphic to a
		 * scalar NumericalValue.
		 */

        feature dimension[0..1] :>> dimensions;
        feature :>> elements : NumericalValue;
    }

    datatype CartesianVectorValue :> NumericalVectorValue {
        doc /*
		 * CartesianVectorValue is a specialization Numerical VectorValue for which there are 
		 * specific implementations in VectorFunctions of the abstract vector-space functions.
		 * 
		 * Note: The restriction of the element type to Real is to facilitate
		 * the complete definition of these functions.
		 */

        feature :>> elements : Real;
    }

    datatype ThreeVectorValue :> NumericalVectorValue {
        doc /*
		 * A ThreeVectorValue is a NumericalVectorValue that has dimension 3.
		 */

        feature :>> dimension = 3;
    }

    datatype CartesianThreeVectorValue :> CartesianVectorValue, ThreeVectorValue intersects CartesianVectorValue, ThreeVectorValue {
        doc /*
		 * A CartesianThreeVectorValue is a NumericalVectorValue that is both Cartesian and
		 * has dimension 3.
		 */
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "2bbbb529042a81e0a970b62815302faa024b6068ee2cd7463f9cc60e2edde266") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VectorValues"))) (kind "package") (name "VectorValues") (declared-name "VectorValues"))
    (element (id (node (document "d0") (qualified-name "VectorValues::Array"))) (kind "import") (name "Array") (declared-name "Array") (parent (node (document "d0") (qualified-name "VectorValues"))) (authored (membership (kind Import) (visibility "private") (import (reference "Collections::Array") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind "kermlDecl") (name "CartesianThreeVectorValue") (declared-name "CartesianThreeVectorValue") (parent (node (document "d0") (qualified-name "VectorValues"))))
    (element (id (node (document "d0") (qualified-name "VectorValues::CartesianVectorValue"))) (kind "kermlDecl") (name "CartesianVectorValue") (declared-name "CartesianVectorValue") (parent (node (document "d0") (qualified-name "VectorValues"))))
    (element (id (node (document "d0") (qualified-name "VectorValues::NumericalValue"))) (kind "import") (name "NumericalValue") (declared-name "NumericalValue") (parent (node (document "d0") (qualified-name "VectorValues"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::NumericalValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VectorValues::NumericalVectorValue"))) (kind "kermlDecl") (name "NumericalVectorValue") (declared-name "NumericalVectorValue") (parent (node (document "d0") (qualified-name "VectorValues"))))
    (element (id (node (document "d0") (qualified-name "VectorValues::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "VectorValues"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VectorValues::ThreeVectorValue"))) (kind "kermlDecl") (name "ThreeVectorValue") (declared-name "ThreeVectorValue") (parent (node (document "d0") (qualified-name "VectorValues"))))
    (element (id (node (document "d0") (qualified-name "VectorValues::VectorValue"))) (kind "kermlDecl") (name "VectorValue") (declared-name "VectorValue") (parent (node (document "d0") (qualified-name "VectorValues"))))
    (element (id (node (document "d0") (qualified-name "VectorValues::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VectorValues"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VectorValues::Array"))) (kind membershipImport) (ordinal 0)) (authored-target "Collections::Array") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorValues::NumericalValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::NumericalValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorValues::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 8 19) (end 8 37)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "VectorValues::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 8 19) (end 8 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 19) (end 9 37)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "VectorValues::Array"))
        (kind membershipImport) (ordinal 0) (authored-target "Collections::Array")
        (range (start 9 19) (end 9 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 19) (end 7 47)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "VectorValues::NumericalValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::NumericalValue")
        (range (start 7 19) (end 7 47))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
