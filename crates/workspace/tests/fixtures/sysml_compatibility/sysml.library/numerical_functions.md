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
    doc /*
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
(model
  (namespace
    (library_package 'NumericalFunctions'
      (documentation)
      (namespace_import public -> 'ScalarValues'[unresolved])
      (membership_import private -> 'ControlFunctions::reduce'[unresolved])
      (function_def abstract 'isZero'
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved])))
      (function_def abstract 'isUnit'
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved])))
      (function_def abstract 'abs'
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'NumericalValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '+' :> 'ScalarFunctions::+'[unresolved]
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'NumericalValue'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'NumericalValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '-' :> 'ScalarFunctions::-'[unresolved]
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'NumericalValue'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'NumericalValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '*' :> 'ScalarFunctions::*'[unresolved]
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'NumericalValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '/' :> 'ScalarFunctions::/'[unresolved]
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'NumericalValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '**' :> 'ScalarFunctions::**'[unresolved]
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'NumericalValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '^' :> 'ScalarFunctions::^'[unresolved]
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'NumericalValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '%' :> 'ScalarFunctions::%'[unresolved]
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'NumericalValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '<' :> 'ScalarFunctions::<'[unresolved]
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '>' :> 'ScalarFunctions::>'[unresolved]
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '<=' :> 'ScalarFunctions::<='[unresolved]
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '>=' :> 'ScalarFunctions::>='[unresolved]
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'max' :> 'ScalarFunctions::max'[unresolved]
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'NumericalValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'min' :> 'ScalarFunctions::min'[unresolved]
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'NumericalValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'sum'
        (feature_def in 'collection' : 'ScalarValue'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'product'
        (feature_def in 'collection' : 'ScalarValue'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'sum0'
        (feature_def in 'collection' : 'NumericalValue'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'zero' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (invariant_def
          (result_expr_membership))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (feature_value (=)))))
      (function_def 'product1'
        (feature_def in 'collection' : 'ScalarValue'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'one' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (invariant_def
          (result_expr_membership))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (feature_value (=))))))))
~~~
