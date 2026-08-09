# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/ScalarFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package ScalarFunctions {
	doc
	/*
	 * This package defines abstract functions that specialize the DataFunctions for use with ScalarValues. 
	 */

	public import ScalarValues::*;
	
	abstract function '+' specializes DataFunctions::'+' { in x: ScalarValue[1]; in y: ScalarValue[0..1]; return : ScalarValue[1]; }
	abstract function '-' specializes DataFunctions::'-' { in x: ScalarValue[1]; in y: ScalarValue[0..1]; return : ScalarValue[1]; }
	abstract function '*' specializes DataFunctions::'*' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	abstract function '/' specializes DataFunctions::'/' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	abstract function '**' specializes DataFunctions::'**' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	abstract function '^' specializes DataFunctions::'^' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	abstract function '%' specializes DataFunctions::'%' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	
	abstract function 'not' specializes DataFunctions::'not' { in x: ScalarValue[1]; return : ScalarValue[1]; }
	abstract function 'xor' specializes DataFunctions::'xor' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }

	abstract function '~' specializes DataFunctions::'~' { in x: ScalarValue[1]; return : ScalarValue[1]; }	
	abstract function '|' specializes DataFunctions::'|' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	abstract function '&' specializes DataFunctions::'&' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	
	abstract function '<' specializes DataFunctions::'<' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : Boolean[1]; }
	abstract function '>' specializes DataFunctions::'>' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : Boolean[1]; }
	abstract function '<=' specializes DataFunctions::'<=' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : Boolean[1]; }
	abstract function '>=' specializes DataFunctions::'>=' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : Boolean[1]; }
	
	abstract function max specializes DataFunctions::max { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	abstract function min specializes DataFunctions::min { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
	
	abstract function '..' specializes DataFunctions::'..' { in lower: ScalarValue[1]; in upper: ScalarValue[1]; return : ScalarValue[0..*]; }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'DataFunctions::+'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::-'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::*'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::/'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::**'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::^'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::%'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::not'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::xor'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::~'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::|'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::&'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::<'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::>'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::<='
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::>='
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::max'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::min'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::..'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'DataFunctions::+'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::-'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::*'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::/'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::**'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::^'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::%'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::not'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::xor'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::~'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::|'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::&'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::<'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::>'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::<='
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::>='
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::max'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::min'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'DataFunctions::..'
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
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ScalarFunctions'
    (documentation)
    (import_decl public 'ScalarValues::*')
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'ScalarValue' multiplicity)
      (feature_def in 'y' : 'ScalarValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'lower' : 'ScalarValue' multiplicity)
      (feature_def in 'upper' : 'ScalarValue' multiplicity)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package ScalarFunctions {
    doc /*
	 * This package defines abstract functions that specialize the DataFunctions for use with ScalarValues. 
	 */

    public import ScalarValues::*;

    abstract function '+' specializes DataFunctions::'+' { in x: ScalarValue[1]; in y: ScalarValue[0..1]; return : ScalarValue[1]; }
    abstract function '-' specializes DataFunctions::'-' { in x: ScalarValue[1]; in y: ScalarValue[0..1]; return : ScalarValue[1]; }
    abstract function '*' specializes DataFunctions::'*' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
    abstract function '/' specializes DataFunctions::'/' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
    abstract function '**' specializes DataFunctions::'**' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
    abstract function '^' specializes DataFunctions::'^' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
    abstract function '%' specializes DataFunctions::'%' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }

    abstract function 'not' specializes DataFunctions::'not' { in x: ScalarValue[1]; return : ScalarValue[1]; }
    abstract function 'xor' specializes DataFunctions::'xor' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }

    abstract function '~' specializes DataFunctions::'~' { in x: ScalarValue[1]; return : ScalarValue[1]; }
    abstract function '|' specializes DataFunctions::'|' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
    abstract function '&' specializes DataFunctions::'&' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }

    abstract function '<' specializes DataFunctions::'<' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : Boolean[1]; }
    abstract function '>' specializes DataFunctions::'>' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : Boolean[1]; }
    abstract function '<=' specializes DataFunctions::'<=' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : Boolean[1]; }
    abstract function '>=' specializes DataFunctions::'>=' { in x: ScalarValue[1]; in y: ScalarValue[1]; return : Boolean[1]; }

    abstract function max specializes DataFunctions::max { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }
    abstract function min specializes DataFunctions::min { in x: ScalarValue[1]; in y: ScalarValue[1]; return : ScalarValue[1]; }

    abstract function '..' specializes DataFunctions::'..' { in lower: ScalarValue[1]; in upper: ScalarValue[1]; return : ScalarValue[0..*]; }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'ScalarFunctions'
      (documentation)
      (namespace_import public -> 'ScalarValues'[unresolved])
      (function_def abstract '+' :> 'DataFunctions::+'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '-' :> 'DataFunctions::-'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '*' :> 'DataFunctions::*'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '/' :> 'DataFunctions::/'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '**' :> 'DataFunctions::**'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '^' :> 'DataFunctions::^'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '%' :> 'DataFunctions::%'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'not' :> 'DataFunctions::not'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'xor' :> 'DataFunctions::xor'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '~' :> 'DataFunctions::~'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '|' :> 'DataFunctions::|'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '&' :> 'DataFunctions::&'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '<' :> 'DataFunctions::<'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '>' :> 'DataFunctions::>'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '<=' :> 'DataFunctions::<='[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '>=' :> 'DataFunctions::>='[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'max' :> 'DataFunctions::max'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'min' :> 'DataFunctions::min'[unresolved]
        (feature_def in 'x' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '..' :> 'DataFunctions::..'[unresolved]
        (feature_def in 'lower' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'upper' : 'ScalarValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved]
            (multiplicity_range [0..*])))))))
~~~
