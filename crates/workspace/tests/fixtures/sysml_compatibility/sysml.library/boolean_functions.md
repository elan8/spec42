# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/BooleanFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package BooleanFunctions {
	doc
	/*
	 * This package defines functions on Boolean values, including those corresponding to 
	 * (non-conditional) logical operators in the KerML expression notation.
	 */

	public import ScalarValues::*;
	
	function 'not' specializes ScalarFunctions::'not' { in x: Boolean[1]; return : Boolean[1]; }
	function 'xor' specializes ScalarFunctions::'xor' { in x: Boolean[1]; in y: Boolean[1]; return : Boolean[1]; }
	
	function '|' specializes ScalarFunctions::'|' { in x: Boolean[1]; in y: Boolean[1]; return : Boolean[1]; }
	function '&' specializes ScalarFunctions::'&' { in x: Boolean[1]; in y: Boolean[1]; return : Boolean[1]; }
	
	function '==' specializes DataFunctions::'==' { in x: Boolean[0..1]; in y: Boolean[0..1]; return : Boolean[1]; }
	
	function ToString specializes BaseFunctions::ToString { in x: Boolean[1]; return : String[1]; }
	function ToBoolean { in x: String[1]; return : Boolean[1]; }
	
}
	
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ScalarFunctions::not'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::xor'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::|'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::&'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::=='
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'BaseFunctions::ToString'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarFunctions::not'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::xor'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::|'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::&'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::=='
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'BaseFunctions::ToString'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
CloseCurly,
EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'BooleanFunctions'
    (documentation)
    (import_decl public 'ScalarValues::*')
    (function_def
      (feature_def in 'x' : 'Boolean' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Boolean' multiplicity)
      (feature_def in 'y' : 'Boolean' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Boolean' multiplicity)
      (feature_def in 'y' : 'Boolean' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Boolean' multiplicity)
      (feature_def in 'y' : 'Boolean' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Boolean' multiplicity)
      (feature_def in 'y' : 'Boolean' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Boolean' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package BooleanFunctions {
    doc
    /*
	 * This package defines functions on Boolean values, including those corresponding to 
	 * (non-conditional) logical operators in the KerML expression notation.
	 */

    public import ScalarValues::*;

    function 'not' specializes ScalarFunctions::'not' { in x: Boolean[1]; return : Boolean[1]; }
    function 'xor' specializes ScalarFunctions::'xor' { in x: Boolean[1]; in y: Boolean[1]; return : Boolean[1]; }

    function '|' specializes ScalarFunctions::'|' { in x: Boolean[1]; in y: Boolean[1]; return : Boolean[1]; }
    function '&' specializes ScalarFunctions::'&' { in x: Boolean[1]; in y: Boolean[1]; return : Boolean[1]; }

    function '==' specializes DataFunctions::'==' { in x: Boolean[0..1]; in y: Boolean[0..1]; return : Boolean[1]; }

    function ToString specializes BaseFunctions::ToString { in x: Boolean[1]; return : String[1]; }
    function ToBoolean { in x: String[1]; return : Boolean[1]; }

}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "BooleanFunctions"))) (name "BooleanFunctions") (declared-name "BooleanFunctions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "BooleanFunctions::*"))) (name "*") (declared-name "*"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BooleanFunctions::ToBoolean"))) (name "ToBoolean") (declared-name "ToBoolean"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BooleanFunctions::ToString"))) (name "ToString") (declared-name "ToString"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "BooleanFunctions::_documentation"))) (name ""))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BooleanFunctions::function"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BooleanFunctions::function#kermlDecl"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BooleanFunctions::function#kermlDecl2"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BooleanFunctions::function#kermlDecl3"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BooleanFunctions::function#kermlDecl4"))) (name "function") (declared-name "function"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "BooleanFunctions::_documentation"))) (to (node (document "d0") (qualified-name "BooleanFunctions"))) (provenance authored))
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
  (document "sysml.library/boolean_functions.md"
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
        (range (start 10 1) (end 10 111))
      )
    )
  )
)
~~~
