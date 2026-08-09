# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/RationalFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package RationalFunctions {
	doc
	/*
	 * This package defines Functions on Rational values, including concrete specializations of the 
	 * general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
	
	function rat { in numer: Integer[1]; in denum: Integer[1]; return : Rational[1]; }
	function numer { in rat: Rational[1]; return : Integer[1]; }
	function denom { in rat: Rational[1]; return : Integer[1]; }
	
	function abs specializes RealFunctions::abs { in x: Rational[1]; return : Rational[1]; }

	function '+' specializes RealFunctions::'+' { in x: Rational[1]; in y: Rational[0..1]; return : Rational[1]; }
	function '-' specializes RealFunctions::'-' { in x: Rational[1]; in y: Rational[0..1]; return : Rational[1]; }
	function '*' specializes RealFunctions::'*' { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	function '/' specializes RealFunctions::'/' { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	function '**' specializes RealFunctions::'**' { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	function '^' specializes RealFunctions::'^' { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	
	function '<' specializes RealFunctions::'<' { in x: Rational[1]; in y: Rational[1]; return : Boolean[1]; }
	function '>' specializes RealFunctions::'>' { in x: Rational[1]; in y: Rational[1]; return : Boolean[1]; }
	function '<=' specializes RealFunctions::'<=' { in x: Rational[1]; in y: Rational[1]; return : Boolean[1]; }
	function '>=' specializes RealFunctions::'>=' { in x: Rational[1]; in y: Rational[1]; return : Boolean[1]; }

	function max specializes RealFunctions::max { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	function min specializes RealFunctions::min { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }

	function '==' specializes RealFunctions::'==' { in x: Rational[0..1]; in y: Rational[0..1]; return : Boolean[1]; }
	
	function gcd{ in x: Rational[1]; in y: Rational[1]; return : Integer[1]; }
		
	function floor specializes RealFunctions::floor { in x: Rational[1]; return : Integer[1]; }
	function round specializes RealFunctions::round { in x: Rational[1]; return : Integer[1]; }
	
	function ToString specializes RealFunctions::ToString { in x: Rational[1]; return : String[1]; }
	function ToInteger{ in x: Rational[1]; return : Integer[1]; }
	function ToRational{ in x: String[1]; return : Rational[1]; }
	
	function sum specializes RealFunctions::sum { in collection: Rational[0..*];
		return : Rational[1] default NumericalFunctions::sum0(collection, rat(0, 1));
	}
	
	function product specializes RealFunctions::product { in collection: Rational[0..*];
		return : Rational[1] default NumericalFunctions::product1(collection, rat(1, 1));
	}	
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RealFunctions::abs'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::+'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::-'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::*'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::/'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::**'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::^'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::<'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RealFunctions::>'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RealFunctions::<='
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RealFunctions::>='
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RealFunctions::max'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::min'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::=='
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RealFunctions::floor'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RealFunctions::round'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RealFunctions::ToString'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::sum'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::product'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RealFunctions::abs'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::+'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::-'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::*'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::/'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::**'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::^'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::<'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RealFunctions::>'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RealFunctions::<='
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RealFunctions::>='
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RealFunctions::max'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::min'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::=='
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RealFunctions::floor'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RealFunctions::round'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RealFunctions::ToString'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::sum'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RealFunctions::product'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'Rational'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,ColonColon,Ident,OpenParen,Ident,Comma,Ident,OpenParen,DecimalValue,Comma,DecimalValue,CloseParen,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,ColonColon,Ident,OpenParen,Ident,Comma,Ident,OpenParen,DecimalValue,Comma,DecimalValue,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'RationalFunctions'
    (documentation)
    (import_decl public 'ScalarValues::*')
    (function_def
      (feature_def in 'numer' : 'Integer' multiplicity)
      (feature_def in 'denum' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'rat' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'rat' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (feature_def in 'y' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (feature_def in 'y' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (feature_def in 'y' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (feature_def in 'y' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (feature_def in 'y' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (feature_def in 'y' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (feature_def in 'y' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (feature_def in 'y' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (feature_def in 'y' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (feature_def in 'y' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (feature_def in 'y' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (feature_def in 'y' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (feature_def in 'y' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (feature_def in 'y' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Rational' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Rational' multiplicity)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package RationalFunctions {
	doc
	/*
	 * This package defines Functions on Rational values, including concrete specializations of the 
	 * general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
	
	function rat { in numer: Integer[1]; in denum: Integer[1]; return : Rational[1]; }
	function numer { in rat: Rational[1]; return : Integer[1]; }
	function denom { in rat: Rational[1]; return : Integer[1]; }
	
	function abs specializes RealFunctions::abs { in x: Rational[1]; return : Rational[1]; }

	function '+' specializes RealFunctions::'+' { in x: Rational[1]; in y: Rational[0..1]; return : Rational[1]; }
	function '-' specializes RealFunctions::'-' { in x: Rational[1]; in y: Rational[0..1]; return : Rational[1]; }
	function '*' specializes RealFunctions::'*' { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	function '/' specializes RealFunctions::'/' { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	function '**' specializes RealFunctions::'**' { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	function '^' specializes RealFunctions::'^' { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	
	function '<' specializes RealFunctions::'<' { in x: Rational[1]; in y: Rational[1]; return : Boolean[1]; }
	function '>' specializes RealFunctions::'>' { in x: Rational[1]; in y: Rational[1]; return : Boolean[1]; }
	function '<=' specializes RealFunctions::'<=' { in x: Rational[1]; in y: Rational[1]; return : Boolean[1]; }
	function '>=' specializes RealFunctions::'>=' { in x: Rational[1]; in y: Rational[1]; return : Boolean[1]; }

	function max specializes RealFunctions::max { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }
	function min specializes RealFunctions::min { in x: Rational[1]; in y: Rational[1]; return : Rational[1]; }

	function '==' specializes RealFunctions::'==' { in x: Rational[0..1]; in y: Rational[0..1]; return : Boolean[1]; }
	
	function gcd{ in x: Rational[1]; in y: Rational[1]; return : Integer[1]; }
		
	function floor specializes RealFunctions::floor { in x: Rational[1]; return : Integer[1]; }
	function round specializes RealFunctions::round { in x: Rational[1]; return : Integer[1]; }
	
	function ToString specializes RealFunctions::ToString { in x: Rational[1]; return : String[1]; }
	function ToInteger{ in x: Rational[1]; return : Integer[1]; }
	function ToRational{ in x: String[1]; return : Rational[1]; }
	
	function sum specializes RealFunctions::sum { in collection: Rational[0..*];
		return : Rational[1] default NumericalFunctions::sum0(collection, rat(0, 1));
	}
	
	function product specializes RealFunctions::product { in collection: Rational[0..*];
		return : Rational[1] default NumericalFunctions::product1(collection, rat(1, 1));
	}	
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "RationalFunctions"))) (name "RationalFunctions") (declared-name "RationalFunctions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "RationalFunctions::*"))) (name "*") (declared-name "*"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::ToInteger"))) (name "ToInteger") (declared-name "ToInteger"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::ToRational"))) (name "ToRational") (declared-name "ToRational"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::ToString"))) (name "ToString") (declared-name "ToString"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "RationalFunctions::_documentation"))) (name ""))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::abs"))) (name "abs") (declared-name "abs"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::denom"))) (name "denom") (declared-name "denom"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::floor"))) (name "floor") (declared-name "floor"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::function"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl10"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl2"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl3"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl4"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl5"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl6"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl7"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl8"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl9"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::gcd"))) (name "gcd") (declared-name "gcd"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::max"))) (name "max") (declared-name "max"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::min"))) (name "min") (declared-name "min"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::numer"))) (name "numer") (declared-name "numer"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::product"))) (name "product") (declared-name "product"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::rat"))) (name "rat") (declared-name "rat"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::round"))) (name "round") (declared-name "round"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "RationalFunctions::sum"))) (name "sum") (declared-name "sum"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RationalFunctions::_documentation"))) (to (node (document "d0") (qualified-name "RationalFunctions"))))
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
  (document "sysml.library/rational_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 1) (end 7 31))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 16 1) (end 16 111))
      )
    )
  )
)
~~~
