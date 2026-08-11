# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/IntegerFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package IntegerFunctions {
	doc
	/*
	 * This package defines functions on Integer values, including concrete specializations of the 
	 * general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
	
	function abs specializes RationalFunctions::abs { in x: Integer[1]; return : Natural[1]; }
	
	function '+' specializes RationalFunctions::'+' { in x: Integer[1]; in y: Integer[0..1]; return : Integer[1]; }
	function '-' specializes RationalFunctions::'-' { in x: Integer[1]; in y: Integer[0..1]; return : Integer[1]; }
	function '*' specializes RationalFunctions::'*' { in x: Integer[1]; in y: Integer[1]; return : Integer[1]; }
	function '/' specializes RationalFunctions::'/' { in x: Integer[1]; in y: Integer[1]; return : Rational[1]; }
	function '**' specializes RationalFunctions::'**' { in x: Integer[1]; in y: Natural[1]; return : Integer[1]; }
	function '^' specializes RationalFunctions::'^' { in x: Integer[1]; in y: Natural[1]; return : Integer[1]; }
	function '%' specializes NumericalFunctions::'%' { in x: Integer[1]; in y: Integer[1]; return : Integer[1]; }
	
	function '<' specializes RationalFunctions::'<' { in x: Integer[1]; in y: Integer[1]; return : Boolean[1]; }
	function '>' specializes RationalFunctions::'>' { in x: Integer[1]; in y: Integer[1]; return : Boolean[1]; }
	function '<=' specializes RationalFunctions::'<=' { in x: Integer[1]; in y: Integer[1]; return : Boolean[1]; }
	function '>=' specializes RationalFunctions::'>=' { in x: Integer[1]; in y: Integer[1]; return : Boolean[1]; }

	function max specializes RationalFunctions::max { in x: Integer[1]; in y: Integer[1]; return : Integer[1]; }
	function min specializes RationalFunctions::min { in x: Integer[1]; in y: Integer[1]; return : Integer[1]; }

	function '==' specializes DataFunctions::'==' { in x: Integer[0..1]; in y: Integer[0..1]; return : Boolean[1]; }
	
	function '..' specializes ScalarFunctions::'..' { in lower: Integer[1]; in upper: Integer[1]; return : Integer[0..*]; }
	
	function ToString specializes RationalFunctions::ToString { in x: Integer[1]; return : String[1]; }
	function ToNatural { in x: Integer[1]; return : Natural[1]; }
	function ToInteger { in x: String[1]; return : Integer[1]; }
	
	function sum specializes RationalFunctions::sum { in collection: Integer[0..*]; 
		return : Integer[1] default NumericalFunctions::sum0(collection, 0);
	}
	
	function product specializes RationalFunctions::product { in collection: Integer[0..*];
		return : Integer[1] default NumericalFunctions::product1(collection, 1);
	}
}	
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "integer_functions.md"
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d2a3388ebc05faa3b5e4d38af970d9ec4e3018ead2ce5c681973bd4a580de788") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "IntegerFunctions"))) (kind "package") (name "IntegerFunctions") (declared-name "IntegerFunctions"))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "IntegerFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::ToInteger"))) (kind "kermlDecl") (name "ToInteger") (declared-name "ToInteger") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::ToNatural"))) (kind "kermlDecl") (name "ToNatural") (declared-name "ToNatural") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::ToString"))) (kind "kermlDecl") (name "ToString") (declared-name "ToString") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::abs"))) (kind "kermlDecl") (name "abs") (declared-name "abs") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl10"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl11"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl12"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl6"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl7"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl8"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl9"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::max"))) (kind "kermlDecl") (name "max") (declared-name "max") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::min"))) (kind "kermlDecl") (name "min") (declared-name "min") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::product"))) (kind "kermlDecl") (name "product") (declared-name "product") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::sum"))) (kind "kermlDecl") (name "sum") (declared-name "sum") (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "IntegerFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
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
        (source (document "d0") (qualified-name "IntegerFunctions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 7 15) (end 7 27))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
