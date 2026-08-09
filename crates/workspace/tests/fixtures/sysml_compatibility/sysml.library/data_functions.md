# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/DataFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package DataFunctions {
	doc
	/*
	 * This package defines the abstract base functions corresponding to all the unary and binary operators 
	 * in the KerML expression notation that might be defined on various kinds of DataValues.
	 */

	private import Base::DataValue;
	private import ScalarValues::Boolean;
	private import ControlFunctions::reduce;	
	
	abstract function '==' specializes BaseFunctions::'==' { in x: DataValue[0..1]; in y: DataValue[0..1]; 
		return : Boolean[1];
	}
	function '===' specializes BaseFunctions::'==='{ in x: DataValue[0..1]; in y: DataValue[0..1]; 
		return : Boolean[1] = x == y;
	}
	
	abstract function '+' { in x: DataValue[1]; in y: DataValue[0..1]; return : DataValue[1]; }
	abstract function '-' { in x: DataValue[1]; in y: DataValue[0..1]; return : DataValue[1]; }
	abstract function '*' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '/' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '**' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '^' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '%' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	
	abstract function 'not' { in x: DataValue[1]; return : DataValue[1]; }
	abstract function 'xor' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }

	abstract function '~' { in x: DataValue[1]; return : DataValue[1]; }
	abstract function '|' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '&' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	
	abstract function '<' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	abstract function '>' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	abstract function '<=' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	abstract function '>=' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	
	abstract function max { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function min { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	
	abstract function '..' { in lower: DataValue[1]; in upper: DataValue[1]; return : DataValue[0..*] ordered; }	
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'BaseFunctions::=='
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'BaseFunctions::==='
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'BaseFunctions::=='
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'BaseFunctions::==='
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,EqEq,Ident,Semicolon,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'DataFunctions'
    (documentation)
    (import_decl private 'Base::DataValue')
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'ControlFunctions::reduce')
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'DataValue' multiplicity)
      (feature_def in 'y' : 'DataValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'lower' : 'DataValue' multiplicity)
      (feature_def in 'upper' : 'DataValue' multiplicity)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package DataFunctions {
	doc
	/*
	 * This package defines the abstract base functions corresponding to all the unary and binary operators 
	 * in the KerML expression notation that might be defined on various kinds of DataValues.
	 */

	private import Base::DataValue;
	private import ScalarValues::Boolean;
	private import ControlFunctions::reduce;	
	
	abstract function '==' specializes BaseFunctions::'==' { in x: DataValue[0..1]; in y: DataValue[0..1]; 
		return : Boolean[1];
	}
	function '===' specializes BaseFunctions::'==='{ in x: DataValue[0..1]; in y: DataValue[0..1]; 
		return : Boolean[1] = x == y;
	}
	
	abstract function '+' { in x: DataValue[1]; in y: DataValue[0..1]; return : DataValue[1]; }
	abstract function '-' { in x: DataValue[1]; in y: DataValue[0..1]; return : DataValue[1]; }
	abstract function '*' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '/' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '**' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '^' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '%' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	
	abstract function 'not' { in x: DataValue[1]; return : DataValue[1]; }
	abstract function 'xor' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }

	abstract function '~' { in x: DataValue[1]; return : DataValue[1]; }
	abstract function '|' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function '&' { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	
	abstract function '<' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	abstract function '>' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	abstract function '<=' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	abstract function '>=' { in x: DataValue[1]; in y: DataValue[1]; return : Boolean[1]; }
	
	abstract function max { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	abstract function min { in x: DataValue[1]; in y: DataValue[1]; return : DataValue[1]; }
	
	abstract function '..' { in lower: DataValue[1]; in upper: DataValue[1]; return : DataValue[0..*] ordered; }	
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "DataFunctions"))) (name "DataFunctions") (declared-name "DataFunctions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "DataFunctions::Boolean"))) (name "Boolean") (declared-name "Boolean"))
        (element (kind "import") (id (node (document "d0") (qualified-name "DataFunctions::DataValue"))) (name "DataValue") (declared-name "DataValue"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "DataFunctions::_documentation"))) (name ""))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl10"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl11"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl12"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl13"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl14"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl15"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl16"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl17"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl18"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl2"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl3"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl4"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl5"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl6"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl7"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl8"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::function#kermlDecl9"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::max"))) (name "max") (declared-name "max"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DataFunctions::min"))) (name "min") (declared-name "min"))
        (element (kind "import") (id (node (document "d0") (qualified-name "DataFunctions::reduce"))) (name "reduce") (declared-name "reduce"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DataFunctions::_documentation"))) (to (node (document "d0") (qualified-name "DataFunctions"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
