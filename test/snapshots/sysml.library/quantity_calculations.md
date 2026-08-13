# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/QuantityCalculations
type=file
~~~
# SOURCE
~~~sysml
standard library package QuantityCalculations {
	doc
	/*
	 * This package package defines calculations for the construction of and computations on ScalarQuantityValues.
	 */
	 
	private import ScalarValues::*;
    private import Quantities::ScalarQuantityValue;
    private import MeasurementReferences::ScalarMeasurementReference;
    private import MeasurementReferences::DimensionOneValue;
    
    calc def '[' specializes BaseFunctions::'[' { 
    	in num: Number[1]; 
    	in mRef: ScalarMeasurementReference[1]; 
    	return quantity : ScalarQuantityValue[1];
    }

    calc def isZero specializes NumericalFunctions::isZero { 
    	in x: ScalarQuantityValue[1]; 
        return : Boolean[1] = NumericalFunctions::isZero(x.num);
    }
    calc def isUnit specializes NumericalFunctions::isUnit { 
    	in x: ScalarQuantityValue[1]; 
        return : Boolean[1] = NumericalFunctions::isUnit(x.num);
    }
    
	calc def abs specializes NumericalFunctions::abs { in x: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }

	calc def '+' specializes NumericalFunctions::'+' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[0..1]; return : ScalarQuantityValue; }
	calc def '-' specializes NumericalFunctions::'-' { in x: ScalarQuantityValue; in y: ScalarQuantityValue[0..1]; return : ScalarQuantityValue[1]; }
	calc def '*' specializes NumericalFunctions::'*' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }
	calc def '/' specializes NumericalFunctions::'/' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }
	calc def '**' specializes NumericalFunctions::'**' { in x: ScalarQuantityValue[1]; in y: Real[1]; return : ScalarQuantityValue[1]; }
	calc def '^' specializes NumericalFunctions::'^' { in x: ScalarQuantityValue[1]; in y: Real[1]; return : ScalarQuantityValue[1]; }
	
	calc def '<' specializes NumericalFunctions::'<' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : Boolean[1]; }
	calc def '>' specializes NumericalFunctions::'>' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : Boolean[1]; }
	calc def '<=' specializes NumericalFunctions::'<=' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : Boolean[1]; }
	calc def '>=' specializes NumericalFunctions::'>=' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : Boolean[1]; }

	calc def max specializes NumericalFunctions::max { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }
	calc def min specializes NumericalFunctions::min { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }

	calc def '==' specializes DataFunctions::'==' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : Boolean[1]; }
		
	calc def sqrt{ in x: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }

	calc def floor { in x: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }
	calc def round { in x: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }
	
	calc def ToString specializes BaseFunctions::ToString { in x: ScalarQuantityValue[1]; return : String; }
	calc def ToInteger { in x: ScalarQuantityValue[1]; return : Integer[1]; }
	calc def ToRational { in x: ScalarQuantityValue[1]; return : Rational[1]; }
	calc def ToReal { in x: ScalarQuantityValue[1]; return : Real[1]; }
	calc def ToDimensionOneValue { in x: Real[1]; return : DimensionOneValue[1]; }
	
	calc def sum specializes NumericalFunctions::sum { in collection: ScalarQuantityValue[0..*]; 
        private attribute zero : ScalarQuantityValue[1];
		assert constraint { isZero(zero) }
		return : ScalarQuantityValue = NumericalFunctions::sum0(collection, zero);
	}
	
	calc def product specializes NumericalFunctions::product { in collection: ScalarQuantityValue[0..*]; 
		private attribute one : ScalarQuantityValue[1];
		assert constraint { isUnit(one) }
        return : ScalarQuantityValue = NumericalFunctions::product1(collection, one);
	}

    calc def ConvertQuantity{ in x: ScalarQuantityValue[1]; in targetMRef: ScalarMeasurementReference[1]; return : ScalarQuantityValue[1]; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/quantity_calculations.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 11 29) (end 11 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 12 5) (end 12 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 13 5) (end 13 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 14 5) (end 14 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 14 21) (end 15 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 17 32) (end 17 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 18 5) (end 18 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 19 8) (end 19 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 21 32) (end 21 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 22 5) (end 22 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 23 8) (end 23 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 26 26) (end 26 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 26 52) (end 26 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 26 82) (end 26 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 28 26) (end 28 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 28 52) (end 28 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 28 82) (end 28 114))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 28 115) (end 28 144))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 29 26) (end 29 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 29 52) (end 29 78))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 29 79) (end 29 111))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 29 112) (end 29 144))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 30 26) (end 30 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 30 52) (end 30 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 30 82) (end 30 111))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 30 112) (end 30 144))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 31 26) (end 31 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 31 52) (end 31 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 31 82) (end 31 111))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 31 112) (end 31 144))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 32 27) (end 32 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 32 54) (end 32 83))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 32 84) (end 32 98))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 32 99) (end 32 131))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 33 26) (end 33 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 33 52) (end 33 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 33 82) (end 33 96))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 33 97) (end 33 129))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 35 26) (end 35 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 35 52) (end 35 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 35 82) (end 35 111))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 35 112) (end 35 132))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 36 26) (end 36 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 36 52) (end 36 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 36 82) (end 36 111))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 36 112) (end 36 132))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 37 27) (end 37 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 37 54) (end 37 83))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 37 84) (end 37 113))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 37 114) (end 37 134))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 38 27) (end 38 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 38 54) (end 38 83))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 38 84) (end 38 113))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 38 114) (end 38 134))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 40 26) (end 40 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 40 52) (end 40 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 40 82) (end 40 111))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 40 112) (end 40 144))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 41 26) (end 41 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 41 52) (end 41 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 41 82) (end 41 111))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 41 112) (end 41 144))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 43 27) (end 43 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 43 49) (end 43 78))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 43 79) (end 43 108))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 43 109) (end 43 129))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 45 16) (end 45 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 45 46) (end 45 78))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 47 18) (end 47 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 47 48) (end 47 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 48 18) (end 48 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 48 48) (end 48 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 50 31) (end 50 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 50 57) (end 50 86))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 50 87) (end 50 103))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 51 22) (end 51 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 51 52) (end 51 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 52 23) (end 52 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 52 53) (end 52 74))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 53 19) (end 53 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 53 49) (end 53 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 54 32) (end 54 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 54 47) (end 54 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 56 26) (end 56 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 56 52) (end 56 93))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 57 8) (end 57 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 57 16) (end 57 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 57 26) (end 57 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 57 31) (end 58 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 58 2) (end 58 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 58 9) (end 58 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 58 20) (end 59 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 59 2) (end 59 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 62 30) (end 62 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 62 60) (end 62 101))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 63 2) (end 63 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 63 10) (end 63 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 63 20) (end 63 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 63 24) (end 64 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 64 2) (end 64 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 64 9) (end 64 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 64 20) (end 65 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 65 8) (end 65 85))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 68 30) (end 68 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 68 60) (end 68 105))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 68 106) (end 68 138))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:7f5be5954e0a181f8d7a24827f111a25d513fe0471efb1538f82fb586b5b6b7f") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::ScalarQuantityValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::ScalarMeasurementReference") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::DimensionOneValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::*"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::*"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::**"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::**"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::+"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::+"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::-"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::-"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::/"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::/"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::<"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::<"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::<="))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::<="))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::=="))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::=="))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::>"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::>"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::>="))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::>="))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::ConvertQuantity"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::ToDimensionOneValue"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::ToInteger"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::ToRational"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::ToReal"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::ToString"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BaseFunctions::ToString"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::["))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BaseFunctions::["))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::^"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::^"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::abs"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::abs"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::floor"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::isUnit"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::isUnit"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::isZero"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::isZero"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::max"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::max"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::min"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::min"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::product"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::product"))))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::round"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::sqrt"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::sum"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalFunctions::sum"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::ScalarMeasurementReference")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::*"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::*")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::**"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::**")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::+"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::+")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::-"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::-")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::/"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::/")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::<"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::<")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::<="))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::<=")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::=="))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::==")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::>"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::>")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::>="))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::>=")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::ToString"))) (kind specialization) (ordinal 0))
      (authored-target "BaseFunctions::ToString")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::["))) (kind specialization) (ordinal 0))
      (authored-target "BaseFunctions::[")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::^"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::^")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::abs"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::abs")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::isUnit"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::isUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::isZero"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::isZero")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::max"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::max")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::min"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::min")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::product"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::product")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::sum"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalFunctions::sum")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 6 16) (end 6 31)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 7 19) (end 7 50)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 8 19) (end 8 68)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::ScalarMeasurementReference")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 9 19) (end 9 59)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 30 26) (end 30 49)) (probe (position 30 26))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::*"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::*")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 32 27) (end 32 51)) (probe (position 32 27))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::**"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::**")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 28 26) (end 28 49)) (probe (position 28 26))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::+"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::+")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 29 26) (end 29 49)) (probe (position 29 26))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::-"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::-")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 31 26) (end 31 49)) (probe (position 31 26))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::/"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::/")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 35 26) (end 35 49)) (probe (position 35 26))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::<"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::<")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 37 27) (end 37 51)) (probe (position 37 27))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::<="))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::<=")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 43 27) (end 43 46)) (probe (position 43 27))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::=="))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::==")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 36 26) (end 36 49)) (probe (position 36 26))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::>"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::>")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 38 27) (end 38 51)) (probe (position 38 27))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::>="))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::>=")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 50 31) (end 50 54)) (probe (position 50 31))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::ToString"))) (kind specialization) (ordinal 0) (authored-target "BaseFunctions::ToString")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 11 29) (end 11 47)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::["))) (kind specialization) (ordinal 0) (authored-target "BaseFunctions::[")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 33 26) (end 33 49)) (probe (position 33 26))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::^"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::^")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 26 26) (end 26 49)) (probe (position 26 26))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::abs"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::abs")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 21 32) (end 21 58)) (probe (position 21 32))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::isUnit"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::isUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 17 32) (end 17 58)) (probe (position 17 32))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::isZero"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::isZero")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 40 26) (end 40 49)) (probe (position 40 26))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::max"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::max")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 41 26) (end 41 49)) (probe (position 41 26))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::min"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::min")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 62 30) (end 62 57)) (probe (position 62 30))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::product"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::product")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/quantity_calculations.md") (range (start 56 26) (end 56 49)) (probe (position 56 26))
    (reference (id (source (node (document "memory://snapshot/quantity_calculations.md") (qualified-name "QuantityCalculations::sum"))) (kind specialization) (ordinal 0) (authored-target "NumericalFunctions::sum")
      (outcome (status unresolved)))
  )
)
~~~
