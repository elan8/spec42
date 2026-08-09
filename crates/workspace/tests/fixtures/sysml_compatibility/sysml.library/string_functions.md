# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/StringFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package StringFunctions {
	doc
	/*
	 * This package defines functions on String values, including those corresponding to string concatenation 
	 * and comparison operators in the KerML expression notation.
	 */

	public import ScalarValues::*;
	
	function '+' specializes ScalarFunctions::'+' { in x: String[1]; in y:String[1]; return : String[1]; }
	
	function Length{ in x: String[1]; return : Natural[1]; }
	function Substring{ in x: String[1]; in lower: Integer[1]; in upper: Integer[1]; return : String[1]; }
	
	function '<' specializes ScalarFunctions::'<' { in x: String[1]; in y: String[1]; return : Boolean[1]; }
	function '>' specializes ScalarFunctions::'>' { in x: String[1]; in y: String[1]; return : Boolean[1]; }
	function '<=' specializes ScalarFunctions::'<=' { in x: String[1]; in y: String[1]; return : Boolean[1]; }
	function '>=' specializes ScalarFunctions::'>=' { in x: String[1]; in y: String[1]; return : Boolean[1]; }

	function '==' specializes DataFunctions::'==' { in x: String[0..1]; in y: String[0..1]; return : Boolean[1]; }
	
	function ToString specializes BaseFunctions::ToString { in x: String[1];
		return : String[1] = x;
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ScalarFunctions::+'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'String'
semantic.unresolved_name 'ScalarFunctions::<'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::>'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::<='
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::>='
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::=='
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'BaseFunctions::ToString'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarFunctions::+'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'String'
semantic.unresolved_name 'ScalarFunctions::<'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::>'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::<='
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::>='
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::=='
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'BaseFunctions::ToString'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'StringFunctions'
    (documentation)
    (import_decl public 'ScalarValues::*')
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (feature_def in 'y' : 'String' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (feature_def in 'lower' : 'Integer' multiplicity)
      (feature_def in 'upper' : 'Integer' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (feature_def in 'y' : 'String' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (feature_def in 'y' : 'String' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (feature_def in 'y' : 'String' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (feature_def in 'y' : 'String' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (feature_def in 'y' : 'String' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package StringFunctions {
    doc /*
	 * This package defines functions on String values, including those corresponding to string concatenation 
	 * and comparison operators in the KerML expression notation.
	 */

    public import ScalarValues::*;

    function '+' specializes ScalarFunctions::'+' { in x: String[1]; in y:String[1]; return : String[1]; }

    function Length{ in x: String[1]; return : Natural[1]; }
    function Substring{ in x: String[1]; in lower: Integer[1]; in upper: Integer[1]; return : String[1]; }

    function '<' specializes ScalarFunctions::'<' { in x: String[1]; in y: String[1]; return : Boolean[1]; }
    function '>' specializes ScalarFunctions::'>' { in x: String[1]; in y: String[1]; return : Boolean[1]; }
    function '<=' specializes ScalarFunctions::'<=' { in x: String[1]; in y: String[1]; return : Boolean[1]; }
    function '>=' specializes ScalarFunctions::'>=' { in x: String[1]; in y: String[1]; return : Boolean[1]; }

    function '==' specializes DataFunctions::'==' { in x: String[0..1]; in y: String[0..1]; return : Boolean[1]; }

    function ToString specializes BaseFunctions::ToString { in x: String[1];
		return : String[1] = x;
	}
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "StringFunctions"))) (name "StringFunctions") (declared-name "StringFunctions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "StringFunctions::*"))) (name "*") (declared-name "*"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "StringFunctions::Length"))) (name "Length") (declared-name "Length"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "StringFunctions::Substring"))) (name "Substring") (declared-name "Substring"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "StringFunctions::ToString"))) (name "ToString") (declared-name "ToString"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "StringFunctions::_documentation"))) (name ""))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "StringFunctions::function"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "StringFunctions::function#kermlDecl"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "StringFunctions::function#kermlDecl2"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "StringFunctions::function#kermlDecl3"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "StringFunctions::function#kermlDecl4"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "StringFunctions::function#kermlDecl5"))) (name "function") (declared-name "function"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StringFunctions::_documentation"))) (to (node (document "d0") (qualified-name "StringFunctions"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
