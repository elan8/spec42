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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "VectorCalculations"))) (name "VectorCalculations") (declared-name "VectorCalculations")
      (contains
        (element (kind "alias") (id (node (document "d0") (qualified-name "VectorCalculations::*"))) (name "*") (declared-name "*"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::+"))) (name "+") (declared-name "+")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::+::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::+")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::+::VectorQuantityValue#in_out_parameter"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::+")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::-"))) (name "-") (declared-name "-")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::-::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::-")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::-::VectorQuantityValue#in_out_parameter"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::-")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "VectorCalculations::Boolean"))) (name "Boolean") (declared-name "Boolean"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VectorCalculations::CoordinateTransformation"))) (name "CoordinateTransformation") (declared-name "CoordinateTransformation"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VectorCalculations::Number"))) (name "Number") (declared-name "Number"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VectorCalculations::ScalarQuantityValue"))) (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VectorCalculations::VectorMeasurementReference"))) (name "VectorMeasurementReference") (declared-name "VectorMeasurementReference"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VectorCalculations::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::["))) (name "[") (declared-name "[")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::[::elements"))) (name "elements") (declared-name "elements") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::[")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::[::mRef"))) (name "mRef") (declared-name "mRef") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::[")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "VectorCalculations::_documentation"))) (name ""))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::angle"))) (name "angle") (declared-name "angle")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::angle::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::angle")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::angle::VectorQuantityValue#in_out_parameter"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::angle")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::inner"))) (name "inner") (declared-name "inner")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::inner::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::inner")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::inner::VectorQuantityValue#in_out_parameter"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::inner")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::isUnitVectorQuantity"))) (name "isUnitVectorQuantity") (declared-name "isUnitVectorQuantity")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::isUnitVectorQuantity::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::isUnitVectorQuantity")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::isZeroVectorQuantity"))) (name "isZeroVectorQuantity") (declared-name "isZeroVectorQuantity")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::isZeroVectorQuantity::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::isZeroVectorQuantity")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::norm"))) (name "norm") (declared-name "norm")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::norm::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::norm")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::outer"))) (name "outer") (declared-name "outer")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::outer::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::outer")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::outer::VectorQuantityValue#in_out_parameter"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::outer")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::scalarQuantityVectorMult"))) (name "scalarQuantityVectorMult") (declared-name "scalarQuantityVectorMult")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::scalarQuantityVectorMult::ScalarQuantityValue"))) (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::scalarQuantityVectorMult")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::scalarQuantityVectorMult::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::scalarQuantityVectorMult")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::scalarVectorMult"))) (name "scalarVectorMult") (declared-name "scalarVectorMult")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::scalarVectorMult::Number"))) (name "Number") (declared-name "Number") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::scalarVectorMult")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::scalarVectorMult::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::scalarVectorMult")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::transform"))) (name "transform") (declared-name "transform")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::transform::sourceVector"))) (name "sourceVector") (declared-name "sourceVector") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::transform")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::transform::transformation"))) (name "transformation") (declared-name "transformation") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::transform")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarDiv"))) (name "vectorScalarDiv") (declared-name "vectorScalarDiv")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarDiv::Number"))) (name "Number") (declared-name "Number") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::vectorScalarDiv")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarDiv::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::vectorScalarDiv")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarMult"))) (name "vectorScalarMult") (declared-name "vectorScalarMult")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarMult::Number"))) (name "Number") (declared-name "Number") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::vectorScalarMult")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarMult::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::vectorScalarMult")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityDiv"))) (name "vectorScalarQuantityDiv") (declared-name "vectorScalarQuantityDiv")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityDiv::ScalarQuantityValue"))) (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityDiv")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityDiv::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityDiv")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityMult"))) (name "vectorScalarQuantityMult") (declared-name "vectorScalarQuantityMult")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityMult::ScalarQuantityValue"))) (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityMult")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityMult::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityMult")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VectorCalculations::_documentation"))) (to (node (document "d0") (qualified-name "VectorCalculations"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::+"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::-"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::["))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::angle"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::inner"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::isUnitVectorQuantity"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::isZeroVectorQuantity"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::norm"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::outer"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::scalarQuantityVectorMult"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::scalarVectorMult"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::transform"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::vectorScalarDiv"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::vectorScalarMult"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityDiv"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityMult"))) (status missing-prerequisite) (target "Calculations::Calculation"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/vector_calculations.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 19) (end 10 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 19) (end 11 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 5) (end 14 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 5) (end 15 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 5) (end 21 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 5) (end 25 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 40) (end 30 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 69) (end 30 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 40) (end 31 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 69) (end 31 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 66) (end 34 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 82) (end 34 110))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 66) (end 35 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 95) (end 35 110))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 40) (end 36 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 69) (end 36 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 40) (end 37 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 69) (end 37 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 64) (end 38 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 93) (end 38 108))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 39) (end 39 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 68) (end 39 96))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 44) (end 40 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 73) (end 40 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 21) (end 41 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 50) (end 41 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 46 42) (end 46 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 44) (end 47 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 73) (end 47 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 5) (end 51 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 5) (end 52 94))
      )
    )
  )
)
~~~
