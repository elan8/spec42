# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/ComplexFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package ComplexFunctions {
	doc
	/*
	 * This package defines functions on Complex values, including concrete specializations of the 
	 * general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
		
	feature i: Complex[1] = rect(0.0, 1.0);
	
	function rect { in re: Real[1]; in im: Real[1]; return : Complex[1]; }
	function polar { in abs: Real[1]; in arg: Real[1]; return : Complex[1]; }
	
	function re { in x: Complex[1]; return : Real[1]; }
	function im { in x: Complex[1]; return : Real[1]; }
	
	function isZero specializes NumericalFunctions::isZero { in x : Complex[1];
		return : Boolean[1] = re(x) == 0.0 and im(x) == 0.0;
	}
	function isUnit specializes NumericalFunctions::isUnit { in x : Complex[1];
		return : Boolean[1] = re(x) == 1.0 and im(x) == 0.0;
	}
	
	function abs specializes NumericalFunctions::abs { in x: Complex[1]; return : Real[1]; }
	function arg { in x: Complex[1]; return : Real[1]; }
	
	function '+' specializes NumericalFunctions::'+' { in x: Complex[1]; in y: Complex[0..1]; return : Complex[1]; }
	function '-' specializes NumericalFunctions::'-' { in x: Complex[1]; in y: Complex[0..1]; return : Complex[1]; }
	function '*' specializes NumericalFunctions::'*' { in x: Complex[1]; in y: Complex[1]; return : Complex[1]; }
	function '/' specializes NumericalFunctions::'/' { in x: Complex[1]; in y: Complex[1]; return : Complex[1]; }
	function '**' specializes NumericalFunctions::'**' { in x: Complex[1]; in y: Complex[1]; return : Complex[1]; }
	function '^' specializes NumericalFunctions::'^' { in x: Complex[1]; in y: Complex[1]; return : Complex[1]; }
	
	function '==' specializes DataFunctions::'==' { in x: Complex[0..1]; in y: Complex[0..1]; return : Boolean[1]; }
	
	function ToString specializes BaseFunctions::ToString { in x: Complex[1]; return : String[1]; }
	function ToComplex { in x: String[1]; return : Complex[1]; }
	
	function sum specializes NumericalFunctions::sum { in collection: Complex[0..*];
		return : Complex[1] default NumericalFunctions::sum0(collection, rect(0.0, 0.0));
	}
	
	function product specializes NumericalFunctions::product { in collection: Complex[0..*];
		return : Complex[1] default NumericalFunctions::product1(collection, rect(1.0, 0.0));
	}	
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'NumericalFunctions::isZero'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::isUnit'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::abs'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'NumericalFunctions::+'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'NumericalFunctions::-'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'NumericalFunctions::*'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'NumericalFunctions::/'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'NumericalFunctions::**'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'NumericalFunctions::^'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'DataFunctions::=='
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'BaseFunctions::ToString'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'NumericalFunctions::sum'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'NumericalFunctions::product'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'NumericalFunctions::isZero'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::isUnit'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::abs'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'NumericalFunctions::+'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'NumericalFunctions::-'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'NumericalFunctions::*'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'NumericalFunctions::/'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'NumericalFunctions::**'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'NumericalFunctions::^'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'DataFunctions::=='
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'BaseFunctions::ToString'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'NumericalFunctions::sum'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'NumericalFunctions::product'
semantic.unresolved_name 'Complex'
semantic.unresolved_name 'Complex'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,OpenParen,DecimalValue,Dot,DecimalValue,Comma,DecimalValue,Dot,DecimalValue,CloseParen,Semicolon,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,OpenParen,Ident,CloseParen,EqEq,DecimalValue,Dot,DecimalValue,KwAnd,Ident,OpenParen,Ident,CloseParen,EqEq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,OpenParen,Ident,CloseParen,EqEq,DecimalValue,Dot,DecimalValue,KwAnd,Ident,OpenParen,Ident,CloseParen,EqEq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,ColonColon,Ident,OpenParen,Ident,Comma,Ident,OpenParen,DecimalValue,Dot,DecimalValue,Comma,DecimalValue,Dot,DecimalValue,CloseParen,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,ColonColon,Ident,OpenParen,Ident,Comma,Ident,OpenParen,DecimalValue,Dot,DecimalValue,Comma,DecimalValue,Dot,DecimalValue,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ComplexFunctions'
    (documentation)
    (import_decl public 'ScalarValues::*')
    (feature_def 'i' : 'Complex' multiplicity value)
    (function_def
      (feature_def in 're' : 'Real' multiplicity)
      (feature_def in 'im' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'abs' : 'Real' multiplicity)
      (feature_def in 'arg' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Complex' multiplicity)
      (feature_def in 'y' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Complex' multiplicity)
      (feature_def in 'y' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Complex' multiplicity)
      (feature_def in 'y' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Complex' multiplicity)
      (feature_def in 'y' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Complex' multiplicity)
      (feature_def in 'y' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Complex' multiplicity)
      (feature_def in 'y' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Complex' multiplicity)
      (feature_def in 'y' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Complex' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'collection' : 'Complex' multiplicity)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package ComplexFunctions {
    doc /*
	 * This package defines functions on Complex values, including concrete specializations of the 
	 * general arithmetic and comparison operations.
	 */

    public import ScalarValues::*;

    feature i : Complex [1] = rect(0.0, 1.0);

    function rect { in re: Real[1]; in im: Real[1]; return : Complex[1]; }
    function polar { in abs: Real[1]; in arg: Real[1]; return : Complex[1]; }

    function re { in x: Complex[1]; return : Real[1]; }
    function im { in x: Complex[1]; return : Real[1]; }

    function isZero specializes NumericalFunctions::isZero { in x : Complex[1];
		return : Boolean[1] = re(x) == 0.0 and im(x) == 0.0;
	}
    function isUnit specializes NumericalFunctions::isUnit { in x : Complex[1];
		return : Boolean[1] = re(x) == 1.0 and im(x) == 0.0;
	}

    function abs specializes NumericalFunctions::abs { in x: Complex[1]; return : Real[1]; }
    function arg { in x: Complex[1]; return : Real[1]; }

    function '+' specializes NumericalFunctions::'+' { in x: Complex[1]; in y: Complex[0..1]; return : Complex[1]; }
    function '-' specializes NumericalFunctions::'-' { in x: Complex[1]; in y: Complex[0..1]; return : Complex[1]; }
    function '*' specializes NumericalFunctions::'*' { in x: Complex[1]; in y: Complex[1]; return : Complex[1]; }
    function '/' specializes NumericalFunctions::'/' { in x: Complex[1]; in y: Complex[1]; return : Complex[1]; }
    function '**' specializes NumericalFunctions::'**' { in x: Complex[1]; in y: Complex[1]; return : Complex[1]; }
    function '^' specializes NumericalFunctions::'^' { in x: Complex[1]; in y: Complex[1]; return : Complex[1]; }

    function '==' specializes DataFunctions::'==' { in x: Complex[0..1]; in y: Complex[0..1]; return : Boolean[1]; }

    function ToString specializes BaseFunctions::ToString { in x: Complex[1]; return : String[1]; }
    function ToComplex { in x: String[1]; return : Complex[1]; }

    function sum specializes NumericalFunctions::sum { in collection: Complex[0..*];
		return : Complex[1] default NumericalFunctions::sum0(collection, rect(0.0, 0.0));
	}

    function product specializes NumericalFunctions::product { in collection: Complex[0..*];
		return : Complex[1] default NumericalFunctions::product1(collection, rect(1.0, 0.0));
	}
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'ComplexFunctions'
      (documentation)
      (namespace_import public -> 'ScalarValues'[unresolved])
      (feature_def 'i' : 'Complex'[unresolved]
        (multiplicity_range [1])
        (feature_value (=)))
      (function_def 'rect'
        (feature_def in 're' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'im' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Complex'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'polar'
        (feature_def in 'abs' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'arg' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Complex'[unresolved]
            (multiplicity_range [1]))))
      (function_def 're'
        (feature_def in 'x' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'im'
        (feature_def in 'x' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'isZero' :> 'NumericalFunctions::isZero'[unresolved]
        (feature_def in 'x' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'isUnit' :> 'NumericalFunctions::isUnit'[unresolved]
        (feature_def in 'x' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'abs' :> 'NumericalFunctions::abs'[unresolved]
        (feature_def in 'x' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'arg'
        (feature_def in 'x' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def '+' :> 'NumericalFunctions::+'[unresolved]
        (feature_def in 'x' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Complex'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Complex'[unresolved]
            (multiplicity_range [1]))))
      (function_def '-' :> 'NumericalFunctions::-'[unresolved]
        (feature_def in 'x' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Complex'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Complex'[unresolved]
            (multiplicity_range [1]))))
      (function_def '*' :> 'NumericalFunctions::*'[unresolved]
        (feature_def in 'x' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Complex'[unresolved]
            (multiplicity_range [1]))))
      (function_def '/' :> 'NumericalFunctions::/'[unresolved]
        (feature_def in 'x' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Complex'[unresolved]
            (multiplicity_range [1]))))
      (function_def '**' :> 'NumericalFunctions::**'[unresolved]
        (feature_def in 'x' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Complex'[unresolved]
            (multiplicity_range [1]))))
      (function_def '^' :> 'NumericalFunctions::^'[unresolved]
        (feature_def in 'x' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Complex'[unresolved]
            (multiplicity_range [1]))))
      (function_def '==' :> 'DataFunctions::=='[unresolved]
        (feature_def in 'x' : 'Complex'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def in 'y' : 'Complex'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'ToString' :> 'BaseFunctions::ToString'[unresolved]
        (feature_def in 'x' : 'Complex'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'String'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'ToComplex'
        (feature_def in 'x' : 'String'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Complex'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'sum' :> 'NumericalFunctions::sum'[unresolved]
        (feature_def in 'collection' : 'Complex'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Complex'[unresolved]
            (multiplicity_range [1])
            (feature_value (default =)))))
      (function_def 'product' :> 'NumericalFunctions::product'[unresolved]
        (feature_def in 'collection' : 'Complex'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Complex'[unresolved]
            (multiplicity_range [1])
            (feature_value (default =))))))))
~~~
