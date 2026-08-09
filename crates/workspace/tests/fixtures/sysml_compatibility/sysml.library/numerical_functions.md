# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/NumericalFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package NumericalFunctions {
	doc
	/*
	 * This package defines abstract functions on Numerical values for general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
	private import ControlFunctions::reduce;
	
	abstract function isZero{ in x: NumericalValue[1]; return : Boolean; }
	abstract function isUnit{ in x : NumericalValue[1]; return : Boolean; }
	
	abstract function abs{ in x: NumericalValue[1]; return : NumericalValue[1]; }
		
	abstract function '+' specializes ScalarFunctions::'+' { in x: NumericalValue[1]; in y: NumericalValue[0..1]; return : NumericalValue[1]; }
	abstract function '-' specializes ScalarFunctions::'-' { in x: NumericalValue[1]; in y: NumericalValue[0..1]; return : NumericalValue[1]; }
	abstract function '*' specializes ScalarFunctions::'*' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '/' specializes ScalarFunctions::'/' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '**' specializes ScalarFunctions::'**' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '^' specializes ScalarFunctions::'^' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '%' specializes ScalarFunctions::'%' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	
	abstract function '<' specializes ScalarFunctions::'<' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	abstract function '>' specializes ScalarFunctions::'>' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	abstract function '<=' specializes ScalarFunctions::'<=' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	abstract function '>=' specializes ScalarFunctions::'>=' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	
	abstract function max specializes ScalarFunctions::max { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function min specializes ScalarFunctions::min { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	
	abstract function sum { in collection: ScalarValue[0..*]; return : ScalarValue[1]; }	
	abstract function product { in collection: ScalarValue[0..*]; return : ScalarValue[1]; }
	
	function sum0 { in collection: NumericalValue[0..*]; in zero: ScalarValue[1]; 
 		inv { isZero(zero) }		
        return : ScalarValue = collection->reduce '+' ?? zero;
	}
	
	function product1 { in collection: ScalarValue[0..*]; in one: ScalarValue[1]; 
		inv { isUnit(one) }		
        return : ScalarValue = collection->reduce '*' ?? one;
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::+'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::-'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::*'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::/'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::**'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::^'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::%'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::<'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::>'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::<='
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::>='
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::max'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::min'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::+'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::-'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::*'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::/'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::**'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::^'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::%'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::<'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::>'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::<='
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::>='
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarFunctions::max'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarFunctions::min'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,Semicolon,CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,Semicolon,CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,CloseCurly,
KwReturn,Colon,Ident,Eq,Ident,Arrow,Ident,UnrestrictedName,QuestionQuestion,Ident,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,CloseCurly,
KwReturn,Colon,Ident,Eq,Ident,Arrow,Ident,UnrestrictedName,QuestionQuestion,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'NumericalFunctions'
    (documentation)
    (import_decl public 'ScalarValues::*')
    (import_decl private 'ControlFunctions::reduce')
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (feature_def in 'y' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (feature_def in 'y' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (feature_def in 'y' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (feature_def in 'y' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (feature_def in 'y' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (feature_def in 'y' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (feature_def in 'y' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (feature_def in 'y' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (feature_def in 'y' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (feature_def in 'y' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (feature_def in 'y' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (feature_def in 'y' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (feature_def in 'y' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'collection' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'collection' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'collection' : 'NumericalValue' multiplicity)
      (feature_def in 'zero' : 'ScalarValue' multiplicity)
      (invariant_def
        (result_expr_member))
      (return_member))
    (function_def
      (feature_def in 'collection' : 'ScalarValue' multiplicity)
      (feature_def in 'one' : 'ScalarValue' multiplicity)
      (invariant_def
        (result_expr_member))
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package NumericalFunctions {
	doc
	/*
	 * This package defines abstract functions on Numerical values for general arithmetic and comparison operations.
	 */

	public import ScalarValues::*;
	private import ControlFunctions::reduce;
	
	abstract function isZero{ in x: NumericalValue[1]; return : Boolean; }
	abstract function isUnit{ in x : NumericalValue[1]; return : Boolean; }
	
	abstract function abs{ in x: NumericalValue[1]; return : NumericalValue[1]; }
		
	abstract function '+' specializes ScalarFunctions::'+' { in x: NumericalValue[1]; in y: NumericalValue[0..1]; return : NumericalValue[1]; }
	abstract function '-' specializes ScalarFunctions::'-' { in x: NumericalValue[1]; in y: NumericalValue[0..1]; return : NumericalValue[1]; }
	abstract function '*' specializes ScalarFunctions::'*' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '/' specializes ScalarFunctions::'/' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '**' specializes ScalarFunctions::'**' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '^' specializes ScalarFunctions::'^' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function '%' specializes ScalarFunctions::'%' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	
	abstract function '<' specializes ScalarFunctions::'<' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	abstract function '>' specializes ScalarFunctions::'>' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	abstract function '<=' specializes ScalarFunctions::'<=' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	abstract function '>=' specializes ScalarFunctions::'>=' { in x: NumericalValue[1]; in y: NumericalValue[1]; return : Boolean[1]; }
	
	abstract function max specializes ScalarFunctions::max { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	abstract function min specializes ScalarFunctions::min { in x: NumericalValue[1]; in y: NumericalValue[1]; return : NumericalValue[1]; }
	
	abstract function sum { in collection: ScalarValue[0..*]; return : ScalarValue[1]; }	
	abstract function product { in collection: ScalarValue[0..*]; return : ScalarValue[1]; }
	
	function sum0 { in collection: NumericalValue[0..*]; in zero: ScalarValue[1]; 
 		inv { isZero(zero) }		
        return : ScalarValue = collection->reduce '+' ?? zero;
	}
	
	function product1 { in collection: ScalarValue[0..*]; in one: ScalarValue[1]; 
		inv { isUnit(one) }		
        return : ScalarValue = collection->reduce '*' ?? one;
	}
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "NumericalFunctions"))) (name "NumericalFunctions") (declared-name "NumericalFunctions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "NumericalFunctions::*"))) (name "*") (declared-name "*"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "NumericalFunctions::_documentation"))) (name ""))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::abs"))) (name "abs") (declared-name "abs"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::function"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl10"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl2"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl3"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl4"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl5"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl6"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl7"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl8"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl9"))) (name "function") (declared-name "function"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::isUnit"))) (name "isUnit") (declared-name "isUnit"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::isZero"))) (name "isZero") (declared-name "isZero"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::max"))) (name "max") (declared-name "max"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::min"))) (name "min") (declared-name "min"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::product"))) (name "product") (declared-name "product"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::product1"))) (name "product1") (declared-name "product1"))
        (element (kind "import") (id (node (document "d0") (qualified-name "NumericalFunctions::reduce"))) (name "reduce") (declared-name "reduce"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::sum"))) (name "sum") (declared-name "sum"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "NumericalFunctions::sum0"))) (name "sum0") (declared-name "sum0"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "NumericalFunctions::_documentation"))) (to (node (document "d0") (qualified-name "NumericalFunctions"))))
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
  (document "sysml.library/numerical_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 1) (end 6 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 1) (end 7 41))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 15 1) (end 15 140))
      )
    )
  )
)
~~~
