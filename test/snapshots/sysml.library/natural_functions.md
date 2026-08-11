# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/NaturalFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package NaturalFunctions {
	doc
	/*
	 * This package defines functions on Natural values, including concrete specialization of the 
	 * general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
	
	function '+' specializes IntegerFunctions::'+' { in x: Natural[1]; in y: Natural[0..1]; return : Natural[1]; }
	function '*' specializes IntegerFunctions::'*' { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }
	function '/' specializes IntegerFunctions::'/' { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }
	function '%' specializes IntegerFunctions::'%' { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }
	
	function '<' specializes IntegerFunctions::'<' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }
	function '>' specializes IntegerFunctions::'>' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }
	function '<=' specializes IntegerFunctions::'<=' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }
	function '>=' specializes IntegerFunctions::'>=' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }	

	function max specializes IntegerFunctions::max { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }
	function min specializes IntegerFunctions::min { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }

	function '==' specializes IntegerFunctions::'==' { in x: Natural[0..1]; in y: Natural[0..1]; return : Boolean[1]; }
	
	function ToString specializes IntegerFunctions::ToString { in x: Natural[1]; return : String[1]; }
	function ToNatural{ in x: String[1]; return : Natural[1]; }
}	
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "natural_functions.md"
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
standard library package NaturalFunctions {
    doc
    /*
	 * This package defines functions on Natural values, including concrete specialization of the 
	 * general arithmetic and comparison operations.
	 */

    public import ScalarValues::*;

    function '+' specializes IntegerFunctions::'+' { in x: Natural[1]; in y: Natural[0..1]; return : Natural[1]; }
    function '*' specializes IntegerFunctions::'*' { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }
    function '/' specializes IntegerFunctions::'/' { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }
    function '%' specializes IntegerFunctions::'%' { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }

    function '<' specializes IntegerFunctions::'<' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }
    function '>' specializes IntegerFunctions::'>' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }
    function '<=' specializes IntegerFunctions::'<=' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }
    function '>=' specializes IntegerFunctions::'>=' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }

    function max specializes IntegerFunctions::max { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }
    function min specializes IntegerFunctions::min { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }

    function '==' specializes IntegerFunctions::'==' { in x: Natural[0..1]; in y: Natural[0..1]; return : Boolean[1]; }

    function ToString specializes IntegerFunctions::ToString { in x: Natural[1]; return : String[1]; }
    function ToNatural{ in x: String[1]; return : Natural[1]; }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "71f3271616f231a82596d0aac46e7514f8f8697810ee32b2cf5e8e26475b011c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "NaturalFunctions"))) (kind "package") (name "NaturalFunctions") (declared-name "NaturalFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 1622))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 15)) (end (line 7) (character 27))))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::ToNatural"))) (kind "kermlDecl") (name "ToNatural") (declared-name "ToNatural") (range (start (line 25) (character 1)) (end (line 25) (character 60))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::ToString"))) (kind "kermlDecl") (name "ToString") (declared-name "ToString") (range (start (line 24) (character 1)) (end (line 24) (character 99))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1622))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 9) (character 1)) (end (line 9) (character 111))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 10) (character 1)) (end (line 10) (character 108))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 11) (character 1)) (end (line 11) (character 108))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 12) (character 1)) (end (line 12) (character 108))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 14) (character 1)) (end (line 14) (character 108))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 15) (character 1)) (end (line 15) (character 108))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl6"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 16) (character 1)) (end (line 16) (character 110))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl7"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 17) (character 1)) (end (line 17) (character 110))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl8"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 22) (character 1)) (end (line 22) (character 116))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::max"))) (kind "kermlDecl") (name "max") (declared-name "max") (range (start (line 19) (character 1)) (end (line 19) (character 108))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NaturalFunctions::min"))) (kind "kermlDecl") (name "min") (declared-name "min") (range (start (line 20) (character 1)) (end (line 20) (character 108))) (parent (node (document "d0") (qualified-name "NaturalFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "NaturalFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 7) (character 15)) (end (line 7) (character 27))) (outcome (status unresolved)))
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
        (source (document "d0") (qualified-name "NaturalFunctions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 7 15) (end 7 27))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
