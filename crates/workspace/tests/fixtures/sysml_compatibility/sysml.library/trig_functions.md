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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "TrigFunctions"))) (name "TrigFunctions") (declared-name "TrigFunctions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "TrigFunctions::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TrigFunctions::UnitBoundedReal"))) (name "UnitBoundedReal") (declared-name "UnitBoundedReal"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "TrigFunctions::_documentation"))) (name ""))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TrigFunctions::arccos"))) (name "arccos") (declared-name "arccos"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TrigFunctions::arcsin"))) (name "arcsin") (declared-name "arcsin"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TrigFunctions::arctan"))) (name "arctan") (declared-name "arctan"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TrigFunctions::cos"))) (name "cos") (declared-name "cos"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TrigFunctions::cot"))) (name "cot") (declared-name "cot"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TrigFunctions::deg"))) (name "deg") (declared-name "deg"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "TrigFunctions::pi"))) (name "pi") (declared-name "pi"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TrigFunctions::piPrecision"))) (name "piPrecision") (declared-name "piPrecision"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TrigFunctions::rad"))) (name "rad") (declared-name "rad"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TrigFunctions::sin"))) (name "sin") (declared-name "sin"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TrigFunctions::tan"))) (name "tan") (declared-name "tan"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TrigFunctions::_documentation"))) (to (node (document "d0") (qualified-name "TrigFunctions"))) (provenance authored))
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
  (document "sysml.library/trig_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 18) (end 5 36))
      )
    )
  )
)
~~~
