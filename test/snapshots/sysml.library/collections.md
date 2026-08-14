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
  (document "memory://snapshot/collections.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 31))
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
        (range (start 11 16) (end 11 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 66 28) (end 66 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 72 22) (end 72 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 72 35) (end 72 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 73 31) (end 73 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 73 64) (end 73 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 31) (end 74 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 36) (end 74 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 117 21) (end 117 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 118 21) (end 118 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a4bca9c80db0a608a8328ca0f3b7ef13e8aef68ca5f220fe37f0925a1bdafb5b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package defines a standard set of Collection data types. Unlike sequences of values \n\t * defined directly using multiplicity, these data types allow for the possibility of collections \n\t * as elements of collections.\n\t "))))
    (declaration (id (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Base") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 3)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "IntegerFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 4)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ControlFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t     * An Array is a fixed size, multi-dimensional Collection of which the elements are nonunique and ordered. \n\t     * Its dimensions specify how many dimensions the array has, and how many elements there are in each dimension. \n\t     * The rank is equal to the number of dimensions. The flattenedSize is equal to the total number of elements \n\t     * in the array.\n\t     * \n\t     * Feature elements is a flattened sequence of all elements of an Array and can be accessed by a tuple of indices. \n\t     * The number of indices is equal to rank. The elements are packed according to row-major convention, as in the C programming language.\n\t     * \n\t     * The elements of an Array can be assessed by a tuple of indices. The number of indices in such tuple is equal to rank. \n\t\t * The packing of the elements, i.e. the flattened representation, follows the row-major convention, \n\t\t * as in the C programming language.\n\t\t * \n\t\t * Note 1. Feature dimensions may be empty, which denotes a zero dimensional array, allowing an Array to collapse to a single element. \n\t\t * This is useful to allow for specialization of an Array into a type restricted to represent a scalar. \n\t\t * The flattenedSize of a zero dimensional array is 1.\n\t\t * \n\t\t * Note 2: An Array can represent the generalized mathematical concept of an infinite matrix of any rank, i.e. not limited to rank two.\n\t     "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OrderedCollection"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (named (kind kerml-classifier) (name "Array")) (anonymous (kind kerml-invariant) (ordinal 0)))))) (kind kerml-invariant) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "flattenedSize")) (expressionOperand (reference "elements")) (invocationCallee (reference "size"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::dimensions"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers ordered nonunique) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text " Feature `dimensions` defines the N-dimensional shape of the Array\n             * The alternative name `shape` (as used in many programming languages) is not used as it would interfere with a geometric shape feature.\n \t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Positive"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Positive")) (expressionOperand (reference "dimensions")) (expressionOperand (reference "*"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::rank"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Natural")) (expressionOperand (reference "dimensions")) (invocationCallee (reference "size"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Bag"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Bag is a variable-size, unordered collection of nonunique elements.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Collection"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * Collection is the top level abstract supertype of all collection types.\n\t\t * The name elements is used to denote the members or contents of the collection.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection::elements"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers nonunique) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::KeyValuePair"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * KeyValuePair is a tuple of a key and a value for use in Map collections.\n\t\t * The key must be immutable.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::KeyValuePair::key"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers ordered nonunique) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::KeyValuePair::val"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers ordered nonunique) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::List"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * List is a variable-size, ordered collection of nonunique elements.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OrderedCollection"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Map is a variable-size, unordered collection of elements that are key-value pairs.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Collection"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map::elements"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "KeyValuePair")) (redefinition (reference "Collection::elements"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * OrderedCollection is the abstract supertype for all ordered collection types.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Collection"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection::elements"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers ordered nonunique) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Collection::elements"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedMap"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * OrderedMap is a variable-size, ordered collection of elements that are key-value pairs. \n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Map"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedMap::elements"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers ordered) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "KeyValuePair")) (redefinition (reference "Map::elements"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * OrderedSet is a variable-size, ordered collection of unique elements.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OrderedCollection")) (specialization (reference "UniqueCollection"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet::elements"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers ordered) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "OrderedCollection::elements")) (redefinition (reference "UniqueCollection::elements"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Set"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Set is a variable-size, unordered collection of unique elements.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "UniqueCollection"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * UniqueCollection is the abstract supertype for all collection types with unique elements.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Collection"))))
    (declaration (id (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection::elements"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Collection::elements"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Base")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 3)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "IntegerFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 4)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ControlFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 2)))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array"))) (kind specialization) (ordinal 0))
      (authored-target "OrderedCollection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (named (kind kerml-classifier) (name "Array")) (anonymous (kind kerml-invariant) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "flattenedSize")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::flattenedSize")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (named (kind kerml-classifier) (name "Array")) (anonymous (kind kerml-invariant) (ordinal 0)))))) (kind expressionOperand) (ordinal 1))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (named (kind kerml-classifier) (name "Array")) (anonymous (kind kerml-invariant) (ordinal 0)))))) (kind invocationCallee) (ordinal 0))
      (authored-target "size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::dimensions"))) (kind featureTyping) (ordinal 0))
      (authored-target "Positive")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (kind featureTyping) (ordinal 0))
      (authored-target "Positive")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (kind expressionOperand) (ordinal 0))
      (authored-target "dimensions")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::dimensions")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (kind expressionOperand) (ordinal 1))
      (authored-target "*")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::rank"))) (kind featureTyping) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::rank"))) (kind expressionOperand) (ordinal 0))
      (authored-target "dimensions")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::dimensions")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::rank"))) (kind invocationCallee) (ordinal 0))
      (authored-target "size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Bag"))) (kind specialization) (ordinal 0))
      (authored-target "Collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::KeyValuePair::key"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::KeyValuePair::val"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::List"))) (kind specialization) (ordinal 0))
      (authored-target "OrderedCollection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map"))) (kind specialization) (ordinal 0))
      (authored-target "Collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map::elements"))) (kind featureTyping) (ordinal 0))
      (authored-target "KeyValuePair")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::KeyValuePair")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map::elements"))) (kind redefinition) (ordinal 0))
      (authored-target "Collection::elements")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection::elements")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection"))) (kind specialization) (ordinal 0))
      (authored-target "Collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection::elements"))) (kind redefinition) (ordinal 0))
      (authored-target "Collection::elements")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection::elements")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedMap"))) (kind specialization) (ordinal 0))
      (authored-target "Map")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedMap::elements"))) (kind featureTyping) (ordinal 0))
      (authored-target "KeyValuePair")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::KeyValuePair")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedMap::elements"))) (kind redefinition) (ordinal 0))
      (authored-target "Map::elements")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map::elements")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet"))) (kind specialization) (ordinal 0))
      (authored-target "OrderedCollection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet"))) (kind specialization) (ordinal 1))
      (authored-target "UniqueCollection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet::elements"))) (kind redefinition) (ordinal 0))
      (authored-target "OrderedCollection::elements")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection::elements")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet::elements"))) (kind redefinition) (ordinal 1))
      (authored-target "UniqueCollection::elements")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection::elements")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Set"))) (kind specialization) (ordinal 0))
      (authored-target "UniqueCollection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection"))) (kind specialization) (ordinal 0))
      (authored-target "Collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection")))))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection::elements"))) (kind redefinition) (ordinal 0))
      (authored-target "Collection::elements")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection::elements")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array"))) (kind specialization) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (named (kind kerml-classifier) (name "Array")) (anonymous (kind kerml-invariant) (ordinal 0)))))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (named (kind kerml-classifier) (name "Array")) (anonymous (kind kerml-invariant) (ordinal 0)))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::dimensions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::rank"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::dimensions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::rank"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Bag"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Bag"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::List"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::List"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map::elements"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::KeyValuePair"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map::elements"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map::elements"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection::elements"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map::elements"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection::elements"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection::elements"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection::elements"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedMap"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedMap"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedMap::elements"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::KeyValuePair"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedMap::elements"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedMap::elements"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map::elements"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedMap::elements"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet"))) (kind specialization) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet::elements"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection::elements"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet::elements"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet::elements"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection::elements"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet::elements"))) (kind redefinition) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Set"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Set"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection::elements"))) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection::elements"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection::elements"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::rank"))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/collections.md") (range (start 8 16) (end 8 23)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "Base")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/collections.md") (range (start 9 16) (end 9 31)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/collections.md") (range (start 11 16) (end 11 35)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 3)))))) (kind namespaceImport) (ordinal 0) (authored-target "IntegerFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/collections.md") (range (start 12 16) (end 12 35)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 4)))))) (kind namespaceImport) (ordinal 0) (authored-target "ControlFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/collections.md") (range (start 10 16) (end 10 39)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (anonymous (kind import) (ordinal 2)))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/collections.md") (range (start 44 22) (end 44 39)) (probe (position 44 22))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array"))) (kind specialization) (ordinal 0) (authored-target "OrderedCollection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 74 14) (end 74 27)) (probe (position 74 14))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (named (kind kerml-classifier) (name "Array")) (anonymous (kind kerml-invariant) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "flattenedSize")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::flattenedSize")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 74 36) (end 74 44)) (probe (position 74 36))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (named (kind kerml-classifier) (name "Array")) (anonymous (kind kerml-invariant) (ordinal 0)))))) (kind expressionOperand) (ordinal 1) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/collections.md") (range (start 74 31) (end 74 35)) (probe (position 74 31))
    (reference (id (source (node (document "memory://snapshot/collections.md") (path (named (kind library-package) (name "Collections")) (named (kind kerml-classifier) (name "Array")) (anonymous (kind kerml-invariant) (ordinal 0)))))) (kind invocationCallee) (ordinal 0) (authored-target "size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/collections.md") (range (start 66 28) (end 66 36)) (probe (position 66 28))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::dimensions"))) (kind featureTyping) (ordinal 0) (authored-target "Positive")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/collections.md") (range (start 73 31) (end 73 39)) (probe (position 73 31))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (kind featureTyping) (ordinal 0) (authored-target "Positive")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/collections.md") (range (start 73 45) (end 73 55)) (probe (position 73 45))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (kind expressionOperand) (ordinal 0) (authored-target "dimensions")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::dimensions")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 73 64) (end 73 67)) (probe (position 73 64))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (kind expressionOperand) (ordinal 1) (authored-target "*")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/collections.md") (range (start 72 22) (end 72 29)) (probe (position 72 22))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::rank"))) (kind featureTyping) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/collections.md") (range (start 72 40) (end 72 50)) (probe (position 72 40))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::rank"))) (kind expressionOperand) (ordinal 0) (authored-target "dimensions")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::dimensions")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 72 35) (end 72 39)) (probe (position 72 35))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Array::rank"))) (kind invocationCallee) (ordinal 0) (authored-target "size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/collections.md") (range (start 77 17) (end 77 27)) (probe (position 77 17))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Bag"))) (kind specialization) (ordinal 0) (authored-target "Collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 117 21) (end 117 29)) (probe (position 117 21))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::KeyValuePair::key"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/collections.md") (range (start 118 21) (end 118 29)) (probe (position 118 21))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::KeyValuePair::val"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/collections.md") (range (start 103 18) (end 103 35)) (probe (position 103 18))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::List"))) (kind specialization) (ordinal 0) (authored-target "OrderedCollection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 121 20) (end 121 30)) (probe (position 121 20))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map"))) (kind specialization) (ordinal 0) (authored-target "Collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 127 20) (end 127 32)) (probe (position 127 20))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map::elements"))) (kind featureTyping) (ordinal 0) (authored-target "KeyValuePair")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::KeyValuePair")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 127 43) (end 127 63)) (probe (position 127 43))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map::elements"))) (kind redefinition) (ordinal 0) (authored-target "Collection::elements")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection::elements")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 24 43) (end 24 53)) (probe (position 24 43))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection"))) (kind specialization) (ordinal 0) (authored-target "Collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 30 47) (end 30 67)) (probe (position 30 47))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection::elements"))) (kind redefinition) (ordinal 0) (authored-target "Collection::elements")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection::elements")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 135 27) (end 135 30)) (probe (position 135 27))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedMap"))) (kind specialization) (ordinal 0) (authored-target "Map")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 141 20) (end 141 32)) (probe (position 141 20))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedMap::elements"))) (kind featureTyping) (ordinal 0) (authored-target "KeyValuePair")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::KeyValuePair")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 141 51) (end 141 64)) (probe (position 141 51))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedMap::elements"))) (kind redefinition) (ordinal 0) (authored-target "Map::elements")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Map::elements")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 91 24) (end 91 41)) (probe (position 91 24))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet"))) (kind specialization) (ordinal 0) (authored-target "OrderedCollection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 91 43) (end 91 59)) (probe (position 91 43))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet"))) (kind specialization) (ordinal 1) (authored-target "UniqueCollection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 98 37) (end 98 64)) (probe (position 98 37))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet::elements"))) (kind redefinition) (ordinal 0) (authored-target "OrderedCollection::elements")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedCollection::elements")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 98 66) (end 98 92)) (probe (position 98 66))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::OrderedSet::elements"))) (kind redefinition) (ordinal 1) (authored-target "UniqueCollection::elements")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection::elements")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 84 17) (end 84 33)) (probe (position 84 17))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Set"))) (kind specialization) (ordinal 0) (authored-target "UniqueCollection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 33 42) (end 33 52)) (probe (position 33 42))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection"))) (kind specialization) (ordinal 0) (authored-target "Collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection")))))
  )
  (query (document "memory://snapshot/collections.md") (range (start 39 29) (end 39 49)) (probe (position 39 29))
    (reference (id (source (node (document "memory://snapshot/collections.md") (qualified-name "Collections::UniqueCollection::elements"))) (kind redefinition) (ordinal 0) (authored-target "Collection::elements")
      (outcome (status resolved) (target (node (document "memory://snapshot/collections.md") (qualified-name "Collections::Collection::elements")))))
  )
)
~~~
