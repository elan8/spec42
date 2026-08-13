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
  (document "memory://snapshot/tensor_calculations.md"
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 14 29) (end 14 47))
      )
      (diagnostic
        (severity error)
        (code "recovered_calc_body_element")
        (source "parser")
        (range (start 15 5) (end 16 5))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 15 5) (end 16 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 14) (end 16 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 12) (end 17 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 17 20) (end 18 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 5) (end 18 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 13) (end 18 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 23) (end 18 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 18 25) (end 19 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 12) (end 22 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 23 5) (end 23 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 12) (end 26 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 27 5) (end 27 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 31 20) (end 31 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 31 99) (end 31 131))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 32 20) (end 32 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 32 99) (end 32 131))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 35 77) (end 35 109))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 36 77) (end 36 109))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 37 98) (end 37 130))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 38 98) (end 38 130))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 39 90) (end 39 122))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 40 90) (end 40 122))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 41 90) (end 41 122))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 45 28) (end 45 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 46 26) (end 46 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 47 8) (end 47 50))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:2f389f45a2013aabb415fcb4473243cf867f1b992f5a1ee36539c82c518b723d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Number") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::ScalarQuantityValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::VectorQuantityValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::TensorQuantityValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::TensorMeasurementReference") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::CoordinateTransformation") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::+"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::+"))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::-"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::-"))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::TensorScalarMult"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::TensorScalarQuantityMult"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BaseFunctions::[")) (expressionOperand (reference "quantity")) (expressionOperand (reference "private")) (expressionOperand (reference "attribute")) (expressionOperand (reference "n"))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::[::mRef"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TensorMeasurementReference") (direction in))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::isUnitTensorQuantity"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::isUnitTensorQuantity::x"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TensorQuantityValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::isZeroTensorQuantity"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::isZeroTensorQuantity::x"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TensorQuantityValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::scalarQuantityTensorMult"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::scalarTensorMult"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::tensorTensorMult"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::tensorVectorMult"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::transform"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::transform::sourceTensor"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TensorQuantityValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::transform::transformation"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CoordinateTransformation") (direction in))))
    (declaration (id (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::vectorTensorMult"))) (kind calc-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Number")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::TensorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::TensorMeasurementReference")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::CoordinateTransformation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::+"))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::+")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::-"))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::-")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (kind specialization) (ordinal 0))
      (authored-target "BaseFunctions::[")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (kind expressionOperand) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (kind expressionOperand) (ordinal 1))
      (authored-target "private")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (kind expressionOperand) (ordinal 2))
      (authored-target "attribute")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (kind expressionOperand) (ordinal 3))
      (authored-target "n")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::[::mRef"))) (kind featureTyping) (ordinal 0))
      (authored-target "TensorMeasurementReference")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::isUnitTensorQuantity::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "TensorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::isZeroTensorQuantity::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "TensorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::transform::sourceTensor"))) (kind featureTyping) (ordinal 0))
      (authored-target "TensorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::transform::transformation"))) (kind featureTyping) (ordinal 0))
      (authored-target "CoordinateTransformation")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (value (kind unresolved-operand)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 6 19) (end 6 40)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 7 19) (end 7 39)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Number")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 8 19) (end 8 50)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 9 19) (end 9 50)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 10 19) (end 10 50)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::TensorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 11 19) (end 11 68)) (probe (position 11 19))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::TensorMeasurementReference")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 12 19) (end 12 66)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::CoordinateTransformation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 31 20) (end 31 38)) (probe (position 31 20))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::+"))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::+")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 32 20) (end 32 38)) (probe (position 32 20))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::-"))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::-")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 14 29) (end 14 47)) (probe (position 14 29))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (kind specialization) (ordinal 0) (authored-target "BaseFunctions::[")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 17 12) (end 17 20)) (probe (position 17 12))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (kind expressionOperand) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 18 5) (end 18 12)) (probe (position 18 5))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (kind expressionOperand) (ordinal 1) (authored-target "private")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 18 13) (end 18 22)) (probe (position 18 13))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (kind expressionOperand) (ordinal 2) (authored-target "attribute")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 18 23) (end 18 24)) (probe (position 18 23))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::["))) (kind expressionOperand) (ordinal 3) (authored-target "n")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 16 14) (end 16 40)) (probe (position 16 14))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::[::mRef"))) (kind featureTyping) (ordinal 0) (authored-target "TensorMeasurementReference")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 26 12) (end 26 31)) (probe (position 26 12))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::isUnitTensorQuantity::x"))) (kind featureTyping) (ordinal 0) (authored-target "TensorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 22 12) (end 22 31)) (probe (position 22 12))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::isZeroTensorQuantity::x"))) (kind featureTyping) (ordinal 0) (authored-target "TensorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 46 26) (end 46 45)) (probe (position 46 26))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::transform::sourceTensor"))) (kind featureTyping) (ordinal 0) (authored-target "TensorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/tensor_calculations.md") (range (start 45 28) (end 45 52)) (probe (position 45 28))
    (reference (id (source (node (document "memory://snapshot/tensor_calculations.md") (qualified-name "TensorCalculations::transform::transformation"))) (kind featureTyping) (ordinal 0) (authored-target "CoordinateTransformation")
      (outcome (status unresolved)))
  )
)
~~~
