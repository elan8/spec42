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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "complex_functions.md"
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "af43a7a76ae97f2baf7d5b6a370de9c0cae9b96cb9bfc07f6ec980b8780d7cda") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ComplexFunctions"))) (kind "package") (name "ComplexFunctions") (declared-name "ComplexFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 2266))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 15)) (end (line 7) (character 27))))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::ToComplex"))) (kind "kermlDecl") (name "ToComplex") (declared-name "ToComplex") (range (start (line 37) (character 1)) (end (line 37) (character 61))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::ToString"))) (kind "kermlDecl") (name "ToString") (declared-name "ToString") (range (start (line 36) (character 1)) (end (line 36) (character 96))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2266))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::abs"))) (kind "kermlDecl") (name "abs") (declared-name "abs") (range (start (line 24) (character 1)) (end (line 24) (character 89))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::arg"))) (kind "kermlDecl") (name "arg") (declared-name "arg") (range (start (line 25) (character 1)) (end (line 25) (character 53))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 27) (character 1)) (end (line 27) (character 113))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 28) (character 1)) (end (line 28) (character 113))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 29) (character 1)) (end (line 29) (character 110))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 30) (character 1)) (end (line 30) (character 110))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 31) (character 1)) (end (line 31) (character 112))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 32) (character 1)) (end (line 32) (character 110))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::function#kermlDecl6"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 34) (character 1)) (end (line 34) (character 113))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::i"))) (kind "feature decl") (name "i") (declared-name "i") (range (start (line 9) (character 1)) (end (line 9) (character 40))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::im"))) (kind "kermlDecl") (name "im") (declared-name "im") (range (start (line 15) (character 1)) (end (line 15) (character 52))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::isUnit"))) (kind "kermlDecl") (name "isUnit") (declared-name "isUnit") (range (start (line 20) (character 1)) (end (line 20) (character 134))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::isZero"))) (kind "kermlDecl") (name "isZero") (declared-name "isZero") (range (start (line 17) (character 1)) (end (line 17) (character 134))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::polar"))) (kind "kermlDecl") (name "polar") (declared-name "polar") (range (start (line 12) (character 1)) (end (line 12) (character 74))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::product"))) (kind "kermlDecl") (name "product") (declared-name "product") (range (start (line 43) (character 1)) (end (line 43) (character 180))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::re"))) (kind "kermlDecl") (name "re") (declared-name "re") (range (start (line 14) (character 1)) (end (line 14) (character 52))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::rect"))) (kind "kermlDecl") (name "rect") (declared-name "rect") (range (start (line 11) (character 1)) (end (line 11) (character 71))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
    (element (id (node (document "d0") (qualified-name "ComplexFunctions::sum"))) (kind "kermlDecl") (name "sum") (declared-name "sum") (range (start (line 39) (character 1)) (end (line 39) (character 168))) (parent (node (document "d0") (qualified-name "ComplexFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ComplexFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 7) (character 15)) (end (line 7) (character 27))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
