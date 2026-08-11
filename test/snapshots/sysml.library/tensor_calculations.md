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
    (element (id (node (document "d0") (qualified-name "TensorCalculations"))) (kind "package") (name "TensorCalculations") (declared-name "TensorCalculations"))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::+"))) (kind "calc def") (name "+") (declared-name "+") (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::+::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::+"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::+::TensorQuantityValue#in_out_parameter"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::+"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::-"))) (kind "calc def") (name "-") (declared-name "-") (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::-::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::-"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::-::TensorQuantityValue#in_out_parameter"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::-"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (parent (node (document "d0") (qualified-name "TensorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::CoordinateTransformation"))) (kind "import") (name "CoordinateTransformation") (declared-name "CoordinateTransformation") (parent (node (document "d0") (qualified-name "TensorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::CoordinateTransformation") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::Number"))) (kind "import") (name "Number") (declared-name "Number") (parent (node (document "d0") (qualified-name "TensorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Number") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::ScalarQuantityValue"))) (kind "import") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::ScalarQuantityValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorMeasurementReference"))) (kind "import") (name "TensorMeasurementReference") (declared-name "TensorMeasurementReference") (parent (node (document "d0") (qualified-name "TensorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::TensorMeasurementReference") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorQuantityValue"))) (kind "import") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::TensorQuantityValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult"))) (kind "calc def") (name "TensorScalarMult") (declared-name "TensorScalarMult") (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult::Number"))) (kind "in out parameter") (name "Number") (declared-name "Number") (parent (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult"))) (authored (relationships (typing (reference ": Number[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult"))) (kind "calc def") (name "TensorScalarQuantityMult") (declared-name "TensorScalarQuantityMult") (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult::ScalarQuantityValue"))) (kind "in out parameter") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult"))) (authored (relationships (typing (reference ": ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::VectorQuantityValue"))) (kind "import") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::VectorQuantityValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::["))) (kind "calc def") (name "[") (declared-name "[") (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::[::elements"))) (kind "in out parameter") (name "elements") (declared-name "elements") (parent (node (document "d0") (qualified-name "TensorCalculations::["))) (authored (relationships (typing (reference "elements: Number[1..n] ordered")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::[::mRef"))) (kind "in out parameter") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "TensorCalculations::["))) (authored (relationships (typing (reference "mRef: TensorMeasurementReference[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::isUnitTensorQuantity"))) (kind "calc def") (name "isUnitTensorQuantity") (declared-name "isUnitTensorQuantity") (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::isUnitTensorQuantity::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "TensorCalculations::isUnitTensorQuantity"))) (authored (relationships (typing (reference "x : TensorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::isZeroTensorQuantity"))) (kind "calc def") (name "isZeroTensorQuantity") (declared-name "isZeroTensorQuantity") (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::isZeroTensorQuantity::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "TensorCalculations::isZeroTensorQuantity"))) (authored (relationships (typing (reference "x : TensorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult"))) (kind "calc def") (name "scalarQuantityTensorMult") (declared-name "scalarQuantityTensorMult") (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult::ScalarQuantityValue"))) (kind "in out parameter") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult"))) (authored (relationships (typing (reference ": ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult"))) (kind "calc def") (name "scalarTensorMult") (declared-name "scalarTensorMult") (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult::Number"))) (kind "in out parameter") (name "Number") (declared-name "Number") (parent (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult"))) (authored (relationships (typing (reference ": Number[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult"))) (kind "calc def") (name "tensorTensorMult") (declared-name "tensorTensorMult") (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult::TensorQuantityValue#in_out_parameter"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult"))) (kind "calc def") (name "tensorVectorMult") (declared-name "tensorVectorMult") (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::transform"))) (kind "calc def") (name "transform") (declared-name "transform") (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::transform::sourceTensor"))) (kind "in out parameter") (name "sourceTensor") (declared-name "sourceTensor") (parent (node (document "d0") (qualified-name "TensorCalculations::transform"))) (authored (relationships (typing (reference "TensorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::transform::targetTensor"))) (kind "return parameter") (name "targetTensor") (declared-name "targetTensor") (parent (node (document "d0") (qualified-name "TensorCalculations::transform"))) (authored (relationships (typing (reference "TensorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::transform::transformation"))) (kind "in out parameter") (name "transformation") (declared-name "transformation") (parent (node (document "d0") (qualified-name "TensorCalculations::transform"))) (authored (relationships (typing (reference "CoordinateTransformation")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult"))) (kind "calc def") (name "vectorTensorMult") (declared-name "vectorTensorMult") (parent (node (document "d0") (qualified-name "TensorCalculations"))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult::TensorQuantityValue"))) (kind "in out parameter") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult"))) (authored (relationships (typing (reference ": TensorQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (parent (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::+::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::+::TensorQuantityValue#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::-::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::-::TensorQuantityValue#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::CoordinateTransformation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::CoordinateTransformation") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::Number"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Number") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::ScalarQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::ScalarQuantityValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::TensorMeasurementReference"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::TensorMeasurementReference") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::TensorQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::TensorQuantityValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult::Number"))) (kind featureTyping) (ordinal 0)) (authored-target ": Number[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::TensorScalarMult::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult::ScalarQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::TensorScalarQuantityMult::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::VectorQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::VectorQuantityValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::[::elements"))) (kind featureTyping) (ordinal 0)) (authored-target "elements: Number[1..n] ordered") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::[::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "mRef: TensorMeasurementReference[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::isUnitTensorQuantity::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x : TensorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::isZeroTensorQuantity::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x : TensorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult::ScalarQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::scalarQuantityTensorMult::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult::Number"))) (kind featureTyping) (ordinal 0)) (authored-target ": Number[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::scalarTensorMult::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::tensorTensorMult::TensorQuantityValue#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::tensorVectorMult::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::transform::sourceTensor"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "TensorCalculations::TensorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::transform::targetTensor"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "TensorCalculations::TensorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::transform::transformation"))) (kind featureTyping) (ordinal 0)) (authored-target "CoordinateTransformation") (outcome (status resolved) (target (node (document "d0") (qualified-name "TensorCalculations::CoordinateTransformation")))))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": TensorQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TensorCalculations::vectorTensorMult::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (outcome (status unresolved)))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 19) (end 7 39)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "TensorCalculations::Number"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Number")
        (range (start 7 19) (end 7 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 19) (end 6 40)) (probe (position 6 19))
      (reference
        (source (document "d0") (qualified-name "TensorCalculations::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 6 19) (end 6 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 19) (end 8 50)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "TensorCalculations::ScalarQuantityValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::ScalarQuantityValue")
        (range (start 8 19) (end 8 50))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 19) (end 9 50)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "TensorCalculations::VectorQuantityValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::VectorQuantityValue")
        (range (start 9 19) (end 9 50))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 19) (end 10 50)) (probe (position 10 19))
      (reference
        (source (document "d0") (qualified-name "TensorCalculations::TensorQuantityValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::TensorQuantityValue")
        (range (start 10 19) (end 10 50))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 19) (end 12 66)) (probe (position 12 19))
      (reference
        (source (document "d0") (qualified-name "TensorCalculations::CoordinateTransformation"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::CoordinateTransformation")
        (range (start 12 19) (end 12 66))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 19) (end 11 68)) (probe (position 11 19))
      (reference
        (source (document "d0") (qualified-name "TensorCalculations::TensorMeasurementReference"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::TensorMeasurementReference")
        (range (start 11 19) (end 11 68))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
