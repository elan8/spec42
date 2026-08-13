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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/trig_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 18) (end 5 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 7 4) (end 7 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 8 4) (end 8 82))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 4) (end 8 82))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 10 4) (end 12 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 4) (end 12 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 13 4) (end 15 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 4) (end 15 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 17 4) (end 19 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 17 4) (end 19 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 21 4) (end 21 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 21 4) (end 21 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 22 4) (end 22 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 22 4) (end 22 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 23 4) (end 25 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 23 4) (end 25 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 26 4) (end 28 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 26 4) (end 28 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 30 4) (end 30 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 30 4) (end 30 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 31 4) (end 31 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 31 4) (end 31 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 32 4) (end 32 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 32 4) (end 32 57))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:000dea9026902fb35d3bb23a3023acdbc3abc8f6b2d1edbe86f352c730446237") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/trig_functions.md") (qualified-name "TrigFunctions"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/trig_functions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/trig_functions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/trig_functions.md") (range (start 5 18) (end 5 36)) (probe (position 5 18))
    (reference (id (source (node (document "memory://snapshot/trig_functions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
)
~~~
