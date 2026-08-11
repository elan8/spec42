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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ff9911b06ae7a78efebbc6e8c6b386614c56a10add04946989d7e35e8ccf314d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DataFunctions"))) (kind "package") (name "DataFunctions") (declared-name "DataFunctions"))
    (element (id (node (document "d0") (qualified-name "DataFunctions::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (parent (node (document "d0") (qualified-name "DataFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::DataValue"))) (kind "import") (name "DataValue") (declared-name "DataValue") (parent (node (document "d0") (qualified-name "DataFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::DataValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl10"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl11"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl12"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl13"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl14"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl15"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl16"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl17"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl18"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl6"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl7"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl8"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl9"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::max"))) (kind "kermlDecl") (name "max") (declared-name "max") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::min"))) (kind "kermlDecl") (name "min") (declared-name "min") (parent (node (document "d0") (qualified-name "DataFunctions"))))
    (element (id (node (document "d0") (qualified-name "DataFunctions::reduce"))) (kind "import") (name "reduce") (declared-name "reduce") (parent (node (document "d0") (qualified-name "DataFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::reduce") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DataFunctions::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DataFunctions::DataValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::DataValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DataFunctions::reduce"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::reduce") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 7 16) (end 7 31)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "DataFunctions::DataValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::DataValue")
        (range (start 7 16) (end 7 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 37)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "DataFunctions::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 8 16) (end 8 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 40)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "DataFunctions::reduce"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::reduce")
        (range (start 9 16) (end 9 40))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
