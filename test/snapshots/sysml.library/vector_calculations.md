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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vector_calculations.md"
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
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "2d859db5ce8faf084977dcf80bf20f55a87836288eed83a32816c56f7b68a325") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VectorCalculations"))) (kind "package") (name "VectorCalculations") (declared-name "VectorCalculations") (range (start (line 0) (character 0)) (end (line 0) (character 3046))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::*"))) (kind "alias") (name "*") (declared-name "*") (range (start (line 43) (character 4)) (end (line 43) (character 35))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::+"))) (kind "calc def") (name "+") (declared-name "+") (range (start (line 30) (character 1)) (end (line 30) (character 132))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::+::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 30) (character 40)) (end (line 30) (character 68))) (parent (node (document "d0") (qualified-name "VectorCalculations::+"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::+::VectorQuantityValue#in_out_parameter"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 30) (character 69)) (end (line 30) (character 97))) (parent (node (document "d0") (qualified-name "VectorCalculations::+"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::-"))) (kind "calc def") (name "-") (declared-name "-") (range (start (line 31) (character 1)) (end (line 31) (character 132))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::-::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 31) (character 40)) (end (line 31) (character 68))) (parent (node (document "d0") (qualified-name "VectorCalculations::-"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::-::VectorQuantityValue#in_out_parameter"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 31) (character 69)) (end (line 31) (character 97))) (parent (node (document "d0") (qualified-name "VectorCalculations::-"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 6) (character 1)) (end (line 6) (character 38))) (parent (node (document "d0") (qualified-name "VectorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 37))))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::CoordinateTransformation"))) (kind "import") (name "CoordinateTransformation") (declared-name "CoordinateTransformation") (range (start (line 11) (character 4)) (end (line 11) (character 67))) (parent (node (document "d0") (qualified-name "VectorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::CoordinateTransformation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 19)) (end (line 11) (character 66))))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::Number"))) (kind "import") (name "Number") (declared-name "Number") (range (start (line 7) (character 1)) (end (line 7) (character 37))) (parent (node (document "d0") (qualified-name "VectorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Number") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 36))))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::ScalarQuantityValue"))) (kind "import") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (range (start (line 8) (character 4)) (end (line 8) (character 51))) (parent (node (document "d0") (qualified-name "VectorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::ScalarQuantityValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 19)) (end (line 8) (character 50))))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::VectorMeasurementReference"))) (kind "import") (name "VectorMeasurementReference") (declared-name "VectorMeasurementReference") (range (start (line 10) (character 4)) (end (line 10) (character 69))) (parent (node (document "d0") (qualified-name "VectorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::VectorMeasurementReference") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 19)) (end (line 10) (character 68))))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::VectorQuantityValue"))) (kind "import") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 9) (character 4)) (end (line 9) (character 51))) (parent (node (document "d0") (qualified-name "VectorCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::VectorQuantityValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 19)) (end (line 9) (character 50))))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::["))) (kind "calc def") (name "[") (declared-name "[") (range (start (line 13) (character 4)) (end (line 13) (character 228))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::[::elements"))) (kind "in out parameter") (name "elements") (declared-name "elements") (range (start (line 14) (character 5)) (end (line 14) (character 39))) (parent (node (document "d0") (qualified-name "VectorCalculations::["))) (authored (relationships (typing (reference "elements: Number[1..n] ordered") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::[::mRef"))) (kind "in out parameter") (name "mRef") (declared-name "mRef") (range (start (line 15) (character 5)) (end (line 15) (character 44))) (parent (node (document "d0") (qualified-name "VectorCalculations::["))) (authored (relationships (typing (reference "mRef: VectorMeasurementReference[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 3046))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::angle"))) (kind "calc def") (name "angle") (declared-name "angle") (range (start (line 47) (character 1)) (end (line 47) (character 123))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::angle::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 47) (character 44)) (end (line 47) (character 72))) (parent (node (document "d0") (qualified-name "VectorCalculations::angle"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::angle::VectorQuantityValue#in_out_parameter"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 47) (character 73)) (end (line 47) (character 101))) (parent (node (document "d0") (qualified-name "VectorCalculations::angle"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::inner"))) (kind "calc def") (name "inner") (declared-name "inner") (range (start (line 40) (character 1)) (end (line 40) (character 123))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::inner::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 40) (character 44)) (end (line 40) (character 72))) (parent (node (document "d0") (qualified-name "VectorCalculations::inner"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::inner::VectorQuantityValue#in_out_parameter"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 40) (character 73)) (end (line 40) (character 101))) (parent (node (document "d0") (qualified-name "VectorCalculations::inner"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::isUnitVectorQuantity"))) (kind "calc def") (name "isUnitVectorQuantity") (declared-name "isUnitVectorQuantity") (range (start (line 24) (character 4)) (end (line 24) (character 103))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::isUnitVectorQuantity::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 25) (character 5)) (end (line 25) (character 33))) (parent (node (document "d0") (qualified-name "VectorCalculations::isUnitVectorQuantity"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::isZeroVectorQuantity"))) (kind "calc def") (name "isZeroVectorQuantity") (declared-name "isZeroVectorQuantity") (range (start (line 20) (character 4)) (end (line 20) (character 136))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::isZeroVectorQuantity::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 21) (character 5)) (end (line 21) (character 33))) (parent (node (document "d0") (qualified-name "VectorCalculations::isZeroVectorQuantity"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::norm"))) (kind "calc def") (name "norm") (declared-name "norm") (range (start (line 46) (character 1)) (end (line 46) (character 92))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::norm::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 46) (character 42)) (end (line 46) (character 70))) (parent (node (document "d0") (qualified-name "VectorCalculations::norm"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::outer"))) (kind "calc def") (name "outer") (declared-name "outer") (range (start (line 41) (character 4)) (end (line 41) (character 113))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::outer::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 41) (character 21)) (end (line 41) (character 49))) (parent (node (document "d0") (qualified-name "VectorCalculations::outer"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::outer::VectorQuantityValue#in_out_parameter"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 41) (character 50)) (end (line 41) (character 78))) (parent (node (document "d0") (qualified-name "VectorCalculations::outer"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::scalarQuantityVectorMult"))) (kind "calc def") (name "scalarQuantityVectorMult") (declared-name "scalarQuantityVectorMult") (range (start (line 36) (character 4)) (end (line 36) (character 132))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::scalarQuantityVectorMult::ScalarQuantityValue"))) (kind "in out parameter") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (range (start (line 36) (character 40)) (end (line 36) (character 68))) (parent (node (document "d0") (qualified-name "VectorCalculations::scalarQuantityVectorMult"))) (authored (relationships (typing (reference ": ScalarQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::scalarQuantityVectorMult::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 36) (character 69)) (end (line 36) (character 97))) (parent (node (document "d0") (qualified-name "VectorCalculations::scalarQuantityVectorMult"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::scalarVectorMult"))) (kind "calc def") (name "scalarVectorMult") (declared-name "scalarVectorMult") (range (start (line 34) (character 1)) (end (line 34) (character 145))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::scalarVectorMult::Number"))) (kind "in out parameter") (name "Number") (declared-name "Number") (range (start (line 34) (character 66)) (end (line 34) (character 81))) (parent (node (document "d0") (qualified-name "VectorCalculations::scalarVectorMult"))) (authored (relationships (typing (reference ": Number[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::scalarVectorMult::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 34) (character 82)) (end (line 34) (character 110))) (parent (node (document "d0") (qualified-name "VectorCalculations::scalarVectorMult"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::transform"))) (kind "calc def") (name "transform") (declared-name "transform") (range (start (line 50) (character 1)) (end (line 50) (character 341))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::transform::sourceVector"))) (kind "in out parameter") (name "sourceVector") (declared-name "sourceVector") (range (start (line 52) (character 5)) (end (line 52) (character 94))) (parent (node (document "d0") (qualified-name "VectorCalculations::transform"))) (authored (relationships (typing (reference "VectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::transform::transformation"))) (kind "in out parameter") (name "transformation") (declared-name "transformation") (range (start (line 51) (character 5)) (end (line 51) (character 50))) (parent (node (document "d0") (qualified-name "VectorCalculations::transform"))) (authored (relationships (typing (reference "CoordinateTransformation") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarDiv"))) (kind "calc def") (name "vectorScalarDiv") (declared-name "vectorScalarDiv") (range (start (line 38) (character 1)) (end (line 38) (character 143))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarDiv::Number"))) (kind "in out parameter") (name "Number") (declared-name "Number") (range (start (line 38) (character 93)) (end (line 38) (character 108))) (parent (node (document "d0") (qualified-name "VectorCalculations::vectorScalarDiv"))) (authored (relationships (typing (reference ": Number[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarDiv::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 38) (character 64)) (end (line 38) (character 92))) (parent (node (document "d0") (qualified-name "VectorCalculations::vectorScalarDiv"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarMult"))) (kind "calc def") (name "vectorScalarMult") (declared-name "vectorScalarMult") (range (start (line 35) (character 1)) (end (line 35) (character 145))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarMult::Number"))) (kind "in out parameter") (name "Number") (declared-name "Number") (range (start (line 35) (character 95)) (end (line 35) (character 110))) (parent (node (document "d0") (qualified-name "VectorCalculations::vectorScalarMult"))) (authored (relationships (typing (reference ": Number[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarMult::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 35) (character 66)) (end (line 35) (character 94))) (parent (node (document "d0") (qualified-name "VectorCalculations::vectorScalarMult"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityDiv"))) (kind "calc def") (name "vectorScalarQuantityDiv") (declared-name "vectorScalarQuantityDiv") (range (start (line 39) (character 4)) (end (line 39) (character 131))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityDiv::ScalarQuantityValue"))) (kind "in out parameter") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (range (start (line 39) (character 68)) (end (line 39) (character 96))) (parent (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityDiv"))) (authored (relationships (typing (reference ": ScalarQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityDiv::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 39) (character 39)) (end (line 39) (character 67))) (parent (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityDiv"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityMult"))) (kind "calc def") (name "vectorScalarQuantityMult") (declared-name "vectorScalarQuantityMult") (range (start (line 37) (character 4)) (end (line 37) (character 132))) (parent (node (document "d0") (qualified-name "VectorCalculations"))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityMult::ScalarQuantityValue"))) (kind "in out parameter") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (range (start (line 37) (character 69)) (end (line 37) (character 97))) (parent (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityMult"))) (authored (relationships (typing (reference ": ScalarQuantityValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityMult::VectorQuantityValue"))) (kind "in out parameter") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 37) (character 40)) (end (line 37) (character 68))) (parent (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityMult"))) (authored (relationships (typing (reference ": VectorQuantityValue[1]") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::+::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::+::VectorQuantityValue#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::-::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::-::VectorQuantityValue#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 6) (character 16)) (end (line 6) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::CoordinateTransformation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::CoordinateTransformation") (range (start (line 11) (character 19)) (end (line 11) (character 66))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::Number"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Number") (range (start (line 7) (character 16)) (end (line 7) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::ScalarQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::ScalarQuantityValue") (range (start (line 8) (character 19)) (end (line 8) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::VectorMeasurementReference"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::VectorMeasurementReference") (range (start (line 10) (character 19)) (end (line 10) (character 68))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::VectorQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::VectorQuantityValue") (range (start (line 9) (character 19)) (end (line 9) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::[::elements"))) (kind featureTyping) (ordinal 0)) (authored-target "elements: Number[1..n] ordered") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::[::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "mRef: VectorMeasurementReference[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::angle::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::angle::VectorQuantityValue#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::inner::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::inner::VectorQuantityValue#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::isUnitVectorQuantity::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::isZeroVectorQuantity::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::norm::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::outer::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::outer::VectorQuantityValue#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::scalarQuantityVectorMult::ScalarQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": ScalarQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::scalarQuantityVectorMult::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::scalarVectorMult::Number"))) (kind featureTyping) (ordinal 0)) (authored-target ": Number[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::scalarVectorMult::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::transform::sourceVector"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VectorCalculations::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::transform::transformation"))) (kind featureTyping) (ordinal 0)) (authored-target "CoordinateTransformation") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VectorCalculations::CoordinateTransformation")))))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::vectorScalarDiv::Number"))) (kind featureTyping) (ordinal 0)) (authored-target ": Number[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::vectorScalarDiv::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::vectorScalarMult::Number"))) (kind featureTyping) (ordinal 0)) (authored-target ": Number[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::vectorScalarMult::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityDiv::ScalarQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": ScalarQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityDiv::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityMult::ScalarQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": ScalarQuantityValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorCalculations::vectorScalarQuantityMult::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target ": VectorQuantityValue[1]") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VectorCalculations::transform::sourceVector"))) (target (node (document "d0") (qualified-name "VectorCalculations::VectorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VectorCalculations::transform::sourceVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VectorCalculations::transform::transformation"))) (target (node (document "d0") (qualified-name "VectorCalculations::CoordinateTransformation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VectorCalculations::transform::transformation"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "VectorCalculations::[")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "VectorCalculations::transform")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
