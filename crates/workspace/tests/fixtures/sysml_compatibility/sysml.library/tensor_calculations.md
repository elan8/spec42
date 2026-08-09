# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/TensorCalculations
type=file
~~~
# SOURCE
~~~sysml
standard library package TensorCalculations {
	doc
	/*
	 * This package package defines calculations for the construction of and computations on TensorQuantityValues.
	 */
	 
    private import ScalarValues::Boolean;
    private import ScalarValues::Number;
    private import Quantities::ScalarQuantityValue;
    private import Quantities::VectorQuantityValue;
    private import Quantities::TensorQuantityValue;
    private import MeasurementReferences::TensorMeasurementReference;
    private import MeasurementReferences::CoordinateTransformation;
    
    calc def '[' specializes BaseFunctions::'[' { 
    	in elements: Number[1..n] ordered; 
    	in mRef: TensorMeasurementReference[1]; 
    	return quantity: TensorQuantityValue[1];
    	private attribute n = mRef.flattenedSize;
    }

    calc def isZeroTensorQuantity { 
    	in x : TensorQuantityValue[1]; 
    	return : Boolean[1];
    }
    calc def isUnitTensorQuantity { 
    	in x : TensorQuantityValue[1]; 
    	return : Boolean[1];
    }

    /* Addition and subtraction */
    calc def '+' :> DataFunctions::'+' { in : TensorQuantityValue[1]; in : TensorQuantityValue[1]; return : TensorQuantityValue[1]; }
    calc def '-' :> DataFunctions::'-' { in : TensorQuantityValue[1]; in : TensorQuantityValue[1]; return : TensorQuantityValue[1]; }

    /* Multiplication and division */
    calc def scalarTensorMult { in : Number[1]; in : TensorQuantityValue[1]; return : TensorQuantityValue[1]; }
    calc def TensorScalarMult { in : TensorQuantityValue[1]; in : Number[1]; return : TensorQuantityValue[1]; }
    calc def scalarQuantityTensorMult { in : ScalarQuantityValue[1]; in : TensorQuantityValue[1]; return : TensorQuantityValue[1]; }
    calc def TensorScalarQuantityMult { in : TensorQuantityValue[1]; in : ScalarQuantityValue[1]; return : TensorQuantityValue[1]; }
    calc def tensorVectorMult { in : TensorQuantityValue[1]; in : VectorQuantityValue[1]; return : VectorQuantityValue[1]; }
    calc def vectorTensorMult { in : VectorQuantityValue[1]; in : TensorQuantityValue[1]; return : VectorQuantityValue[1]; }
    calc def tensorTensorMult { in : TensorQuantityValue[1]; in : TensorQuantityValue[1]; return : TensorQuantityValue[1]; }
    
    /* Tensor transformation */
    calc def transform {
        in transformation : CoordinateTransformation;
        in sourceTensor : TensorQuantityValue;
        return targetTensor : TensorQuantityValue;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'BaseFunctions::['
semantic.unresolved_name 'Number'
semantic.unresolved_name 'TensorMeasurementReference'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::+'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'DataFunctions::-'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'CoordinateTransformation'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'BaseFunctions::['
semantic.unresolved_name 'Number'
semantic.unresolved_name 'TensorMeasurementReference'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::+'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'DataFunctions::-'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'CoordinateTransformation'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'TensorQuantityValue'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Ident,CloseSquare,KwOrdered,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPrivate,KwAttribute,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
RegularComment,
KwCalc,KwDef,UnrestrictedName,ColonGt,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,ColonGt,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
RegularComment,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
RegularComment,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'TensorCalculations'
    (documentation)
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'ScalarValues::Number')
    (import_decl private 'Quantities::ScalarQuantityValue')
    (import_decl private 'Quantities::VectorQuantityValue')
    (import_decl private 'Quantities::TensorQuantityValue')
    (import_decl private 'MeasurementReferences::TensorMeasurementReference')
    (import_decl private 'MeasurementReferences::CoordinateTransformation')
    (calc_def ''['' :> 'BaseFunctions::'[''
      (default_ref_usage in 'elements' : 'Number' multiplicity ordered)
      (default_ref_usage in 'mRef' : 'TensorMeasurementReference' multiplicity)
      (return_member)
      (attribute_usage private 'n' value))
    (calc_def 'isZeroTensorQuantity'
      (default_ref_usage in 'x' : 'TensorQuantityValue' multiplicity)
      (return_member))
    (calc_def 'isUnitTensorQuantity'
      (default_ref_usage in 'x' : 'TensorQuantityValue' multiplicity)
      (return_member))
    (comment)
    (calc_def ''+'' :> 'DataFunctions::'+''
      (default_ref_usage in : 'TensorQuantityValue' multiplicity)
      (default_ref_usage in : 'TensorQuantityValue' multiplicity)
      (return_member))
    (calc_def ''-'' :> 'DataFunctions::'-''
      (default_ref_usage in : 'TensorQuantityValue' multiplicity)
      (default_ref_usage in : 'TensorQuantityValue' multiplicity)
      (return_member))
    (comment)
    (calc_def 'scalarTensorMult'
      (default_ref_usage in : 'Number' multiplicity)
      (default_ref_usage in : 'TensorQuantityValue' multiplicity)
      (return_member))
    (calc_def 'TensorScalarMult'
      (default_ref_usage in : 'TensorQuantityValue' multiplicity)
      (default_ref_usage in : 'Number' multiplicity)
      (return_member))
    (calc_def 'scalarQuantityTensorMult'
      (default_ref_usage in : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in : 'TensorQuantityValue' multiplicity)
      (return_member))
    (calc_def 'TensorScalarQuantityMult'
      (default_ref_usage in : 'TensorQuantityValue' multiplicity)
      (default_ref_usage in : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'tensorVectorMult'
      (default_ref_usage in : 'TensorQuantityValue' multiplicity)
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (return_member))
    (calc_def 'vectorTensorMult'
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (default_ref_usage in : 'TensorQuantityValue' multiplicity)
      (return_member))
    (calc_def 'tensorTensorMult'
      (default_ref_usage in : 'TensorQuantityValue' multiplicity)
      (default_ref_usage in : 'TensorQuantityValue' multiplicity)
      (return_member))
    (comment)
    (calc_def 'transform'
      (default_ref_usage in 'transformation' : 'CoordinateTransformation')
      (default_ref_usage in 'sourceTensor' : 'TensorQuantityValue')
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package TensorCalculations {
    doc /*
	 * This package package defines calculations for the construction of and computations on TensorQuantityValues.
	 */

    private import ScalarValues::Boolean;
    private import ScalarValues::Number;
    private import Quantities::ScalarQuantityValue;
    private import Quantities::VectorQuantityValue;
    private import Quantities::TensorQuantityValue;
    private import MeasurementReferences::TensorMeasurementReference;
    private import MeasurementReferences::CoordinateTransformation;

    calc def '[' specializes BaseFunctions::'[' {
        in elements : Number [1..n] ordered;
        in mRef : TensorMeasurementReference [1];
        return quantity: TensorQuantityValue[1];
        private attribute n = mRef.flattenedSize;
    }

    calc def isZeroTensorQuantity {
        in x : TensorQuantityValue [1];
        return : Boolean[1];
    }
    calc def isUnitTensorQuantity {
        in x : TensorQuantityValue [1];
        return : Boolean[1];
    }

    /* Addition and subtraction */
    calc def '+' :> DataFunctions::'+' {
        in : TensorQuantityValue[1];
        in : TensorQuantityValue[1];
        return : TensorQuantityValue[1];
    }
    calc def '-' :> DataFunctions::'-' {
        in : TensorQuantityValue[1];
        in : TensorQuantityValue[1];
        return : TensorQuantityValue[1];
    }

    /* Multiplication and division */
    calc def scalarTensorMult {
        in : Number[1];
        in : TensorQuantityValue[1];
        return : TensorQuantityValue[1];
    }
    calc def TensorScalarMult {
        in : TensorQuantityValue[1];
        in : Number[1];
        return : TensorQuantityValue[1];
    }
    calc def scalarQuantityTensorMult {
        in : ScalarQuantityValue[1];
        in : TensorQuantityValue[1];
        return : TensorQuantityValue[1];
    }
    calc def TensorScalarQuantityMult {
        in : TensorQuantityValue[1];
        in : ScalarQuantityValue[1];
        return : TensorQuantityValue[1];
    }
    calc def tensorVectorMult {
        in : TensorQuantityValue[1];
        in : VectorQuantityValue[1];
        return : VectorQuantityValue[1];
    }
    calc def vectorTensorMult {
        in : VectorQuantityValue[1];
        in : TensorQuantityValue[1];
        return : VectorQuantityValue[1];
    }
    calc def tensorTensorMult {
        in : TensorQuantityValue[1];
        in : TensorQuantityValue[1];
        return : TensorQuantityValue[1];
    }

    /* Tensor transformation */
    calc def transform {
        in transformation : CoordinateTransformation;
        in sourceTensor : TensorQuantityValue;
        return targetTensor : TensorQuantityValue;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'TensorCalculations'
      (documentation)
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'ScalarValues::Number'[unresolved])
      (membership_import private -> 'Quantities::ScalarQuantityValue'[unresolved])
      (membership_import private -> 'Quantities::VectorQuantityValue'[unresolved])
      (membership_import private -> 'Quantities::TensorQuantityValue'[unresolved])
      (membership_import private -> 'MeasurementReferences::TensorMeasurementReference'[unresolved])
      (membership_import private -> 'MeasurementReferences::CoordinateTransformation'[unresolved])
      (calculation_def '[' :> 'BaseFunctions::['[unresolved]
        (reference_usage in reference ordered 'elements' : 'Number'[unresolved]
          (multiplicity_range [1..?]))
        (reference_usage in reference 'mRef' : 'TensorMeasurementReference'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out 'quantity' : 'TensorQuantityValue'[unresolved]
            (multiplicity_range [1])))
        (attribute_usage composite 'n'
          (feature_value (=))))
      (calculation_def 'isZeroTensorQuantity'
        (reference_usage in reference 'x' : 'TensorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'isUnitTensorQuantity'
        (reference_usage in reference 'x' : 'TensorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '+' :> 'DataFunctions::+'[unresolved]
        (reference_usage in reference : 'TensorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'TensorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'TensorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '-' :> 'DataFunctions::-'[unresolved]
        (reference_usage in reference : 'TensorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'TensorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'TensorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'scalarTensorMult'
        (reference_usage in reference : 'Number'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'TensorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'TensorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'TensorScalarMult'
        (reference_usage in reference : 'TensorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'Number'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'TensorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'scalarQuantityTensorMult'
        (reference_usage in reference : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'TensorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'TensorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'TensorScalarQuantityMult'
        (reference_usage in reference : 'TensorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'TensorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'tensorVectorMult'
        (reference_usage in reference : 'TensorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'VectorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'vectorTensorMult'
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'TensorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'VectorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'tensorTensorMult'
        (reference_usage in reference : 'TensorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'TensorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'TensorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'transform'
        (reference_usage in reference 'transformation' : 'CoordinateTransformation'[unresolved])
        (reference_usage in reference 'sourceTensor' : 'TensorQuantityValue'[unresolved])
        (return_parameter_membership
          (feature_def out 'targetTensor' : 'TensorQuantityValue'[unresolved]))))))
~~~
