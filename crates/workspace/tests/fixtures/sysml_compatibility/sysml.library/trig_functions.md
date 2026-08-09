# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/TrigFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package TrigFunctions {
    doc /*
	 * This package defines basic trigonometric functions on real numbers.
	 */

    public import ScalarValues::Real;

    feature pi : Real;
    inv piPrecision { RealFunctions::round(pi * 1E20) == 314159265358979323846.0 }

    function deg { in theta_rad : Real[1];
		return : Real[1] = theta_rad * 180 / pi;
	}
    function rad { in theta_deg : Real;
		return : Real[1] = theta_deg * pi / 180;
	}

    datatype UnitBoundedReal :> Real {
        inv unitBound { -1.0 <= that & that <= 1.0 }
    }

    function sin { in theta : Real[1]; return : UnitBoundedReal[1]; }
    function cos { in theta : Real[1]; return : UnitBoundedReal[1]; }
    function tan { in theta : Real[1]; 
        return : Real = sin(theta) / cos(theta);
	}
    function cot { in theta : Real; 
        return : Real = cos(theta) / sin(theta);
	}

    function arcsin { in x : UnitBoundedReal[1]; return : Real[1]; }
    function arccos { in x : UnitBoundedReal[1]; return : Real[1]; }
    function arctan { in x : Real[1]; return : Real[1]; }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,RegularComment,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,Semicolon,
KwInv,Ident,OpenCurly,Ident,ColonColon,Ident,OpenParen,Ident,Star,ExponentialValue,CloseParen,EqEq,DecimalValue,Dot,DecimalValue,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Star,DecimalValue,Slash,Ident,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Star,Ident,Slash,DecimalValue,Semicolon,
CloseCurly,
KwDatatype,Ident,ColonGt,Ident,OpenCurly,
KwInv,Ident,OpenCurly,Minus,DecimalValue,Dot,DecimalValue,LtEq,Ident,Ampersand,Ident,LtEq,DecimalValue,Dot,DecimalValue,CloseCurly,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Slash,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Slash,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'TrigFunctions'
    (documentation)
    (import_decl public 'ScalarValues::Real')
    (feature_def 'pi' : 'Real')
    (invariant_def
      (result_expr_member))
    (function_def
      (feature_def in 'theta_rad' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'theta_deg' : 'Real')
      (return_member))
    (datatype_def 'UnitBoundedReal' :> 'Real'
      (invariant_def
        (result_expr_member)))
    (function_def
      (feature_def in 'theta' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'theta' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'theta' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'theta' : 'Real')
      (return_member))
    (function_def
      (feature_def in 'x' : 'UnitBoundedReal' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'UnitBoundedReal' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package TrigFunctions {
    doc /*
	 * This package defines basic trigonometric functions on real numbers.
	 */

    public import ScalarValues::Real;

    feature pi : Real;
    inv piPrecision { RealFunctions::round(pi * 1E20) == 314159265358979323846.0 }

    function deg { in theta_rad : Real[1];
		return : Real[1] = theta_rad * 180 / pi;
	}
    function rad { in theta_deg : Real;
		return : Real[1] = theta_deg * pi / 180;
	}

    datatype UnitBoundedReal :> Real {
        inv unitBound { -1.0 <= that & that <= 1.0 }
    }

    function sin { in theta : Real[1]; return : UnitBoundedReal[1]; }
    function cos { in theta : Real[1]; return : UnitBoundedReal[1]; }
    function tan { in theta : Real[1]; 
        return : Real = sin(theta) / cos(theta);
	}
    function cot { in theta : Real; 
        return : Real = cos(theta) / sin(theta);
	}

    function arcsin { in x : UnitBoundedReal[1]; return : Real[1]; }
    function arccos { in x : UnitBoundedReal[1]; return : Real[1]; }
    function arctan { in x : Real[1]; return : Real[1]; }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'TrigFunctions'
      (documentation)
      (membership_import public -> 'ScalarValues::Real'[unresolved])
      (feature_def 'pi' : 'Real'[unresolved])
      (invariant_def 'piPrecision'
        (result_expr_membership))
      (function_def 'deg'
        (feature_def in 'theta_rad' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'rad'
        (feature_def in 'theta_deg' : 'Real'[unresolved])
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (datatype_def 'UnitBoundedReal' :> 'Real'[unresolved]
        (invariant_def 'unitBound'
          (result_expr_membership)))
      (function_def 'sin'
        (feature_def in 'theta' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'TrigFunctions::UnitBoundedReal'[datatype_def]
            (multiplicity_range [1]))))
      (function_def 'cos'
        (feature_def in 'theta' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'TrigFunctions::UnitBoundedReal'[datatype_def]
            (multiplicity_range [1]))))
      (function_def 'tan'
        (feature_def in 'theta' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (feature_value (=)))))
      (function_def 'cot'
        (feature_def in 'theta' : 'Real'[unresolved])
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (feature_value (=)))))
      (function_def 'arcsin'
        (feature_def in 'x' : 'TrigFunctions::UnitBoundedReal'[datatype_def]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'arccos'
        (feature_def in 'x' : 'TrigFunctions::UnitBoundedReal'[datatype_def]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (function_def 'arctan'
        (feature_def in 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1])))))))
~~~
