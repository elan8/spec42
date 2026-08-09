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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "TensorCalculations"))) (name "TensorCalculations") (declared-name "TensorCalculations")
      (contains
        (element (kind "calc def") (id (node (document "d0") (qualified-name "TensorCalculations::+"))) (name "+") (declared-name "+")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::+::TensorQuantityValue"))) (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::+")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::+::TensorQuantityValue#in_out_parameter"))) (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::+")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "TensorCalculations::-"))) (name "-") (declared-name "-")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::-::TensorQuantityValue"))) (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::-")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::-::TensorQuantityValue#in_out_parameter"))) (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::-")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "TensorCalculations::Boolean"))) (name "Boolean") (declared-name "Boolean"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TensorCalculations::CoordinateTransformation"))) (name "CoordinateTransformation") (declared-name "CoordinateTransformation"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TensorCalculations::Number"))) (name "Number") (declared-name "Number"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TensorCalculations::ScalarQuantityValue"))) (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TensorCalculations::TensorMeasurementReference"))) (name "TensorMeasurementReference") (declared-name "TensorMeasurementReference"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TensorCalculations::TensorQuantityValue"))) (name "TensorQuantityValue") (declared-name "TensorQuantityValue"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult"))) (name "TensorScalarMult") (declared-name "TensorScalarMult")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult::Number"))) (name "Number") (declared-name "Number") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult::TensorQuantityValue"))) (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult"))) (name "TensorScalarQuantityMult") (declared-name "TensorScalarQuantityMult")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult::ScalarQuantityValue"))) (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult::TensorQuantityValue"))) (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "TensorCalculations::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "TensorCalculations::["))) (name "[") (declared-name "[")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::[::elements"))) (name "elements") (declared-name "elements") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::[")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::[::mRef"))) (name "mRef") (declared-name "mRef") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::[")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "TensorCalculations::_documentation"))) (name ""))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "TensorCalculations::isUnitTensorQuantity"))) (name "isUnitTensorQuantity") (declared-name "isUnitTensorQuantity")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::isUnitTensorQuantity::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::isUnitTensorQuantity")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "TensorCalculations::isZeroTensorQuantity"))) (name "isZeroTensorQuantity") (declared-name "isZeroTensorQuantity")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::isZeroTensorQuantity::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::isZeroTensorQuantity")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult"))) (name "scalarQuantityTensorMult") (declared-name "scalarQuantityTensorMult")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult::ScalarQuantityValue"))) (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult::TensorQuantityValue"))) (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult"))) (name "scalarTensorMult") (declared-name "scalarTensorMult")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult::Number"))) (name "Number") (declared-name "Number") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult::TensorQuantityValue"))) (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult"))) (name "tensorTensorMult") (declared-name "tensorTensorMult")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult::TensorQuantityValue"))) (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult::TensorQuantityValue#in_out_parameter"))) (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult"))) (name "tensorVectorMult") (declared-name "tensorVectorMult")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult::TensorQuantityValue"))) (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "TensorCalculations::transform"))) (name "transform") (declared-name "transform")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::transform::sourceTensor"))) (name "sourceTensor") (declared-name "sourceTensor") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::transform")))))
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "TensorCalculations::transform::targetTensor"))) (name "targetTensor") (declared-name "targetTensor") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::transform")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::transform::transformation"))) (name "transformation") (declared-name "transformation") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::transform")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult"))) (name "vectorTensorMult") (declared-name "vectorTensorMult")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult::TensorQuantityValue"))) (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (effective (featuring-type (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TensorCalculations::_documentation"))) (to (node (document "d0") (qualified-name "TensorCalculations"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "TensorCalculations::+"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "TensorCalculations::-"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "TensorCalculations::["))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "TensorCalculations::isUnitTensorQuantity"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "TensorCalculations::isZeroTensorQuantity"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "TensorCalculations::transform"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult"))) (status missing-prerequisite) (target "Calculations::Calculation"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/tensor_calculations.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 19) (end 6 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 39))
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
        (range (start 10 19) (end 10 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 19) (end 11 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 19) (end 12 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 5) (end 15 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 5) (end 16 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 5) (end 22 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 5) (end 26 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 41) (end 31 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 70) (end 31 98))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 41) (end 32 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 70) (end 32 98))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 32) (end 35 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 48) (end 35 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 32) (end 36 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 61) (end 36 76))
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
        (range (start 38 40) (end 38 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 69) (end 38 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 32) (end 39 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 61) (end 39 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 32) (end 40 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 61) (end 40 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 32) (end 41 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 61) (end 41 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 45 8) (end 45 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 46 8) (end 46 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 8) (end 47 50))
      )
    )
  )
)
~~~
