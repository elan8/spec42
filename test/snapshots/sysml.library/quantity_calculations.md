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
  (document "quantity_calculations.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 28))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 5) (end 12 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 5) (end 13 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 5) (end 18 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 5) (end 22 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 52) (end 26 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 52) (end 28 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 82) (end 28 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 79) (end 29 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 52) (end 30 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 82) (end 30 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 52) (end 31 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 82) (end 31 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 54) (end 32 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 84) (end 32 98))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 52) (end 33 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 82) (end 33 96))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 52) (end 35 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 82) (end 35 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 52) (end 36 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 82) (end 36 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 54) (end 37 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 84) (end 37 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 54) (end 38 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 84) (end 38 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 52) (end 40 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 82) (end 40 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 52) (end 41 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 82) (end 41 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 49) (end 43 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 79) (end 43 108))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 45 16) (end 45 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 18) (end 47 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 48 18) (end 48 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 57) (end 50 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 87) (end 50 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 22) (end 51 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 23) (end 52 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 19) (end 53 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 32) (end 54 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 52) (end 56 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 62 60) (end 62 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 68 30) (end 68 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 68 60) (end 68 105))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "df295a871c6aaef5986e7a2a289b620cb32c02a2179076a52d6e3603f8fe6771") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "QuantityCalculations"))) (kind "package") (name "QuantityCalculations") (declared-name "QuantityCalculations"))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "QuantityCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::*#calc_def"))) (kind "calc def") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::*#calc_def::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::*#calc_def"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::*#calc_def::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "QuantityCalculations::*#calc_def"))) (authored (relationships (typing (reference "y: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::**"))) (kind "calc def") (name "**") (declared-name "**") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::**::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::**"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::**::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "QuantityCalculations::**"))) (authored (relationships (typing (reference "y: Real[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::+"))) (kind "calc def") (name "+") (declared-name "+") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::+::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "QuantityCalculations::+"))) (authored (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::+::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::+"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::+::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "QuantityCalculations::+"))) (authored (relationships (typing (reference "y: ScalarQuantityValue[0..1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::-"))) (kind "calc def") (name "-") (declared-name "-") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::-::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::-"))) (authored (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::-::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "QuantityCalculations::-"))) (authored (relationships (typing (reference "y: ScalarQuantityValue[0..1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::/"))) (kind "calc def") (name "/") (declared-name "/") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::/::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::/"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::/::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "QuantityCalculations::/"))) (authored (relationships (typing (reference "y: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::<"))) (kind "calc def") (name "<") (declared-name "<") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::<::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::<"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::<::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "QuantityCalculations::<"))) (authored (relationships (typing (reference "y: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::<="))) (kind "calc def") (name "<=") (declared-name "<=") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::<=::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::<="))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::<=::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "QuantityCalculations::<="))) (authored (relationships (typing (reference "y: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::=="))) (kind "calc def") (name "==") (declared-name "==") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::==::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::=="))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::==::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "QuantityCalculations::=="))) (authored (relationships (typing (reference "y: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::>"))) (kind "calc def") (name ">") (declared-name ">") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::>::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::>"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::>::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "QuantityCalculations::>"))) (authored (relationships (typing (reference "y: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::>="))) (kind "calc def") (name ">=") (declared-name ">=") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::>=::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::>="))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::>=::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "QuantityCalculations::>="))) (authored (relationships (typing (reference "y: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ConvertQuantity"))) (kind "calc def") (name "ConvertQuantity") (declared-name "ConvertQuantity") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ConvertQuantity::targetMRef"))) (kind "in out parameter") (name "targetMRef") (declared-name "targetMRef") (parent (node (document "d0") (qualified-name "QuantityCalculations::ConvertQuantity"))) (authored (relationships (typing (reference "targetMRef: ScalarMeasurementReference[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ConvertQuantity::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::ConvertQuantity"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::DimensionOneValue"))) (kind "import") (name "DimensionOneValue") (declared-name "DimensionOneValue") (parent (node (document "d0") (qualified-name "QuantityCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::DimensionOneValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ScalarMeasurementReference"))) (kind "import") (name "ScalarMeasurementReference") (declared-name "ScalarMeasurementReference") (parent (node (document "d0") (qualified-name "QuantityCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::ScalarMeasurementReference") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ScalarQuantityValue"))) (kind "import") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (parent (node (document "d0") (qualified-name "QuantityCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::ScalarQuantityValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ToDimensionOneValue"))) (kind "calc def") (name "ToDimensionOneValue") (declared-name "ToDimensionOneValue") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ToDimensionOneValue::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::ToDimensionOneValue"))) (authored (relationships (typing (reference "x: Real[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ToInteger"))) (kind "calc def") (name "ToInteger") (declared-name "ToInteger") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ToInteger::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::ToInteger"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ToRational"))) (kind "calc def") (name "ToRational") (declared-name "ToRational") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ToRational::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::ToRational"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ToReal"))) (kind "calc def") (name "ToReal") (declared-name "ToReal") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ToReal::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::ToReal"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ToString"))) (kind "calc def") (name "ToString") (declared-name "ToString") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ToString::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "QuantityCalculations::ToString"))) (authored (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::ToString::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::ToString"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::["))) (kind "calc def") (name "[") (declared-name "[") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::[::mRef"))) (kind "in out parameter") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "QuantityCalculations::["))) (authored (relationships (typing (reference "mRef: ScalarMeasurementReference[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::[::num"))) (kind "in out parameter") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "QuantityCalculations::["))) (authored (relationships (typing (reference "num: Number[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::^"))) (kind "calc def") (name "^") (declared-name "^") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::^::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::^"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::^::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "QuantityCalculations::^"))) (authored (relationships (typing (reference "y: Real[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::abs"))) (kind "calc def") (name "abs") (declared-name "abs") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::abs::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::abs"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::floor"))) (kind "calc def") (name "floor") (declared-name "floor") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::floor::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::floor"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::isUnit"))) (kind "calc def") (name "isUnit") (declared-name "isUnit") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::isUnit::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::isUnit"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::isZero"))) (kind "calc def") (name "isZero") (declared-name "isZero") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::isZero::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::isZero"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::max"))) (kind "calc def") (name "max") (declared-name "max") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::max::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::max"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::max::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "QuantityCalculations::max"))) (authored (relationships (typing (reference "y: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::min"))) (kind "calc def") (name "min") (declared-name "min") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::min::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::min"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::min::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "QuantityCalculations::min"))) (authored (relationships (typing (reference "y: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::product"))) (kind "calc def") (name "product") (declared-name "product") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::product::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "QuantityCalculations::product"))) (authored (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::product::collection"))) (kind "in out parameter") (name "collection") (declared-name "collection") (parent (node (document "d0") (qualified-name "QuantityCalculations::product"))) (authored (relationships (typing (reference "collection: ScalarQuantityValue[0..*]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::round"))) (kind "calc def") (name "round") (declared-name "round") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::round::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::round"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::sqrt"))) (kind "calc def") (name "sqrt") (declared-name "sqrt") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::sqrt::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "QuantityCalculations::sqrt"))) (authored (relationships (typing (reference "x: ScalarQuantityValue[1]")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::sum"))) (kind "calc def") (name "sum") (declared-name "sum") (parent (node (document "d0") (qualified-name "QuantityCalculations"))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::sum::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "QuantityCalculations::sum"))) (authored (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "QuantityCalculations::sum::collection"))) (kind "in out parameter") (name "collection") (declared-name "collection") (parent (node (document "d0") (qualified-name "QuantityCalculations::sum"))) (authored (relationships (typing (reference "collection: ScalarQuantityValue[0..*]")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::*#calc_def::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::*#calc_def::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::**::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::**::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: Real[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::+::"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "QuantityCalculations::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::+::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::+::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: ScalarQuantityValue[0..1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::-::x"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "QuantityCalculations::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::-::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: ScalarQuantityValue[0..1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::/::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::/::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::<::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::<::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::<=::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::<=::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::==::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::==::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::>::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::>::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::>=::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::>=::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::ConvertQuantity::targetMRef"))) (kind featureTyping) (ordinal 0)) (authored-target "targetMRef: ScalarMeasurementReference[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::ConvertQuantity::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::DimensionOneValue"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::DimensionOneValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::ScalarMeasurementReference"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::ScalarMeasurementReference") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::ScalarQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::ScalarQuantityValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::ToDimensionOneValue::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: Real[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::ToInteger::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::ToRational::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::ToReal::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::ToString::"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::ToString::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::[::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "mRef: ScalarMeasurementReference[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::[::num"))) (kind featureTyping) (ordinal 0)) (authored-target "num: Number[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::^::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::^::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: Real[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::abs::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::floor::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::isUnit::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::isZero::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::max::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::max::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::min::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::min::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::product::"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "QuantityCalculations::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::product::collection"))) (kind featureTyping) (ordinal 0)) (authored-target "collection: ScalarQuantityValue[0..*]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::round::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::sqrt::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarQuantityValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::sum::"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "QuantityCalculations::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "QuantityCalculations::sum::collection"))) (kind featureTyping) (ordinal 0)) (authored-target "collection: ScalarQuantityValue[0..*]") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "QuantityCalculations::+::"))) (target (node (document "d0") (qualified-name "QuantityCalculations::ScalarQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "QuantityCalculations::+::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "QuantityCalculations::-::x"))) (target (node (document "d0") (qualified-name "QuantityCalculations::ScalarQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "QuantityCalculations::-::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "QuantityCalculations::product::"))) (target (node (document "d0") (qualified-name "QuantityCalculations::ScalarQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "QuantityCalculations::product::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "QuantityCalculations::sum::"))) (target (node (document "d0") (qualified-name "QuantityCalculations::ScalarQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "QuantityCalculations::sum::"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "QuantityCalculations::[")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "QuantityCalculations::product")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "QuantityCalculations::sum")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 6 16) (end 6 28)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "QuantityCalculations::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 6 16) (end 6 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 19) (end 7 50)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "QuantityCalculations::ScalarQuantityValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::ScalarQuantityValue")
        (range (start 7 19) (end 7 50))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 19) (end 9 59)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "QuantityCalculations::DimensionOneValue"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::DimensionOneValue")
        (range (start 9 19) (end 9 59))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 19) (end 8 68)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "QuantityCalculations::ScalarMeasurementReference"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::ScalarMeasurementReference")
        (range (start 8 19) (end 8 68))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
