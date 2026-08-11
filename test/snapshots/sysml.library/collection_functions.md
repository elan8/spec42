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
    (element (id (node (document "d0") (qualified-name "CollectionFunctions"))) (kind "package") (name "CollectionFunctions") (declared-name "CollectionFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 2457))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 8) (character 1)) (end (line 8) (character 32))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 28))))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 12) (character 1)) (end (line 12) (character 30))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "Collections::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 12) (character 15)) (end (line 12) (character 26))))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 30))))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2457))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::contains"))) (kind "kermlDecl") (name "contains") (declared-name "contains") (range (start (line 30) (character 1)) (end (line 30) (character 126))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::containsAll"))) (kind "kermlDecl") (name "containsAll") (declared-name "containsAll") (range (start (line 34) (character 1)) (end (line 34) (character 130))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::equals"))) (kind "import") (name "equals") (declared-name "equals") (range (start (line 9) (character 1)) (end (line 9) (character 42))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::equals") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 41))))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::exists"))) (kind "import") (name "exists") (declared-name "exists") (range (start (line 11) (character 1)) (end (line 11) (character 41))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::exists") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 40))))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 14) (character 1)) (end (line 14) (character 167))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 50) (character 1)) (end (line 50) (character 273))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 55) (character 1)) (end (line 55) (character 559))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::head"))) (kind "kermlDecl") (name "head") (declared-name "head") (range (start (line 38) (character 1)) (end (line 38) (character 117))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::includes"))) (kind "import") (name "includes") (declared-name "includes") (range (start (line 10) (character 1)) (end (line 10) (character 44))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::includes") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 43))))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::isEmpty"))) (kind "kermlDecl") (name "isEmpty") (declared-name "isEmpty") (range (start (line 22) (character 1)) (end (line 22) (character 112))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::last"))) (kind "kermlDecl") (name "last") (declared-name "last") (range (start (line 46) (character 1)) (end (line 46) (character 117))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::notEmpty"))) (kind "kermlDecl") (name "notEmpty") (declared-name "notEmpty") (range (start (line 26) (character 1)) (end (line 26) (character 114))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::size"))) (kind "kermlDecl") (name "size") (declared-name "size") (range (start (line 18) (character 1)) (end (line 18) (character 105))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
    (element (id (node (document "d0") (qualified-name "CollectionFunctions::tail"))) (kind "kermlDecl") (name "tail") (declared-name "tail") (range (start (line 42) (character 1)) (end (line 42) (character 136))) (parent (node (document "d0") (qualified-name "CollectionFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "CollectionFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 8) (character 16)) (end (line 8) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CollectionFunctions::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Collections::*") (range (start (line 12) (character 15)) (end (line 12) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CollectionFunctions::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (range (start (line 7) (character 16)) (end (line 7) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CollectionFunctions::equals"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::equals") (range (start (line 9) (character 16)) (end (line 9) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CollectionFunctions::exists"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::exists") (range (start (line 11) (character 16)) (end (line 11) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CollectionFunctions::includes"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::includes") (range (start (line 10) (character 16)) (end (line 10) (character 43))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
