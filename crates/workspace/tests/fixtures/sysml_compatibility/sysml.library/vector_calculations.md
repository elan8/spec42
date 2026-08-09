# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/VectorCalculations
type=file
~~~
# SOURCE
~~~sysml
standard library package VectorCalculations {
	doc
	/*
	 * This package package defines calculations for the construction of and computations on VectorQuantityValues.
	 */
	 
	private import ScalarValues::Boolean;
	private import ScalarValues::Number;
    private import Quantities::ScalarQuantityValue;
    private import Quantities::VectorQuantityValue;
    private import MeasurementReferences::VectorMeasurementReference;
    private import MeasurementReferences::CoordinateTransformation;
    
    calc def '[' :> BaseFunctions::'[' { 
    	in elements: Number[1..n] ordered; 
    	in mRef: VectorMeasurementReference[1]; 
    	return quantity : VectorQuantityValue[1];
    	private attribute n = mRef.flattenedSize;
    }

    calc def isZeroVectorQuantity :> VectorFunctions::isZeroVector { 
    	in : VectorQuantityValue[1]; 
    	return : Boolean[1];
    }
    calc def isUnitVectorQuantity { 
    	in : VectorQuantityValue[1]; 
    	return : Boolean[1];
    }

    /* Addition and subtraction */
	calc def '+' :> VectorFunctions::'+' { in : VectorQuantityValue[1]; in : VectorQuantityValue[1]; return : VectorQuantityValue[1]; }
	calc def '-' :> VectorFunctions::'-' { in : VectorQuantityValue[1]; in : VectorQuantityValue[1]; return : VectorQuantityValue[1]; }

    /* Multiplication and division */
	calc def scalarVectorMult :> VectorFunctions::scalarVectorMult { in : Number[1]; in : VectorQuantityValue[1]; return : VectorQuantityValue[1]; }
	calc def vectorScalarMult :> VectorFunctions::vectorScalarMult { in : VectorQuantityValue[1]; in : Number[1]; return : VectorQuantityValue[1]; }
    calc def scalarQuantityVectorMult { in : ScalarQuantityValue[1]; in : VectorQuantityValue[1]; return : VectorQuantityValue[1]; }
    calc def vectorScalarQuantityMult { in : VectorQuantityValue[1]; in : ScalarQuantityValue[1]; return : VectorQuantityValue[1]; }
	calc def vectorScalarDiv :> VectorFunctions::vectorScalarDiv { in : VectorQuantityValue[1]; in : Number[1]; return : VectorQuantityValue[1]; }
    calc def vectorScalarQuantityDiv { in : VectorQuantityValue[1]; in : ScalarQuantityValue[1]; return : VectorQuantityValue[1]; }
	calc def inner :> VectorFunctions::inner { in : VectorQuantityValue[1]; in : VectorQuantityValue[1]; return : Number[1]; }
    calc def outer { in : VectorQuantityValue[1]; in : VectorQuantityValue[1]; return : VectorQuantityValue[1]; }
	
    alias '*' for scalarVectorMult;
    
    /* Norm and angle */
	calc def norm :> VectorFunctions::norm { in : VectorQuantityValue[1]; return : Number[1]; }
	calc def angle :> VectorFunctions::angle { in : VectorQuantityValue[1]; in : VectorQuantityValue[1]; return : Number[1]; }
	
	/* Coordinate transformation */
	calc def transform {
	    in transformation : CoordinateTransformation;
	    in sourceVector : VectorQuantityValue {
	        :>> mRef = transformation.source;
	    }
	    return targetVector : VectorQuantityValue {
	        :>> mRef = transformation.target {
	            :>> dimensions = sourceVector.mRef.dimensions;
	        }
    	}
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'BaseFunctions::['
semantic.unresolved_name 'Number'
semantic.unresolved_name 'VectorMeasurementReference'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorFunctions::isZeroVector'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'VectorFunctions::+'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorFunctions::-'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorFunctions::scalarVectorMult'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorFunctions::vectorScalarMult'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorFunctions::vectorScalarDiv'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorFunctions::inner'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorFunctions::norm'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'VectorFunctions::angle'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'CoordinateTransformation'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'dimensions'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'BaseFunctions::['
semantic.unresolved_name 'Number'
semantic.unresolved_name 'VectorMeasurementReference'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorFunctions::isZeroVector'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'VectorFunctions::+'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorFunctions::-'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorFunctions::scalarVectorMult'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorFunctions::vectorScalarMult'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorFunctions::vectorScalarDiv'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorFunctions::inner'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorFunctions::norm'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'VectorFunctions::angle'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'CoordinateTransformation'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'dimensions'
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
KwCalc,KwDef,UnrestrictedName,ColonGt,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Ident,CloseSquare,KwOrdered,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPrivate,KwAttribute,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
RegularComment,
KwCalc,KwDef,UnrestrictedName,ColonGt,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,ColonGt,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
RegularComment,
KwCalc,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAlias,UnrestrictedName,KwFor,Ident,Semicolon,
RegularComment,
KwCalc,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
RegularComment,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwReturn,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Dot,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'VectorCalculations'
    (documentation)
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'ScalarValues::Number')
    (import_decl private 'Quantities::ScalarQuantityValue')
    (import_decl private 'Quantities::VectorQuantityValue')
    (import_decl private 'MeasurementReferences::VectorMeasurementReference')
    (import_decl private 'MeasurementReferences::CoordinateTransformation')
    (calc_def ''['' :> 'BaseFunctions::'[''
      (default_ref_usage in 'elements' : 'Number' multiplicity ordered)
      (default_ref_usage in 'mRef' : 'VectorMeasurementReference' multiplicity)
      (return_member)
      (attribute_usage private 'n' value))
    (calc_def 'isZeroVectorQuantity' :> 'VectorFunctions::isZeroVector'
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (return_member))
    (calc_def 'isUnitVectorQuantity'
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (return_member))
    (comment)
    (calc_def ''+'' :> 'VectorFunctions::'+''
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (return_member))
    (calc_def ''-'' :> 'VectorFunctions::'-''
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (return_member))
    (comment)
    (calc_def 'scalarVectorMult' :> 'VectorFunctions::scalarVectorMult'
      (default_ref_usage in : 'Number' multiplicity)
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (return_member))
    (calc_def 'vectorScalarMult' :> 'VectorFunctions::vectorScalarMult'
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (default_ref_usage in : 'Number' multiplicity)
      (return_member))
    (calc_def 'scalarQuantityVectorMult'
      (default_ref_usage in : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (return_member))
    (calc_def 'vectorScalarQuantityMult'
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (default_ref_usage in : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'vectorScalarDiv' :> 'VectorFunctions::vectorScalarDiv'
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (default_ref_usage in : 'Number' multiplicity)
      (return_member))
    (calc_def 'vectorScalarQuantityDiv'
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (default_ref_usage in : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'inner' :> 'VectorFunctions::inner'
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (return_member))
    (calc_def 'outer'
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (return_member))
    (alias_member ''*'' for 'scalarVectorMult')
    (comment)
    (calc_def 'norm' :> 'VectorFunctions::norm'
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (return_member))
    (calc_def 'angle' :> 'VectorFunctions::angle'
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (default_ref_usage in : 'VectorQuantityValue' multiplicity)
      (return_member))
    (comment)
    (calc_def 'transform'
      (default_ref_usage in 'transformation' : 'CoordinateTransformation')
      (default_ref_usage in 'sourceVector' : 'VectorQuantityValue'
        (default_ref_usage :>> 'mRef' value))
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package VectorCalculations {
    doc /*
	 * This package package defines calculations for the construction of and computations on VectorQuantityValues.
	 */

    private import ScalarValues::Boolean;
    private import ScalarValues::Number;
    private import Quantities::ScalarQuantityValue;
    private import Quantities::VectorQuantityValue;
    private import MeasurementReferences::VectorMeasurementReference;
    private import MeasurementReferences::CoordinateTransformation;

    calc def '[' :> BaseFunctions::'[' {
        in elements : Number [1..n] ordered;
        in mRef : VectorMeasurementReference [1];
        return quantity : VectorQuantityValue[1];
        private attribute n = mRef.flattenedSize;
    }

    calc def isZeroVectorQuantity :> VectorFunctions::isZeroVector {
        in : VectorQuantityValue[1];
        return : Boolean[1];
    }
    calc def isUnitVectorQuantity {
        in : VectorQuantityValue[1];
        return : Boolean[1];
    }

    /* Addition and subtraction */
    calc def '+' :> VectorFunctions::'+' {
        in : VectorQuantityValue[1];
        in : VectorQuantityValue[1];
        return : VectorQuantityValue[1];
    }
    calc def '-' :> VectorFunctions::'-' {
        in : VectorQuantityValue[1];
        in : VectorQuantityValue[1];
        return : VectorQuantityValue[1];
    }

    /* Multiplication and division */
    calc def scalarVectorMult :> VectorFunctions::scalarVectorMult {
        in : Number[1];
        in : VectorQuantityValue[1];
        return : VectorQuantityValue[1];
    }
    calc def vectorScalarMult :> VectorFunctions::vectorScalarMult {
        in : VectorQuantityValue[1];
        in : Number[1];
        return : VectorQuantityValue[1];
    }
    calc def scalarQuantityVectorMult {
        in : ScalarQuantityValue[1];
        in : VectorQuantityValue[1];
        return : VectorQuantityValue[1];
    }
    calc def vectorScalarQuantityMult {
        in : VectorQuantityValue[1];
        in : ScalarQuantityValue[1];
        return : VectorQuantityValue[1];
    }
    calc def vectorScalarDiv :> VectorFunctions::vectorScalarDiv {
        in : VectorQuantityValue[1];
        in : Number[1];
        return : VectorQuantityValue[1];
    }
    calc def vectorScalarQuantityDiv {
        in : VectorQuantityValue[1];
        in : ScalarQuantityValue[1];
        return : VectorQuantityValue[1];
    }
    calc def inner :> VectorFunctions::inner {
        in : VectorQuantityValue[1];
        in : VectorQuantityValue[1];
        return : Number[1];
    }
    calc def outer {
        in : VectorQuantityValue[1];
        in : VectorQuantityValue[1];
        return : VectorQuantityValue[1];
    }

    alias '*' for scalarVectorMult;

    /* Norm and angle */
    calc def norm :> VectorFunctions::norm {
        in : VectorQuantityValue[1];
        return : Number[1];
    }
    calc def angle :> VectorFunctions::angle {
        in : VectorQuantityValue[1];
        in : VectorQuantityValue[1];
        return : Number[1];
    }

    /* Coordinate transformation */
    calc def transform {
        in transformation : CoordinateTransformation;
        in sourceVector : VectorQuantityValue {
            :>> mRef = transformation.source;
        }
        return targetVector : VectorQuantityValue {
	        :>> mRef = transformation.target {
	            :>> dimensions = sourceVector.mRef.dimensions;
	        }
    	}
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'VectorCalculations'
      (documentation)
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'ScalarValues::Number'[unresolved])
      (membership_import private -> 'Quantities::ScalarQuantityValue'[unresolved])
      (membership_import private -> 'Quantities::VectorQuantityValue'[unresolved])
      (membership_import private -> 'MeasurementReferences::VectorMeasurementReference'[unresolved])
      (membership_import private -> 'MeasurementReferences::CoordinateTransformation'[unresolved])
      (calculation_def '[' :> 'BaseFunctions::['[unresolved]
        (reference_usage in reference ordered 'elements' : 'Number'[unresolved]
          (multiplicity_range [1..?]))
        (reference_usage in reference 'mRef' : 'VectorMeasurementReference'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out 'quantity' : 'VectorQuantityValue'[unresolved]
            (multiplicity_range [1])))
        (attribute_usage composite 'n'
          (feature_value (=))))
      (calculation_def 'isZeroVectorQuantity' :> 'VectorFunctions::isZeroVector'[unresolved]
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'isUnitVectorQuantity'
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '+' :> 'VectorFunctions::+'[unresolved]
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'VectorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '-' :> 'VectorFunctions::-'[unresolved]
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'VectorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'scalarVectorMult' :> 'VectorFunctions::scalarVectorMult'[unresolved]
        (reference_usage in reference : 'Number'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'VectorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'vectorScalarMult' :> 'VectorFunctions::vectorScalarMult'[unresolved]
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'Number'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'VectorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'scalarQuantityVectorMult'
        (reference_usage in reference : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'VectorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'vectorScalarQuantityMult'
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'VectorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'vectorScalarDiv' :> 'VectorFunctions::vectorScalarDiv'[unresolved]
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'Number'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'VectorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'vectorScalarQuantityDiv'
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'VectorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'inner' :> 'VectorFunctions::inner'[unresolved]
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Number'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'outer'
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'VectorQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (alias_member '*' -> 'VectorCalculations::scalarVectorMult'[calculation_def])
      (calculation_def 'norm' :> 'VectorFunctions::norm'[unresolved]
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Number'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'angle' :> 'VectorFunctions::angle'[unresolved]
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Number'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'transform'
        (reference_usage in reference 'transformation' : 'CoordinateTransformation'[unresolved])
        (reference_usage in reference 'sourceVector' : 'VectorQuantityValue'[unresolved]
          (reference_usage reference :>> 'mRef'[unresolved]
            (feature_value (=))))
        (return_parameter_membership
          (feature_def out 'targetVector' : 'VectorQuantityValue'[unresolved]
            (feature_def :>> 'mRef'[unresolved]
              (feature_value (=))
              (feature_def :>> 'dimensions'[unresolved]
                (feature_value (=))))))))))
~~~
