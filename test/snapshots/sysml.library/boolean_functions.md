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
    (element (id (node (document "d0") (qualified-name "BooleanFunctions"))) (kind "package") (name "BooleanFunctions") (declared-name "BooleanFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 959))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "BooleanFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 15)) (end (line 7) (character 27))))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::ToBoolean"))) (kind "kermlDecl") (name "ToBoolean") (declared-name "ToBoolean") (range (start (line 18) (character 1)) (end (line 18) (character 61))) (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::ToString"))) (kind "kermlDecl") (name "ToString") (declared-name "ToString") (range (start (line 17) (character 1)) (end (line 17) (character 96))) (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 959))) (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 9) (character 1)) (end (line 9) (character 93))) (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 10) (character 1)) (end (line 10) (character 111))) (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 12) (character 1)) (end (line 12) (character 107))) (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 13) (character 1)) (end (line 13) (character 107))) (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
    (element (id (node (document "d0") (qualified-name "BooleanFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 15) (character 1)) (end (line 15) (character 113))) (parent (node (document "d0") (qualified-name "BooleanFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "BooleanFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 7) (character 15)) (end (line 7) (character 27))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
