# META
~~~ini
description=SysML Example (Analysis): Dynamics
type=file
~~~
# SOURCE
~~~sysml
package Dynamics {
	private import ScalarValues::Real;
	private import ISQ::*;
	
	// Function definitions
	
	calc def Power {
		in whlpwr : PowerValue; in Cd : Real; in Cf : Real; in tm : MassValue; in v : SpeedValue;
		return tp : PowerValue = whlpwr - Cd * v - Cf * tm * v;
	}
	
	calc def Acceleration { in dt : TimeValue; in tm : MassValue; in tp: PowerValue; 
		return a : AccelerationValue = tp * dt * tp;
	}
	
	calc def Velocity { in dt : TimeValue; in v0 : SpeedValue; in a : AccelerationValue;
		return v : SpeedValue = v0 + a * dt;
	}
 	
	calc def Position { in dt : TimeValue; in x0 : LengthValue; in v : SpeedValue; 
		return x : LengthValue = x0 + v * dt;
	}

	// Analysis action def
	
	action def StraightLineVehicleDynamics {
		
		in attribute dt : TimeValue;
		in attribute whlpwr : PowerValue;
		in attribute Cd : Real;
		in attribute Cf: Real;
		in attribute tm : MassValue;
		in attribute v_in : SpeedValue;
		in attribute x_in : LengthValue;
		
		out attribute a_out : AccelerationValue;
		out attribute v_out : SpeedValue;
		out attribute x_out : LengthValue;
			
		assert constraint {
			attribute tp : PowerValue;
			
			tp == Power(whlpwr, Cd, Cf, tm, v_in) &
			a_out == Acceleration(dt, tm, tp) &
			v_out == Velocity(dt, v_in, a_out) &
			x_out == Position(dt, x_in, v_in)
		}
	}
	

	// Analysis actions
	
	action dyn1 : StraightLineVehicleDynamics {
        in attribute dt : TimeValue;
        in attribute whlpwr : PowerValue;
        in attribute Cd : Real;
        in attribute Cf: Real;
        in attribute tm : MassValue;
        in attribute v_in : SpeedValue;
        in attribute x_in : LengthValue;

		attribute tp : PowerValue = Power(whlpwr, Cd, Cf, tm, v_in);
		
		out attribute :>> a_out : AccelerationValue = Acceleration(dt, tm, tp);
		out attribute :>> v_out : SpeedValue = Velocity(dt, v_in, a_out);
		out attribute :>> x_out : LengthValue = Position(dt, x_in, v_in);
	}	
	
	action dyn2 : StraightLineVehicleDynamics {
		calc acc : Acceleration {
			in dt = dyn2::dt;
			in tm = dyn2::tm;
			in tp = Power(whlpwr, Cd, Cf, tm, v_in);
		}
		bind a_out = acc.a;
		
		calc vel : Velocity {
			in dt = dyn2::dt;
			in v0 = dyn2::v_in; 
			in a = acc.a;
		}
		bind v_out = vel.v;
		
		calc pos : Position {
			in dt = dyn2::dt;
			in x0 = dyn2::x_in;
			in v0 = vel.v;
		}
		bind x_out = pos.x;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "dynamics.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 2) (end 7 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 54) (end 7 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 73) (end 7 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 2) (end 8 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 25) (end 11 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 44) (end 11 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 63) (end 11 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 2) (end 12 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 21) (end 15 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 40) (end 15 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 60) (end 15 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 2) (end 16 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 21) (end 19 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 40) (end 19 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 61) (end 19 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 2) (end 20 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 2) (end 27 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 2) (end 28 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 2) (end 31 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 2) (end 32 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 2) (end 33 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 2) (end 35 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 2) (end 36 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 2) (end 37 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 8) (end 53 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 8) (end 54 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 57 8) (end 57 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 58 8) (end 58 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 8) (end 59 40))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 63 2) (end 63 76))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 63 2) (end 63 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 7) (end 74 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 15) (end 74 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 81 7) (end 81 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 81 15) (end 81 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 88 7) (end 88 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 88 15) (end 88 20))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "51133ca8e2d0a1de875db8df0fa2b7ad14c5816a79dc97ff7fab993fece071ef") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Dynamics"))) (kind "package") (name "Dynamics") (declared-name "Dynamics"))
    (element (id (node (document "d0") (qualified-name "Dynamics::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Dynamics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Acceleration"))) (kind "calc def") (name "Acceleration") (declared-name "Acceleration") (parent (node (document "d0") (qualified-name "Dynamics"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Acceleration::a"))) (kind "return parameter") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "Dynamics::Acceleration"))) (authored (relationships (typing (reference "AccelerationValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Acceleration::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (parent (node (document "d0") (qualified-name "Dynamics::Acceleration"))) (authored (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Acceleration::tm"))) (kind "in out parameter") (name "tm") (declared-name "tm") (parent (node (document "d0") (qualified-name "Dynamics::Acceleration"))) (authored (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Acceleration::tp"))) (kind "in out parameter") (name "tp") (declared-name "tp") (parent (node (document "d0") (qualified-name "Dynamics::Acceleration"))) (authored (relationships (typing (reference "PowerValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Position"))) (kind "calc def") (name "Position") (declared-name "Position") (parent (node (document "d0") (qualified-name "Dynamics"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Position::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (parent (node (document "d0") (qualified-name "Dynamics::Position"))) (authored (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Position::v"))) (kind "in out parameter") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "Dynamics::Position"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Position::x"))) (kind "return parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "Dynamics::Position"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Position::x0"))) (kind "in out parameter") (name "x0") (declared-name "x0") (parent (node (document "d0") (qualified-name "Dynamics::Position"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Power"))) (kind "calc def") (name "Power") (declared-name "Power") (parent (node (document "d0") (qualified-name "Dynamics"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Power::Cd"))) (kind "in out parameter") (name "Cd") (declared-name "Cd") (parent (node (document "d0") (qualified-name "Dynamics::Power"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Power::Cf"))) (kind "in out parameter") (name "Cf") (declared-name "Cf") (parent (node (document "d0") (qualified-name "Dynamics::Power"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Power::tm"))) (kind "in out parameter") (name "tm") (declared-name "tm") (parent (node (document "d0") (qualified-name "Dynamics::Power"))) (authored (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Power::tp"))) (kind "return parameter") (name "tp") (declared-name "tp") (parent (node (document "d0") (qualified-name "Dynamics::Power"))) (authored (relationships (typing (reference "PowerValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Power::v"))) (kind "in out parameter") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "Dynamics::Power"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Power::whlpwr"))) (kind "in out parameter") (name "whlpwr") (declared-name "whlpwr") (parent (node (document "d0") (qualified-name "Dynamics::Power"))) (authored (relationships (typing (reference "PowerValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "Dynamics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (kind "action def") (name "StraightLineVehicleDynamics") (declared-name "StraightLineVehicleDynamics") (parent (node (document "d0") (qualified-name "Dynamics"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cd"))) (kind "in out parameter") (name "Cd") (declared-name "Cd") (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cf"))) (kind "in out parameter") (name "Cf") (declared-name "Cf") (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::a_out"))) (kind "in out parameter") (name "a_out") (declared-name "a_out") (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "AccelerationValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::tm"))) (kind "in out parameter") (name "tm") (declared-name "tm") (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_in"))) (kind "in out parameter") (name "v_in") (declared-name "v_in") (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_out"))) (kind "in out parameter") (name "v_out") (declared-name "v_out") (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::whlpwr"))) (kind "in out parameter") (name "whlpwr") (declared-name "whlpwr") (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "PowerValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_in"))) (kind "in out parameter") (name "x_in") (declared-name "x_in") (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_out"))) (kind "in out parameter") (name "x_out") (declared-name "x_out") (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Velocity"))) (kind "calc def") (name "Velocity") (declared-name "Velocity") (parent (node (document "d0") (qualified-name "Dynamics"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Velocity::a"))) (kind "in out parameter") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "Dynamics::Velocity"))) (authored (relationships (typing (reference "AccelerationValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Velocity::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (parent (node (document "d0") (qualified-name "Dynamics::Velocity"))) (authored (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Velocity::v"))) (kind "return parameter") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "Dynamics::Velocity"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Velocity::v0"))) (kind "in out parameter") (name "v0") (declared-name "v0") (parent (node (document "d0") (qualified-name "Dynamics::Velocity"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1"))) (kind "action") (name "dyn1") (declared-name "dyn1") (parent (node (document "d0") (qualified-name "Dynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "StraightLineVehicleDynamics")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::Cd"))) (kind "in out parameter") (name "Cd") (declared-name "Cd") (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::Cf"))) (kind "in out parameter") (name "Cf") (declared-name "Cf") (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))) (authored (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::tm"))) (kind "in out parameter") (name "tm") (declared-name "tm") (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))) (authored (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::tp : PowerValue = Power(whlpwr, Cd, Cf, tm, v_in)"))) (kind "action body decl") (name "tp : PowerValue = Power(whlpwr, Cd, Cf, tm, v_in)") (declared-name "tp : PowerValue = Power(whlpwr, Cd, Cf, tm, v_in)") (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::v_in"))) (kind "in out parameter") (name "v_in") (declared-name "v_in") (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::whlpwr"))) (kind "in out parameter") (name "whlpwr") (declared-name "whlpwr") (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))) (authored (relationships (typing (reference "PowerValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::x_in"))) (kind "in out parameter") (name "x_in") (declared-name "x_in") (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind "action") (name "dyn2") (declared-name "dyn2") (parent (node (document "d0") (qualified-name "Dynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "StraightLineVehicleDynamics")))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn2::acc : Acceleration"))) (kind "action body decl") (name "acc : Acceleration") (declared-name "acc : Acceleration") (parent (node (document "d0") (qualified-name "Dynamics::dyn2"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn2::pos : Position"))) (kind "action body decl") (name "pos : Position") (declared-name "pos : Position") (parent (node (document "d0") (qualified-name "Dynamics::dyn2"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn2::vel : Velocity"))) (kind "action body decl") (name "vel : Velocity") (declared-name "vel : Velocity") (parent (node (document "d0") (qualified-name "Dynamics::dyn2"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Acceleration::a"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Acceleration::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Acceleration::tm"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Acceleration::tp"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Position::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Position::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Position::x"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Position::x0"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Power::Cd"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Power::Cf"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Power::tm"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Power::tp"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Power::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Power::whlpwr"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cd"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cf"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::a_out"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::tm"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_in"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_out"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::whlpwr"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_in"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_out"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Velocity::a"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Velocity::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Velocity::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Velocity::v0"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1"))) (kind featureTyping) (ordinal 0)) (authored-target "StraightLineVehicleDynamics") (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1::Cd"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1::Cf"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1::tm"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1::v_in"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1::whlpwr"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1::x_in"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind featureTyping) (ordinal 0)) (authored-target "StraightLineVehicleDynamics") (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind bindSource) (ordinal 0)) (authored-target "a_out") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind bindSource) (ordinal 1)) (authored-target "v_out") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind bindSource) (ordinal 2)) (authored-target "x_out") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind bindTarget) (ordinal 0)) (authored-target "acc::a") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind bindTarget) (ordinal 1)) (authored-target "vel::v") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind bindTarget) (ordinal 2)) (authored-target "pos::x") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::Power::Cd"))) (target (node (document "d0") (qualified-name "Dynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::Power::Cd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::Power::Cf"))) (target (node (document "d0") (qualified-name "Dynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::Power::Cf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cd"))) (target (node (document "d0") (qualified-name "Dynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cf"))) (target (node (document "d0") (qualified-name "Dynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::dyn1"))) (target (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::dyn1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::dyn1::Cd"))) (target (node (document "d0") (qualified-name "Dynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::dyn1::Cd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::dyn1::Cf"))) (target (node (document "d0") (qualified-name "Dynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::dyn1::Cf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (target (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 16) (end 2 19)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Dynamics::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 2 16) (end 2 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 74 7) (end 74 12)) (probe (position 74 7))
      (reference
        (source (document "d0") (qualified-name "Dynamics::dyn2"))
        (kind bindSource) (ordinal 0) (authored-target "a_out")
        (range (start 74 7) (end 74 12))
        (outcome (status unresolved))
      )
    )
    (query (range (start 74 15) (end 74 20)) (probe (position 74 15))
      (reference
        (source (document "d0") (qualified-name "Dynamics::dyn2"))
        (kind bindTarget) (ordinal 0) (authored-target "acc::a")
        (range (start 74 15) (end 74 20))
        (outcome (status unresolved))
      )
    )
    (query (range (start 81 7) (end 81 12)) (probe (position 81 7))
      (reference
        (source (document "d0") (qualified-name "Dynamics::dyn2"))
        (kind bindSource) (ordinal 1) (authored-target "v_out")
        (range (start 81 7) (end 81 12))
        (outcome (status unresolved))
      )
    )
    (query (range (start 81 15) (end 81 20)) (probe (position 81 15))
      (reference
        (source (document "d0") (qualified-name "Dynamics::dyn2"))
        (kind bindTarget) (ordinal 1) (authored-target "vel::v")
        (range (start 81 15) (end 81 20))
        (outcome (status unresolved))
      )
    )
    (query (range (start 88 7) (end 88 12)) (probe (position 88 7))
      (reference
        (source (document "d0") (qualified-name "Dynamics::dyn2"))
        (kind bindSource) (ordinal 2) (authored-target "x_out")
        (range (start 88 7) (end 88 12))
        (outcome (status unresolved))
      )
    )
    (query (range (start 88 15) (end 88 20)) (probe (position 88 15))
      (reference
        (source (document "d0") (qualified-name "Dynamics::dyn2"))
        (kind bindTarget) (ordinal 2) (authored-target "pos::x")
        (range (start 88 15) (end 88 20))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Dynamics::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
