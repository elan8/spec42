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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "control_functions.md"
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
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ae3684db479e2de911a2d35d859a5e04d1d20a6151c6b9832402070ec2a2e2f1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ControlFunctions"))) (kind "package") (name "ControlFunctions") (declared-name "ControlFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 3603))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "ControlFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 30))))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 9) (character 1)) (end (line 9) (character 38))) (parent (node (document "d0") (qualified-name "ControlFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 37))))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::ScalarValue"))) (kind "import") (name "ScalarValue") (declared-name "ScalarValue") (range (start (line 8) (character 1)) (end (line 8) (character 42))) (parent (node (document "d0") (qualified-name "ControlFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::ScalarValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 41))))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 3603))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::allTrue"))) (kind "kermlDecl") (name "allTrue") (declared-name "allTrue") (range (start (line 94) (character 1)) (end (line 94) (character 109))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::anyTrue"))) (kind "kermlDecl") (name "anyTrue") (declared-name "anyTrue") (range (start (line 99) (character 1)) (end (line 99) (character 108))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::collect"))) (kind "kermlDecl") (name "collect") (declared-name "collect") (range (start (line 52) (character 1)) (end (line 52) (character 224))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::exists"))) (kind "kermlDecl") (name "exists") (declared-name "exists") (range (start (line 88) (character 1)) (end (line 88) (character 177))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::forAll"))) (kind "kermlDecl") (name "forAll") (declared-name "forAll") (range (start (line 82) (character 1)) (end (line 82) (character 178))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 13) (character 1)) (end (line 13) (character 202))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 21) (character 1)) (end (line 21) (character 243))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 28) (character 1)) (end (line 28) (character 199))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 34) (character 1)) (end (line 34) (character 125))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::in"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 40) (character 1)) (end (line 40) (character 123))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::in#kermlDecl"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 46) (character 1)) (end (line 46) (character 128))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::max"))) (kind "import") (name "max") (declared-name "max") (range (start (line 11) (character 1)) (end (line 11) (character 37))) (parent (node (document "d0") (qualified-name "ControlFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarFunctions::max") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 36))))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::maximize"))) (kind "kermlDecl") (name "maximize") (declared-name "maximize") (range (start (line 110) (character 1)) (end (line 110) (character 209))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::min"))) (kind "import") (name "min") (declared-name "min") (range (start (line 10) (character 1)) (end (line 10) (character 37))) (parent (node (document "d0") (qualified-name "ControlFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarFunctions::min") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 36))))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::minimize"))) (kind "kermlDecl") (name "minimize") (declared-name "minimize") (range (start (line 104) (character 1)) (end (line 104) (character 211))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::reduce"))) (kind "kermlDecl") (name "reduce") (declared-name "reduce") (range (start (line 76) (character 1)) (end (line 76) (character 231))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::reject"))) (kind "kermlDecl") (name "reject") (declared-name "reject") (range (start (line 70) (character 1)) (end (line 70) (character 203))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::select"))) (kind "kermlDecl") (name "select") (declared-name "select") (range (start (line 58) (character 1)) (end (line 58) (character 204))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::selectOne"))) (kind "kermlDecl") (name "selectOne") (declared-name "selectOne") (range (start (line 64) (character 1)) (end (line 64) (character 226))) (parent (node (document "d0") (qualified-name "ControlFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ControlFunctions::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (range (start (line 7) (character 16)) (end (line 7) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlFunctions::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 9) (character 16)) (end (line 9) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlFunctions::ScalarValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::ScalarValue") (range (start (line 8) (character 16)) (end (line 8) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlFunctions::max"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarFunctions::max") (range (start (line 11) (character 16)) (end (line 11) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlFunctions::min"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarFunctions::min") (range (start (line 10) (character 16)) (end (line 10) (character 36))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
