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
# EXPECTED
~~~
semantic.unresolved_name 'BaseFunctions::=='
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'OrderedCollection'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'OrderedCollection'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'OrderedCollection'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'BaseFunctions::#'
semantic.unresolved_name 'OrderedCollection'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'BaseFunctions::#'
semantic.unresolved_name 'Array'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Array'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'BaseFunctions::=='
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Collection'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'OrderedCollection'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'OrderedCollection'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'OrderedCollection'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'BaseFunctions::#'
semantic.unresolved_name 'OrderedCollection'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'BaseFunctions::#'
semantic.unresolved_name 'Array'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Array'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,Arrow,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,ColonColon,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,ColonColon,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,ColonColon,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,OpenParen,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Eq,Ident,ColonColon,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Eq,Ident,ColonColon,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Eq,Ident,ColonColon,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
LineComment,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Eq,OpenParen,Ident,Dot,Ident,KwAs,Ident,CloseParen,Hash,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,Ident,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwPrivate,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
LineComment,
KwPrivate,KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIf,Ident,LtEq,DecimalValue,Question,Ident,Hash,OpenParen,DecimalValue,CloseParen,KwElse,Ident,Dot,Ident,Hash,OpenParen,Ident,CloseParen,Star,OpenParen,Ident,OpenParen,Ident,Comma,Ident,Minus,DecimalValue,Comma,Ident,CloseParen,Minus,DecimalValue,CloseParen,Plus,Ident,Hash,OpenParen,Ident,CloseParen,
CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Eq,
KwIf,Ident,EqEq,DecimalValue,KwOr,OpenParen,DecimalValue,DotDot,Ident,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,Hash,OpenParen,Ident,CloseParen,CloseAngle,Ident,Dot,Ident,Hash,OpenParen,Ident,CloseParen,CloseCurly,Question,KwNull,
KwElse,Ident,Dot,Ident,Hash,OpenParen,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'CollectionFunctions'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'ScalarValues::*')
    (import_decl private 'SequenceFunctions::equals')
    (import_decl private 'SequenceFunctions::includes')
    (import_decl private 'ControlFunctions::exists')
    (import_decl public 'Collections::*')
    (function_def
      (feature_def in 'col1' : 'Collection' multiplicity)
      (feature_def in 'col2' : 'Collection' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'col' : 'Collection' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'col' : 'Collection' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'col' : 'Collection' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'col' : 'Collection' multiplicity)
      (feature_def in 'values' : 'Anything' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'col1' : 'Collection' multiplicity)
      (feature_def in 'col2' : 'Collection' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'col' : 'OrderedCollection' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'col' : 'OrderedCollection' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'col' : 'OrderedCollection' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'col' : 'OrderedCollection' multiplicity)
      (feature_def in 'index' : 'Positive' multiplicity)
      (line_comment)
      (return_member))
    (function_def
      (feature_def in 'arr' : 'Array' multiplicity)
      (feature_def in 'indexes' : 'Positive' multiplicity ordered nonunique)
      (feature_def private 'n' : 'Natural' multiplicity value)
      (line_comment)
      (function_def
        (feature_def in 'arr' : 'Array' multiplicity)
        (feature_def in 'i' : 'Natural')
        (feature_def in 'indexes' : 'Positive' multiplicity)
        (result_expr_member))
      (return_member))))
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "CollectionFunctions"))) (name "CollectionFunctions") (declared-name "CollectionFunctions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "CollectionFunctions::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "CollectionFunctions::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "CollectionFunctions::Anything"))) (name "Anything") (declared-name "Anything"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "CollectionFunctions::_documentation"))) (name ""))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "CollectionFunctions::contains"))) (name "contains") (declared-name "contains"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "CollectionFunctions::containsAll"))) (name "containsAll") (declared-name "containsAll"))
        (element (kind "import") (id (node (document "d0") (qualified-name "CollectionFunctions::equals"))) (name "equals") (declared-name "equals"))
        (element (kind "import") (id (node (document "d0") (qualified-name "CollectionFunctions::exists"))) (name "exists") (declared-name "exists"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "CollectionFunctions::function"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "CollectionFunctions::function#kermlDecl"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "CollectionFunctions::function#kermlDecl2"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "CollectionFunctions::head"))) (name "head") (declared-name "head"))
        (element (kind "import") (id (node (document "d0") (qualified-name "CollectionFunctions::includes"))) (name "includes") (declared-name "includes"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "CollectionFunctions::isEmpty"))) (name "isEmpty") (declared-name "isEmpty"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "CollectionFunctions::last"))) (name "last") (declared-name "last"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "CollectionFunctions::notEmpty"))) (name "notEmpty") (declared-name "notEmpty"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "CollectionFunctions::size"))) (name "size") (declared-name "size"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "CollectionFunctions::tail"))) (name "tail") (declared-name "tail"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CollectionFunctions::_documentation"))) (to (node (document "d0") (qualified-name "CollectionFunctions"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/collection_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 1) (end 7 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 1) (end 8 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 1) (end 9 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 1) (end 10 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 1) (end 11 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 1) (end 12 30))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 50 1) (end 50 273))
      )
    )
  )
)
~~~
