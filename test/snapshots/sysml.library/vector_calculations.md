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
  (document "memory://snapshot/vector_calculations.md"
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 13 20) (end 13 38))
      )
      (diagnostic
        (severity error)
        (code "recovered_calc_body_element")
        (source "parser")
        (range (start 14 5) (end 15 5))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 14 5) (end 15 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 14) (end 15 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 12) (end 16 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 16 21) (end 17 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 5) (end 17 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 13) (end 17 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 23) (end 17 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 17 25) (end 18 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 20 37) (end 20 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 22 5) (end 22 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 26 5) (end 26 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 30 17) (end 30 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 30 98) (end 30 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 31 17) (end 31 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 31 98) (end 31 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 34 30) (end 34 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 34 111) (end 34 143))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 35 30) (end 35 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 35 111) (end 35 143))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 36 98) (end 36 130))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 37 98) (end 37 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 38 29) (end 38 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 38 109) (end 38 141))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 39 97) (end 39 129))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 40 19) (end 40 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 40 102) (end 40 121))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 41 79) (end 41 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 46 18) (end 46 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 46 71) (end 46 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 47 19) (end 47 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 47 102) (end 47 121))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 25) (end 51 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 23) (end 52 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 12) (end 55 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 55 25) (end 60 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:cf109ab4e6563103889612ca5a3f9a77ebdc78269c2fd5a15d026a5544e13ed4") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Number") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::ScalarQuantityValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::VectorQuantityValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::VectorMeasurementReference") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::CoordinateTransformation") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::*"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "scalarVectorMult"))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::+"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorFunctions::+"))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::-"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorFunctions::-"))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::["))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BaseFunctions::[")) (expressionOperand (reference "quantity")) (expressionOperand (reference "private")) (expressionOperand (reference "attribute")) (expressionOperand (reference "n"))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::[::mRef"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorMeasurementReference") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::angle"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorFunctions::angle"))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::inner"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorFunctions::inner"))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::isUnitVectorQuantity"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::isZeroVectorQuantity"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorFunctions::isZeroVector"))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::norm"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorFunctions::norm"))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::outer"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::scalarQuantityVectorMult"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::scalarVectorMult"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorFunctions::scalarVectorMult"))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::transform"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "targetVector"))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::transform::sourceVector"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorQuantityValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::transform::transformation"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CoordinateTransformation") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::vectorScalarDiv"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorFunctions::vectorScalarDiv"))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::vectorScalarMult"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorFunctions::vectorScalarMult"))))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::vectorScalarQuantityDiv"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::vectorScalarQuantityMult"))) (kind calc-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Number")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::VectorMeasurementReference")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::CoordinateTransformation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::*"))) (kind aliasBinding) (ordinal 0))
      (authored-target "scalarVectorMult")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::scalarVectorMult")))))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::+"))) (kind specialization) (ordinal 0))
      (authored-target "VectorFunctions::+")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::-"))) (kind specialization) (ordinal 0))
      (authored-target "VectorFunctions::-")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::["))) (kind specialization) (ordinal 0))
      (authored-target "BaseFunctions::[")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::["))) (kind expressionOperand) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::["))) (kind expressionOperand) (ordinal 1))
      (authored-target "private")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::["))) (kind expressionOperand) (ordinal 2))
      (authored-target "attribute")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::["))) (kind expressionOperand) (ordinal 3))
      (authored-target "n")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::[::mRef"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorMeasurementReference")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::angle"))) (kind specialization) (ordinal 0))
      (authored-target "VectorFunctions::angle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::inner"))) (kind specialization) (ordinal 0))
      (authored-target "VectorFunctions::inner")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::isZeroVectorQuantity"))) (kind specialization) (ordinal 0))
      (authored-target "VectorFunctions::isZeroVector")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::norm"))) (kind specialization) (ordinal 0))
      (authored-target "VectorFunctions::norm")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::scalarVectorMult"))) (kind specialization) (ordinal 0))
      (authored-target "VectorFunctions::scalarVectorMult")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::transform"))) (kind expressionOperand) (ordinal 0))
      (authored-target "targetVector")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::transform::sourceVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::transform::transformation"))) (kind featureTyping) (ordinal 0))
      (authored-target "CoordinateTransformation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::vectorScalarDiv"))) (kind specialization) (ordinal 0))
      (authored-target "VectorFunctions::vectorScalarDiv")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::vectorScalarMult"))) (kind specialization) (ordinal 0))
      (authored-target "VectorFunctions::vectorScalarMult")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::*"))) (target (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::scalarVectorMult"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::*"))) (kind aliasBinding) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vector_calculations.md") (range (start 6 16) (end 6 37)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 7 16) (end 7 36)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Number")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 8 19) (end 8 50)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 9 19) (end 9 50)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 10 19) (end 10 68)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::VectorMeasurementReference")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 11 19) (end 11 66)) (probe (position 11 19))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::CoordinateTransformation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 43 18) (end 43 34)) (probe (position 43 18))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::*"))) (kind aliasBinding) (ordinal 0) (authored-target "scalarVectorMult")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::scalarVectorMult")))))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 30 17) (end 30 37)) (probe (position 30 17))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::+"))) (kind specialization) (ordinal 0) (authored-target "VectorFunctions::+")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 31 17) (end 31 37)) (probe (position 31 17))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::-"))) (kind specialization) (ordinal 0) (authored-target "VectorFunctions::-")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 13 20) (end 13 38)) (probe (position 13 20))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::["))) (kind specialization) (ordinal 0) (authored-target "BaseFunctions::[")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 16 12) (end 16 20)) (probe (position 16 12))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::["))) (kind expressionOperand) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 17 5) (end 17 12)) (probe (position 17 5))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::["))) (kind expressionOperand) (ordinal 1) (authored-target "private")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 17 13) (end 17 22)) (probe (position 17 13))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::["))) (kind expressionOperand) (ordinal 2) (authored-target "attribute")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 17 23) (end 17 24)) (probe (position 17 23))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::["))) (kind expressionOperand) (ordinal 3) (authored-target "n")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 15 14) (end 15 40)) (probe (position 15 14))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::[::mRef"))) (kind featureTyping) (ordinal 0) (authored-target "VectorMeasurementReference")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 47 19) (end 47 41)) (probe (position 47 19))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::angle"))) (kind specialization) (ordinal 0) (authored-target "VectorFunctions::angle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 40 19) (end 40 41)) (probe (position 40 19))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::inner"))) (kind specialization) (ordinal 0) (authored-target "VectorFunctions::inner")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 20 37) (end 20 66)) (probe (position 20 37))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::isZeroVectorQuantity"))) (kind specialization) (ordinal 0) (authored-target "VectorFunctions::isZeroVector")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 46 18) (end 46 39)) (probe (position 46 18))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::norm"))) (kind specialization) (ordinal 0) (authored-target "VectorFunctions::norm")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 34 30) (end 34 63)) (probe (position 34 30))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::scalarVectorMult"))) (kind specialization) (ordinal 0) (authored-target "VectorFunctions::scalarVectorMult")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 55 12) (end 55 24)) (probe (position 55 12))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::transform"))) (kind expressionOperand) (ordinal 0) (authored-target "targetVector")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 52 23) (end 52 42)) (probe (position 52 23))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::transform::sourceVector"))) (kind featureTyping) (ordinal 0) (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 51 25) (end 51 49)) (probe (position 51 25))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::transform::transformation"))) (kind featureTyping) (ordinal 0) (authored-target "CoordinateTransformation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 38 29) (end 38 61)) (probe (position 38 29))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::vectorScalarDiv"))) (kind specialization) (ordinal 0) (authored-target "VectorFunctions::vectorScalarDiv")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_calculations.md") (range (start 35 30) (end 35 63)) (probe (position 35 30))
    (reference (id (source (node (document "memory://snapshot/vector_calculations.md") (qualified-name "VectorCalculations::vectorScalarMult"))) (kind specialization) (ordinal 0) (authored-target "VectorFunctions::vectorScalarMult")
      (outcome (status unresolved)))
  )
)
~~~
