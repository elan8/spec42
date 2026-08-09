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
    doc /*
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
(model
  (namespace
    (library_package 'ControlFunctions'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (membership_import private -> 'ScalarValues::ScalarValue'[unresolved])
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'ScalarFunctions::min'[unresolved])
      (membership_import private -> 'ScalarFunctions::max'[unresolved])
      (function_def abstract '.'
        (feature_def in 'source' : 'Anything'[unresolved]
          (multiplicity_range [0..*])
          (feature_def abstract 'target' : 'Anything'[unresolved]
            (multiplicity_range [0..*])))
        (feature_def 'chain' :> 'ControlFunctions::.::source'[feature_def] :> 'ControlFunctions::.::source::target'[feature_def])
        (result_expr_membership))
      (function_def abstract 'if'
        (feature_def in 'test' : 'Boolean'[unresolved]
          (multiplicity_range [1]))
        (expression_usage in 'thenValue'
          (multiplicity_range [0..1])
          (return_parameter_membership
            (feature_def out ordered : 'Anything'[unresolved]
              (multiplicity_range [0..*]))))
        (expression_usage in 'elseValue'
          (multiplicity_range [0..1])
          (return_parameter_membership
            (feature_def out ordered : 'Anything'[unresolved]
              (multiplicity_range [0..*]))))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*]))))
      (function_def abstract '??'
        (feature_def in ordered 'firstValue' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (expression_usage in 'secondValue'
          (multiplicity_range [0..1])
          (return_parameter_membership
            (feature_def out ordered : 'Anything'[unresolved]
              (multiplicity_range [0..*]))))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*]))))
      (function_def 'and'
        (feature_def in 'firstValue' : 'Boolean'[unresolved]
          (multiplicity_range [1]))
        (expression_usage in 'secondValue'
          (multiplicity_range [0..1])
          (return_parameter_membership
            (feature_def out : 'Boolean'[unresolved]
              (multiplicity_range [1]))))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'or'
        (feature_def in 'firstValue' : 'Boolean'[unresolved]
          (multiplicity_range [1]))
        (expression_usage in 'secondValue'
          (multiplicity_range [0..1])
          (return_parameter_membership
            (feature_def out : 'Boolean'[unresolved]
              (multiplicity_range [1]))))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'implies'
        (feature_def in 'firstValue' : 'Boolean'[unresolved]
          (multiplicity_range [1]))
        (expression_usage in 'secondValue'
          (multiplicity_range [0..1])
          (return_parameter_membership
            (feature_def out : 'Boolean'[unresolved]
              (multiplicity_range [1]))))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'collect'
        (feature_def in ordered 'collection' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (expression_usage in 'mapper'
          (multiplicity_range [0..*])
          (feature_def in 'argument' : 'Anything'[unresolved]
            (multiplicity_range [1]))
          (return_parameter_membership
            (feature_def out ordered : 'Anything'[unresolved]
              (multiplicity_range [0..*]))))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*]))))
      (function_def abstract 'select'
        (feature_def in ordered 'collection' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (expression_usage in 'selector'
          (multiplicity_range [0..*])
          (feature_def in 'argument' : 'Anything'[unresolved]
            (multiplicity_range [1]))
          (return_parameter_membership
            (feature_def out : 'Boolean'[unresolved]
              (multiplicity_range [1]))))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*]))))
      (function_def 'selectOne'
        (feature_def in ordered 'collection' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (expression_usage in 'selector1'
          (multiplicity_range [0..*])
          (feature_def in 'argument' : 'Anything'[unresolved]
            (multiplicity_range [1]))
          (return_parameter_membership
            (feature_def out : 'Boolean'[unresolved]
              (multiplicity_range [1]))))
        (return_parameter_membership
          (feature_def out : 'Anything'[unresolved]
            (multiplicity_range [0..1])
            (feature_value (=)))))
      (function_def abstract 'reject'
        (feature_def in ordered 'collection' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (expression_usage in 'rejector'
          (multiplicity_range [0..*])
          (feature_def in 'argument' : 'Anything'[unresolved]
            (multiplicity_range [1]))
          (return_parameter_membership
            (feature_def out : 'Boolean'[unresolved]
              (multiplicity_range [1]))))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*]))))
      (function_def abstract 'reduce'
        (feature_def in ordered 'collection' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (expression_usage in 'reducer'
          (multiplicity_range [0..*])
          (feature_def in 'firstArg' : 'Anything'[unresolved]
            (multiplicity_range [1]))
          (feature_def in 'secondArg' : 'Anything'[unresolved]
            (multiplicity_range [1]))
          (return_parameter_membership
            (feature_def out : 'Anything'[unresolved]
              (multiplicity_range [1]))))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*]))))
      (function_def abstract 'forAll'
        (feature_def in ordered 'collection' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (expression_usage in 'test'
          (multiplicity_range [0..*])
          (feature_def in 'argument' : 'Anything'[unresolved]
            (multiplicity_range [1]))
          (return_parameter_membership
            (feature_def out : 'Boolean'[unresolved]
              (multiplicity_range [1]))))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'exists'
        (feature_def in ordered 'collection' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (expression_usage in 'test'
          (multiplicity_range [0..*])
          (feature_def in 'argument' : 'Anything'[unresolved]
            (multiplicity_range [1]))
          (return_parameter_membership
            (feature_def out : 'Boolean'[unresolved]
              (multiplicity_range [1]))))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'allTrue'
        (feature_def in 'collection' : 'Boolean'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'anyTrue'
        (feature_def in 'collection' : 'Boolean'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'minimize'
        (feature_def in 'collection' : 'ScalarValue'[unresolved]
          (multiplicity_range [1..*]))
        (expression_usage in 'fn'
          (multiplicity_range [0..*])
          (feature_def in 'argument' : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))
          (return_parameter_membership
            (feature_def out : 'ScalarValue'[unresolved]
              (multiplicity_range [1]))))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'maximize'
        (feature_def in 'collection' : 'ScalarValue'[unresolved]
          (multiplicity_range [1..*]))
        (expression_usage in 'fn'
          (multiplicity_range [0..*])
          (feature_def in 'argument' : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))
          (return_parameter_membership
            (feature_def out : 'ScalarValue'[unresolved]
              (multiplicity_range [1]))))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (feature_value (=))))))))
~~~
