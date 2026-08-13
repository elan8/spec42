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
  (document "memory://snapshot/dynamics.md"
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
        (range (start 2 16) (end 2 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 14) (end 7 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 34) (end 7 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 48) (end 7 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 62) (end 7 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 80) (end 7 90))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 8 2) (end 8 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 33) (end 11 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 52) (end 11 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 70) (end 11 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 12 2) (end 12 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 29) (end 15 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 48) (end 15 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 67) (end 15 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 16 2) (end 16 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 29) (end 19 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 48) (end 19 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 68) (end 19 78))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 20 2) (end 20 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 20) (end 27 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 24) (end 28 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 20) (end 29 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 19) (end 30 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 20) (end 31 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 22) (end 32 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 22) (end 33 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 24) (end 35 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 24) (end 36 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 24) (end 37 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 39 2) (end 46 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 26) (end 53 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 30) (end 54 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 26) (end 55 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 25) (end 56 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 57 26) (end 57 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 58 28) (end 58 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 28) (end 59 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 61 2) (end 61 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 63 28) (end 63 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 64 28) (end 64 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 28) (end 65 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 69 2) (end 73 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 74 2) (end 74 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 76 2) (end 80 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 81 2) (end 81 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 83 2) (end 87 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 88 2) (end 88 21))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:707f73ee9387843460c50b7fd9a659c3d69080e46e567797e3fb26ab223d482c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Acceleration"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Acceleration::dt"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Acceleration::tm"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Acceleration::tp"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Position"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Position::dt"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Position::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Position::x0"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::Cd"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::Cf"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::tm"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::whlpwr"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cd"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cf"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::a_out"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AccelerationValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::dt"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::tm"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_in"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_out"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::whlpwr"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_in"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_out"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Velocity"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Velocity::a"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AccelerationValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Velocity::dt"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Velocity::v0"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StraightLineVehicleDynamics"))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AccelerationValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (anonymous (kind parameter) (ordinal 2))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::Cd"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::Cf"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::dt"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::tm"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::v_in"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::whlpwr"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::x_in"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn2"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StraightLineVehicleDynamics"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Acceleration::dt"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Acceleration::tm"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Acceleration::tp"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Position::dt"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Position::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Position::x0"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::Cd"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::Cf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::tm"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::whlpwr"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cd"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::a_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::dt"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::tm"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_in"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::whlpwr"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_in"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Velocity::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Velocity::dt"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Velocity::v0"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1"))) (kind featureTyping) (ordinal 0))
      (authored-target "StraightLineVehicleDynamics")
      (outcome (status resolved) (target (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (anonymous (kind parameter) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (anonymous (kind parameter) (ordinal 2))))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::Cd"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::Cf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::dt"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::tm"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::v_in"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::whlpwr"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::x_in"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn2"))) (kind featureTyping) (ordinal 0))
      (authored-target "StraightLineVehicleDynamics")
      (outcome (status resolved) (target (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1"))) (target (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn2"))) (target (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/dynamics.md") (range (start 2 16) (end 2 22)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 11 33) (end 11 42)) (probe (position 11 33))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Acceleration::dt"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 11 52) (end 11 61)) (probe (position 11 52))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Acceleration::tm"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 11 70) (end 11 80)) (probe (position 11 70))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Acceleration::tp"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 19 29) (end 19 38)) (probe (position 19 29))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Position::dt"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 19 68) (end 19 78)) (probe (position 19 68))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Position::v"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 19 48) (end 19 59)) (probe (position 19 48))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Position::x0"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 7 34) (end 7 38)) (probe (position 7 34))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::Cd"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 7 48) (end 7 52)) (probe (position 7 48))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::Cf"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 7 62) (end 7 71)) (probe (position 7 62))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::tm"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 7 80) (end 7 90)) (probe (position 7 80))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::v"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 7 14) (end 7 24)) (probe (position 7 14))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Power::whlpwr"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 29 20) (end 29 24)) (probe (position 29 20))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cd"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 30 19) (end 30 23)) (probe (position 30 19))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cf"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 35 24) (end 35 41)) (probe (position 35 24))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::a_out"))) (kind featureTyping) (ordinal 0) (authored-target "AccelerationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 27 20) (end 27 29)) (probe (position 27 20))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::dt"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 31 20) (end 31 29)) (probe (position 31 20))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::tm"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 32 22) (end 32 32)) (probe (position 32 22))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_in"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 36 24) (end 36 34)) (probe (position 36 24))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_out"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 28 24) (end 28 34)) (probe (position 28 24))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::whlpwr"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 33 22) (end 33 33)) (probe (position 33 22))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_in"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 37 24) (end 37 35)) (probe (position 37 24))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_out"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 15 67) (end 15 84)) (probe (position 15 67))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Velocity::a"))) (kind featureTyping) (ordinal 0) (authored-target "AccelerationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 15 29) (end 15 38)) (probe (position 15 29))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Velocity::dt"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 15 48) (end 15 58)) (probe (position 15 48))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::Velocity::v0"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 52 15) (end 52 42)) (probe (position 52 15))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1"))) (kind featureTyping) (ordinal 0) (authored-target "StraightLineVehicleDynamics")
      (outcome (status resolved) (target (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 63 28) (end 63 45)) (probe (position 63 28))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "AccelerationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 64 28) (end 64 38)) (probe (position 64 28))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (anonymous (kind parameter) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 65 28) (end 65 39)) (probe (position 65 28))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (anonymous (kind parameter) (ordinal 2))))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 55 26) (end 55 30)) (probe (position 55 26))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::Cd"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 56 25) (end 56 29)) (probe (position 56 25))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::Cf"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 53 26) (end 53 35)) (probe (position 53 26))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::dt"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 57 26) (end 57 35)) (probe (position 57 26))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::tm"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 58 28) (end 58 38)) (probe (position 58 28))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::v_in"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 54 30) (end 54 40)) (probe (position 54 30))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::whlpwr"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 59 28) (end 59 39)) (probe (position 59 28))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn1::x_in"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/dynamics.md") (range (start 68 15) (end 68 42)) (probe (position 68 15))
    (reference (id (source (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::dyn2"))) (kind featureTyping) (ordinal 0) (authored-target "StraightLineVehicleDynamics")
      (outcome (status resolved) (target (node (document "memory://snapshot/dynamics.md") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
  )
)
~~~
