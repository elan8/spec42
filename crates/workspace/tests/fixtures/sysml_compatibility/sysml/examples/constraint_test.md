# META
~~~ini
description=SysML Example (Simple Tests): ConstraintTest
type=file
~~~
# SOURCE
~~~sysml
package ConstraintTest {
	private import ISQ::MassValue;
	private import SI::kg;
	private import NumericalFunctions::sum;
	
	constraint def MassAnalysis {
		attribute totalMass: MassValue;
		attribute componentMasses: MassValue[0..*];		

		totalMass == sum(componentMasses)
	}
	
	part def Component {
		attribute mass: MassValue;
	}
	
	part vehicle : Component {	
		part engine : Component;
		part frontAxleAssembly : Component;
		part rearAxleAssembly : Component;	
	}
		
	part vehicle1a :> vehicle {
		assert constraint massAnalysis : MassAnalysis {
			attribute redefines totalMass;
			attribute redefines componentMasses;
		}
		
		bind massAnalysis.totalMass = mass;
		bind massAnalysis.componentMasses = engine.mass;
		bind massAnalysis.componentMasses = frontAxleAssembly.mass;
		bind massAnalysis.componentMasses = rearAxleAssembly.mass;
	}
	
	part vehicle1b :> vehicle {		
		assert constraint massAnalysis : MassAnalysis {
			attribute redefines totalMass = mass;
			attribute redefines componentMasses = (engine.mass, frontAxleAssembly.mass, rearAxleAssembly.mass);		
		}	
	}
		
	constraint def MassAnalysis2 { 
		in totalMass : MassValue;
		in componentMasses: MassValue[0..*];
		
		totalMass == sum(componentMasses)
	}
	
	part vehicle2a :> vehicle {
		assert constraint massConstraint : MassAnalysis2;
		
		bind massConstraint.totalMass = mass;
		bind massConstraint.componentMasses = engine.mass;
		bind massConstraint.componentMasses = frontAxleAssembly.mass;
		bind massConstraint.componentMasses = rearAxleAssembly.mass;
	}
		
	part vehicle2b :> vehicle {
		assert constraint massAnalysis2 : MassAnalysis2 {
			in totalMass = mass;
			in componentMasses = (engine.mass, frontAxleAssembly.mass, rearAxleAssembly.mass);
		}
	}
	
	constraint def MassAnalysis3 {
		in totalMass : MassValue;
		in componentMasses: MassValue[0..*];
	}
	
	constraint massAnalysis3 : MassAnalysis3 {
		in totalMass : MassValue;
		in componentMasses: MassValue[0..*];
		
		totalMass == sum(componentMasses)
	}
	
	part vehicle3 :> vehicle {
		assert massAnalysis3 {
			in totalMass = mass;
			in componentMasses = (engine.mass, frontAxleAssembly.mass, rearAxleAssembly.mass);
		}
	}
	
	part vehicle4 :> vehicle {
		assert constraint { mass == engine.mass + frontAxleAssembly.mass + rearAxleAssembly.mass }
	}
	
	constraint massLimitation { mass : MassValue; massLimit : MassValue; mass < massLimit }
	assert not massLimitation { :>> mass = vehicle3.mass; :>> massLimit = vehicle4.mass; }
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwConstraint,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
Ident,EqEq,Ident,OpenParen,Ident,CloseParen,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAssert,KwConstraint,Ident,Colon,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Semicolon,
CloseCurly,
KwBind,Ident,Dot,Ident,Eq,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAssert,KwConstraint,Ident,Colon,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,OpenParen,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
KwConstraint,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
Ident,EqEq,Ident,OpenParen,Ident,CloseParen,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAssert,KwConstraint,Ident,Colon,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAssert,KwConstraint,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,OpenParen,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
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
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAssert,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,OpenParen,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAssert,KwConstraint,OpenCurly,Ident,EqEq,Ident,Dot,Ident,Plus,Ident,Dot,Ident,Plus,Ident,Dot,Ident,CloseCurly,
CloseCurly,
KwConstraint,Ident,OpenCurly,Ident,Colon,Ident,Semicolon,Ident,Colon,Ident,Semicolon,Ident,OpenAngle,Ident,CloseCurly,
KwAssert,KwNot,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ConstraintTest'
    (import_decl private 'ISQ::MassValue')
    (import_decl private 'SI::kg')
    (import_decl private 'NumericalFunctions::sum')
    (constraint_def 'MassAnalysis'
      (attribute_usage 'totalMass' : 'MassValue')
      (attribute_usage 'componentMasses' : 'MassValue' multiplicity)
      (result_expr_member))
    (part_def 'Component'
      (attribute_usage 'mass' : 'MassValue'))
    (part_usage 'vehicle' : 'Component'
      (part_usage 'engine' : 'Component')
      (part_usage 'frontAxleAssembly' : 'Component')
      (part_usage 'rearAxleAssembly' : 'Component'))
    (part_usage 'vehicle1a' :> 'vehicle'
      (sysml_decl 'massAnalysis' : 'MassAnalysis'
        (attribute_usage :>> 'totalMass')
        (attribute_usage :>> 'componentMasses'))
      (binding_as_usage
        (connector_end)
        (connector_end))
      (binding_as_usage
        (connector_end)
        (connector_end))
      (binding_as_usage
        (connector_end)
        (connector_end))
      (binding_as_usage
        (connector_end)
        (connector_end)))
    (part_usage 'vehicle1b' :> 'vehicle'
      (sysml_decl 'massAnalysis' : 'MassAnalysis'
        (attribute_usage :>> 'totalMass' value)
        (attribute_usage :>> 'componentMasses' value)))
    (constraint_def 'MassAnalysis2'
      (default_ref_usage in 'totalMass' : 'MassValue')
      (default_ref_usage in 'componentMasses' : 'MassValue' multiplicity)
      (result_expr_member))
    (part_usage 'vehicle2a' :> 'vehicle'
      (sysml_decl 'massConstraint' : 'MassAnalysis2')
      (binding_as_usage
        (connector_end)
        (connector_end))
      (binding_as_usage
        (connector_end)
        (connector_end))
      (binding_as_usage
        (connector_end)
        (connector_end))
      (binding_as_usage
        (connector_end)
        (connector_end)))
    (part_usage 'vehicle2b' :> 'vehicle'
      (sysml_decl 'massAnalysis2' : 'MassAnalysis2'
        (default_ref_usage in 'totalMass' value)
        (default_ref_usage in 'componentMasses' value)))
    (constraint_def 'MassAnalysis3'
      (default_ref_usage in 'totalMass' : 'MassValue')
      (default_ref_usage in 'componentMasses' : 'MassValue' multiplicity))
    (constraint_usage 'massAnalysis3' : 'MassAnalysis3'
      (default_ref_usage in 'totalMass' : 'MassValue')
      (default_ref_usage in 'componentMasses' : 'MassValue' multiplicity)
      (result_expr_member))
    (part_usage 'vehicle3' :> 'vehicle'
      (sysml_decl 'massAnalysis3'
        (default_ref_usage in 'totalMass' value)
        (default_ref_usage in 'componentMasses' value)))
    (part_usage 'vehicle4' :> 'vehicle'
      (sysml_decl
        (result_expr_member)))
    (constraint_usage 'massLimitation'
      (default_ref_usage 'mass' : 'MassValue')
      (default_ref_usage 'massLimit' : 'MassValue')
      (result_expr_member))
    (sysml_decl 'massLimitation'
      (default_ref_usage :>> 'mass' value)
      (default_ref_usage :>> 'massLimit' value))))
~~~
# FORMAT
~~~sysml
package ConstraintTest {
    private import ISQ::MassValue;
    private import SI::kg;
    private import NumericalFunctions::sum;

    constraint def MassAnalysis {
        attribute totalMass : MassValue;
        attribute componentMasses : MassValue [0..*];

        = totalMass == sum(componentMasses);
    }

    part def Component {
        attribute mass : MassValue;
    }

    part vehicle : Component {
        part engine : Component;
        part frontAxleAssembly : Component;
        part rearAxleAssembly : Component;
    }

    part vehicle1a :> vehicle {
        assert constraint massAnalysis : MassAnalysis {
            attribute redefines totalMass;
            attribute redefines componentMasses;
        }

        bind massAnalysis.totalMass = mass;
        bind massAnalysis.componentMasses = engine.mass;
        bind massAnalysis.componentMasses = frontAxleAssembly.mass;
        bind massAnalysis.componentMasses = rearAxleAssembly.mass;
    }

    part vehicle1b :> vehicle {
        assert constraint massAnalysis : MassAnalysis {
            attribute redefines totalMass = mass;
            attribute redefines componentMasses = (engine.mass, frontAxleAssembly.mass, rearAxleAssembly.mass);
        }
    }

    constraint def MassAnalysis2 {
        in totalMass : MassValue;
        in componentMasses : MassValue [0..*];

        = totalMass == sum(componentMasses);
    }

    part vehicle2a :> vehicle {
        assert constraint massConstraint : MassAnalysis2;

        bind massConstraint.totalMass = mass;
        bind massConstraint.componentMasses = engine.mass;
        bind massConstraint.componentMasses = frontAxleAssembly.mass;
        bind massConstraint.componentMasses = rearAxleAssembly.mass;
    }

    part vehicle2b :> vehicle {
        assert constraint massAnalysis2 : MassAnalysis2 {
            in totalMass = mass;
            in componentMasses = (engine.mass, frontAxleAssembly.mass, rearAxleAssembly.mass);
        }
    }

    constraint def MassAnalysis3 {
        in totalMass : MassValue;
        in componentMasses : MassValue [0..*];
    }

    constraint massAnalysis3 : MassAnalysis3 {
        in totalMass : MassValue;
        in componentMasses : MassValue [0..*];

        = totalMass == sum(componentMasses);
    }

    part vehicle3 :> vehicle {
        assert constraint massAnalysis3 {
            in totalMass = mass;
            in componentMasses = (engine.mass, frontAxleAssembly.mass, rearAxleAssembly.mass);
        }
    }

    part vehicle4 :> vehicle {
        assert constraint {
            = mass == engine.mass + frontAxleAssembly.mass + rearAxleAssembly.mass;
        }
    }

    constraint massLimitation {
        mass : MassValue;
        massLimit : MassValue;
        = mass < massLimit;
    }
    assert not constraint massLimitation {
        :>> mass = vehicle3.mass;
        :>> massLimit = vehicle4.mass;
    }
}
~~~
# EXPECTED
~~~
semantic.duplicate_name 'massLimitation'
semantic.ambiguous_member 'massLimitation'
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
semantic.unresolved_name 'mass'
semantic.unresolved_name 'massLimit'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'massLimitation'
semantic.ambiguous_member 'massLimitation'
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
semantic.unresolved_name 'mass'
semantic.unresolved_name 'massLimit'
~~~
# SMG
~~~
(model
  (namespace
    (package 'ConstraintTest'
      (membership_import private -> 'ISQ::MassValue'[unresolved])
      (membership_import private -> 'SI::kg'[unresolved])
      (membership_import private -> 'NumericalFunctions::sum'[unresolved])
      (constraint_def 'MassAnalysis'
        (attribute_usage composite 'totalMass' : 'MassValue'[unresolved])
        (attribute_usage composite 'componentMasses' : 'MassValue'[unresolved]
          (multiplicity_range [0..*]))
        (result_expr_membership))
      (part_def 'Component'
        (attribute_usage composite 'mass' : 'MassValue'[unresolved]))
      (part_usage 'vehicle' : 'ConstraintTest::Component'[part_def]
        (part_usage composite 'engine' : 'ConstraintTest::Component'[part_def])
        (part_usage composite 'frontAxleAssembly' : 'ConstraintTest::Component'[part_def])
        (part_usage composite 'rearAxleAssembly' : 'ConstraintTest::Component'[part_def]))
      (part_usage 'vehicle1a' :> 'ConstraintTest::vehicle'[part_usage]
        (assert_constraint_usage 'massAnalysis' : 'ConstraintTest::MassAnalysis'[constraint_def]
          (attribute_usage :>> 'ConstraintTest::MassAnalysis::totalMass'[attribute_usage])
          (attribute_usage :>> 'ConstraintTest::MassAnalysis::componentMasses'[attribute_usage]))
        (binding_connector_def
          (connector_end 'massAnalysis.totalMass')
          (connector_end 'mass'))
        (binding_connector_def
          (connector_end 'massAnalysis.componentMasses')
          (connector_end 'engine.mass'))
        (binding_connector_def
          (connector_end 'massAnalysis.componentMasses')
          (connector_end 'frontAxleAssembly.mass'))
        (binding_connector_def
          (connector_end 'massAnalysis.componentMasses')
          (connector_end 'rearAxleAssembly.mass')))
      (part_usage 'vehicle1b' :> 'ConstraintTest::vehicle'[part_usage]
        (assert_constraint_usage 'massAnalysis' : 'ConstraintTest::MassAnalysis'[constraint_def]
          (attribute_usage :>> 'ConstraintTest::MassAnalysis::totalMass'[attribute_usage]
            (feature_value (=)))
          (attribute_usage :>> 'ConstraintTest::MassAnalysis::componentMasses'[attribute_usage]
            (feature_value (=)))))
      (constraint_def 'MassAnalysis2'
        (reference_usage in reference 'totalMass' : 'MassValue'[unresolved])
        (reference_usage in reference 'componentMasses' : 'MassValue'[unresolved]
          (multiplicity_range [0..*]))
        (result_expr_membership))
      (part_usage 'vehicle2a' :> 'ConstraintTest::vehicle'[part_usage]
        (assert_constraint_usage 'massConstraint' : 'ConstraintTest::MassAnalysis2'[constraint_def])
        (binding_connector_def
          (connector_end 'massConstraint.totalMass')
          (connector_end 'mass'))
        (binding_connector_def
          (connector_end 'massConstraint.componentMasses')
          (connector_end 'engine.mass'))
        (binding_connector_def
          (connector_end 'massConstraint.componentMasses')
          (connector_end 'frontAxleAssembly.mass'))
        (binding_connector_def
          (connector_end 'massConstraint.componentMasses')
          (connector_end 'rearAxleAssembly.mass')))
      (part_usage 'vehicle2b' :> 'ConstraintTest::vehicle'[part_usage]
        (assert_constraint_usage 'massAnalysis2' : 'ConstraintTest::MassAnalysis2'[constraint_def]
          (reference_usage in reference 'totalMass'
            (feature_value (=)))
          (reference_usage in reference 'componentMasses'
            (feature_value (=)))))
      (constraint_def 'MassAnalysis3'
        (reference_usage in reference 'totalMass' : 'MassValue'[unresolved])
        (reference_usage in reference 'componentMasses' : 'MassValue'[unresolved]
          (multiplicity_range [0..*])))
      (constraint_usage 'massAnalysis3' : 'ConstraintTest::MassAnalysis3'[constraint_def]
        (reference_usage in reference 'totalMass' : 'MassValue'[unresolved])
        (reference_usage in reference 'componentMasses' : 'MassValue'[unresolved]
          (multiplicity_range [0..*]))
        (result_expr_membership))
      (part_usage 'vehicle3' :> 'ConstraintTest::vehicle'[part_usage]
        (assert_constraint_usage 'massAnalysis3'
          (reference_usage in reference 'totalMass'
            (feature_value (=)))
          (reference_usage in reference 'componentMasses'
            (feature_value (=)))))
      (part_usage 'vehicle4' :> 'ConstraintTest::vehicle'[part_usage]
        (assert_constraint_usage
          (result_expr_membership)))
      (constraint_usage 'massLimitation'
        (reference_usage reference 'mass' : 'MassValue'[unresolved])
        (reference_usage reference 'massLimit' : 'MassValue'[unresolved])
        (result_expr_membership))
      (assert_constraint_usage not 'massLimitation'
        (reference_usage reference :>> 'mass'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'massLimit'[unresolved]
          (feature_value (=)))))))
~~~
