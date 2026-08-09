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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "MassConstraintExample"))) (name "MassConstraintExample") (declared-name "MassConstraintExample")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "MassConstraintExample::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "MassConstraintExample::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "MassConstraintExample::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "MassConstraintExample::Engine"))) (name "Engine") (declared-name "Engine") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Engine::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Engine")))))
          )
        )
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "MassConstraintExample::MassConstraint3"))) (name "MassConstraint3") (declared-name "MassConstraint3"))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "MassConstraintExample::MassConstraint4"))) (name "MassConstraint4") (declared-name "MassConstraint4"))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "MassConstraintExample::MassLimit"))) (name "MassLimit") (declared-name "MassLimit"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))) (name "Transmission") (declared-name "Transmission") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Transmission::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Transmission")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1"))) (name "Vehicle1") (declared-name "Vehicle1") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (name "eng") (declared-name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Engine")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "+") (children (expression (kind "memberAccess") (reference "m") (children (expression (kind "featureReference") (reference "eng")))) (expression (kind "memberAccess") (reference "m") (children (expression (kind "featureReference") (reference "trans")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::m"))) (role feature-value))))
            (element (kind "part") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (name "trans") (declared-name "trans") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Transmission")))))
              )
            )
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2"))) (name "Vehicle2") (declared-name "Vehicle2") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (name "eng") (declared-name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Engine")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (name "trans") (declared-name "trans") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Transmission")))))
              )
            )
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3"))) (name "Vehicle3") (declared-name "Vehicle3") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::eng"))) (name "eng") (declared-name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::eng::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::trans"))) (name "trans") (declared-name "trans") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3::trans::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle3")))))
              )
            )
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4"))) (name "Vehicle4") (declared-name "Vehicle4") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (name "eng") (declared-name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Engine")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (name "trans") (declared-name "trans") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Transmission")))))
              )
            )
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5"))) (name "Vehicle5") (declared-name "Vehicle5") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (name "eng") (declared-name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Engine")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "+") (children (expression (kind "memberAccess") (reference "m") (children (expression (kind "featureReference") (reference "eng")))) (expression (kind "memberAccess") (reference "m") (children (expression (kind "featureReference") (reference "trans")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::m"))) (role feature-value))))
            (element (kind "part") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (name "trans") (declared-name "trans") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassConstraintExample::Transmission")))))
              )
            )
          )
        )
        (element (kind "constraint") (id (node (document "d0") (qualified-name "MassConstraintExample::mc"))) (name "mc") (declared-name "mc"))
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng::m"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Engine::m"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans::m"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Transmission::m"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng::m"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Engine::m"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans::m"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Transmission::m"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng::m"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Engine::m"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans::m"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Transmission::m"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng::m"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Engine::m"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans::m"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Transmission::m"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::eng"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle1::trans"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::eng"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle2::trans"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::eng"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle4::trans"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::eng"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::Vehicle5::trans"))) (to (node (document "d0") (qualified-name "MassConstraintExample::Transmission"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassConstraintExample::mc"))) (to (node (document "d0") (qualified-name "MassConstraintExample::MassConstraint4"))))
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
  (document "sysml/examples/mass_constraint_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 1) (end 3 38))
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
        (range (start 17 3) (end 17 31))
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
        (range (start 28 2) (end 28 26))
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
        (range (start 35 3) (end 35 31))
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
        (range (start 55 3) (end 55 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 3) (end 59 27))
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
        (range (start 84 3) (end 84 31))
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
        (range (start 105 2) (end 105 44))
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
        (range (start 112 3) (end 112 31))
      )
    )
  )
)
~~~
