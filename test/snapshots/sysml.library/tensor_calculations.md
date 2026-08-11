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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "tensor_calculations.md"
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
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "baf0b26e2b22fb74d8ebe0339f999d6a125e5b802529bcef81faa261e70da686") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "TensorCalculations"))) (kind "package") (name "TensorCalculations") (declared-name "TensorCalculations") (range (start (line 0) (character 0)) (end (line 0) (character 2436))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::+"))) (kind "calc def") (name "+") (declared-name "+") (range (start (line 31) (character 4)) (end (line 31) (character 133))) (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::+::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (range (start (line 31) (character 41)) (end (line 31) (character 69))) (parent (node (document "d0") (qualified-name "TensorCalculations::+"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::+::TensorQuantityValue#in_out_parameter"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (range (start (line 31) (character 70)) (end (line 31) (character 98))) (parent (node (document "d0") (qualified-name "TensorCalculations::+"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::-"))) (kind "calc def") (name "-") (declared-name "-") (range (start (line 32) (character 4)) (end (line 32) (character 133))) (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::-::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (range (start (line 32) (character 41)) (end (line 32) (character 69))) (parent (node (document "d0") (qualified-name "TensorCalculations::-"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::-::TensorQuantityValue#in_out_parameter"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (range (start (line 32) (character 70)) (end (line 32) (character 98))) (parent (node (document "d0") (qualified-name "TensorCalculations::-"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 6) (character 4)) (end (line 6) (character 41))) (parent (node (document "d0") (qualified-name "TensorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 19)) (end (line 6) (character 40))))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::CoordinateTransformation"))) (kind "import") (name "CoordinateTransformation") (declared-name "CoordinateTransformation") (range (start (line 12) (character 4)) (end (line 12) (character 67))) (parent (node (document "d0") (qualified-name "TensorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::CoordinateTransformation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 19)) (end (line 12) (character 66))))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::Number"))) (kind "import") (name "Number") (declared-name "Number") (range (start (line 7) (character 4)) (end (line 7) (character 40))) (parent (node (document "d0") (qualified-name "TensorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Number") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 19)) (end (line 7) (character 39))))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::ScalarQuantityValue"))) (kind "import") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (range (start (line 8) (character 4)) (end (line 8) (character 51))) (parent (node (document "d0") (qualified-name "TensorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::ScalarQuantityValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 19)) (end (line 8) (character 50))))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorMeasurementReference"))) (kind "import") (name "TensorMeasurementReference") (declared-name "TensorMeasurementReference") (range (start (line 11) (character 4)) (end (line 11) (character 69))) (parent (node (document "d0") (qualified-name "TensorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::TensorMeasurementReference") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 19)) (end (line 11) (character 68))))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorQuantityValue"))) (kind "import") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (range (start (line 10) (character 4)) (end (line 10) (character 51))) (parent (node (document "d0") (qualified-name "TensorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::TensorQuantityValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 19)) (end (line 10) (character 50))))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult"))) (kind "calc def") (name "TensorScalarMult") (declared-name "TensorScalarMult") (range (start (line 36) (character 4)) (end (line 36) (character 111))) (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult::Number"))) (kind "in out parameter") (name "Number") (declared-name "Number") (range (start (line 36) (character 61)) (end (line 36) (character 76))) (parent (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult"))) (authored (relationships (typing (reference ": Number[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (range (start (line 36) (character 32)) (end (line 36) (character 60))) (parent (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult"))) (kind "calc def") (name "TensorScalarQuantityMult") (declared-name "TensorScalarQuantityMult") (range (start (line 38) (character 4)) (end (line 38) (character 132))) (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult::ScalarQuantityValue"))) (kind "in out parameter") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (range (start (line 38) (character 69)) (end (line 38) (character 97))) (parent (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult"))) (authored (relationships (typing (reference ": ScalarQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (range (start (line 38) (character 40)) (end (line 38) (character 68))) (parent (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::VectorQuantityValue"))) (kind "import") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 9) (character 4)) (end (line 9) (character 51))) (parent (node (document "d0") (qualified-name "TensorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::VectorQuantityValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 19)) (end (line 9) (character 50))))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::["))) (kind "calc def") (name "[") (declared-name "[") (range (start (line 14) (character 4)) (end (line 14) (character 236))) (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::[::elements"))) (kind "in out parameter") (name "elements") (declared-name "elements") (range (start (line 15) (character 5)) (end (line 15) (character 39))) (parent (node (document "d0") (qualified-name "TensorCalculations::["))) (authored (relationships (typing (reference "elements: Number[1..n] ordered") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::[::mRef"))) (kind "in out parameter") (name "mRef") (declared-name "mRef") (range (start (line 16) (character 5)) (end (line 16) (character 44))) (parent (node (document "d0") (qualified-name "TensorCalculations::["))) (authored (relationships (typing (reference "mRef: TensorMeasurementReference[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2436))) (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::isUnitTensorQuantity"))) (kind "calc def") (name "isUnitTensorQuantity") (declared-name "isUnitTensorQuantity") (range (start (line 25) (character 4)) (end (line 25) (character 105))) (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::isUnitTensorQuantity::x"))) (kind "in out parameter") (name "x") (declared-name "x") (range (start (line 26) (character 5)) (end (line 26) (character 35))) (parent (node (document "d0") (qualified-name "TensorCalculations::isUnitTensorQuantity"))) (authored (relationships (typing (reference "x : TensorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::isZeroTensorQuantity"))) (kind "calc def") (name "isZeroTensorQuantity") (declared-name "isZeroTensorQuantity") (range (start (line 21) (character 4)) (end (line 21) (character 105))) (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::isZeroTensorQuantity::x"))) (kind "in out parameter") (name "x") (declared-name "x") (range (start (line 22) (character 5)) (end (line 22) (character 35))) (parent (node (document "d0") (qualified-name "TensorCalculations::isZeroTensorQuantity"))) (authored (relationships (typing (reference "x : TensorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult"))) (kind "calc def") (name "scalarQuantityTensorMult") (declared-name "scalarQuantityTensorMult") (range (start (line 37) (character 4)) (end (line 37) (character 132))) (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult::ScalarQuantityValue"))) (kind "in out parameter") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (range (start (line 37) (character 40)) (end (line 37) (character 68))) (parent (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult"))) (authored (relationships (typing (reference ": ScalarQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (range (start (line 37) (character 69)) (end (line 37) (character 97))) (parent (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult"))) (kind "calc def") (name "scalarTensorMult") (declared-name "scalarTensorMult") (range (start (line 35) (character 4)) (end (line 35) (character 111))) (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult::Number"))) (kind "in out parameter") (name "Number") (declared-name "Number") (range (start (line 35) (character 32)) (end (line 35) (character 47))) (parent (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult"))) (authored (relationships (typing (reference ": Number[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (range (start (line 35) (character 48)) (end (line 35) (character 76))) (parent (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult"))) (kind "calc def") (name "tensorTensorMult") (declared-name "tensorTensorMult") (range (start (line 41) (character 4)) (end (line 41) (character 124))) (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (range (start (line 41) (character 32)) (end (line 41) (character 60))) (parent (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult::TensorQuantityValue#in_out_parameter"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (range (start (line 41) (character 61)) (end (line 41) (character 89))) (parent (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult"))) (kind "calc def") (name "tensorVectorMult") (declared-name "tensorVectorMult") (range (start (line 39) (character 4)) (end (line 39) (character 124))) (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (range (start (line 39) (character 32)) (end (line 39) (character 60))) (parent (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 39) (character 61)) (end (line 39) (character 89))) (parent (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::transform"))) (kind "calc def") (name "transform") (declared-name "transform") (range (start (line 44) (character 4)) (end (line 44) (character 182))) (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::transform::sourceTensor"))) (kind "in out parameter") (name "sourceTensor") (declared-name "sourceTensor") (range (start (line 46) (character 8)) (end (line 46) (character 46))) (parent (node (document "d0") (qualified-name "TensorCalculations::transform"))) (authored (relationships (typing (reference "TensorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::transform::targetTensor"))) (kind "return parameter") (name "targetTensor") (declared-name "targetTensor") (range (start (line 47) (character 8)) (end (line 47) (character 50))) (parent (node (document "d0") (qualified-name "TensorCalculations::transform"))) (authored (relationships (typing (reference "TensorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::transform::transformation"))) (kind "in out parameter") (name "transformation") (declared-name "transformation") (range (start (line 45) (character 8)) (end (line 45) (character 53))) (parent (node (document "d0") (qualified-name "TensorCalculations::transform"))) (authored (relationships (typing (reference "CoordinateTransformation") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult"))) (kind "calc def") (name "vectorTensorMult") (declared-name "vectorTensorMult") (range (start (line 40) (character 4)) (end (line 40) (character 124))) (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (range (start (line 40) (character 61)) (end (line 40) (character 89))) (parent (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 40) (character 32)) (end (line 40) (character 60))) (parent (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::+::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::+::TensorQuantityValue#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::-::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::-::TensorQuantityValue#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 6) (character 19)) (end (line 6) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::CoordinateTransformation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::CoordinateTransformation") (range (start (line 12) (character 19)) (end (line 12) (character 66))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::Number"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Number") (range (start (line 7) (character 19)) (end (line 7) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::ScalarQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::ScalarQuantityValue") (range (start (line 8) (character 19)) (end (line 8) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::TensorMeasurementReference"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::TensorMeasurementReference") (range (start (line 11) (character 19)) (end (line 11) (character 68))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::TensorQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::TensorQuantityValue") (range (start (line 10) (character 19)) (end (line 10) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult::Number"))) (kind featureTyping) (ordinal 0)) (authored-target ": Number[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult::ScalarQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": ScalarQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::VectorQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::VectorQuantityValue") (range (start (line 9) (character 19)) (end (line 9) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::[::elements"))) (kind featureTyping) (ordinal 0)) (authored-target "elements: Number[1..n] ordered") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::[::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "mRef: TensorMeasurementReference[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::isUnitTensorQuantity::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x : TensorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::isZeroTensorQuantity::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x : TensorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult::ScalarQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": ScalarQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult::Number"))) (kind featureTyping) (ordinal 0)) (authored-target ": Number[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult::TensorQuantityValue#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::transform::sourceTensor"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "TensorCalculations::TensorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::transform::targetTensor"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "TensorCalculations::TensorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::transform::transformation"))) (kind featureTyping) (ordinal 0)) (authored-target "CoordinateTransformation") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "TensorCalculations::CoordinateTransformation")))))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TensorCalculations::transform::sourceTensor"))) (target (node (document "d0") (qualified-name "TensorCalculations::TensorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TensorCalculations::transform::sourceTensor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TensorCalculations::transform::targetTensor"))) (target (node (document "d0") (qualified-name "TensorCalculations::TensorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TensorCalculations::transform::targetTensor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TensorCalculations::transform::transformation"))) (target (node (document "d0") (qualified-name "TensorCalculations::CoordinateTransformation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TensorCalculations::transform::transformation"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "TensorCalculations::[")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
