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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "string_functions.md"
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
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "49d66d13bdf64b1b0268bb52aea5c2b0fe202a6aa71ced56c82361559702ae14") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "StringFunctions"))) (kind "package") (name "StringFunctions") (declared-name "StringFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 1180))))
    (element (id (node (document "d0") (qualified-name "StringFunctions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "StringFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 15)) (end (line 7) (character 27))))))
    (element (id (node (document "d0") (qualified-name "StringFunctions::Length"))) (kind "kermlDecl") (name "Length") (declared-name "Length") (range (start (line 11) (character 1)) (end (line 11) (character 57))) (parent (node (document "d0") (qualified-name "StringFunctions"))))
    (element (id (node (document "d0") (qualified-name "StringFunctions::Substring"))) (kind "kermlDecl") (name "Substring") (declared-name "Substring") (range (start (line 12) (character 1)) (end (line 12) (character 103))) (parent (node (document "d0") (qualified-name "StringFunctions"))))
    (element (id (node (document "d0") (qualified-name "StringFunctions::ToString"))) (kind "kermlDecl") (name "ToString") (declared-name "ToString") (range (start (line 21) (character 1)) (end (line 21) (character 102))) (parent (node (document "d0") (qualified-name "StringFunctions"))))
    (element (id (node (document "d0") (qualified-name "StringFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1180))) (parent (node (document "d0") (qualified-name "StringFunctions"))))
    (element (id (node (document "d0") (qualified-name "StringFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 9) (character 1)) (end (line 9) (character 103))) (parent (node (document "d0") (qualified-name "StringFunctions"))))
    (element (id (node (document "d0") (qualified-name "StringFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 14) (character 1)) (end (line 14) (character 105))) (parent (node (document "d0") (qualified-name "StringFunctions"))))
    (element (id (node (document "d0") (qualified-name "StringFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 15) (character 1)) (end (line 15) (character 105))) (parent (node (document "d0") (qualified-name "StringFunctions"))))
    (element (id (node (document "d0") (qualified-name "StringFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 16) (character 1)) (end (line 16) (character 107))) (parent (node (document "d0") (qualified-name "StringFunctions"))))
    (element (id (node (document "d0") (qualified-name "StringFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 17) (character 1)) (end (line 17) (character 107))) (parent (node (document "d0") (qualified-name "StringFunctions"))))
    (element (id (node (document "d0") (qualified-name "StringFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 19) (character 1)) (end (line 19) (character 111))) (parent (node (document "d0") (qualified-name "StringFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "StringFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 7) (character 15)) (end (line 7) (character 27))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
