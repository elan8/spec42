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
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwDatatype,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwDatatype,Ident,ColonGt,Ident,Comma,Ident,KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,RegularComment,
KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,ColonGtGt,Ident,Semicolon,
KwFeature,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwDatatype,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
KwFeature,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwDatatype,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
KwFeature,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwDatatype,Ident,ColonGt,Ident,Comma,Ident,KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'VectorValues'
    (documentation)
    (import_decl private 'ScalarValues::NumericalValue')
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Collections::Array')
    (datatype_def abstract 'VectorValue'
      (documentation))
    (datatype_def 'NumericalVectorValue' :> 'VectorValue', 'Array' intersects 'VectorValue', 'Array'
      (documentation)
      (feature_def 'dimension' multiplicity :>> 'dimensions')
      (feature_def :>> 'elements' : 'NumericalValue'))
    (datatype_def 'CartesianVectorValue' :> 'NumericalVectorValue'
      (documentation)
      (feature_def :>> 'elements' : 'Real'))
    (datatype_def 'ThreeVectorValue' :> 'NumericalVectorValue'
      (documentation)
      (feature_def :>> 'dimension' value))
    (datatype_def 'CartesianThreeVectorValue' :> 'CartesianVectorValue', 'ThreeVectorValue' intersects 'CartesianVectorValue', 'ThreeVectorValue'
      (documentation))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Array'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Array'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
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
    (element (id (node (document "d0") (qualified-name "VectorValues"))) (kind "package") (name "VectorValues") (declared-name "VectorValues") (range (start (line 0) (character 0)) (end (line 0) (character 1962))))
    (element (id (node (document "d0") (qualified-name "VectorValues::Array"))) (kind "import") (name "Array") (declared-name "Array") (range (start (line 9) (character 4)) (end (line 9) (character 38))) (parent (node (document "d0") (qualified-name "VectorValues"))) (authored (membership (kind Import) (visibility "private") (import (reference "Collections::Array") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 19)) (end (line 9) (character 37))))))
    (element (id (node (document "d0") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind "kermlDecl") (name "CartesianThreeVectorValue") (declared-name "CartesianThreeVectorValue") (range (start (line 50) (character 4)) (end (line 50) (character 267))) (parent (node (document "d0") (qualified-name "VectorValues"))))
    (element (id (node (document "d0") (qualified-name "VectorValues::CartesianVectorValue"))) (kind "kermlDecl") (name "CartesianVectorValue") (declared-name "CartesianVectorValue") (range (start (line 30) (character 4)) (end (line 30) (character 429))) (parent (node (document "d0") (qualified-name "VectorValues"))))
    (element (id (node (document "d0") (qualified-name "VectorValues::NumericalValue"))) (kind "import") (name "NumericalValue") (declared-name "NumericalValue") (range (start (line 7) (character 4)) (end (line 7) (character 48))) (parent (node (document "d0") (qualified-name "VectorValues"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::NumericalValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 19)) (end (line 7) (character 47))))))
    (element (id (node (document "d0") (qualified-name "VectorValues::NumericalVectorValue"))) (kind "kermlDecl") (name "NumericalVectorValue") (declared-name "NumericalVectorValue") (range (start (line 18) (character 4)) (end (line 18) (character 501))) (parent (node (document "d0") (qualified-name "VectorValues"))))
    (element (id (node (document "d0") (qualified-name "VectorValues::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 8) (character 4)) (end (line 8) (character 38))) (parent (node (document "d0") (qualified-name "VectorValues"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 19)) (end (line 8) (character 37))))))
    (element (id (node (document "d0") (qualified-name "VectorValues::ThreeVectorValue"))) (kind "kermlDecl") (name "ThreeVectorValue") (declared-name "ThreeVectorValue") (range (start (line 42) (character 4)) (end (line 42) (character 190))) (parent (node (document "d0") (qualified-name "VectorValues"))))
    (element (id (node (document "d0") (qualified-name "VectorValues::VectorValue"))) (kind "kermlDecl") (name "VectorValue") (declared-name "VectorValue") (range (start (line 11) (character 4)) (end (line 11) (character 166))) (parent (node (document "d0") (qualified-name "VectorValues"))))
    (element (id (node (document "d0") (qualified-name "VectorValues::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1962))) (parent (node (document "d0") (qualified-name "VectorValues"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VectorValues::Array"))) (kind membershipImport) (ordinal 0)) (authored-target "Collections::Array") (range (start (line 9) (character 19)) (end (line 9) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorValues::NumericalValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::NumericalValue") (range (start (line 7) (character 19)) (end (line 7) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorValues::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 8) (character 19)) (end (line 8) (character 37))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
