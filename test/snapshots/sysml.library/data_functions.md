# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/DataFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package DataFunctions {
	doc
	/*
	 * This package defines the abstract base functions corresponding to all the unary and binary operators 
	 * in the KerML expression notation that might be defined on various kinds of DataValues.
	 */

	private import Base::DataValue;
	private import ScalarValues::Boolean;
	private import ControlFunctions::reduce;	
	
	abstract function '==' specializes BaseFunctions::'==' { in x: DataValue[0..1]; in y: DataValue[0..1]; 
		return : Boolean[1];
	}
	function '===' specializes BaseFunctions::'==='{ in x: DataValue[0..1]; in y: DataValue[0..1]; 
		return : Boolean[1] = x == y;
	}
	
	abstract function '+' { in x: DataValue[1]; in y: DataValue[0..1]; return : DataValue[1]; }
	abstract function '-' { in x: DataValue[1]; in y: DataValue[0..1]; return : DataValue[1]; }
	abstract function '*' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '/' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '**' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '^' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '%' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	
	abstract function 'not' { in x: DataValue[1]; return : DataValue[1]; }
	abstract function 'xor' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }

	abstract function '~' { in x: DataValue[1]; return : DataValue[1]; }
	abstract function '|' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '&' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	
	abstract function '<' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	abstract function '>' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	abstract function '<=' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	abstract function '>=' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	
	abstract function max { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function min { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	
	abstract function '..' { in lower: DataValue[1]; in upper: DataValue[1]; return : DataValue[0..*] ordered; }	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "data_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 40))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package DataFunctions {
	doc
	/*
	 * This package defines the abstract base functions corresponding to all the unary and binary operators 
	 * in the KerML expression notation that might be defined on various kinds of DataValues.
	 */

	private import Base::DataValue;
	private import ScalarValues::Boolean;
	private import ControlFunctions::reduce;	
	
	abstract function '==' specializes BaseFunctions::'==' { in x: DataValue[0..1]; in y: DataValue[0..1]; 
		return : Boolean[1];
	}
	function '===' specializes BaseFunctions::'==='{ in x: DataValue[0..1]; in y: DataValue[0..1]; 
		return : Boolean[1] = x == y;
	}
	
	abstract function '+' { in x: DataValue[1]; in y: DataValue[0..1]; return : DataValue[1]; }
	abstract function '-' { in x: DataValue[1]; in y: DataValue[0..1]; return : DataValue[1]; }
	abstract function '*' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '/' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '**' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '^' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '%' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	
	abstract function 'not' { in x: DataValue[1]; return : DataValue[1]; }
	abstract function 'xor' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }

	abstract function '~' { in x: DataValue[1]; return : DataValue[1]; }
	abstract function '|' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '&' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	
	abstract function '<' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	abstract function '>' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	abstract function '<=' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	abstract function '>=' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	
	abstract function max { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function min { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	
	abstract function '..' { in lower: DataValue[1]; in upper: DataValue[1]; return : DataValue[0..*] ordered; }	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ff9911b06ae7a78efebbc6e8c6b386614c56a10add04946989d7e35e8ccf314d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DataFunctions"))) (kind "package") (name "DataFunctions") (declared-name "DataFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 2341))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 8) (character 1)) (end (line 8) (character 38))) (parent (node (document "d0") (qualified-name "DataFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 37))))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::DataValue"))) (kind "import") (name "DataValue") (declared-name "DataValue") (range (start (line 7) (character 1)) (end (line 7) (character 32))) (parent (node (document "d0") (qualified-name "DataFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::DataValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 31))))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2341))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 11) (character 1)) (end (line 11) (character 130))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 14) (character 1)) (end (line 14) (character 131))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl10"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 27) (character 1)) (end (line 27) (character 91))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl11"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 29) (character 1)) (end (line 29) (character 69))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl12"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 30) (character 1)) (end (line 30) (character 89))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl13"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 31) (character 1)) (end (line 31) (character 89))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl14"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 33) (character 1)) (end (line 33) (character 87))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl15"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 34) (character 1)) (end (line 34) (character 87))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl16"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 35) (character 1)) (end (line 35) (character 88))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl17"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 36) (character 1)) (end (line 36) (character 88))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl18"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 41) (character 1)) (end (line 41) (character 109))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 18) (character 1)) (end (line 18) (character 92))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 19) (character 1)) (end (line 19) (character 92))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 20) (character 1)) (end (line 20) (character 89))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 21) (character 1)) (end (line 21) (character 89))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl6"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 22) (character 1)) (end (line 22) (character 90))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl7"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 23) (character 1)) (end (line 23) (character 89))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl8"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 24) (character 1)) (end (line 24) (character 89))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl9"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 26) (character 1)) (end (line 26) (character 71))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::max"))) (kind "kermlDecl") (name "max") (declared-name "max") (range (start (line 38) (character 1)) (end (line 38) (character 89))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::min"))) (kind "kermlDecl") (name "min") (declared-name "min") (range (start (line 39) (character 1)) (end (line 39) (character 89))) (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::reduce"))) (kind "import") (name "reduce") (declared-name "reduce") (range (start (line 9) (character 1)) (end (line 9) (character 41))) (parent (node (document "d0") (qualified-name "DataFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::reduce") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 40))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DataFunctions::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 8) (character 16)) (end (line 8) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DataFunctions::DataValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::DataValue") (range (start (line 7) (character 16)) (end (line 7) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DataFunctions::reduce"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::reduce") (range (start (line 9) (character 16)) (end (line 9) (character 40))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
