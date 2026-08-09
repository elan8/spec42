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
# EXPECTED
~~~
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAbstract,KwDatatype,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,
CloseCurly,
KwAbstract,KwDatatype,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwDatatype,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGtGt,Ident,ColonColon,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
KwDatatype,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Arrow,Ident,UnrestrictedName,QuestionQuestion,DecimalValue,Semicolon,
KwInv,OpenCurly,Ident,EqEq,Ident,OpenParen,Ident,CloseParen,CloseCurly,
CloseCurly,
KwDatatype,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwDatatype,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwDatatype,Ident,ColonGt,Ident,Comma,Ident,
KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
KwDatatype,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwDatatype,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
CloseCurly,
KwDatatype,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGtGt,Ident,ColonColon,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
KwDatatype,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,ColonGtGt,Ident,ColonColon,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Collections'
    (documentation)
    (import_decl private 'Base::*')
    (import_decl private 'ScalarValues::*')
    (import_decl private 'SequenceFunctions::size')
    (import_decl private 'IntegerFunctions::*')
    (import_decl private 'ControlFunctions::*')
    (datatype_def abstract 'Collection'
      (documentation)
      (feature_def 'elements' multiplicity nonunique))
    (datatype_def abstract 'OrderedCollection' :> 'Collection'
      (documentation)
      (feature_def 'elements' multiplicity :>> 'Collection::elements' ordered nonunique))
    (datatype_def abstract 'UniqueCollection' :> 'Collection'
      (documentation)
      (feature_def 'elements' multiplicity :>> 'Collection::elements'
        (comment)))
    (datatype_def 'Array' :> 'OrderedCollection'
      (documentation)
      (feature_def 'dimensions' : 'Positive' multiplicity ordered nonunique
        (documentation))
      (feature_def 'rank' : 'Natural' multiplicity value)
      (feature_def 'flattenedSize' : 'Positive' multiplicity value)
      (invariant_def
        (result_expr_member)))
    (datatype_def 'Bag' :> 'Collection'
      (documentation))
    (datatype_def 'Set' :> 'UniqueCollection'
      (documentation))
    (datatype_def 'OrderedSet' :> 'OrderedCollection', 'UniqueCollection' intersects 'OrderedCollection', 'UniqueCollection'
      (documentation)
      (feature_def 'elements' multiplicity :>> 'OrderedCollection::elements', 'UniqueCollection::elements' ordered
        (comment)))
    (datatype_def 'List' :> 'OrderedCollection'
      (documentation))
    (datatype_def 'KeyValuePair'
      (documentation)
      (feature_def 'key' : 'Anything' multiplicity ordered nonunique)
      (feature_def 'val' : 'Anything' multiplicity ordered nonunique))
    (datatype_def 'Map' :> 'Collection'
      (documentation)
      (feature_def 'elements' : 'KeyValuePair' multiplicity :>> 'Collection::elements'
        (comment)))
    (datatype_def 'OrderedMap' :> 'Map'
      (documentation)
      (feature_def 'elements' : 'KeyValuePair' multiplicity :>> 'Map::elements' ordered
        (comment)))))
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Collections"))) (name "Collections") (declared-name "Collections")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Collections::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Collections::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Collections::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Collections::*#import3"))) (name "*") (declared-name "*"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Collections::Array"))) (name "Array") (declared-name "Array"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Collections::Bag"))) (name "Bag") (declared-name "Bag"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Collections::Collection"))) (name "Collection") (declared-name "Collection"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Collections::KeyValuePair"))) (name "KeyValuePair") (declared-name "KeyValuePair"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Collections::List"))) (name "List") (declared-name "List"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Collections::Map"))) (name "Map") (declared-name "Map"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Collections::OrderedCollection"))) (name "OrderedCollection") (declared-name "OrderedCollection"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Collections::OrderedMap"))) (name "OrderedMap") (declared-name "OrderedMap"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Collections::OrderedSet"))) (name "OrderedSet") (declared-name "OrderedSet"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Collections::Set"))) (name "Set") (declared-name "Set"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Collections::UniqueCollection"))) (name "UniqueCollection") (declared-name "UniqueCollection"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Collections::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "Collections::size"))) (name "size") (declared-name "size"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Collections::_documentation"))) (to (node (document "d0") (qualified-name "Collections"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
