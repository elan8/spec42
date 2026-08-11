# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/RealFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package RealFunctions {
	doc
	/*
	 * This package defines Functions on Real values, including concrete specializations of the 
	 * general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
	
	function re :> ComplexFunctions::re{ in x: Real[1]; 
        return : Real[1] = x;
	}
	function im :> ComplexFunctions::im{ in x: Real[1]; 
        return : Real[1] = 0.0;
	}
	
	function abs specializes ComplexFunctions::abs { in x: Real[1]; return : Real[1]; }
	function arg specializes ComplexFunctions::arg { in x: Real[1]; 
        return : Real[1] = 0.0;
	}

	function '+' specializes ComplexFunctions::'+' { in x: Real[1]; in y: Real[0..1]; return : Real[1]; }
	function '-' specializes ComplexFunctions::'-' { in x: Real[1]; in y: Real[0..1]; return : Real[1]; }
	function '*' specializes ComplexFunctions::'*' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	function '/' specializes ComplexFunctions::'/' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	function '**' specializes ComplexFunctions::'**' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	function '^' specializes ComplexFunctions::'^' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	
	function '<' specializes NumericalFunctions::'<' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }
	function '>' specializes NumericalFunctions::'>' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }
	function '<=' specializes NumericalFunctions::'<=' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }
	function '>=' specializes NumericalFunctions::'>=' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }

	function max specializes NumericalFunctions::max { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	function min specializes NumericalFunctions::min { in x: Real[1]; in y: Real[1]; return : Real[1]; }

	function '==' specializes ComplexFunctions::'==' { in x: Real[0..1]; in y: Real[0..1]; return : Boolean[1]; }
			
	function sqrt{ in x: Real[1]; return : Real[1]; }

	function floor{ in x: Real[1]; return : Integer[1]; }
	function round{ in x: Real[1]; return : Integer[1]; }
	
	function ToString specializes ComplexFunctions::ToString { in x: Real[1]; return : String[1]; }
	function ToInteger{ in x: Real[1]; return : Integer[1]; }
	function ToRational{ in x: Real[1]; return : Rational[1]; }
	function ToReal{ in x: String[1]; return : Real[1]; }
	
	function sum specializes ComplexFunctions::sum { in collection: Real[0..*]; 
        return : Real default NumericalFunctions::sum0(collection, 0.0);
	}
	
	function product specializes ComplexFunctions::product { in collection: Real[0..*]; 
        return : Real default NumericalFunctions::product1(collection, 1.0);
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "real_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 15) (end 7 27))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package RealFunctions {
	doc
	/*
	 * This package defines Functions on Real values, including concrete specializations of the 
	 * general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
	
	function re :> ComplexFunctions::re{ in x: Real[1]; 
        return : Real[1] = x;
	}
	function im :> ComplexFunctions::im{ in x: Real[1]; 
        return : Real[1] = 0.0;
	}
	
	function abs specializes ComplexFunctions::abs { in x: Real[1]; return : Real[1]; }
	function arg specializes ComplexFunctions::arg { in x: Real[1]; 
        return : Real[1] = 0.0;
	}

	function '+' specializes ComplexFunctions::'+' { in x: Real[1]; in y: Real[0..1]; return : Real[1]; }
	function '-' specializes ComplexFunctions::'-' { in x: Real[1]; in y: Real[0..1]; return : Real[1]; }
	function '*' specializes ComplexFunctions::'*' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	function '/' specializes ComplexFunctions::'/' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	function '**' specializes ComplexFunctions::'**' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	function '^' specializes ComplexFunctions::'^' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	
	function '<' specializes NumericalFunctions::'<' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }
	function '>' specializes NumericalFunctions::'>' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }
	function '<=' specializes NumericalFunctions::'<=' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }
	function '>=' specializes NumericalFunctions::'>=' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }

	function max specializes NumericalFunctions::max { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	function min specializes NumericalFunctions::min { in x: Real[1]; in y: Real[1]; return : Real[1]; }

	function '==' specializes ComplexFunctions::'==' { in x: Real[0..1]; in y: Real[0..1]; return : Boolean[1]; }
			
	function sqrt{ in x: Real[1]; return : Real[1]; }

	function floor{ in x: Real[1]; return : Integer[1]; }
	function round{ in x: Real[1]; return : Integer[1]; }
	
	function ToString specializes ComplexFunctions::ToString { in x: Real[1]; return : String[1]; }
	function ToInteger{ in x: Real[1]; return : Integer[1]; }
	function ToRational{ in x: Real[1]; return : Rational[1]; }
	function ToReal{ in x: String[1]; return : Real[1]; }
	
	function sum specializes ComplexFunctions::sum { in collection: Real[0..*]; 
        return : Real default NumericalFunctions::sum0(collection, 0.0);
	}
	
	function product specializes ComplexFunctions::product { in collection: Real[0..*]; 
        return : Real default NumericalFunctions::product1(collection, 1.0);
	}	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "bab8cc8658308459d8e5eb30b1265d6c9cbe636bb68faca916ca1cf78d3945a7") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RealFunctions"))) (kind "package") (name "RealFunctions") (declared-name "RealFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 2716))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "RealFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 15)) (end (line 7) (character 27))))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::ToInteger"))) (kind "kermlDecl") (name "ToInteger") (declared-name "ToInteger") (range (start (line 44) (character 1)) (end (line 44) (character 58))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::ToRational"))) (kind "kermlDecl") (name "ToRational") (declared-name "ToRational") (range (start (line 45) (character 1)) (end (line 45) (character 60))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::ToReal"))) (kind "kermlDecl") (name "ToReal") (declared-name "ToReal") (range (start (line 46) (character 1)) (end (line 46) (character 54))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::ToString"))) (kind "kermlDecl") (name "ToString") (declared-name "ToString") (range (start (line 43) (character 1)) (end (line 43) (character 96))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2716))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::abs"))) (kind "kermlDecl") (name "abs") (declared-name "abs") (range (start (line 16) (character 1)) (end (line 16) (character 84))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::arg"))) (kind "kermlDecl") (name "arg") (declared-name "arg") (range (start (line 17) (character 1)) (end (line 17) (character 100))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::floor"))) (kind "kermlDecl") (name "floor") (declared-name "floor") (range (start (line 40) (character 1)) (end (line 40) (character 54))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 21) (character 1)) (end (line 21) (character 102))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 22) (character 1)) (end (line 22) (character 102))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl10"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 36) (character 1)) (end (line 36) (character 110))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 23) (character 1)) (end (line 23) (character 99))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 24) (character 1)) (end (line 24) (character 99))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 25) (character 1)) (end (line 25) (character 101))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 26) (character 1)) (end (line 26) (character 99))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl6"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 28) (character 1)) (end (line 28) (character 104))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl7"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 29) (character 1)) (end (line 29) (character 104))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl8"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 30) (character 1)) (end (line 30) (character 106))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl9"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 31) (character 1)) (end (line 31) (character 106))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::im"))) (kind "kermlDecl") (name "im") (declared-name "im") (range (start (line 12) (character 1)) (end (line 12) (character 88))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::max"))) (kind "kermlDecl") (name "max") (declared-name "max") (range (start (line 33) (character 1)) (end (line 33) (character 101))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::min"))) (kind "kermlDecl") (name "min") (declared-name "min") (range (start (line 34) (character 1)) (end (line 34) (character 101))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::product"))) (kind "kermlDecl") (name "product") (declared-name "product") (range (start (line 52) (character 1)) (end (line 52) (character 165))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::re"))) (kind "kermlDecl") (name "re") (declared-name "re") (range (start (line 9) (character 1)) (end (line 9) (character 86))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::round"))) (kind "kermlDecl") (name "round") (declared-name "round") (range (start (line 41) (character 1)) (end (line 41) (character 54))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::sqrt"))) (kind "kermlDecl") (name "sqrt") (declared-name "sqrt") (range (start (line 38) (character 1)) (end (line 38) (character 50))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::sum"))) (kind "kermlDecl") (name "sum") (declared-name "sum") (range (start (line 48) (character 1)) (end (line 48) (character 153))) (parent (node (document "d0") (qualified-name "RealFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RealFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 7) (character 15)) (end (line 7) (character 27))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
