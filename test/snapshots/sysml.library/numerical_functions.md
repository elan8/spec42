# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/NumericalFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package NumericalFunctions {
	doc
	/*
	 * This package defines abstract functions on Numerical values for general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
	private import ControlFunctions::reduce;
	
	abstract function isZero{ in x: NumericalValue[1]; return : Boolean; }
	abstract function isUnit{ in x : NumericalValue[1]; return : Boolean; }
	
	abstract function abs{ in x: NumericalValue[1]; return : NumericalValue[1]; }
		
	abstract function '+' specializes ScalarFunctions::'+' { in x: NumericalValue[1]; in y: NumericalValue[0..1]; return : NumericalValue[1]; }
	abstract function '-' specializes ScalarFunctions::'-' { in x: NumericalValue[1]; in y: NumericalValue[0..1]; return : NumericalValue[1]; }
	abstract function '*' specializes ScalarFunctions::'*' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '/' specializes ScalarFunctions::'/' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '**' specializes ScalarFunctions::'**' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '^' specializes ScalarFunctions::'^' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '%' specializes ScalarFunctions::'%' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	
	abstract function '<' specializes ScalarFunctions::'<' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	abstract function '>' specializes ScalarFunctions::'>' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	abstract function '<=' specializes ScalarFunctions::'<=' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	abstract function '>=' specializes ScalarFunctions::'>=' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	
	abstract function max specializes ScalarFunctions::max { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function min specializes ScalarFunctions::min { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	
	abstract function sum { in collection: ScalarValue[0..*]; return : ScalarValue[1]; }	
	abstract function product { in collection: ScalarValue[0..*]; return : ScalarValue[1]; }
	
	function sum0 { in collection: NumericalValue[0..*]; in zero: ScalarValue[1]; 
 		inv { isZero(zero) }		
        return : ScalarValue = collection->reduce '+' ?? zero;
	}
	
	function product1 { in collection: ScalarValue[0..*]; in one: ScalarValue[1]; 
		inv { isUnit(one) }		
        return : ScalarValue = collection->reduce '*' ?? one;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "numerical_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 15) (end 6 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 40))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package NumericalFunctions {
	doc
	/*
	 * This package defines abstract functions on Numerical values for general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
	private import ControlFunctions::reduce;
	
	abstract function isZero{ in x: NumericalValue[1]; return : Boolean; }
	abstract function isUnit{ in x : NumericalValue[1]; return : Boolean; }
	
	abstract function abs{ in x: NumericalValue[1]; return : NumericalValue[1]; }
		
	abstract function '+' specializes ScalarFunctions::'+' { in x: NumericalValue[1]; in y: NumericalValue[0..1]; return : NumericalValue[1]; }
	abstract function '-' specializes ScalarFunctions::'-' { in x: NumericalValue[1]; in y: NumericalValue[0..1]; return : NumericalValue[1]; }
	abstract function '*' specializes ScalarFunctions::'*' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '/' specializes ScalarFunctions::'/' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '**' specializes ScalarFunctions::'**' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '^' specializes ScalarFunctions::'^' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '%' specializes ScalarFunctions::'%' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	
	abstract function '<' specializes ScalarFunctions::'<' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	abstract function '>' specializes ScalarFunctions::'>' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	abstract function '<=' specializes ScalarFunctions::'<=' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	abstract function '>=' specializes ScalarFunctions::'>=' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	
	abstract function max specializes ScalarFunctions::max { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function min specializes ScalarFunctions::min { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	
	abstract function sum { in collection: ScalarValue[0..*]; return : ScalarValue[1]; }	
	abstract function product { in collection: ScalarValue[0..*]; return : ScalarValue[1]; }
	
	function sum0 { in collection: NumericalValue[0..*]; in zero: ScalarValue[1]; 
 		inv { isZero(zero) }		
        return : ScalarValue = collection->reduce '+' ?? zero;
	}
	
	function product1 { in collection: ScalarValue[0..*]; in one: ScalarValue[1]; 
		inv { isUnit(one) }		
        return : ScalarValue = collection->reduce '*' ?? one;
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e11446ca51e09c518389d0e7da9390ff28d0c1a0c659c45c475d5183cb194d43") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "NumericalFunctions"))) (kind "package") (name "NumericalFunctions") (declared-name "NumericalFunctions"))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "NumericalFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::abs"))) (kind "kermlDecl") (name "abs") (declared-name "abs") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl10"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl6"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl7"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl8"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl9"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::isUnit"))) (kind "kermlDecl") (name "isUnit") (declared-name "isUnit") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::isZero"))) (kind "kermlDecl") (name "isZero") (declared-name "isZero") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::max"))) (kind "kermlDecl") (name "max") (declared-name "max") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::min"))) (kind "kermlDecl") (name "min") (declared-name "min") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::product"))) (kind "kermlDecl") (name "product") (declared-name "product") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::product1"))) (kind "kermlDecl") (name "product1") (declared-name "product1") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::reduce"))) (kind "import") (name "reduce") (declared-name "reduce") (parent (node (document "d0") (qualified-name "NumericalFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::reduce") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::sum"))) (kind "kermlDecl") (name "sum") (declared-name "sum") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::sum0"))) (kind "kermlDecl") (name "sum0") (declared-name "sum0") (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "NumericalFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "NumericalFunctions::reduce"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::reduce") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 6 15) (end 6 27)) (probe (position 6 15))
      (reference
        (source (document "d0") (qualified-name "NumericalFunctions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 6 15) (end 6 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 40)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "NumericalFunctions::reduce"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::reduce")
        (range (start 7 16) (end 7 40))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
