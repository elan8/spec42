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
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d2a3388ebc05faa3b5e4d38af970d9ec4e3018ead2ce5c681973bd4a580de788") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "IntegerFunctions"))) (kind "package") (name "IntegerFunctions") (declared-name "IntegerFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 2575))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 15)) (end (line 7) (character 27))))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::ToInteger"))) (kind "kermlDecl") (name "ToInteger") (declared-name "ToInteger") (range (start (line 33) (character 1)) (end (line 33) (character 61))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::ToNatural"))) (kind "kermlDecl") (name "ToNatural") (declared-name "ToNatural") (range (start (line 32) (character 1)) (end (line 32) (character 62))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::ToString"))) (kind "kermlDecl") (name "ToString") (declared-name "ToString") (range (start (line 31) (character 1)) (end (line 31) (character 100))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2575))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::abs"))) (kind "kermlDecl") (name "abs") (declared-name "abs") (range (start (line 9) (character 1)) (end (line 9) (character 91))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 11) (character 1)) (end (line 11) (character 112))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 12) (character 1)) (end (line 12) (character 112))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl10"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 22) (character 1)) (end (line 22) (character 111))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl11"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 27) (character 1)) (end (line 27) (character 113))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl12"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 29) (character 1)) (end (line 29) (character 120))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 13) (character 1)) (end (line 13) (character 109))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 14) (character 1)) (end (line 14) (character 110))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 15) (character 1)) (end (line 15) (character 111))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 16) (character 1)) (end (line 16) (character 109))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl6"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 17) (character 1)) (end (line 17) (character 110))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl7"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 19) (character 1)) (end (line 19) (character 109))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl8"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 20) (character 1)) (end (line 20) (character 109))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl9"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 21) (character 1)) (end (line 21) (character 111))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::max"))) (kind "kermlDecl") (name "max") (declared-name "max") (range (start (line 24) (character 1)) (end (line 24) (character 109))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::min"))) (kind "kermlDecl") (name "min") (declared-name "min") (range (start (line 25) (character 1)) (end (line 25) (character 109))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::product"))) (kind "kermlDecl") (name "product") (declared-name "product") (range (start (line 39) (character 1)) (end (line 39) (character 166))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
    (element (id (node (document "d0") (qualified-name "IntegerFunctions::sum"))) (kind "kermlDecl") (name "sum") (declared-name "sum") (range (start (line 35) (character 1)) (end (line 35) (character 155))) (parent (node (document "d0") (qualified-name "IntegerFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "IntegerFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 7) (character 15)) (end (line 7) (character 27))) (outcome (status unresolved)))
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
