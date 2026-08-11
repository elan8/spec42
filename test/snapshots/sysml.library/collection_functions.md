# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/CollectionFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package CollectionFunctions {
	doc
	/*
	 * This package defines functions on Collections (as defined in the Collections package). 
	 * For functions on general sequences of values, see the SequenceFunctions package.
	 */

	private import Base::Anything;
	private import ScalarValues::*;
	private import SequenceFunctions::equals;
	private import SequenceFunctions::includes;
	private import ControlFunctions::exists;
	public import Collections::*;
	
	function '==' specializes BaseFunctions::'==' { in col1: Collection[0..1]; in col2: Collection[0..1];
		return : Boolean[1] = col1.elements->equals(col2.elements);
	}
	
	function size { in col: Collection[1];
		return : Natural[1] = SequenceFunctions::size(col.elements);
	}
	
	function isEmpty { in col: Collection[1]; 
		return : Boolean[1] = SequenceFunctions::isEmpty(col.elements);
	}
	
	function notEmpty { in col: Collection[1]; 
		return : Boolean[1] = SequenceFunctions::notEmpty(col.elements);
	}
	
	function contains { in col: Collection[1]; in values: Anything[*];
		return : Boolean[1] = col.elements->includes(values);
	}
	
	function containsAll { in col1: Collection[1]; in col2: Collection[2]; 
		return : Boolean[1] = contains(col1, col2.elements);
	}	
	
	function head { in col: OrderedCollection[1]; 
		return : Anything[0..1] = SequenceFunctions::head(col.elements);
	}
	
	function tail { in col: OrderedCollection[1]; 
		return : Anything[0..*] ordered nonunique = SequenceFunctions::tail(col.elements);	
	}
	
	function last { in col: OrderedCollection[1]; 
		return : Anything[0..1] = SequenceFunctions::last(col.elements);
	}
	
	function '#' specializes BaseFunctions::'#' { in col: OrderedCollection[1]; in index: Positive[1];
		// Cast ensures this function is not called recursively if the elements of col are OrderedCollections.
		return : Anything[0..1] = (col.elements as Anything)#(index);		
	}
	
	function 'array#' specializes BaseFunctions::'#' { in arr: Array[1]; in indexes: Positive[n] ordered nonunique;		
		private feature n : Natural[1] = arr.rank;
		
		// Assumes row-major ordering for elements.
		private function index { in arr: Array[1]; in i : Natural; in indexes : Positive[1..*];
			if i <= 1? indexes#(1) else arr.dimensions#(i) * (index(arr, i-1, indexes) - 1) + indexes#(i)
		}
		
		return : Anything[0..1] =
			if n == 0 or (1..n)->exists {in i; indexes#(i) > arr.dimensions#(i)}? null 
			else arr.elements#(index(arr, n, indexes));
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "collection_functions.md"
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
        (range (start 8 16) (end 8 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 15) (end 12 26))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package CollectionFunctions {
	doc
	/*
	 * This package defines functions on Collections (as defined in the Collections package). 
	 * For functions on general sequences of values, see the SequenceFunctions package.
	 */

	private import Base::Anything;
	private import ScalarValues::*;
	private import SequenceFunctions::equals;
	private import SequenceFunctions::includes;
	private import ControlFunctions::exists;
	public import Collections::*;
	
	function '==' specializes BaseFunctions::'==' { in col1: Collection[0..1]; in col2: Collection[0..1];
		return : Boolean[1] = col1.elements->equals(col2.elements);
	}
	
	function size { in col: Collection[1];
		return : Natural[1] = SequenceFunctions::size(col.elements);
	}
	
	function isEmpty { in col: Collection[1]; 
		return : Boolean[1] = SequenceFunctions::isEmpty(col.elements);
	}
	
	function notEmpty { in col: Collection[1]; 
		return : Boolean[1] = SequenceFunctions::notEmpty(col.elements);
	}
	
	function contains { in col: Collection[1]; in values: Anything[*];
		return : Boolean[1] = col.elements->includes(values);
	}
	
	function containsAll { in col1: Collection[1]; in col2: Collection[2]; 
		return : Boolean[1] = contains(col1, col2.elements);
	}	
	
	function head { in col: OrderedCollection[1]; 
		return : Anything[0..1] = SequenceFunctions::head(col.elements);
	}
	
	function tail { in col: OrderedCollection[1]; 
		return : Anything[0..*] ordered nonunique = SequenceFunctions::tail(col.elements);	
	}
	
	function last { in col: OrderedCollection[1]; 
		return : Anything[0..1] = SequenceFunctions::last(col.elements);
	}
	
	function '#' specializes BaseFunctions::'#' { in col: OrderedCollection[1]; in index: Positive[1];
		// Cast ensures this function is not called recursively if the elements of col are OrderedCollections.
		return : Anything[0..1] = (col.elements as Anything)#(index);		
	}
	
	function 'array#' specializes BaseFunctions::'#' { in arr: Array[1]; in indexes: Positive[n] ordered nonunique;		
		private feature n : Natural[1] = arr.rank;
		
		// Assumes row-major ordering for elements.
		private function index { in arr: Array[1]; in i : Natural; in indexes : Positive[1..*];
			if i <= 1? indexes#(1) else arr.dimensions#(i) * (index(arr, i-1, indexes) - 1) + indexes#(i)
		}
		
		return : Anything[0..1] =
			if n == 0 or (1..n)->exists {in i; indexes#(i) > arr.dimensions#(i)}? null 
			else arr.elements#(index(arr, n, indexes));
	}	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "3a00e4591bcf247b70aa1764f60816434f5a76d1b06be4f282884f203a3cc2f5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "CollectionFunctions"))) (kind "package") (name "CollectionFunctions") (declared-name "CollectionFunctions"))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "CollectionFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "CollectionFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "Collections::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (parent (node (document "d0") (qualified-name "CollectionFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::contains"))) (kind "kermlDecl") (name "contains") (declared-name "contains") (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::containsAll"))) (kind "kermlDecl") (name "containsAll") (declared-name "containsAll") (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::equals"))) (kind "import") (name "equals") (declared-name "equals") (parent (node (document "d0") (qualified-name "CollectionFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::equals") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::exists"))) (kind "import") (name "exists") (declared-name "exists") (parent (node (document "d0") (qualified-name "CollectionFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::exists") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::head"))) (kind "kermlDecl") (name "head") (declared-name "head") (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::includes"))) (kind "import") (name "includes") (declared-name "includes") (parent (node (document "d0") (qualified-name "CollectionFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::includes") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::isEmpty"))) (kind "kermlDecl") (name "isEmpty") (declared-name "isEmpty") (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::last"))) (kind "kermlDecl") (name "last") (declared-name "last") (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::notEmpty"))) (kind "kermlDecl") (name "notEmpty") (declared-name "notEmpty") (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::size"))) (kind "kermlDecl") (name "size") (declared-name "size") (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::tail"))) (kind "kermlDecl") (name "tail") (declared-name "tail") (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "CollectionFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CollectionFunctions::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Collections::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CollectionFunctions::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CollectionFunctions::equals"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::equals") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CollectionFunctions::exists"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::exists") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CollectionFunctions::includes"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::includes") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 12 15) (end 12 26)) (probe (position 12 15))
      (reference
        (source (document "d0") (qualified-name "CollectionFunctions::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Collections::*")
        (range (start 12 15) (end 12 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 28)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "CollectionFunctions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 8 16) (end 8 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 30)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "CollectionFunctions::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 7 16) (end 7 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 40)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "CollectionFunctions::exists"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::exists")
        (range (start 11 16) (end 11 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 41)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "CollectionFunctions::equals"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::equals")
        (range (start 9 16) (end 9 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 43)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "CollectionFunctions::includes"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::includes")
        (range (start 10 16) (end 10 43))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
