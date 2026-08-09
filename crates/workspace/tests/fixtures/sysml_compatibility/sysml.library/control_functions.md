# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/ControlFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package ControlFunctions {
	doc
	/*
	 * This package defines functions that correspond to operators in the KerML expression notation 
	 * for which one or more operands are expressions whose evaluation is determined by another operand.
	 */

	private import Base::Anything;
	private import ScalarValues::ScalarValue;
	private import ScalarValues::Boolean;
	private import ScalarFunctions::min;
	private import ScalarFunctions::max;
	
	abstract function '.' {
		in feature source : Anything[0..*] nonunique {
	  		abstract feature target : Anything[0..*] nonunique;
	  	}	  	
	  	private feature chain chains source.target;
	    chain
	}
	
	abstract function 'if' { 
		in test: Boolean[1];
		in expr thenValue[0..1] { return : Anything[0..*] ordered nonunique; }
		in expr elseValue[0..1] { return : Anything[0..*] ordered nonunique; }
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function '??' {
		in firstValue: Anything[0..*] ordered nonunique;
		in expr secondValue[0..1] { return : Anything[0..*] ordered nonunique; }
		return : Anything[0..*] ordered nonunique;
	}
	
	function 'and' {
		in firstValue: Boolean[1];
		in expr secondValue[0..1] { return : Boolean[1]; }
		return : Boolean[1];
	}
	
	function 'or'{
		in firstValue: Boolean[1];
		in expr secondValue[0..1] { return : Boolean[1]; }
		return : Boolean[1];
	}
	
	function 'implies'{
		in firstValue: Boolean[1];
		in expr secondValue[0..1] { return : Boolean[1]; }
		return : Boolean[1];
	}
	
	abstract function collect { 
		in collection: Anything[0..*] ordered nonunique;
		in expr mapper[0..*] { in argument: Anything[1]; return : Anything[0..*] ordered nonunique; }
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function select { 
		in collection: Anything[0..*] ordered nonunique; 
		in expr selector[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Anything[0..*] ordered nonunique;
	}
	
	function selectOne { 
		in collection: Anything[0..*] ordered nonunique;
		in expr selector1[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Anything[0..1] = collection->select {in x; selector1(x)}#(1);
	}
	
	abstract function reject{ 
		in collection: Anything[0..*] ordered nonunique; 
		in expr rejector[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function reduce { 
		in collection: Anything[0..*] ordered nonunique; 
		in expr reducer[0..*] { in firstArg: Anything[1]; in secondArg: Anything[1]; return : Anything[1]; }
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function forAll { 
		in collection: Anything[0..*] ordered nonunique; 
		in expr test[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Boolean[1];
	}
	
	abstract function exists { 
		in collection: Anything[0..*] ordered nonunique;
		in expr test[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Boolean[1];
	}
	
	function allTrue {
		in collection: Boolean[0..*]; 
		return : Boolean[1] = collection->forAll {in x; x};
	}
	
	function anyTrue {
		in collection: Boolean[0..*];
		return : Boolean[1] = collection->exists {in x; x};
	}
	
	function minimize {
		in collection: ScalarValue[1..*];
		in expr fn[0..*] { in argument: ScalarValue[1]; return : ScalarValue[1]; }
		return : ScalarValue[1] = collection->collect {in x; fn(x)}->reduce min;
	}
	
	function maximize { 
		in collection: ScalarValue[1..*];
		in expr fn[0..*] { in argument: ScalarValue[1]; return : ScalarValue[1]; }
		return : ScalarValue = collection->collect {in x; fn(x)}->reduce max;
	}
	
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,OpenCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,
CloseCurly,
KwPrivate,KwFeature,Ident,KwChains,Ident,Dot,Ident,Semicolon,
Ident,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,CloseCurly,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
CloseCurly,
KwFunction,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFunction,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFunction,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,OpenParen,Ident,CloseParen,CloseCurly,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,CloseCurly,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,CloseCurly,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,OpenParen,Ident,CloseParen,CloseCurly,Arrow,Ident,Ident,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwReturn,Colon,Ident,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,OpenParen,Ident,CloseParen,CloseCurly,Arrow,Ident,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ControlFunctions'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'ScalarValues::ScalarValue')
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'ScalarFunctions::min')
    (import_decl private 'ScalarFunctions::max')
    (function_def
      (feature_def in 'source' : 'Anything' multiplicity nonunique
        (feature_def abstract 'target' : 'Anything' multiplicity nonunique))
      (feature_def private 'chain' chains 'source.target')
      (result_expr_member))
    (function_def
      (feature_def in 'test' : 'Boolean' multiplicity)
      (expression_usage
        (return_member))
      (expression_usage
        (return_member))
      (return_member))
    (function_def
      (feature_def in 'firstValue' : 'Anything' multiplicity ordered nonunique)
      (expression_usage
        (return_member))
      (return_member))
    (function_def
      (feature_def in 'firstValue' : 'Boolean' multiplicity)
      (expression_usage
        (return_member))
      (return_member))
    (function_def
      (feature_def in 'firstValue' : 'Boolean' multiplicity)
      (expression_usage
        (return_member))
      (return_member))
    (function_def
      (feature_def in 'firstValue' : 'Boolean' multiplicity)
      (expression_usage
        (return_member))
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Anything' multiplicity ordered nonunique)
      (expression_usage
        (feature_def in 'argument' : 'Anything' multiplicity)
        (return_member))
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Anything' multiplicity ordered nonunique)
      (expression_usage
        (feature_def in 'argument' : 'Anything' multiplicity)
        (return_member))
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Anything' multiplicity ordered nonunique)
      (expression_usage
        (feature_def in 'argument' : 'Anything' multiplicity)
        (return_member))
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Anything' multiplicity ordered nonunique)
      (expression_usage
        (feature_def in 'argument' : 'Anything' multiplicity)
        (return_member))
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Anything' multiplicity ordered nonunique)
      (expression_usage
        (feature_def in 'firstArg' : 'Anything' multiplicity)
        (feature_def in 'secondArg' : 'Anything' multiplicity)
        (return_member))
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Anything' multiplicity ordered nonunique)
      (expression_usage
        (feature_def in 'argument' : 'Anything' multiplicity)
        (return_member))
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Anything' multiplicity ordered nonunique)
      (expression_usage
        (feature_def in 'argument' : 'Anything' multiplicity)
        (return_member))
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Boolean' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Boolean' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'collection' : 'ScalarValue' multiplicity)
      (expression_usage
        (feature_def in 'argument' : 'ScalarValue' multiplicity)
        (return_member))
      (return_member))
    (function_def
      (feature_def in 'collection' : 'ScalarValue' multiplicity)
      (expression_usage
        (feature_def in 'argument' : 'ScalarValue' multiplicity)
        (return_member))
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package ControlFunctions {
	doc
	/*
	 * This package defines functions that correspond to operators in the KerML expression notation 
	 * for which one or more operands are expressions whose evaluation is determined by another operand.
	 */

	private import Base::Anything;
	private import ScalarValues::ScalarValue;
	private import ScalarValues::Boolean;
	private import ScalarFunctions::min;
	private import ScalarFunctions::max;
	
	abstract function '.' {
		in feature source : Anything[0..*] nonunique {
	  		abstract feature target : Anything[0..*] nonunique;
	  	}	  	
	  	private feature chain chains source.target;
	    chain
	}
	
	abstract function 'if' { 
		in test: Boolean[1];
		in expr thenValue[0..1] { return : Anything[0..*] ordered nonunique; }
		in expr elseValue[0..1] { return : Anything[0..*] ordered nonunique; }
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function '??' {
		in firstValue: Anything[0..*] ordered nonunique;
		in expr secondValue[0..1] { return : Anything[0..*] ordered nonunique; }
		return : Anything[0..*] ordered nonunique;
	}
	
	function 'and' {
		in firstValue: Boolean[1];
		in expr secondValue[0..1] { return : Boolean[1]; }
		return : Boolean[1];
	}
	
	function 'or'{
		in firstValue: Boolean[1];
		in expr secondValue[0..1] { return : Boolean[1]; }
		return : Boolean[1];
	}
	
	function 'implies'{
		in firstValue: Boolean[1];
		in expr secondValue[0..1] { return : Boolean[1]; }
		return : Boolean[1];
	}
	
	abstract function collect { 
		in collection: Anything[0..*] ordered nonunique;
		in expr mapper[0..*] { in argument: Anything[1]; return : Anything[0..*] ordered nonunique; }
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function select { 
		in collection: Anything[0..*] ordered nonunique; 
		in expr selector[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Anything[0..*] ordered nonunique;
	}
	
	function selectOne { 
		in collection: Anything[0..*] ordered nonunique;
		in expr selector1[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Anything[0..1] = collection->select {in x; selector1(x)}#(1);
	}
	
	abstract function reject{ 
		in collection: Anything[0..*] ordered nonunique; 
		in expr rejector[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function reduce { 
		in collection: Anything[0..*] ordered nonunique; 
		in expr reducer[0..*] { in firstArg: Anything[1]; in secondArg: Anything[1]; return : Anything[1]; }
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function forAll { 
		in collection: Anything[0..*] ordered nonunique; 
		in expr test[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Boolean[1];
	}
	
	abstract function exists { 
		in collection: Anything[0..*] ordered nonunique;
		in expr test[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Boolean[1];
	}
	
	function allTrue {
		in collection: Boolean[0..*]; 
		return : Boolean[1] = collection->forAll {in x; x};
	}
	
	function anyTrue {
		in collection: Boolean[0..*];
		return : Boolean[1] = collection->exists {in x; x};
	}
	
	function minimize {
		in collection: ScalarValue[1..*];
		in expr fn[0..*] { in argument: ScalarValue[1]; return : ScalarValue[1]; }
		return : ScalarValue[1] = collection->collect {in x; fn(x)}->reduce min;
	}
	
	function maximize { 
		in collection: ScalarValue[1..*];
		in expr fn[0..*] { in argument: ScalarValue[1]; return : ScalarValue[1]; }
		return : ScalarValue = collection->collect {in x; fn(x)}->reduce max;
	}
	
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ControlFunctions"))) (name "ControlFunctions") (declared-name "ControlFunctions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ControlFunctions::Anything"))) (name "Anything") (declared-name "Anything"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ControlFunctions::Boolean"))) (name "Boolean") (declared-name "Boolean"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ControlFunctions::ScalarValue"))) (name "ScalarValue") (declared-name "ScalarValue"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ControlFunctions::_documentation"))) (name ""))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::allTrue"))) (name "allTrue") (declared-name "allTrue"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::anyTrue"))) (name "anyTrue") (declared-name "anyTrue"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::collect"))) (name "collect") (declared-name "collect"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::exists"))) (name "exists") (declared-name "exists"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::forAll"))) (name "forAll") (declared-name "forAll"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::function"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::function#kermlDecl"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::function#kermlDecl2"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::function#kermlDecl3"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::in"))) (name "in") (declared-name "in"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::in#kermlDecl"))) (name "in") (declared-name "in"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ControlFunctions::max"))) (name "max") (declared-name "max"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::maximize"))) (name "maximize") (declared-name "maximize"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ControlFunctions::min"))) (name "min") (declared-name "min"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::minimize"))) (name "minimize") (declared-name "minimize"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::reduce"))) (name "reduce") (declared-name "reduce"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::reject"))) (name "reject") (declared-name "reject"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::select"))) (name "select") (declared-name "select"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ControlFunctions::selectOne"))) (name "selectOne") (declared-name "selectOne"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ControlFunctions::_documentation"))) (to (node (document "d0") (qualified-name "ControlFunctions"))))
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
  (document "sysml.library/control_functions.md"
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
        (range (start 8 16) (end 8 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 36))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 21 1) (end 21 243))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 46 1) (end 46 128))
      )
    )
  )
)
~~~
