# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/IntegerFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package IntegerFunctions {
	doc
	/*
	 * This package defines functions on Integer values, including concrete specializations of the 
	 * general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
	
	function abs specializes RationalFunctions::abs { in x: Integer[1]; return : Natural[1]; }
	
	function '+' specializes RationalFunctions::'+' { in x: Integer[1]; in y: Integer[0..1]; return : Integer[1]; }
	function '-' specializes RationalFunctions::'-' { in x: Integer[1]; in y: Integer[0..1]; return : Integer[1]; }
	function '*' specializes RationalFunctions::'*' { in x: Integer[1]; in y: Integer[1]; return : Integer[1]; }
	function '/' specializes RationalFunctions::'/' { in x: Integer[1]; in y: Integer[1]; return : Rational[1]; }
	function '**' specializes RationalFunctions::'**' { in x: Integer[1]; in y: Natural[1]; return : Integer[1]; }
	function '^' specializes RationalFunctions::'^' { in x: Integer[1]; in y: Natural[1]; return : Integer[1]; }
	function '%' specializes NumericalFunctions::'%' { in x: Integer[1]; in y: Integer[1]; return : Integer[1]; }
	
	function '<' specializes RationalFunctions::'<' { in x: Integer[1]; in y: Integer[1]; return : Boolean[1]; }
	function '>' specializes RationalFunctions::'>' { in x: Integer[1]; in y: Integer[1]; return : Boolean[1]; }
	function '<=' specializes RationalFunctions::'<=' { in x: Integer[1]; in y: Integer[1]; return : Boolean[1]; }
	function '>=' specializes RationalFunctions::'>=' { in x: Integer[1]; in y: Integer[1]; return : Boolean[1]; }

	function max specializes RationalFunctions::max { in x: Integer[1]; in y: Integer[1]; return : Integer[1]; }
	function min specializes RationalFunctions::min { in x: Integer[1]; in y: Integer[1]; return : Integer[1]; }

	function '==' specializes DataFunctions::'==' { in x: Integer[0..1]; in y: Integer[0..1]; return : Boolean[1]; }
	
	function '..' specializes ScalarFunctions::'..' { in lower: Integer[1]; in upper: Integer[1]; return : Integer[0..*]; }
	
	function ToString specializes RationalFunctions::ToString { in x: Integer[1]; return : String[1]; }
	function ToNatural { in x: Integer[1]; return : Natural[1]; }
	function ToInteger { in x: String[1]; return : Integer[1]; }
	
	function sum specializes RationalFunctions::sum { in collection: Integer[0..*]; 
		return : Integer[1] default NumericalFunctions::sum0(collection, 0);
	}
	
	function product specializes RationalFunctions::product { in collection: Integer[0..*];
		return : Integer[1] default NumericalFunctions::product1(collection, 1);
	}
}	
~~~
# EXPECTED
~~~
semantic.unresolved_name 'RationalFunctions::abs'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'RationalFunctions::+'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::-'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::*'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::/'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RationalFunctions::**'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::^'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'NumericalFunctions::%'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::<'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RationalFunctions::>'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RationalFunctions::<='
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RationalFunctions::>='
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RationalFunctions::max'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::min'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'DataFunctions::=='
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::..'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::ToString'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::sum'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::product'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'RationalFunctions::abs'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'RationalFunctions::+'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::-'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::*'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::/'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'RationalFunctions::**'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::^'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'NumericalFunctions::%'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::<'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RationalFunctions::>'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RationalFunctions::<='
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RationalFunctions::>='
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'RationalFunctions::max'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::min'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'DataFunctions::=='
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::..'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::ToString'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::sum'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'RationalFunctions::product'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
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
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,ColonColon,Ident,OpenParen,Ident,Comma,DecimalValue,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,ColonColon,Ident,OpenParen,Ident,Comma,DecimalValue,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'IntegerFunctions'
    (documentation)
    (import_decl public 'ScalarValues::*')
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (feature_def in 'y' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (feature_def in 'y' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (feature_def in 'y' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (feature_def in 'y' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (feature_def in 'y' : 'Natural' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (feature_def in 'y' : 'Natural' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (feature_def in 'y' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (feature_def in 'y' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (feature_def in 'y' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (feature_def in 'y' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (feature_def in 'y' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (feature_def in 'y' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (feature_def in 'y' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (feature_def in 'y' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'lower' : 'Integer' multiplicity)
      (feature_def in 'upper' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Integer' multiplicity)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package IntegerFunctions {
    doc /*
	 * This package defines functions on Integer values, including concrete specializations of the 
	 * general arithmetic and comparison operations.
	 */

    public import ScalarValues::*;

    function abs specializes RationalFunctions::abs { in x: Integer[1]; return : Natural[1]; }

    function '+' specializes RationalFunctions::'+' { in x: Integer[1]; in y: Integer[0..1]; return : Integer[1]; }
    function '-' specializes RationalFunctions::'-' { in x: Integer[1]; in y: Integer[0..1]; return : Integer[1]; }
    function '*' specializes RationalFunctions::'*' { in x: Integer[1]; in y: Integer[1]; return : Integer[1]; }
    function '/' specializes RationalFunctions::'/' { in x: Integer[1]; in y: Integer[1]; return : Rational[1]; }
    function '**' specializes RationalFunctions::'**' { in x: Integer[1]; in y: Natural[1]; return : Integer[1]; }
    function '^' specializes RationalFunctions::'^' { in x: Integer[1]; in y: Natural[1]; return : Integer[1]; }
    function '%' specializes NumericalFunctions::'%' { in x: Integer[1]; in y: Integer[1]; return : Integer[1]; }

    function '<' specializes RationalFunctions::'<' { in x: Integer[1]; in y: Integer[1]; return : Boolean[1]; }
    function '>' specializes RationalFunctions::'>' { in x: Integer[1]; in y: Integer[1]; return : Boolean[1]; }
    function '<=' specializes RationalFunctions::'<=' { in x: Integer[1]; in y: Integer[1]; return : Boolean[1]; }
    function '>=' specializes RationalFunctions::'>=' { in x: Integer[1]; in y: Integer[1]; return : Boolean[1]; }

    function max specializes RationalFunctions::max { in x: Integer[1]; in y: Integer[1]; return : Integer[1]; }
    function min specializes RationalFunctions::min { in x: Integer[1]; in y: Integer[1]; return : Integer[1]; }

    function '==' specializes DataFunctions::'==' { in x: Integer[0..1]; in y: Integer[0..1]; return : Boolean[1]; }

    function '..' specializes ScalarFunctions::'..' { in lower: Integer[1]; in upper: Integer[1]; return : Integer[0..*]; }

    function ToString specializes RationalFunctions::ToString { in x: Integer[1]; return : String[1]; }
    function ToNatural { in x: Integer[1]; return : Natural[1]; }
    function ToInteger { in x: String[1]; return : Integer[1]; }

    function sum specializes RationalFunctions::sum { in collection: Integer[0..*]; 
		return : Integer[1] default NumericalFunctions::sum0(collection, 0);
	}

    function product specializes RationalFunctions::product { in collection: Integer[0..*];
		return : Integer[1] default NumericalFunctions::product1(collection, 1);
	}
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'IntegerFunctions'
      (documentation)
      (namespace_import public -> 'ScalarValues'[unresolved])
      (function_def 'abs' :> 'RationalFunctions::abs'[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Natural'[unresolved]
            (multiplicity_range [1]))))
      (function_def '+' :> 'RationalFunctions::+'[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Integer'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def '-' :> 'RationalFunctions::-'[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Integer'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def '*' :> 'RationalFunctions::*'[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def '/' :> 'RationalFunctions::/'[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1]))))
      (function_def '**' :> 'RationalFunctions::**'[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def '^' :> 'RationalFunctions::^'[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def '%' :> 'NumericalFunctions::%'[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def '<' :> 'RationalFunctions::<'[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '>' :> 'RationalFunctions::>'[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '<=' :> 'RationalFunctions::<='[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '>=' :> 'RationalFunctions::>='[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'max' :> 'RationalFunctions::max'[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'min' :> 'RationalFunctions::min'[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def '==' :> 'DataFunctions::=='[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def in 'y' : 'Integer'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '..' :> 'ScalarFunctions::..'[unresolved]
        (feature_def in 'lower' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'upper' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [0..*]))))
      (function_def 'ToString' :> 'RationalFunctions::ToString'[unresolved]
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'String'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'ToNatural'
        (feature_def in 'x' : 'Integer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Natural'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'ToInteger'
        (feature_def in 'x' : 'String'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'sum' :> 'RationalFunctions::sum'[unresolved]
        (feature_def in 'collection' : 'Integer'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1])
            (feature_value (default =)))))
      (function_def 'product' :> 'RationalFunctions::product'[unresolved]
        (feature_def in 'collection' : 'Integer'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1])
            (feature_value (default =))))))))
~~~
