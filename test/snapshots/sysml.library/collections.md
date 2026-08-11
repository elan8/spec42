# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Data Type Library/Collections
type=file
~~~
# SOURCE
~~~kerml
standard library package Collections {
	doc
	/*
	 * This package defines a standard set of Collection data types. Unlike sequences of values 
	 * defined directly using multiplicity, these data types allow for the possibility of collections 
	 * as elements of collections.
	 */

	private import Base::*;
	private import ScalarValues::*;
	private import SequenceFunctions::size;
	private import IntegerFunctions::*;
	private import ControlFunctions::*;

	abstract datatype Collection {
		doc
		/*
		 * Collection is the top level abstract supertype of all collection types.
		 * The name elements is used to denote the members or contents of the collection.
		 */
		
		feature elements[0..*] nonunique;
	}

    abstract datatype OrderedCollection :> Collection {
    	doc
		/*
		 * OrderedCollection is the abstract supertype for all ordered collection types.
		 */
    	
		feature elements[0..*] ordered nonunique :>> Collection::elements;
    }

    abstract datatype UniqueCollection :> Collection {
    	doc
		/*
		 * UniqueCollection is the abstract supertype for all collection types with unique elements.
		 */
		
		feature elements[0..*] :>> Collection::elements {
			/* Note: Redefinition of 'elements' is unique by default. */
		}
    }

    datatype Array :> OrderedCollection {
    	doc
	    /*
	     * An Array is a fixed size, multi-dimensional Collection of which the elements are nonunique and ordered. 
	     * Its dimensions specify how many dimensions the array has, and how many elements there are in each dimension. 
	     * The rank is equal to the number of dimensions. The flattenedSize is equal to the total number of elements 
	     * in the array.
	     * 
	     * Feature elements is a flattened sequence of all elements of an Array and can be accessed by a tuple of indices. 
	     * The number of indices is equal to rank. The elements are packed according to row-major convention, as in the C programming language.
	     * 
	     * The elements of an Array can be assessed by a tuple of indices. The number of indices in such tuple is equal to rank. 
		 * The packing of the elements, i.e. the flattened representation, follows the row-major convention, 
		 * as in the C programming language.
		 * 
		 * Note 1. Feature dimensions may be empty, which denotes a zero dimensional array, allowing an Array to collapse to a single element. 
		 * This is useful to allow for specialization of an Array into a type restricted to represent a scalar. 
		 * The flattenedSize of a zero dimensional array is 1.
		 * 
		 * Note 2: An Array can represent the generalized mathematical concept of an infinite matrix of any rank, i.e. not limited to rank two.
	     */
	     
        feature dimensions: Positive[0..*] ordered nonunique {
        	doc
       		/* Feature `dimensions` defines the N-dimensional shape of the Array
             * The alternative name `shape` (as used in many programming languages) is not used as it would interfere with a geometric shape feature.
 			 */
        }
        feature rank: Natural[1] = size(dimensions);
        feature flattenedSize: Positive[1] = dimensions->reduce '*' ?? 1;
        inv { flattenedSize == size(elements) }
    }
    
	datatype Bag :> Collection {
		doc
		/*
		 * Bag is a variable-size, unordered collection of nonunique elements.
		 */		
	}
	
	datatype Set :> UniqueCollection {
		doc
		/*
		 * Set is a variable-size, unordered collection of unique elements.
		 */
	}

	datatype OrderedSet :> OrderedCollection, UniqueCollection 
		intersects OrderedCollection, UniqueCollection {
		doc
		/*
		 * OrderedSet is a variable-size, ordered collection of unique elements.
		 */	
		
		feature elements[0..*] ordered :>> OrderedCollection::elements, UniqueCollection::elements {
			/* Note: Redefinition of `elements` is unique by default. */
		}
	}
		
	datatype List :> OrderedCollection {
		doc
		/*
		 * List is a variable-size, ordered collection of nonunique elements.
		 */
	}

    datatype KeyValuePair {
    	doc
		/*
		 * KeyValuePair is a tuple of a key and a value for use in Map collections.
		 * The key must be immutable.
		 */
    	
        feature key: Anything[0..*] ordered nonunique;
        feature val: Anything[0..*] ordered nonunique;
    }

    datatype Map :> Collection {
    	doc
		/*
		 * Map is a variable-size, unordered collection of elements that are key-value pairs.
		 */
    	
		feature elements: KeyValuePair[0..*] :>> Collection::elements {
			/* Note: Redefinition of `elements` is unique by default. 
			 * The `key` of any `KeyValuePair` must be unique over the collection of `KeyValuePair`. 
			 * The `val` does not need to be unique. 
			 */			
		}
    }

    datatype OrderedMap :> Map {
    	doc
		/*
		 * OrderedMap is a variable-size, ordered collection of elements that are key-value pairs. 
		 */

		feature elements: KeyValuePair[0..*] ordered :>> Map::elements {
			/* Note: Redefinition of `elements` is unique by default. */
		}
    }
    
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "collections.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 32))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Collections {
	doc
	/*
	 * This package defines a standard set of Collection data types. Unlike sequences of values 
	 * defined directly using multiplicity, these data types allow for the possibility of collections 
	 * as elements of collections.
	 */

	private import Base::*;
	private import ScalarValues::*;
	private import SequenceFunctions::size;
	private import IntegerFunctions::*;
	private import ControlFunctions::*;

	abstract datatype Collection {
		doc
		/*
		 * Collection is the top level abstract supertype of all collection types.
		 * The name elements is used to denote the members or contents of the collection.
		 */
		
		feature elements[0..*] nonunique;
	}

    abstract datatype OrderedCollection :> Collection {
    	doc
		/*
		 * OrderedCollection is the abstract supertype for all ordered collection types.
		 */
    	
		feature elements[0..*] ordered nonunique :>> Collection::elements;
    }

    abstract datatype UniqueCollection :> Collection {
    	doc
		/*
		 * UniqueCollection is the abstract supertype for all collection types with unique elements.
		 */
		
		feature elements[0..*] :>> Collection::elements {
			/* Note: Redefinition of 'elements' is unique by default. */
		}
    }

    datatype Array :> OrderedCollection {
    	doc
	    /*
	     * An Array is a fixed size, multi-dimensional Collection of which the elements are nonunique and ordered. 
	     * Its dimensions specify how many dimensions the array has, and how many elements there are in each dimension. 
	     * The rank is equal to the number of dimensions. The flattenedSize is equal to the total number of elements 
	     * in the array.
	     * 
	     * Feature elements is a flattened sequence of all elements of an Array and can be accessed by a tuple of indices. 
	     * The number of indices is equal to rank. The elements are packed according to row-major convention, as in the C programming language.
	     * 
	     * The elements of an Array can be assessed by a tuple of indices. The number of indices in such tuple is equal to rank. 
		 * The packing of the elements, i.e. the flattened representation, follows the row-major convention, 
		 * as in the C programming language.
		 * 
		 * Note 1. Feature dimensions may be empty, which denotes a zero dimensional array, allowing an Array to collapse to a single element. 
		 * This is useful to allow for specialization of an Array into a type restricted to represent a scalar. 
		 * The flattenedSize of a zero dimensional array is 1.
		 * 
		 * Note 2: An Array can represent the generalized mathematical concept of an infinite matrix of any rank, i.e. not limited to rank two.
	     */
	     
        feature dimensions: Positive[0..*] ordered nonunique {
        	doc
       		/* Feature `dimensions` defines the N-dimensional shape of the Array
             * The alternative name `shape` (as used in many programming languages) is not used as it would interfere with a geometric shape feature.
 			 */
        }
        feature rank: Natural[1] = size(dimensions);
        feature flattenedSize: Positive[1] = dimensions->reduce '*' ?? 1;
        inv { flattenedSize == size(elements) }
    }
    
	datatype Bag :> Collection {
		doc
		/*
		 * Bag is a variable-size, unordered collection of nonunique elements.
		 */		
	}
	
	datatype Set :> UniqueCollection {
		doc
		/*
		 * Set is a variable-size, unordered collection of unique elements.
		 */
	}

	datatype OrderedSet :> OrderedCollection, UniqueCollection 
		intersects OrderedCollection, UniqueCollection {
		doc
		/*
		 * OrderedSet is a variable-size, ordered collection of unique elements.
		 */	
		
		feature elements[0..*] ordered :>> OrderedCollection::elements, UniqueCollection::elements {
			/* Note: Redefinition of `elements` is unique by default. */
		}
	}
		
	datatype List :> OrderedCollection {
		doc
		/*
		 * List is a variable-size, ordered collection of nonunique elements.
		 */
	}

    datatype KeyValuePair {
    	doc
		/*
		 * KeyValuePair is a tuple of a key and a value for use in Map collections.
		 * The key must be immutable.
		 */
    	
        feature key: Anything[0..*] ordered nonunique;
        feature val: Anything[0..*] ordered nonunique;
    }

    datatype Map :> Collection {
    	doc
		/*
		 * Map is a variable-size, unordered collection of elements that are key-value pairs.
		 */
    	
		feature elements: KeyValuePair[0..*] :>> Collection::elements {
			/* Note: Redefinition of `elements` is unique by default. 
			 * The `key` of any `KeyValuePair` must be unique over the collection of `KeyValuePair`. 
			 * The `val` does not need to be unique. 
			 */			
		}
    }

    datatype OrderedMap :> Map {
    	doc
		/*
		 * OrderedMap is a variable-size, ordered collection of elements that are key-value pairs. 
		 */

		feature elements: KeyValuePair[0..*] ordered :>> Map::elements {
			/* Note: Redefinition of `elements` is unique by default. */
		}
    }
    
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c9581171f68c9e099c51b93618bc58424dfb9084bb14bbc66259d9cda60c9574") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Collections"))) (kind "package") (name "Collections") (declared-name "Collections") (range (start (line 0) (character 0)) (end (line 0) (character 4973))))
    (element (id (node (document "d0") (qualified-name "Collections::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 8) (character 1)) (end (line 8) (character 24))) (parent (node (document "d0") (qualified-name "Collections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 20))))))
    (element (id (node (document "d0") (qualified-name "Collections::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 9) (character 1)) (end (line 9) (character 32))) (parent (node (document "d0") (qualified-name "Collections"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Collections::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 11) (character 1)) (end (line 11) (character 36))) (parent (node (document "d0") (qualified-name "Collections"))) (authored (membership (kind Import) (visibility "private") (import (reference "IntegerFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 32))))))
    (element (id (node (document "d0") (qualified-name "Collections::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 12) (character 1)) (end (line 12) (character 36))) (parent (node (document "d0") (qualified-name "Collections"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 32))))))
    (element (id (node (document "d0") (qualified-name "Collections::Array"))) (kind "kermlDecl") (name "Array") (declared-name "Array") (range (start (line 44) (character 4)) (end (line 44) (character 1947))) (parent (node (document "d0") (qualified-name "Collections"))))
    (element (id (node (document "d0") (qualified-name "Collections::Bag"))) (kind "kermlDecl") (name "Bag") (declared-name "Bag") (range (start (line 77) (character 1)) (end (line 77) (character 124))) (parent (node (document "d0") (qualified-name "Collections"))))
    (element (id (node (document "d0") (qualified-name "Collections::Collection"))) (kind "kermlDecl") (name "Collection") (declared-name "Collection") (range (start (line 14) (character 1)) (end (line 14) (character 251))) (parent (node (document "d0") (qualified-name "Collections"))))
    (element (id (node (document "d0") (qualified-name "Collections::KeyValuePair"))) (kind "kermlDecl") (name "KeyValuePair") (declared-name "KeyValuePair") (range (start (line 110) (character 4)) (end (line 110) (character 279))) (parent (node (document "d0") (qualified-name "Collections"))))
    (element (id (node (document "d0") (qualified-name "Collections::List"))) (kind "kermlDecl") (name "List") (declared-name "List") (range (start (line 103) (character 1)) (end (line 103) (character 129))) (parent (node (document "d0") (qualified-name "Collections"))))
    (element (id (node (document "d0") (qualified-name "Collections::Map"))) (kind "kermlDecl") (name "Map") (declared-name "Map") (range (start (line 121) (character 4)) (end (line 121) (character 432))) (parent (node (document "d0") (qualified-name "Collections"))))
    (element (id (node (document "d0") (qualified-name "Collections::OrderedCollection"))) (kind "kermlDecl") (name "OrderedCollection") (declared-name "OrderedCollection") (range (start (line 24) (character 4)) (end (line 24) (character 239))) (parent (node (document "d0") (qualified-name "Collections"))))
    (element (id (node (document "d0") (qualified-name "Collections::OrderedMap"))) (kind "kermlDecl") (name "OrderedMap") (declared-name "OrderedMap") (range (start (line 135) (character 4)) (end (line 135) (character 288))) (parent (node (document "d0") (qualified-name "Collections"))))
    (element (id (node (document "d0") (qualified-name "Collections::OrderedSet"))) (kind "kermlDecl") (name "OrderedSet") (declared-name "OrderedSet") (range (start (line 91) (character 1)) (end (line 91) (character 373))) (parent (node (document "d0") (qualified-name "Collections"))))
    (element (id (node (document "d0") (qualified-name "Collections::Set"))) (kind "kermlDecl") (name "Set") (declared-name "Set") (range (start (line 84) (character 1)) (end (line 84) (character 125))) (parent (node (document "d0") (qualified-name "Collections"))))
    (element (id (node (document "d0") (qualified-name "Collections::UniqueCollection"))) (kind "kermlDecl") (name "UniqueCollection") (declared-name "UniqueCollection") (range (start (line 33) (character 4)) (end (line 33) (character 298))) (parent (node (document "d0") (qualified-name "Collections"))))
    (element (id (node (document "d0") (qualified-name "Collections::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 4973))) (parent (node (document "d0") (qualified-name "Collections"))))
    (element (id (node (document "d0") (qualified-name "Collections::size"))) (kind "import") (name "size") (declared-name "size") (range (start (line 10) (character 1)) (end (line 10) (character 40))) (parent (node (document "d0") (qualified-name "Collections"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 39))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Collections::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Base::*") (range (start (line 8) (character 16)) (end (line 8) (character 20))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Collections::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 9) (character 16)) (end (line 9) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Collections::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "IntegerFunctions::*") (range (start (line 11) (character 16)) (end (line 11) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Collections::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "ControlFunctions::*") (range (start (line 12) (character 16)) (end (line 12) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Collections::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (range (start (line 10) (character 16)) (end (line 10) (character 39))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
