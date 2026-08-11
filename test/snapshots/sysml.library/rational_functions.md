# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/RationalFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package RationalFunctions {
	doc
	/*
	 * This package defines Functions on Rational values, including concrete specializations of the 
	 * general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
	
	function rat { in numer: Integer[1]; in denum: Integer[1]; return : Rational[1]; }
	function numer { in rat: Rational[1]; return : Integer[1]; }
	function denom { in rat: Rational[1]; return : Integer[1]; }
	
	function abs specializes RealFunctions::abs { in x: Rational[1]; return : Rational[1]; }

	function '+' specializes RealFunctions::'+' { in x: Rational[1]; in y: Rational[0..1]; return : Rational[1]; }
	function '-' specializes RealFunctions::'-' { in x: Rational[1]; in y: Rational[0..1]; return : Rational[1]; }
	function '*' specializes RealFunctions::'*' { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	function '/' specializes RealFunctions::'/' { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	function '**' specializes RealFunctions::'**' { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	function '^' specializes RealFunctions::'^' { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	
	function '<' specializes RealFunctions::'<' { in x: Rational[1]; in y: Rational[1]; return : Boolean[1]; }
	function '>' specializes RealFunctions::'>' { in x: Rational[1]; in y: Rational[1]; return : Boolean[1]; }
	function '<=' specializes RealFunctions::'<=' { in x: Rational[1]; in y: Rational[1]; return : Boolean[1]; }
	function '>=' specializes RealFunctions::'>=' { in x: Rational[1]; in y: Rational[1]; return : Boolean[1]; }

	function max specializes RealFunctions::max { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	function min specializes RealFunctions::min { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }

	function '==' specializes RealFunctions::'==' { in x: Rational[0..1]; in y: Rational[0..1]; return : Boolean[1]; }
	
	function gcd{ in x: Rational[1]; in y: Rational[1]; return : Integer[1]; }
		
	function floor specializes RealFunctions::floor { in x: Rational[1]; return : Integer[1]; }
	function round specializes RealFunctions::round { in x: Rational[1]; return : Integer[1]; }
	
	function ToString specializes RealFunctions::ToString { in x: Rational[1]; return : String[1]; }
	function ToInteger{ in x: Rational[1]; return : Integer[1]; }
	function ToRational{ in x: String[1]; return : Rational[1]; }
	
	function sum specializes RealFunctions::sum { in collection: Rational[0..*];
		return : Rational[1] default NumericalFunctions::sum0(collection, rat(0, 1));
	}
	
	function product specializes RealFunctions::product { in collection: Rational[0..*];
		return : Rational[1] default NumericalFunctions::product1(collection, rat(1, 1));
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "rational_functions.md"
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c506d9cab67d66037728d22fb1812aed6fe7852569165406063d70fa3d06738c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RationalFunctions"))) (kind "package") (name "RationalFunctions") (declared-name "RationalFunctions"))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RationalFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::ToInteger"))) (kind "kermlDecl") (name "ToInteger") (declared-name "ToInteger") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::ToRational"))) (kind "kermlDecl") (name "ToRational") (declared-name "ToRational") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::ToString"))) (kind "kermlDecl") (name "ToString") (declared-name "ToString") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::abs"))) (kind "kermlDecl") (name "abs") (declared-name "abs") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::denom"))) (kind "kermlDecl") (name "denom") (declared-name "denom") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::floor"))) (kind "kermlDecl") (name "floor") (declared-name "floor") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl10"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl6"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl7"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl8"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl9"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::gcd"))) (kind "kermlDecl") (name "gcd") (declared-name "gcd") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::max"))) (kind "kermlDecl") (name "max") (declared-name "max") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::min"))) (kind "kermlDecl") (name "min") (declared-name "min") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::numer"))) (kind "kermlDecl") (name "numer") (declared-name "numer") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::product"))) (kind "kermlDecl") (name "product") (declared-name "product") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::rat"))) (kind "kermlDecl") (name "rat") (declared-name "rat") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::round"))) (kind "kermlDecl") (name "round") (declared-name "round") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::sum"))) (kind "kermlDecl") (name "sum") (declared-name "sum") (parent (node (document "d0") (qualified-name "RationalFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RationalFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
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
        (source (document "d0") (qualified-name "RationalFunctions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 7 15) (end 7 27))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
