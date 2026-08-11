# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/ScalarFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package ScalarFunctions {
	doc
	/*
	 * This package defines abstract functions that specialize the DataFunctions for use with ScalarValues. 
	 */

	public import ScalarValues::*;
	
	abstract function '+' specializes DataFunctions::'+' { in x: ScalarValue[1]; in y: ScalarValue[0..1]; return : ScalarValue[1]; }
	abstract function '-' specializes DataFunctions::'-' { in x: ScalarValue[1]; in y: ScalarValue[0..1]; return : ScalarValue[1]; }
	abstract function '*' specializes DataFunctions::'*' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	abstract function '/' specializes DataFunctions::'/' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	abstract function '**' specializes DataFunctions::'**' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	abstract function '^' specializes DataFunctions::'^' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	abstract function '%' specializes DataFunctions::'%' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	
	abstract function 'not' specializes DataFunctions::'not' { in x: ScalarValue[1]; return : ScalarValue[1]; }
	abstract function 'xor' specializes DataFunctions::'xor' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }

	abstract function '~' specializes DataFunctions::'~' { in x: ScalarValue[1]; return : ScalarValue[1]; }	
	abstract function '|' specializes DataFunctions::'|' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	abstract function '&' specializes DataFunctions::'&' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	
	abstract function '<' specializes DataFunctions::'<' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : Boolean[1]; }
	abstract function '>' specializes DataFunctions::'>' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : Boolean[1]; }
	abstract function '<=' specializes DataFunctions::'<=' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : Boolean[1]; }
	abstract function '>=' specializes DataFunctions::'>=' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : Boolean[1]; }
	
	abstract function max specializes DataFunctions::max { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	abstract function min specializes DataFunctions::min { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	
	abstract function '..' specializes DataFunctions::'..' { in lower: ScalarValue[1]; in upper: ScalarValue[1]; return : ScalarValue[0..*]; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "scalar_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 15) (end 6 27))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a41f20a0bba86eac8608372066d24fd03fe9b55fbe6ff3f5cc671c3b653f48bc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ScalarFunctions"))) (kind "package") (name "ScalarFunctions") (declared-name "ScalarFunctions"))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ScalarFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl10"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl11"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl12"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl13"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl14"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl15"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl16"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl6"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl7"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl8"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::function#kermlDecl9"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::max"))) (kind "kermlDecl") (name "max") (declared-name "max") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
    (element (id (node (document "d0") (qualified-name "ScalarFunctions::min"))) (kind "kermlDecl") (name "min") (declared-name "min") (parent (node (document "d0") (qualified-name "ScalarFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ScalarFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
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
        (source (document "d0") (qualified-name "ScalarFunctions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 6 15) (end 6 27))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
