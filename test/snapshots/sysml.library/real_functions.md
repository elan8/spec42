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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "bab8cc8658308459d8e5eb30b1265d6c9cbe636bb68faca916ca1cf78d3945a7") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RealFunctions"))) (kind "package") (name "RealFunctions") (declared-name "RealFunctions"))
    (element (id (node (document "d0") (qualified-name "RealFunctions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RealFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::ToInteger"))) (kind "kermlDecl") (name "ToInteger") (declared-name "ToInteger") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::ToRational"))) (kind "kermlDecl") (name "ToRational") (declared-name "ToRational") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::ToReal"))) (kind "kermlDecl") (name "ToReal") (declared-name "ToReal") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::ToString"))) (kind "kermlDecl") (name "ToString") (declared-name "ToString") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::abs"))) (kind "kermlDecl") (name "abs") (declared-name "abs") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::arg"))) (kind "kermlDecl") (name "arg") (declared-name "arg") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::floor"))) (kind "kermlDecl") (name "floor") (declared-name "floor") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl10"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl6"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl7"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl8"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::function#kermlDecl9"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::im"))) (kind "kermlDecl") (name "im") (declared-name "im") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::max"))) (kind "kermlDecl") (name "max") (declared-name "max") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::min"))) (kind "kermlDecl") (name "min") (declared-name "min") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::product"))) (kind "kermlDecl") (name "product") (declared-name "product") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::re"))) (kind "kermlDecl") (name "re") (declared-name "re") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::round"))) (kind "kermlDecl") (name "round") (declared-name "round") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::sqrt"))) (kind "kermlDecl") (name "sqrt") (declared-name "sqrt") (parent (node (document "d0") (qualified-name "RealFunctions"))))
    (element (id (node (document "d0") (qualified-name "RealFunctions::sum"))) (kind "kermlDecl") (name "sum") (declared-name "sum") (parent (node (document "d0") (qualified-name "RealFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RealFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
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
  (document "d0"
    (query (range (start 7 15) (end 7 27)) (probe (position 7 15))
      (reference
        (source (document "d0") (qualified-name "RealFunctions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 7 15) (end 7 27))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
