# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/RealFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package RealFunctions {
	doc
	/*
	 * This package defines Functions on Real values, including concrete specializations of the 
	 * general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
	
	function re :> ComplexFunctions::re{ in x: Real[1]; 
        return : Real[1] = x;
	}
	function im :> ComplexFunctions::im{ in x: Real[1]; 
        return : Real[1] = 0.0;
	}
	
	function abs specializes ComplexFunctions::abs { in x: Real[1]; return : Real[1]; }
	function arg specializes ComplexFunctions::arg { in x: Real[1]; 
        return : Real[1] = 0.0;
	}

	function '+' specializes ComplexFunctions::'+' { in x: Real[1]; in y: Real[0..1]; return : Real[1]; }
	function '-' specializes ComplexFunctions::'-' { in x: Real[1]; in y: Real[0..1]; return : Real[1]; }
	function '*' specializes ComplexFunctions::'*' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	function '/' specializes ComplexFunctions::'/' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	function '**' specializes ComplexFunctions::'**' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	function '^' specializes ComplexFunctions::'^' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	
	function '<' specializes NumericalFunctions::'<' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }
	function '>' specializes NumericalFunctions::'>' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }
	function '<=' specializes NumericalFunctions::'<=' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }
	function '>=' specializes NumericalFunctions::'>=' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }

	function max specializes NumericalFunctions::max { in x: Real[1]; in y: Real[1]; return : Real[1]; }
	function min specializes NumericalFunctions::min { in x: Real[1]; in y: Real[1]; return : Real[1]; }

	function '==' specializes ComplexFunctions::'==' { in x: Real[0..1]; in y: Real[0..1]; return : Boolean[1]; }
			
	function sqrt{ in x: Real[1]; return : Real[1]; }

	function floor{ in x: Real[1]; return : Integer[1]; }
	function round{ in x: Real[1]; return : Integer[1]; }
	
	function ToString specializes ComplexFunctions::ToString { in x: Real[1]; return : String[1]; }
	function ToInteger{ in x: Real[1]; return : Integer[1]; }
	function ToRational{ in x: Real[1]; return : Rational[1]; }
	function ToReal{ in x: String[1]; return : Real[1]; }
	
	function sum specializes ComplexFunctions::sum { in collection: Real[0..*]; 
        return : Real default NumericalFunctions::sum0(collection, 0.0);
	}
	
	function product specializes ComplexFunctions::product { in collection: Real[0..*]; 
        return : Real default NumericalFunctions::product1(collection, 1.0);
	}	
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ComplexFunctions::re'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::im'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::abs'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::arg'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::+'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::-'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::*'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::/'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::**'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::^'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'NumericalFunctions::<'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::>'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::<='
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::>='
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::max'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'NumericalFunctions::min'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::=='
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'ComplexFunctions::ToString'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::sum'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::product'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ComplexFunctions::re'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::im'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::abs'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::arg'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::+'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::-'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::*'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::/'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::**'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::^'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'NumericalFunctions::<'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::>'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::<='
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::>='
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::max'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'NumericalFunctions::min'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::=='
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'ComplexFunctions::ToString'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::sum'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ComplexFunctions::product'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwFunction,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
CloseCurly,
KwFunction,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
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
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwReturn,Colon,Ident,KwDefault,Ident,ColonColon,Ident,OpenParen,Ident,Comma,DecimalValue,Dot,DecimalValue,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwReturn,Colon,Ident,KwDefault,Ident,ColonColon,Ident,OpenParen,Ident,Comma,DecimalValue,Dot,DecimalValue,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'RealFunctions'
    (documentation)
    (import_decl public 'ScalarValues::*')
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (feature_def in 'y' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (feature_def in 'y' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (feature_def in 'y' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (feature_def in 'y' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (feature_def in 'y' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (feature_def in 'y' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (feature_def in 'y' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (feature_def in 'y' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (feature_def in 'y' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (feature_def in 'y' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (feature_def in 'y' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (feature_def in 'y' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (feature_def in 'y' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Real' multiplicity)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package RealFunctions {
    doc /*
	 * This package defines Functions on Real values, including concrete specializations of the 
	 * general arithmetic and comparison operations.
	 */

    public import ScalarValues::*;

    function re :> ComplexFunctions::re{ in x: Real[1]; 
        return : Real[1] = x;
	}
    function im :> ComplexFunctions::im{ in x: Real[1]; 
        return : Real[1] = 0.0;
	}

    function abs specializes ComplexFunctions::abs { in x: Real[1]; return : Real[1]; }
    function arg specializes ComplexFunctions::arg { in x: Real[1]; 
        return : Real[1] = 0.0;
	}

    function '+' specializes ComplexFunctions::'+' { in x: Real[1]; in y: Real[0..1]; return : Real[1]; }
    function '-' specializes ComplexFunctions::'-' { in x: Real[1]; in y: Real[0..1]; return : Real[1]; }
    function '*' specializes ComplexFunctions::'*' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
    function '/' specializes ComplexFunctions::'/' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
    function '**' specializes ComplexFunctions::'**' { in x: Real[1]; in y: Real[1]; return : Real[1]; }
    function '^' specializes ComplexFunctions::'^' { in x: Real[1]; in y: Real[1]; return : Real[1]; }

    function '<' specializes NumericalFunctions::'<' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }
    function '>' specializes NumericalFunctions::'>' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }
    function '<=' specializes NumericalFunctions::'<=' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }
    function '>=' specializes NumericalFunctions::'>=' { in x: Real[1]; in y: Real[1]; return : Boolean[1]; }

    function max specializes NumericalFunctions::max { in x: Real[1]; in y: Real[1]; return : Real[1]; }
    function min specializes NumericalFunctions::min { in x: Real[1]; in y: Real[1]; return : Real[1]; }

    function '==' specializes ComplexFunctions::'==' { in x: Real[0..1]; in y: Real[0..1]; return : Boolean[1]; }

    function sqrt{ in x: Real[1]; return : Real[1]; }

    function floor{ in x: Real[1]; return : Integer[1]; }
    function round{ in x: Real[1]; return : Integer[1]; }

    function ToString specializes ComplexFunctions::ToString { in x: Real[1]; return : String[1]; }
    function ToInteger{ in x: Real[1]; return : Integer[1]; }
    function ToRational{ in x: Real[1]; return : Rational[1]; }
    function ToReal{ in x: String[1]; return : Real[1]; }

    function sum specializes ComplexFunctions::sum { in collection: Real[0..*]; 
        return : Real default NumericalFunctions::sum0(collection, 0.0);
	}

    function product specializes ComplexFunctions::product { in collection: Real[0..*]; 
        return : Real default NumericalFunctions::product1(collection, 1.0);
	}
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'RealFunctions'
      (documentation)
      (namespace_import public -> 'ScalarValues'[unresolved])
      (function_def 're' :> 'ComplexFunctions::re'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'im' :> 'ComplexFunctions::im'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'abs' :> 'ComplexFunctions::abs'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'arg' :> 'ComplexFunctions::arg'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def '+' :> 'ComplexFunctions::+'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Real'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def '-' :> 'ComplexFunctions::-'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Real'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def '*' :> 'ComplexFunctions::*'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def '/' :> 'ComplexFunctions::/'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def '**' :> 'ComplexFunctions::**'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def '^' :> 'ComplexFunctions::^'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def '<' :> 'NumericalFunctions::<'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '>' :> 'NumericalFunctions::>'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '<=' :> 'NumericalFunctions::<='[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '>=' :> 'NumericalFunctions::>='[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'max' :> 'NumericalFunctions::max'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'min' :> 'NumericalFunctions::min'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def '==' :> 'ComplexFunctions::=='[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def in 'y' : 'Real'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'sqrt'
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'floor'
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'round'
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'ToString' :> 'ComplexFunctions::ToString'[unresolved]
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'String'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'ToInteger'
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'ToRational'
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'ToReal'
        (feature_def in 'x' : 'String'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'sum' :> 'ComplexFunctions::sum'[unresolved]
        (feature_def in 'collection' : 'Real'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (feature_value (default =)))))
      (function_def 'product' :> 'ComplexFunctions::product'[unresolved]
        (feature_def in 'collection' : 'Real'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (feature_value (default =))))))))
~~~
