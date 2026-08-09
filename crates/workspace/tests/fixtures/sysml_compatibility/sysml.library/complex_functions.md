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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ComplexFunctions"))) (name "ComplexFunctions") (declared-name "ComplexFunctions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ComplexFunctions::*"))) (name "*") (declared-name "*"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::ToComplex"))) (name "ToComplex") (declared-name "ToComplex"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::ToString"))) (name "ToString") (declared-name "ToString"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ComplexFunctions::_documentation"))) (name ""))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::abs"))) (name "abs") (declared-name "abs"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::arg"))) (name "arg") (declared-name "arg"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::function"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl2"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl3"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl4"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl5"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl6"))) (name "function") (declared-name "function"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "ComplexFunctions::i"))) (name "i") (declared-name "i"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::im"))) (name "im") (declared-name "im"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::isUnit"))) (name "isUnit") (declared-name "isUnit"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::isZero"))) (name "isZero") (declared-name "isZero"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::polar"))) (name "polar") (declared-name "polar"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::product"))) (name "product") (declared-name "product"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::re"))) (name "re") (declared-name "re"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::rect"))) (name "rect") (declared-name "rect"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ComplexFunctions::sum"))) (name "sum") (declared-name "sum"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ComplexFunctions::_documentation"))) (to (node (document "d0") (qualified-name "ComplexFunctions"))))
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
  (document "sysml.library/complex_functions.md"
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
        (range (start 28 1) (end 28 113))
      )
    )
  )
)
~~~
