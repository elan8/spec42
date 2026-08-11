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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "rational_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 15) (end 7 27))
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c506d9cab67d66037728d22fb1812aed6fe7852569165406063d70fa3d06738c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RationalFunctions"))) (kind "package") (name "RationalFunctions") (declared-name "RationalFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 2812))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "RationalFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 15)) (end (line 7) (character 27))))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::ToInteger"))) (kind "kermlDecl") (name "ToInteger") (declared-name "ToInteger") (range (start (line 38) (character 1)) (end (line 38) (character 62))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::ToRational"))) (kind "kermlDecl") (name "ToRational") (declared-name "ToRational") (range (start (line 39) (character 1)) (end (line 39) (character 62))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::ToString"))) (kind "kermlDecl") (name "ToString") (declared-name "ToString") (range (start (line 37) (character 1)) (end (line 37) (character 97))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2812))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::abs"))) (kind "kermlDecl") (name "abs") (declared-name "abs") (range (start (line 13) (character 1)) (end (line 13) (character 89))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::denom"))) (kind "kermlDecl") (name "denom") (declared-name "denom") (range (start (line 11) (character 1)) (end (line 11) (character 61))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::floor"))) (kind "kermlDecl") (name "floor") (declared-name "floor") (range (start (line 34) (character 1)) (end (line 34) (character 92))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 15) (character 1)) (end (line 15) (character 111))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 16) (character 1)) (end (line 16) (character 111))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl10"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 30) (character 1)) (end (line 30) (character 115))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 17) (character 1)) (end (line 17) (character 108))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 18) (character 1)) (end (line 18) (character 108))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 19) (character 1)) (end (line 19) (character 110))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 20) (character 1)) (end (line 20) (character 108))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl6"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 22) (character 1)) (end (line 22) (character 107))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl7"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 23) (character 1)) (end (line 23) (character 107))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl8"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 24) (character 1)) (end (line 24) (character 109))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::function#kermlDecl9"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 25) (character 1)) (end (line 25) (character 109))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::gcd"))) (kind "kermlDecl") (name "gcd") (declared-name "gcd") (range (start (line 32) (character 1)) (end (line 32) (character 75))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::max"))) (kind "kermlDecl") (name "max") (declared-name "max") (range (start (line 27) (character 1)) (end (line 27) (character 108))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::min"))) (kind "kermlDecl") (name "min") (declared-name "min") (range (start (line 28) (character 1)) (end (line 28) (character 108))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::numer"))) (kind "kermlDecl") (name "numer") (declared-name "numer") (range (start (line 10) (character 1)) (end (line 10) (character 61))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::product"))) (kind "kermlDecl") (name "product") (declared-name "product") (range (start (line 45) (character 1)) (end (line 45) (character 172))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::rat"))) (kind "kermlDecl") (name "rat") (declared-name "rat") (range (start (line 9) (character 1)) (end (line 9) (character 83))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::round"))) (kind "kermlDecl") (name "round") (declared-name "round") (range (start (line 35) (character 1)) (end (line 35) (character 92))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
    (element (id (node (document "d0") (qualified-name "RationalFunctions::sum"))) (kind "kermlDecl") (name "sum") (declared-name "sum") (range (start (line 41) (character 1)) (end (line 41) (character 160))) (parent (node (document "d0") (qualified-name "RationalFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RationalFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 7) (character 15)) (end (line 7) (character 27))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
