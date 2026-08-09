# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/NaturalFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package NaturalFunctions {
	doc
	/*
	 * This package defines functions on Natural values, including concrete specialization of the 
	 * general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
	
	function '+' specializes IntegerFunctions::'+' { in x: Natural[1]; in y: Natural[0..1]; return : Natural[1]; }
	function '*' specializes IntegerFunctions::'*' { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }
	function '/' specializes IntegerFunctions::'/' { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }
	function '%' specializes IntegerFunctions::'%' { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }
	
	function '<' specializes IntegerFunctions::'<' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }
	function '>' specializes IntegerFunctions::'>' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }
	function '<=' specializes IntegerFunctions::'<=' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }
	function '>=' specializes IntegerFunctions::'>=' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }	

	function max specializes IntegerFunctions::max { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }
	function min specializes IntegerFunctions::min { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }

	function '==' specializes IntegerFunctions::'==' { in x: Natural[0..1]; in y: Natural[0..1]; return : Boolean[1]; }
	
	function ToString specializes IntegerFunctions::ToString { in x: Natural[1]; return : String[1]; }
	function ToNatural{ in x: String[1]; return : Natural[1]; }
}	
~~~
# EXPECTED
~~~
semantic.unresolved_name 'IntegerFunctions::+'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'IntegerFunctions::*'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'IntegerFunctions::/'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'IntegerFunctions::%'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'IntegerFunctions::<'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'IntegerFunctions::>'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'IntegerFunctions::<='
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'IntegerFunctions::>='
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'IntegerFunctions::max'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'IntegerFunctions::min'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'IntegerFunctions::=='
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'IntegerFunctions::ToString'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Natural'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'IntegerFunctions::+'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'IntegerFunctions::*'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'IntegerFunctions::/'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'IntegerFunctions::%'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'IntegerFunctions::<'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'IntegerFunctions::>'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'IntegerFunctions::<='
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'IntegerFunctions::>='
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'IntegerFunctions::max'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'IntegerFunctions::min'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'IntegerFunctions::=='
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'IntegerFunctions::ToString'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Natural'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
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
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'NaturalFunctions'
    (documentation)
    (import_decl public 'ScalarValues::*')
    (function_def
      (feature_def in 'x' : 'Natural' multiplicity)
      (feature_def in 'y' : 'Natural' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Natural' multiplicity)
      (feature_def in 'y' : 'Natural' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Natural' multiplicity)
      (feature_def in 'y' : 'Natural' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Natural' multiplicity)
      (feature_def in 'y' : 'Natural' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Natural' multiplicity)
      (feature_def in 'y' : 'Natural' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Natural' multiplicity)
      (feature_def in 'y' : 'Natural' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Natural' multiplicity)
      (feature_def in 'y' : 'Natural' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Natural' multiplicity)
      (feature_def in 'y' : 'Natural' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Natural' multiplicity)
      (feature_def in 'y' : 'Natural' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Natural' multiplicity)
      (feature_def in 'y' : 'Natural' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Natural' multiplicity)
      (feature_def in 'y' : 'Natural' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Natural' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'String' multiplicity)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package NaturalFunctions {
    doc
    /*
	 * This package defines functions on Natural values, including concrete specialization of the 
	 * general arithmetic and comparison operations.
	 */

    public import ScalarValues::*;

    function '+' specializes IntegerFunctions::'+' { in x: Natural[1]; in y: Natural[0..1]; return : Natural[1]; }
    function '*' specializes IntegerFunctions::'*' { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }
    function '/' specializes IntegerFunctions::'/' { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }
    function '%' specializes IntegerFunctions::'%' { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }

    function '<' specializes IntegerFunctions::'<' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }
    function '>' specializes IntegerFunctions::'>' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }
    function '<=' specializes IntegerFunctions::'<=' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }
    function '>=' specializes IntegerFunctions::'>=' { in x: Natural[1]; in y: Natural[1]; return : Boolean[1]; }

    function max specializes IntegerFunctions::max { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }
    function min specializes IntegerFunctions::min { in x: Natural[1]; in y: Natural[1]; return : Natural[1]; }

    function '==' specializes IntegerFunctions::'==' { in x: Natural[0..1]; in y: Natural[0..1]; return : Boolean[1]; }

    function ToString specializes IntegerFunctions::ToString { in x: Natural[1]; return : String[1]; }
    function ToNatural{ in x: String[1]; return : Natural[1]; }
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "NaturalFunctions"))) (name "NaturalFunctions") (declared-name "NaturalFunctions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "NaturalFunctions::*"))) (name "*") (declared-name "*"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NaturalFunctions::ToNatural"))) (name "ToNatural") (declared-name "ToNatural"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NaturalFunctions::ToString"))) (name "ToString") (declared-name "ToString"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "NaturalFunctions::_documentation"))) (name ""))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NaturalFunctions::function"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl2"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl3"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl4"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl5"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl6"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl7"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NaturalFunctions::function#kermlDecl8"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NaturalFunctions::max"))) (name "max") (declared-name "max"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NaturalFunctions::min"))) (name "min") (declared-name "min"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "NaturalFunctions::_documentation"))) (to (node (document "d0") (qualified-name "NaturalFunctions"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
