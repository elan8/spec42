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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "numerical_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 15) (end 6 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 40))
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e11446ca51e09c518389d0e7da9390ff28d0c1a0c659c45c475d5183cb194d43") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "NumericalFunctions"))) (kind "package") (name "NumericalFunctions") (declared-name "NumericalFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 2787))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 6) (character 1)) (end (line 6) (character 31))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 6) (character 15)) (end (line 6) (character 27))))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2787))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::abs"))) (kind "kermlDecl") (name "abs") (declared-name "abs") (range (start (line 12) (character 1)) (end (line 12) (character 78))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 14) (character 1)) (end (line 14) (character 140))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 15) (character 1)) (end (line 15) (character 140))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl10"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 25) (character 1)) (end (line 25) (character 132))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 16) (character 1)) (end (line 16) (character 137))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 17) (character 1)) (end (line 17) (character 137))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl4"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 18) (character 1)) (end (line 18) (character 139))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl5"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 19) (character 1)) (end (line 19) (character 137))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl6"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 20) (character 1)) (end (line 20) (character 137))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl7"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 22) (character 1)) (end (line 22) (character 130))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl8"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 23) (character 1)) (end (line 23) (character 130))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::function#kermlDecl9"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 24) (character 1)) (end (line 24) (character 132))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::isUnit"))) (kind "kermlDecl") (name "isUnit") (declared-name "isUnit") (range (start (line 10) (character 1)) (end (line 10) (character 72))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::isZero"))) (kind "kermlDecl") (name "isZero") (declared-name "isZero") (range (start (line 9) (character 1)) (end (line 9) (character 71))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::max"))) (kind "kermlDecl") (name "max") (declared-name "max") (range (start (line 27) (character 1)) (end (line 27) (character 137))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::min"))) (kind "kermlDecl") (name "min") (declared-name "min") (range (start (line 28) (character 1)) (end (line 28) (character 137))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::product"))) (kind "kermlDecl") (name "product") (declared-name "product") (range (start (line 31) (character 1)) (end (line 31) (character 89))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::product1"))) (kind "kermlDecl") (name "product1") (declared-name "product1") (range (start (line 38) (character 1)) (end (line 38) (character 168))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::reduce"))) (kind "import") (name "reduce") (declared-name "reduce") (range (start (line 7) (character 1)) (end (line 7) (character 41))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::reduce") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 40))))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::sum"))) (kind "kermlDecl") (name "sum") (declared-name "sum") (range (start (line 30) (character 1)) (end (line 30) (character 85))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
    (element (id (node (document "d0") (qualified-name "NumericalFunctions::sum0"))) (kind "kermlDecl") (name "sum0") (declared-name "sum0") (range (start (line 33) (character 1)) (end (line 33) (character 171))) (parent (node (document "d0") (qualified-name "NumericalFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "NumericalFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 6) (character 15)) (end (line 6) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "NumericalFunctions::reduce"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::reduce") (range (start (line 7) (character 16)) (end (line 7) (character 40))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
