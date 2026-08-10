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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ConstraintTest"))) (name "ConstraintTest") (declared-name "ConstraintTest")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "ConstraintTest::Component"))) (name "Component") (declared-name "Component") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ConstraintTest::Component::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ConstraintTest::Component")))))
          )
        )
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis"))) (name "MassAnalysis") (declared-name "MassAnalysis") (declared (own-expression (expression (kind "featureReference") (reference "attribute")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis2"))) (name "MassAnalysis2") (declared-name "MassAnalysis2") (declared (own-expression (expression (kind "binary") (operator "==") (children (expression (kind "featureReference") (reference "totalMass")) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "sum"))) (arguments (argument (expression (kind "featureReference") (reference "componentMasses"))))))))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis3"))) (name "MassAnalysis3") (declared-name "MassAnalysis3"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ConstraintTest::MassValue"))) (name "MassValue") (declared-name "MassValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ConstraintTest::kg"))) (name "kg") (declared-name "kg"))
        (element (kind "constraint") (id (node (document "d0") (qualified-name "ConstraintTest::massAnalysis3"))) (name "massAnalysis3") (declared-name "massAnalysis3") (declared (own-expression (expression (kind "binary") (operator "==") (children (expression (kind "featureReference") (reference "totalMass")) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "sum"))) (arguments (argument (expression (kind "featureReference") (reference "componentMasses"))))))))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference")) (analysis (status "unresolved"))))
        (element (kind "constraint") (id (node (document "d0") (qualified-name "ConstraintTest::massLimitation"))) (name "massLimitation") (declared-name "massLimitation") (declared (own-expression (expression (kind "featureReference") (reference "mass")))) (evaluation (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete"))))
        (element (kind "import") (id (node (document "d0") (qualified-name "ConstraintTest::sum"))) (name "sum") (declared-name "sum"))
        (element (kind "part") (id (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "ConstraintTest::vehicle::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ConstraintTest::Component")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ConstraintTest::Component")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ConstraintTest::Component")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (name "vehicle1a") (declared-name "vehicle1a") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "ConstraintTest::vehicle1b"))) (name "vehicle1b") (declared-name "vehicle1b") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (name "vehicle2a") (declared-name "vehicle2a") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "ConstraintTest::vehicle2b"))) (name "vehicle2b") (declared-name "vehicle2b") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "ConstraintTest::vehicle3"))) (name "vehicle3") (declared-name "vehicle3") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "ConstraintTest::vehicle4"))) (name "vehicle4") (declared-name "vehicle4") (declared (properties (ordered false))))
      )
    )
  )
  (relationships
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (to (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "ConstraintTest::vehicle1b"))) (to (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (to (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "ConstraintTest::vehicle2b"))) (to (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "ConstraintTest::vehicle3"))) (to (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "ConstraintTest::vehicle4"))) (to (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConstraintTest::massAnalysis3"))) (to (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis3"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (to (node (document "d0") (qualified-name "ConstraintTest::Component"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConstraintTest::vehicle::engine"))) (to (node (document "d0") (qualified-name "ConstraintTest::Component"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (to (node (document "d0") (qualified-name "ConstraintTest::Component"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (to (node (document "d0") (qualified-name "ConstraintTest::Component"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (bind (status pending-expression) (document "d0") (source-expression "massAnalysis::componentMasses") (target-expression "engine::mass") (container-prefix "ConstraintTest::vehicle1a"))
    (bind (status pending-expression) (document "d0") (source-expression "massAnalysis::componentMasses") (target-expression "frontAxleAssembly::mass") (container-prefix "ConstraintTest::vehicle1a"))
    (bind (status pending-expression) (document "d0") (source-expression "massAnalysis::componentMasses") (target-expression "rearAxleAssembly::mass") (container-prefix "ConstraintTest::vehicle1a"))
    (bind (status pending-expression) (document "d0") (source-expression "massAnalysis::totalMass") (target-expression "mass") (container-prefix "ConstraintTest::vehicle1a"))
    (bind (status pending-expression) (document "d0") (source-expression "massConstraint::componentMasses") (target-expression "engine::mass") (container-prefix "ConstraintTest::vehicle2a"))
    (bind (status pending-expression) (document "d0") (source-expression "massConstraint::componentMasses") (target-expression "frontAxleAssembly::mass") (container-prefix "ConstraintTest::vehicle2a"))
    (bind (status pending-expression) (document "d0") (source-expression "massConstraint::componentMasses") (target-expression "rearAxleAssembly::mass") (container-prefix "ConstraintTest::vehicle2a"))
    (bind (status pending-expression) (document "d0") (source-expression "massConstraint::totalMass") (target-expression "mass") (container-prefix "ConstraintTest::vehicle2a"))
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::Component"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::Component::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis"))) (status missing-prerequisite) (target "Constraints::ConstraintCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis2"))) (status missing-prerequisite) (target "Constraints::ConstraintCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::MassAnalysis3"))) (status missing-prerequisite) (target "Constraints::ConstraintCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::massAnalysis3"))) (status missing-prerequisite) (target "Constraints::constraintChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::massLimitation"))) (status missing-prerequisite) (target "Constraints::constraintChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::vehicle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::vehicle1a"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::vehicle1b"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::vehicle2a"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::vehicle2b"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::vehicle3"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::vehicle4"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::vehicle::engine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::vehicle::frontAxleAssembly"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConstraintTest::vehicle::rearAxleAssembly"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/constraint_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 2) (end 13 28))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 28 7) (end 28 29))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 29 7) (end 29 35))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 30 7) (end 30 35))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 31 7) (end 31 35))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 51 7) (end 51 31))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 52 7) (end 52 37))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 53 7) (end 53 37))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 54 7) (end 54 37))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 69 1) (end 69 152))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 77 2) (end 77 140))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 77 2) (end 77 140))
      )
    )
  )
)
~~~
