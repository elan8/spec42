# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/BooleanFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package BooleanFunctions {
	doc
	/*
	 * This package defines functions on Boolean values, including those corresponding to 
	 * (non-conditional) logical operators in the KerML expression notation.
	 */

	public import ScalarValues::*;
	
	function 'not' specializes ScalarFunctions::'not' { in x: Boolean[1]; return : Boolean[1]; }
	function 'xor' specializes ScalarFunctions::'xor' { in x: Boolean[1]; in y: Boolean[1]; return : Boolean[1]; }
	
	function '|' specializes ScalarFunctions::'|' { in x: Boolean[1]; in y: Boolean[1]; return : Boolean[1]; }
	function '&' specializes ScalarFunctions::'&' { in x: Boolean[1]; in y: Boolean[1]; return : Boolean[1]; }
	
	function '==' specializes DataFunctions::'==' { in x: Boolean[0..1]; in y: Boolean[0..1]; return : Boolean[1]; }
	
	function ToString specializes BaseFunctions::ToString { in x: Boolean[1]; return : String[1]; }
	function ToBoolean { in x: String[1]; return : Boolean[1]; }
	
}
	
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "boolean_functions.md"
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
standard library package BooleanFunctions {
    doc
    /*
	 * This package defines functions on Boolean values, including those corresponding to 
	 * (non-conditional) logical operators in the KerML expression notation.
	 */

    public import ScalarValues::*;

    function 'not' specializes ScalarFunctions::'not' { in x: Boolean[1]; return : Boolean[1]; }
    function 'xor' specializes ScalarFunctions::'xor' { in x: Boolean[1]; in y: Boolean[1]; return : Boolean[1]; }

    function '|' specializes ScalarFunctions::'|' { in x: Boolean[1]; in y: Boolean[1]; return : Boolean[1]; }
    function '&' specializes ScalarFunctions::'&' { in x: Boolean[1]; in y: Boolean[1]; return : Boolean[1]; }

    function '==' specializes DataFunctions::'==' { in x: Boolean[0..1]; in y: Boolean[0..1]; return : Boolean[1]; }

    function ToString specializes BaseFunctions::ToString { in x: Boolean[1]; return : String[1]; }
    function ToBoolean { in x: String[1]; return : Boolean[1]; }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "58085545663afc81fb5c4f4e6241e1d665e525f4b721f9c9114e2bc4d02c6437") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "BooleanFunctions"))) (kind "package") (name "BooleanFunctions") (declared-name "BooleanFunctions"))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "BooleanFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::ToBoolean"))) (kind "kermlDecl") (name "ToBoolean") (declared-name "ToBoolean") (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::ToString"))) (kind "kermlDecl") (name "ToString") (declared-name "ToString") (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "BooleanFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
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
        (source (document "d0") (qualified-name "BooleanFunctions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 7 15) (end 7 27))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
