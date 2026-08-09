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
    doc /*
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
(model
  (namespace
    (library_package 'CollectionFunctions'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (namespace_import private -> 'ScalarValues'[unresolved])
      (membership_import private -> 'SequenceFunctions::equals'[unresolved])
      (membership_import private -> 'SequenceFunctions::includes'[unresolved])
      (membership_import private -> 'ControlFunctions::exists'[unresolved])
      (namespace_import public -> 'Collections'[unresolved])
      (function_def '==' :> 'BaseFunctions::=='[unresolved]
        (feature_def in 'col1' : 'Collection'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def in 'col2' : 'Collection'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'size'
        (feature_def in 'col' : 'Collection'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Natural'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'isEmpty'
        (feature_def in 'col' : 'Collection'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'notEmpty'
        (feature_def in 'col' : 'Collection'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'contains'
        (feature_def in 'col' : 'Collection'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'values' : 'Anything'[unresolved]
          (multiplicity_range [*]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'containsAll'
        (feature_def in 'col1' : 'Collection'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'col2' : 'Collection'[unresolved]
          (multiplicity_range [2]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'head'
        (feature_def in 'col' : 'OrderedCollection'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Anything'[unresolved]
            (multiplicity_range [0..1])
            (feature_value (=)))))
      (function_def 'tail'
        (feature_def in 'col' : 'OrderedCollection'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*])
            (feature_value (=)))))
      (function_def 'last'
        (feature_def in 'col' : 'OrderedCollection'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Anything'[unresolved]
            (multiplicity_range [0..1])
            (feature_value (=)))))
      (function_def '#' :> 'BaseFunctions::#'[unresolved]
        (feature_def in 'col' : 'OrderedCollection'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'index' : 'Positive'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Anything'[unresolved]
            (multiplicity_range [0..1])
            (feature_value (=)))))
      (function_def 'array#' :> 'BaseFunctions::#'[unresolved]
        (feature_def in 'arr' : 'Array'[unresolved]
          (multiplicity_range [1]))
        (feature_def in ordered 'indexes' : 'Positive'[unresolved]
          (multiplicity_range [?]))
        (feature_def 'n' : 'Natural'[unresolved]
          (multiplicity_range [1])
          (feature_value (=)))
        (function_def 'index'
          (feature_def in 'arr' : 'Array'[unresolved]
            (multiplicity_range [1]))
          (feature_def in 'i' : 'Natural'[unresolved])
          (feature_def in 'indexes' : 'Positive'[unresolved]
            (multiplicity_range [1..*]))
          (result_expr_membership))
        (return_parameter_membership
          (feature_def out : 'Anything'[unresolved]
            (multiplicity_range [0..1])
            (feature_value (=))))))))
~~~
