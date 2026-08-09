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
        assert constraint {
            = m == eng.m + trans.m;
        }

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
        in partMasses : MassValue [0..*];

        = totalMass == sum(partMasses);
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
        in partMasses : MassValue [0..*];
    }

    constraint mc : MassConstraint4 {
        in totalMass : MassValue;
        in partMasses : MassValue [0..*];

        = totalMass == sum(partMasses);
    }

    part def Vehicle4 {
        assert constraint mc {
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

        = mass <= maxMass;
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
(model
  (namespace
    (package 'MassConstraintExample'
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'SI'[unresolved])
      (namespace_import private -> 'NumericalFunctions'[unresolved])
      (part_def 'Engine'
        (attribute_usage composite 'm' :> 'mass'[unresolved]))
      (part_def 'Transmission'
        (attribute_usage composite 'm' :> 'mass'[unresolved]))
      (part_def 'Vehicle1'
        (attribute_usage composite 'm' : 'MassValue'[unresolved]
          (feature_value (=)))
        (part_usage composite 'eng' : 'MassConstraintExample::Engine'[part_def]
          (attribute_usage composite :>> 'MassConstraintExample::Engine::m'[attribute_usage] : 'MassValue'[unresolved]))
        (part_usage composite 'trans' : 'MassConstraintExample::Transmission'[part_def]
          (attribute_usage composite :>> 'MassConstraintExample::Transmission::m'[attribute_usage] : 'MassValue'[unresolved])))
      (part_def 'Vehicle2'
        (assert_constraint_usage
          (result_expr_membership))
        (attribute_usage composite 'm' : 'MassValue'[unresolved])
        (part_usage composite 'eng' : 'MassConstraintExample::Engine'[part_def]
          (attribute_usage composite :>> 'MassConstraintExample::Engine::m'[attribute_usage] : 'MassValue'[unresolved]))
        (part_usage composite 'trans' : 'MassConstraintExample::Transmission'[part_def]
          (attribute_usage composite :>> 'MassConstraintExample::Transmission::m'[attribute_usage] : 'MassValue'[unresolved])))
      (constraint_def 'MassConstraint3'
        (reference_usage in reference 'totalMass' : 'MassValue'[unresolved])
        (reference_usage in reference 'partMasses' : 'MassValue'[unresolved]
          (multiplicity_range [0..*]))
        (result_expr_membership))
      (part_def 'Vehicle3'
        (assert_constraint_usage 'massConstraint' : 'MassConstraintExample::MassConstraint3'[constraint_def]
          (reference_usage in reference 'totalMass'
            (feature_value (=)))
          (reference_usage in reference 'partMasses'
            (feature_value (=))))
        (attribute_usage composite 'm' : 'MassValue'[unresolved])
        (part_usage composite 'eng'
          (attribute_usage composite 'm' : 'MassValue'[unresolved]))
        (part_usage composite 'trans'
          (attribute_usage composite 'm' : 'MassValue'[unresolved])))
      (constraint_def 'MassConstraint4'
        (reference_usage in reference 'totalMass' : 'MassValue'[unresolved])
        (reference_usage in reference 'partMasses' : 'MassValue'[unresolved]
          (multiplicity_range [0..*])))
      (constraint_usage 'mc' : 'MassConstraintExample::MassConstraint4'[constraint_def]
        (reference_usage in reference 'totalMass' : 'MassValue'[unresolved])
        (reference_usage in reference 'partMasses' : 'MassValue'[unresolved]
          (multiplicity_range [0..*]))
        (result_expr_membership))
      (part_def 'Vehicle4'
        (assert_constraint_usage 'mc'
          (reference_usage in reference 'totalMass'
            (feature_value (=)))
          (reference_usage in reference 'partMasses'
            (feature_value (=))))
        (attribute_usage composite 'm' : 'MassValue'[unresolved])
        (part_usage composite 'eng' : 'MassConstraintExample::Engine'[part_def]
          (attribute_usage composite :>> 'MassConstraintExample::Engine::m'[attribute_usage] : 'MassValue'[unresolved]))
        (part_usage composite 'trans' : 'MassConstraintExample::Transmission'[part_def]
          (attribute_usage composite :>> 'MassConstraintExample::Transmission::m'[attribute_usage] : 'MassValue'[unresolved])))
      (constraint_def 'MassLimit'
        (reference_usage in reference 'mass' : 'MassValue'[unresolved])
        (reference_usage in reference 'maxMass' : 'MassValue'[unresolved])
        (result_expr_membership))
      (part_def 'Vehicle5'
        (assert_constraint_usage 'ml' : 'MassConstraintExample::MassLimit'[constraint_def]
          (reference_usage in reference 'mass'
            (feature_value (=)))
          (reference_usage in reference 'maxMass'
            (feature_value (=))))
        (attribute_usage composite 'm' : 'MassValue'[unresolved]
          (feature_value (=)))
        (part_usage composite 'eng' : 'MassConstraintExample::Engine'[part_def]
          (attribute_usage composite :>> 'MassConstraintExample::Engine::m'[attribute_usage] : 'MassValue'[unresolved]))
        (part_usage composite 'trans' : 'MassConstraintExample::Transmission'[part_def]
          (attribute_usage composite :>> 'MassConstraintExample::Transmission::m'[attribute_usage] : 'MassValue'[unresolved]))))))
~~~
