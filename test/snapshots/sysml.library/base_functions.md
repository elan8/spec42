# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/BaseFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package BaseFunctions {
	doc
	/*
	 * This package defines a basic set of functions defined on all kinds of values. 
	 * Most correspond to similarly named operators in the KerML expression syntax.
	 */

	private import Base::Anything;
	private import Objects::Object;
	private import Metaobjects::Metaobject;
	private import KerML::Metaclass;
	private import ScalarValues::*;
	
	abstract function '=='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1];
	}
	function '!='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1] = not (x == y);
	}
	
	abstract function '==='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1];
	}
	function '!=='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1] = not (x === y);
	}
	
	abstract function ToString{ in x: Anything[0..1]; 
		return : String;
	}
	
	abstract function '['{ in x: Anything[0..*] nonunique; in y: Anything[0..*] nonunique; 
		return : Anything[0..*] nonunique;
	}
	abstract function '#'{ in seq: Anything[0..*] ordered nonunique; in index: Positive[1..*] ordered nonunique; 
		return : Anything[0..1];
	}
	abstract function ','{ in seq1: Anything[0..*] ordered nonunique; in seq2: Anything[0..*] ordered nonunique; 
		return : Anything[0..*] ordered nonunique;
	}
	
    abstract function 'all'{
     	return : Object[0..*];
    }
    
	abstract function 'istype'{ 
		in seq: Anything[0..*];
		in 'type': Anything;
		return : Boolean[1];
	}
	
	abstract function 'hastype'{ 
		in seq: Anything[0..*];
		in 'type': Anything;
		return : Boolean;
	}
	
	abstract function '@'{ 
		in seq: Anything[0..*];
		in 'type': Anything;
		return : Boolean[1];
	}
	
	abstract function '@@'{ 
		in seq: Metaobject[0..*];
		in 'type': Metaobject;
		return : Boolean[1];
	}
	
	abstract function 'as'{ 
		in seq: Anything[0..*] ordered nonunique; 
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function 'meta'{ 
		in seq: Metaobject[0..*] ordered nonunique; 
		return : Metaobject[0..*] ordered nonunique;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "base_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 28))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package BaseFunctions {
	doc
	/*
	 * This package defines a basic set of functions defined on all kinds of values. 
	 * Most correspond to similarly named operators in the KerML expression syntax.
	 */

	private import Base::Anything;
	private import Objects::Object;
	private import Metaobjects::Metaobject;
	private import KerML::Metaclass;
	private import ScalarValues::*;
	
	abstract function '=='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1];
	}
	function '!='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1] = not (x == y);
	}
	
	abstract function '==='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1];
	}
	function '!=='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1] = not (x === y);
	}
	
	abstract function ToString{ in x: Anything[0..1]; 
		return : String;
	}
	
	abstract function '['{ in x: Anything[0..*] nonunique; in y: Anything[0..*] nonunique; 
		return : Anything[0..*] nonunique;
	}
	abstract function '#'{ in seq: Anything[0..*] ordered nonunique; in index: Positive[1..*] ordered nonunique; 
		return : Anything[0..1];
	}
	abstract function ','{ in seq1: Anything[0..*] ordered nonunique; in seq2: Anything[0..*] ordered nonunique; 
		return : Anything[0..*] ordered nonunique;
	}
	
    abstract function 'all'{
     	return : Object[0..*];
    }
    
	abstract function 'istype'{ 
		in seq: Anything[0..*];
		in 'type': Anything;
		return : Boolean[1];
	}
	
	abstract function 'hastype'{ 
		in seq: Anything[0..*];
		in 'type': Anything;
		return : Boolean;
	}
	
	abstract function '@'{ 
		in seq: Anything[0..*];
		in 'type': Anything;
		return : Boolean[1];
	}
	
	abstract function '@@'{ 
		in seq: Metaobject[0..*];
		in 'type': Metaobject;
		return : Boolean[1];
	}
	
	abstract function 'as'{ 
		in seq: Anything[0..*] ordered nonunique; 
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function 'meta'{ 
		in seq: Metaobject[0..*] ordered nonunique; 
		return : Metaobject[0..*] ordered nonunique;
	}
	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "84c59994357d016e9798311c0fed0f1c6b48a8efc80142364509c10ea125ab88") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "BaseFunctions"))) (kind "package") (name "BaseFunctions") (declared-name "BaseFunctions"))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "BaseFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (parent (node (document "d0") (qualified-name "BaseFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::Metaclass"))) (kind "import") (name "Metaclass") (declared-name "Metaclass") (parent (node (document "d0") (qualified-name "BaseFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "KerML::Metaclass") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::Metaobject"))) (kind "import") (name "Metaobject") (declared-name "Metaobject") (parent (node (document "d0") (qualified-name "BaseFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::Metaobject") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::Object"))) (kind "import") (name "Object") (declared-name "Object") (parent (node (document "d0") (qualified-name "BaseFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Object") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::ToString"))) (kind "kermlDecl") (name "ToString") (declared-name "ToString") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl10"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl11"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl12"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl2"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl3"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl4"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl5"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl6"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl7"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl8"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl9"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::return"))) (kind "kermlDecl") (name "return") (declared-name "return") (parent (node (document "d0") (qualified-name "BaseFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "BaseFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "BaseFunctions::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "BaseFunctions::Metaclass"))) (kind membershipImport) (ordinal 0)) (authored-target "KerML::Metaclass") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "BaseFunctions::Metaobject"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::Metaobject") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "BaseFunctions::Object"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Object") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 11 16) (end 11 28)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "BaseFunctions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 11 16) (end 11 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 30)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "BaseFunctions::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 7 16) (end 7 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 31)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "BaseFunctions::Object"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::Object")
        (range (start 8 16) (end 8 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 32)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "BaseFunctions::Metaclass"))
        (kind membershipImport) (ordinal 0) (authored-target "KerML::Metaclass")
        (range (start 10 16) (end 10 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 39)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "BaseFunctions::Metaobject"))
        (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::Metaobject")
        (range (start 9 16) (end 9 39))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
