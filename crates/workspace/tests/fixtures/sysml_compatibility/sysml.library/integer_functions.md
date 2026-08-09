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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "IntegerFunctions"))) (name "IntegerFunctions") (declared-name "IntegerFunctions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "IntegerFunctions::*"))) (name "*") (declared-name "*"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::ToInteger"))) (name "ToInteger") (declared-name "ToInteger"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::ToNatural"))) (name "ToNatural") (declared-name "ToNatural"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::ToString"))) (name "ToString") (declared-name "ToString"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "IntegerFunctions::_documentation"))) (name ""))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::abs"))) (name "abs") (declared-name "abs"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::function"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl10"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl11"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl12"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl2"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl3"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl4"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl5"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl6"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl7"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl8"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::function#kermlDecl9"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::max"))) (name "max") (declared-name "max"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::min"))) (name "min") (declared-name "min"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::product"))) (name "product") (declared-name "product"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "IntegerFunctions::sum"))) (name "sum") (declared-name "sum"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "IntegerFunctions::_documentation"))) (to (node (document "d0") (qualified-name "IntegerFunctions"))) (provenance authored))
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
  (document "sysml.library/integer_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 15) (end 7 27))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 12 1) (end 12 112))
      )
    )
  )
)
~~~
