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
(model
  (namespace
    (library_package 'VectorValues'
      (documentation)
      (membership_import private -> 'ScalarValues::NumericalValue'[unresolved])
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (membership_import private -> 'Collections::Array'[unresolved])
      (datatype_def abstract 'VectorValue'
        (documentation))
      (datatype_def 'NumericalVectorValue' :> 'VectorValues::VectorValue'[datatype_def] :> 'Array'[unresolved]
        (intersecting)
        (intersecting)
        (documentation)
        (feature_def 'dimension' :>> 'dimensions'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def :>> 'elements'[unresolved] : 'NumericalValue'[unresolved]))
      (datatype_def 'CartesianVectorValue' :> 'VectorValues::NumericalVectorValue'[datatype_def]
        (documentation)
        (feature_def :>> 'elements'[unresolved] : 'Real'[unresolved]))
      (datatype_def 'ThreeVectorValue' :> 'VectorValues::NumericalVectorValue'[datatype_def]
        (documentation)
        (feature_def :>> 'VectorValues::NumericalVectorValue::dimension'[feature_def]
          (feature_value (=))))
      (datatype_def 'CartesianThreeVectorValue' :> 'VectorValues::CartesianVectorValue'[datatype_def] :> 'VectorValues::ThreeVectorValue'[datatype_def]
        (intersecting)
        (intersecting)
        (documentation)))))
~~~
