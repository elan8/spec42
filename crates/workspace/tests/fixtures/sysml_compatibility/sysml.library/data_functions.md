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
    doc /*
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
(model
  (namespace
    (library_package 'DataFunctions'
      (documentation)
      (membership_import private -> 'Base::DataValue'[unresolved])
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'ControlFunctions::reduce'[unresolved])
      (function_def abstract '==' :> 'BaseFunctions::=='[unresolved]
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '===' :> 'BaseFunctions::==='[unresolved]
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def abstract '+'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'DataValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '-'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'DataValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '*'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'DataValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '/'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'DataValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '**'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'DataValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '^'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'DataValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '%'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'DataValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'not'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'DataValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'xor'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'DataValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '~'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'DataValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '|'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'DataValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '&'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'DataValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '<'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '>'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '<='
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '>='
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'max'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'DataValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'min'
        (feature_def in 'x' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'DataValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '..'
        (feature_def in 'lower' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'upper' : 'DataValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out ordered : 'DataValue'[unresolved]
            (multiplicity_range [0..*])))))))
~~~
