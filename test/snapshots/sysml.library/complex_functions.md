# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/ComplexFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package ComplexFunctions {
	doc
	/*
	 * This package defines functions on Complex values, including concrete specializations of the 
	 * general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
		
	feature i: Complex[1] = rect(0.0, 1.0);
	
	function rect { in re: Real[1]; in im: Real[1]; return : Complex[1]; }
	function polar { in abs: Real[1]; in arg: Real[1]; return : Complex[1]; }
	
	function re { in x: Complex[1]; return : Real[1]; }
	function im { in x: Complex[1]; return : Real[1]; }
	
	function isZero specializes NumericalFunctions::isZero { in x : Complex[1];
		return : Boolean[1] = re(x) == 0.0 and im(x) == 0.0;
	}
	function isUnit specializes NumericalFunctions::isUnit { in x : Complex[1];
		return : Boolean[1] = re(x) == 1.0 and im(x) == 0.0;
	}
	
	function abs specializes NumericalFunctions::abs { in x: Complex[1]; return : Real[1]; }
	function arg { in x: Complex[1]; return : Real[1]; }
	
	function '+' specializes NumericalFunctions::'+' { in x: Complex[1]; in y: Complex[0..1]; return : Complex[1]; }
	function '-' specializes NumericalFunctions::'-' { in x: Complex[1]; in y: Complex[0..1]; return : Complex[1]; }
	function '*' specializes NumericalFunctions::'*' { in x: Complex[1]; in y: Complex[1]; return : Complex[1]; }
	function '/' specializes NumericalFunctions::'/' { in x: Complex[1]; in y: Complex[1]; return : Complex[1]; }
	function '**' specializes NumericalFunctions::'**' { in x: Complex[1]; in y: Complex[1]; return : Complex[1]; }
	function '^' specializes NumericalFunctions::'^' { in x: Complex[1]; in y: Complex[1]; return : Complex[1]; }
	
	function '==' specializes DataFunctions::'==' { in x: Complex[0..1]; in y: Complex[0..1]; return : Boolean[1]; }
	
	function ToString specializes BaseFunctions::ToString { in x: Complex[1]; return : String[1]; }
	function ToComplex { in x: String[1]; return : Complex[1]; }
	
	function sum specializes NumericalFunctions::sum { in collection: Complex[0..*];
		return : Complex[1] default NumericalFunctions::sum0(collection, rect(0.0, 0.0));
	}
	
	function product specializes NumericalFunctions::product { in collection: Complex[0..*];
		return : Complex[1] default NumericalFunctions::product1(collection, rect(1.0, 0.0));
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "complex_functions.md"
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e09e2831ffb9d0edcd243add33f440640d6041e3c91db33246c988c6c30373ba") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ComplexFunctions"))) (kind "package") (name "ComplexFunctions") (declared-name "ComplexFunctions"))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ComplexFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::ToComplex"))) (kind "kermlDecl") (name "ToComplex") (declared-name "ToComplex") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::ToString"))) (kind "kermlDecl") (name "ToString") (declared-name "ToString") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::abs"))) (kind "kermlDecl") (name "abs") (declared-name "abs") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::arg"))) (kind "kermlDecl") (name "arg") (declared-name "arg") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl6"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::i"))) (kind "feature decl") (name "i") (declared-name "i") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::im"))) (kind "kermlDecl") (name "im") (declared-name "im") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::isUnit"))) (kind "kermlDecl") (name "isUnit") (declared-name "isUnit") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::isZero"))) (kind "kermlDecl") (name "isZero") (declared-name "isZero") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::polar"))) (kind "kermlDecl") (name "polar") (declared-name "polar") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::product"))) (kind "kermlDecl") (name "product") (declared-name "product") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::re"))) (kind "kermlDecl") (name "re") (declared-name "re") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::rect"))) (kind "kermlDecl") (name "rect") (declared-name "rect") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::sum"))) (kind "kermlDecl") (name "sum") (declared-name "sum") (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ComplexFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
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
        (source (document "d0") (qualified-name "ComplexFunctions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 7 15) (end 7 27))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
