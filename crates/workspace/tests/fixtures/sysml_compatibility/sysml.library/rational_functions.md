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
    doc /*
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
(model
  (namespace
    (library_package 'RationalFunctions'
      (documentation)
      (namespace_import public -> 'ScalarValues'[unresolved])
      (function_def 'rat'
        (feature_def in 'numer' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'denum' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'numer'
        (feature_def in 'rat' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'denom'
        (feature_def in 'rat' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'abs' :> 'RealFunctions::abs'[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1]))))
      (function_def '+' :> 'RealFunctions::+'[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Rational'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1]))))
      (function_def '-' :> 'RealFunctions::-'[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Rational'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1]))))
      (function_def '*' :> 'RealFunctions::*'[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1]))))
      (function_def '/' :> 'RealFunctions::/'[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1]))))
      (function_def '**' :> 'RealFunctions::**'[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1]))))
      (function_def '^' :> 'RealFunctions::^'[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1]))))
      (function_def '<' :> 'RealFunctions::<'[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '>' :> 'RealFunctions::>'[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '<=' :> 'RealFunctions::<='[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '>=' :> 'RealFunctions::>='[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'max' :> 'RealFunctions::max'[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'min' :> 'RealFunctions::min'[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1]))))
      (function_def '==' :> 'RealFunctions::=='[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def in 'y' : 'Rational'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'gcd'
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'floor' :> 'RealFunctions::floor'[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'round' :> 'RealFunctions::round'[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'ToString' :> 'RealFunctions::ToString'[unresolved]
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'String'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'ToInteger'
        (feature_def in 'x' : 'Rational'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'ToRational'
        (feature_def in 'x' : 'String'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'sum' :> 'RealFunctions::sum'[unresolved]
        (feature_def in 'collection' : 'Rational'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1])
            (feature_value (default =)))))
      (function_def 'product' :> 'RealFunctions::product'[unresolved]
        (feature_def in 'collection' : 'Rational'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1])
            (feature_value (default =))))))))
~~~
