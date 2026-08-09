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
    doc /*
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
(model
  (namespace
    (library_package 'NaturalFunctions'
      (documentation)
      (namespace_import public -> 'ScalarValues'[unresolved])
      (function_def '+' :> 'IntegerFunctions::+'[unresolved]
        (feature_def in 'x' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Natural'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Natural'[unresolved]
            (multiplicity_range [1]))))
      (function_def '*' :> 'IntegerFunctions::*'[unresolved]
        (feature_def in 'x' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Natural'[unresolved]
            (multiplicity_range [1]))))
      (function_def '/' :> 'IntegerFunctions::/'[unresolved]
        (feature_def in 'x' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Natural'[unresolved]
            (multiplicity_range [1]))))
      (function_def '%' :> 'IntegerFunctions::%'[unresolved]
        (feature_def in 'x' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Natural'[unresolved]
            (multiplicity_range [1]))))
      (function_def '<' :> 'IntegerFunctions::<'[unresolved]
        (feature_def in 'x' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '>' :> 'IntegerFunctions::>'[unresolved]
        (feature_def in 'x' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '<=' :> 'IntegerFunctions::<='[unresolved]
        (feature_def in 'x' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '>=' :> 'IntegerFunctions::>='[unresolved]
        (feature_def in 'x' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'max' :> 'IntegerFunctions::max'[unresolved]
        (feature_def in 'x' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Natural'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'min' :> 'IntegerFunctions::min'[unresolved]
        (feature_def in 'x' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'y' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Natural'[unresolved]
            (multiplicity_range [1]))))
      (function_def '==' :> 'IntegerFunctions::=='[unresolved]
        (feature_def in 'x' : 'Natural'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def in 'y' : 'Natural'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'ToString' :> 'IntegerFunctions::ToString'[unresolved]
        (feature_def in 'x' : 'Natural'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'String'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'ToNatural'
        (feature_def in 'x' : 'String'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Natural'[unresolved]
            (multiplicity_range [1])))))))
~~~
