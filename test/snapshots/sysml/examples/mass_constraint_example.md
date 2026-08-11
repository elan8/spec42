# META
~~~ini
description=SysML Example (Mass Roll-up): MassConstraintExample
type=file
~~~
# SOURCE
~~~sysml
package MassConstraintExample {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	part def Engine {
		attribute m :> mass;
	}
	
	part def Transmission {
		attribute m :> mass;
	}
	
	part def Vehicle1 {
		attribute m : MassValue = eng.m + trans.m;
		
		part eng : Engine {
			attribute :>> m : MassValue;
		}
		
		part trans : Transmission {
			attribute :>> m : MassValue;
		}
	}
	
	part def Vehicle2 {
		assert constraint { m == eng.m + trans.m }
		
		attribute m : MassValue;
		
		part eng : Engine {
			attribute :>> m : MassValue;
		}
		
		part trans : Transmission {
			attribute :>> m : MassValue;
		}
	}
	
	constraint def MassConstraint3 {
		in totalMass : MassValue; 
		in partMasses : MassValue[0..*];
			
		totalMass == sum(partMasses)
	}
	
	part def Vehicle3 {
		assert constraint massConstraint : MassConstraint3 {
			in totalMass = m;
			in partMasses = (eng.m, trans.m);
		}
		
		attribute m : MassValue;
		
		part eng {
			attribute m : MassValue;
		}
		
		part trans {
			attribute m : MassValue;
		}
	}
	
	constraint def MassConstraint4 {
		in totalMass : MassValue;
		in partMasses : MassValue[0..*];
	}
	
	constraint mc : MassConstraint4 {
		in totalMass : MassValue; 
		in partMasses : MassValue[0..*];
		
		totalMass == sum(partMasses)
	}
	
	part def Vehicle4 {
		assert mc {
			in totalMass = m;
			in partMasses = (eng.m, trans.m);
		}
		
		attribute m : MassValue;
		
		part eng : Engine {
			attribute :>> m : MassValue;
		}
		
		part trans : Transmission {
			attribute :>> m : MassValue;
		}
	}
	
	constraint def MassLimit {
		in mass : MassValue; 
		in maxMass : MassValue;
			
		mass <= maxMass
	}
	
	part def Vehicle5 {
		assert constraint ml : MassLimit {
			in mass = m;
			in maxMass = 2500 [kg];
		}
		
		attribute m : MassValue = eng.m + trans.m;
		
		part eng : Engine {
			attribute :>> m : MassValue;
		}
		
		part trans : Transmission {
			attribute :>> m : MassValue;
		}
	}	
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "mass_constraint_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 17) (end 6 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 17) (end 10 21))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 14 2) (end 14 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 2) (end 14 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 16) (end 14 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 3) (end 17 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 21) (end 17 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 3) (end 21 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 21) (end 21 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 2) (end 28 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 16) (end 28 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 3) (end 31 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 21) (end 31 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 3) (end 35 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 21) (end 35 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 2) (end 52 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 16) (end 52 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 3) (end 55 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 17) (end 55 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 3) (end 59 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 17) (end 59 26))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 68 1) (end 68 135))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 76 2) (end 76 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 2) (end 81 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 16) (end 81 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 84 3) (end 84 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 84 21) (end 84 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 88 3) (end 88 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 88 21) (end 88 30))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 105 2) (end 105 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 2) (end 105 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 16) (end 105 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 108 3) (end 108 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 108 21) (end 108 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 3) (end 112 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 21) (end 112 30))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Plus,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAssert,KwConstraint,OpenCurly,Ident,EqEq,Ident,Dot,Ident,Plus,Ident,Dot,Ident,CloseCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwConstraint,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
Ident,EqEq,Ident,OpenParen,Ident,CloseParen,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAssert,KwConstraint,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,OpenParen,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwConstraint,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwConstraint,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
Ident,EqEq,Ident,OpenParen,Ident,CloseParen,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAssert,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,OpenParen,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwConstraint,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
Ident,LtEq,Ident,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAssert,KwConstraint,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Plus,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MassConstraintExample'
    (import_decl private 'ISQ::*')
    (import_decl private 'SI::*')
    (import_decl private 'NumericalFunctions::*')
    (part_def 'Engine'
      (attribute_usage 'm' :> 'mass'))
    (part_def 'Transmission'
      (attribute_usage 'm' :> 'mass'))
    (part_def 'Vehicle1'
      (attribute_usage 'm' : 'MassValue' value)
      (part_usage 'eng' : 'Engine'
        (attribute_usage :>> 'm' : 'MassValue'))
      (part_usage 'trans' : 'Transmission'
        (attribute_usage :>> 'm' : 'MassValue')))
    (part_def 'Vehicle2'
      (sysml_decl
        (result_expr_member))
      (attribute_usage 'm' : 'MassValue')
      (part_usage 'eng' : 'Engine'
        (attribute_usage :>> 'm' : 'MassValue'))
      (part_usage 'trans' : 'Transmission'
        (attribute_usage :>> 'm' : 'MassValue')))
    (constraint_def 'MassConstraint3'
      (default_ref_usage in 'totalMass' : 'MassValue')
      (default_ref_usage in 'partMasses' : 'MassValue' multiplicity)
      (result_expr_member))
    (part_def 'Vehicle3'
      (sysml_decl 'massConstraint' : 'MassConstraint3'
        (default_ref_usage in 'totalMass' value)
        (default_ref_usage in 'partMasses' value))
      (attribute_usage 'm' : 'MassValue')
      (part_usage 'eng'
        (attribute_usage 'm' : 'MassValue'))
      (part_usage 'trans'
        (attribute_usage 'm' : 'MassValue')))
    (constraint_def 'MassConstraint4'
      (default_ref_usage in 'totalMass' : 'MassValue')
      (default_ref_usage in 'partMasses' : 'MassValue' multiplicity))
    (constraint_usage 'mc' : 'MassConstraint4'
      (default_ref_usage in 'totalMass' : 'MassValue')
      (default_ref_usage in 'partMasses' : 'MassValue' multiplicity)
      (result_expr_member))
    (part_def 'Vehicle4'
      (sysml_decl 'mc'
        (default_ref_usage in 'totalMass' value)
        (default_ref_usage in 'partMasses' value))
      (attribute_usage 'm' : 'MassValue')
      (part_usage 'eng' : 'Engine'
        (attribute_usage :>> 'm' : 'MassValue'))
      (part_usage 'trans' : 'Transmission'
        (attribute_usage :>> 'm' : 'MassValue')))
    (constraint_def 'MassLimit'
      (default_ref_usage in 'mass' : 'MassValue')
      (default_ref_usage in 'maxMass' : 'MassValue')
      (result_expr_member))
    (part_def 'Vehicle5'
      (sysml_decl 'ml' : 'MassLimit'
        (default_ref_usage in 'mass' value)
        (default_ref_usage in 'maxMass' value))
      (attribute_usage 'm' : 'MassValue' value)
      (part_usage 'eng' : 'Engine'
        (attribute_usage :>> 'm' : 'MassValue'))
      (part_usage 'trans' : 'Transmission'
        (attribute_usage :>> 'm' : 'MassValue')))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'mass'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'mass'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# FORMAT
~~~sysml
package MassConstraintExample {
    private import ISQ::*;
    private import SI::*;
    private import NumericalFunctions::*;

    part def Engine {
        attribute m :> mass;
    }

    part def Transmission {
        attribute m :> mass;
    }

    part def Vehicle1 {
        attribute m : MassValue = eng.m + trans.m;

        part eng : Engine {
            attribute :>> m : MassValue;
        }

        part trans : Transmission {
            attribute :>> m : MassValue;
        }
    }

    part def Vehicle2 {
        assert constraint { m == eng.m + trans.m }

        attribute m : MassValue;

        part eng : Engine {
            attribute :>> m : MassValue;
        }

        part trans : Transmission {
            attribute :>> m : MassValue;
        }
    }

    constraint def MassConstraint3 {
        in totalMass : MassValue;
        in partMasses : MassValue[0..*];

        totalMass == sum(partMasses)
    }

    part def Vehicle3 {
        assert constraint massConstraint : MassConstraint3 {
            in totalMass = m;
            in partMasses = (eng.m, trans.m);
        }

        attribute m : MassValue;

        part eng {
            attribute m : MassValue;
        }

        part trans {
            attribute m : MassValue;
        }
    }

    constraint def MassConstraint4 {
        in totalMass : MassValue;
        in partMasses : MassValue[0..*];
    }

    constraint mc : MassConstraint4 {
        in totalMass : MassValue;
        in partMasses : MassValue[0..*];

        totalMass == sum(partMasses)
    }

    part def Vehicle4 {
        assert mc {
            in totalMass = m;
            in partMasses = (eng.m, trans.m);
        }

        attribute m : MassValue;

        part eng : Engine {
            attribute :>> m : MassValue;
        }

        part trans : Transmission {
            attribute :>> m : MassValue;
        }
    }

    constraint def MassLimit {
        in mass : MassValue;
        in maxMass : MassValue;

        mass <= maxMass
    }

    part def Vehicle5 {
        assert constraint ml : MassLimit {
            in mass = m;
            in maxMass = 2500 [kg];
        }

        attribute m : MassValue = eng.m + trans.m;

        part eng : Engine {
            attribute :>> m : MassValue;
        }

        part trans : Transmission {
            attribute :>> m : MassValue;
        }
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "9fd0566c0f51bdc9111e7479c1d0bd257d5402f1644d46d82586d55cc36ab3ce") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MassConstraintExample"))) (kind "package") (name "MassConstraintExample") (declared-name "MassConstraintExample") (range (start (line 0) (character 0)) (end (line 0) (character 1958))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 23))) (parent (node (document "d0") (qualified-name "MassConstraintExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 19))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 22))) (parent (node (document "d0") (qualified-name "MassConstraintExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 18))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 38))) (parent (node (document "d0") (qualified-name "MassConstraintExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 34))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 5) (character 1)) (end (line 5) (character 44))) (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Engine::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 6) (character 2)) (end (line 6) (character 22))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Engine"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "mass") (range (start (line 6) (character 17)) (end (line 6) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::MassConstraint3"))) (kind "constraint def") (name "MassConstraint3") (declared-name "MassConstraint3") (range (start (line 39) (character 1)) (end (line 39) (character 135))) (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::MassConstraint4"))) (kind "constraint def") (name "MassConstraint4") (declared-name "MassConstraint4") (range (start (line 63) (character 1)) (end (line 63) (character 99))) (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::MassLimit"))) (kind "constraint def") (name "MassLimit") (declared-name "MassLimit") (range (start (line 92) (character 1)) (end (line 92) (character 102))) (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (range (start (line 9) (character 1)) (end (line 9) (character 50))) (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Transmission::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 10) (character 2)) (end (line 10) (character 22))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "mass") (range (start (line 10) (character 17)) (end (line 10) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1"))) (kind "part def") (name "Vehicle1") (declared-name "Vehicle1") (range (start (line 13) (character 1)) (end (line 13) (character 198))) (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 16) (character 2)) (end (line 16) (character 57))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 16) (character 13)) (end (line 16) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 17) (character 3)) (end (line 17) (character 31))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 17) (character 21)) (end (line 17) (character 30)))) (redefinition (reference "m") (range (start (line 17) (character 17)) (end (line 17) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 14) (character 2)) (end (line 14) (character 44))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 14) (character 16)) (end (line 14) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (kind "part") (name "trans") (declared-name "trans") (range (start (line 20) (character 2)) (end (line 20) (character 65))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 20) (character 15)) (end (line 20) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 21) (character 3)) (end (line 21) (character 31))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 21) (character 21)) (end (line 21) (character 30)))) (redefinition (reference "m") (range (start (line 21) (character 17)) (end (line 21) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2"))) (kind "part def") (name "Vehicle2") (declared-name "Vehicle2") (range (start (line 25) (character 1)) (end (line 25) (character 228))) (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 30) (character 2)) (end (line 30) (character 57))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 30) (character 13)) (end (line 30) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 31) (character 3)) (end (line 31) (character 31))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 31) (character 21)) (end (line 31) (character 30)))) (redefinition (reference "m") (range (start (line 31) (character 17)) (end (line 31) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 28) (character 2)) (end (line 28) (character 26))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 28) (character 16)) (end (line 28) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (kind "part") (name "trans") (declared-name "trans") (range (start (line 34) (character 2)) (end (line 34) (character 65))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 34) (character 15)) (end (line 34) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 35) (character 3)) (end (line 35) (character 31))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 35) (character 21)) (end (line 35) (character 30)))) (redefinition (reference "m") (range (start (line 35) (character 17)) (end (line 35) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3"))) (kind "part def") (name "Vehicle3") (declared-name "Vehicle3") (range (start (line 46) (character 1)) (end (line 46) (character 268))) (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 54) (character 2)) (end (line 54) (character 44))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::eng::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 55) (character 3)) (end (line 55) (character 27))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::eng"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 55) (character 17)) (end (line 55) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 52) (character 2)) (end (line 52) (character 26))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 52) (character 16)) (end (line 52) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::trans"))) (kind "part") (name "trans") (declared-name "trans") (range (start (line 58) (character 2)) (end (line 58) (character 46))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::trans::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 59) (character 3)) (end (line 59) (character 27))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::trans"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 59) (character 17)) (end (line 59) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4"))) (kind "part def") (name "Vehicle4") (declared-name "Vehicle4") (range (start (line 75) (character 1)) (end (line 75) (character 259))) (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 83) (character 2)) (end (line 83) (character 57))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 83) (character 13)) (end (line 83) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 84) (character 3)) (end (line 84) (character 31))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 84) (character 21)) (end (line 84) (character 30)))) (redefinition (reference "m") (range (start (line 84) (character 17)) (end (line 84) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 81) (character 2)) (end (line 81) (character 26))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 81) (character 16)) (end (line 81) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (kind "part") (name "trans") (declared-name "trans") (range (start (line 87) (character 2)) (end (line 87) (character 65))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 87) (character 15)) (end (line 87) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 88) (character 3)) (end (line 88) (character 31))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 88) (character 21)) (end (line 88) (character 30)))) (redefinition (reference "m") (range (start (line 88) (character 17)) (end (line 88) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5"))) (kind "part def") (name "Vehicle5") (declared-name "Vehicle5") (range (start (line 99) (character 1)) (end (line 99) (character 285))) (parent (node (document "d0") (qualified-name "MassConstraintExample"))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 107) (character 2)) (end (line 107) (character 57))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 107) (character 13)) (end (line 107) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 108) (character 3)) (end (line 108) (character 31))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 108) (character 21)) (end (line 108) (character 30)))) (redefinition (reference "m") (range (start (line 108) (character 17)) (end (line 108) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 105) (character 2)) (end (line 105) (character 44))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 105) (character 16)) (end (line 105) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (kind "part") (name "trans") (declared-name "trans") (range (start (line 111) (character 2)) (end (line 111) (character 65))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 111) (character 15)) (end (line 111) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 112) (character 3)) (end (line 112) (character 31))) (parent (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 112) (character 21)) (end (line 112) (character 30)))) (redefinition (reference "m") (range (start (line 112) (character 17)) (end (line 112) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "MassConstraintExample::mc"))) (kind "constraint") (name "mc") (declared-name "mc") (range (start (line 68) (character 1)) (end (line 68) (character 135))) (parent (node (document "d0") (qualified-name "MassConstraintExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassConstraint4") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 1) (character 16)) (end (line 1) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 2) (character 16)) (end (line 2) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (range (start (line 3) (character 16)) (end (line 3) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Engine::m"))) (kind subsetting) (ordinal 0)) (authored-target "mass") (range (start (line 6) (character 17)) (end (line 6) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Transmission::m"))) (kind subsetting) (ordinal 0)) (authored-target "mass") (range (start (line 10) (character 17)) (end (line 10) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 16) (character 13)) (end (line 16) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 17) (character 21)) (end (line 17) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (range (start (line 17) (character 17)) (end (line 17) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 14) (character 16)) (end (line 14) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 20) (character 15)) (end (line 20) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 21) (character 21)) (end (line 21) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (range (start (line 21) (character 17)) (end (line 21) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 30) (character 13)) (end (line 30) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 31) (character 21)) (end (line 31) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (range (start (line 31) (character 17)) (end (line 31) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 28) (character 16)) (end (line 28) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 34) (character 15)) (end (line 34) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 35) (character 21)) (end (line 35) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (range (start (line 35) (character 17)) (end (line 35) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::eng::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::eng::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 55) (character 17)) (end (line 55) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 52) (character 16)) (end (line 52) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::trans::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::trans::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 59) (character 17)) (end (line 59) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 83) (character 13)) (end (line 83) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 84) (character 21)) (end (line 84) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (range (start (line 84) (character 17)) (end (line 84) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 81) (character 16)) (end (line 81) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 87) (character 15)) (end (line 87) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 88) (character 21)) (end (line 88) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (range (start (line 88) (character 17)) (end (line 88) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 107) (character 13)) (end (line 107) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 108) (character 21)) (end (line 108) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (range (start (line 108) (character 17)) (end (line 108) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 105) (character 16)) (end (line 105) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 111) (character 15)) (end (line 111) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 112) (character 21)) (end (line 112) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (range (start (line 112) (character 17)) (end (line 112) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m")))))
    (reference (id (source (node (document "d0") (qualified-name "MassConstraintExample::mc"))) (kind featureTyping) (ordinal 0)) (authored-target "MassConstraint4") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassConstraintExample::MassConstraint4")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (target (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassConstraintExample::mc"))) (target (node (document "d0") (qualified-name "MassConstraintExample::MassConstraint4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassConstraintExample::mc"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "MassConstraintExample::MassConstraint3")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "MassConstraintExample::MassLimit")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::m")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::m")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "MassConstraintExample::mc")) (expression (status "unresolved") (error "expression has an unresolved reference")) (analysis (status "unresolved")))
  )
)
~~~
