# META
~~~ini
description=SysML Training 20 (Assignment Actions): Assignment Example
type=file
~~~
# SOURCE
~~~sysml
package 'For Loop Example' {
	private import SequenceFunctions::*;
	
    action def StraightLineDynamics {
        in power : ISQ::PowerValue;
        in mass : ISQ::MassValue;
        in delta_t : ISQ::TimeValue;
        in x_in : ISQ::LengthValue;
        in v_in : ISQ::SpeedValue;
        out x_out : ISQ::LengthValue;
        out v_out : ISQ::SpeedValue;
    }
	    
	action def ComputeMotion {
		in attribute powerProfile :> ISQ::power[*];
		in attribute vehicleMass :> ISQ::mass;
		in attribute initialPosition :> ISQ::length;
		in attribute initialSpeed :> ISQ::speed;
		in attribute deltaT :> ISQ::time;
		out attribute positions :> ISQ::length[*] := ( );
		
		private attribute position := initialPosition;
		private attribute speed := initialSpeed;
		
		for vehiclePower in powerProfile {
			perform action dynamics : StraightLineDynamics {
				in power = vehiclePower;
				in mass = vehicleMass;
				in delta_t = deltaT;
				in x_in = position;
				in v_in = speed;
				out x_out;
				out v_out;
			}
			then assign position := dynamics.x_out;
			then assign speed := dynamics.v_out;
			then assign positions := positions->including(position);
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "20_assignment_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 8) (end 4 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 8) (end 5 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 8) (end 6 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 8) (end 7 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 8) (end 8 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 8) (end 9 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 8) (end 10 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 2) (end 14 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 2) (end 16 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 2) (end 17 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 2) (end 18 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 2) (end 19 51))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'For Loop Example' {
    private import SequenceFunctions::*;

    action def StraightLineDynamics {
        in power : ISQ::PowerValue;
        in mass : ISQ::MassValue;
        in delta_t : ISQ::TimeValue;
        in x_in : ISQ::LengthValue;
        in v_in : ISQ::SpeedValue;
        out x_out : ISQ::LengthValue;
        out v_out : ISQ::SpeedValue;
    }

    action def ComputeMotion {
        in attribute powerProfile :> ISQ::power[*];
        in attribute vehicleMass :> ISQ::mass;
        in attribute initialPosition :> ISQ::length;
        in attribute initialSpeed :> ISQ::speed;
        in attribute deltaT :> ISQ::time;
        out attribute positions :> ISQ::length[*] := ( );

        private attribute position := initialPosition;
        private attribute speed := initialSpeed;

        for vehiclePower in powerProfile {
            perform action dynamics : StraightLineDynamics {
                in power = vehiclePower;
                in mass = vehicleMass;
                in delta_t = deltaT;
                in x_in = position;
                in v_in = speed;
                out x_out;
                out v_out;
            }
            then assign position := dynamics.x_out;
            then assign speed := dynamics.v_out;
            then assign positions := positions->including(position);
        }
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4eae89d3b7e4188908457480010b421fcd53a8d8d27310da1e90a6b6e8dc811c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "For Loop Example"))) (kind "package") (name "For Loop Example") (declared-name "For Loop Example") (range (start (line 0) (character 0)) (end (line 0) (character 1166))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 37))) (parent (node (document "d0") (qualified-name "For Loop Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 33))))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (kind "action def") (name "ComputeMotion") (declared-name "ComputeMotion") (range (start (line 13) (character 1)) (end (line 13) (character 790))) (parent (node (document "d0") (qualified-name "For Loop Example"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::deltaT"))) (kind "in out parameter") (name "deltaT") (declared-name "deltaT") (range (start (line 18) (character 2)) (end (line 18) (character 35))) (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (authored (relationships (typing (reference "ISQ::time") (range none)))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower"))) (kind "for loop") (name "vehiclePower") (declared-name "vehiclePower") (range (start (line 24) (character 2)) (end (line 24) (character 396))) (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 34) (character 3)) (end (line 34) (character 42))) (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::_assign#assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 35) (character 3)) (end (line 35) (character 39))) (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::_assign#assign2"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 36) (character 3)) (end (line 36) (character 59))) (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::dynamics"))) (kind "perform") (name "dynamics") (declared-name "dynamics") (range (start (line 25) (character 3)) (end (line 25) (character 212))) (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower"))) (authored (relationships (typing (reference "StraightLineDynamics") (range none)))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::initialPosition"))) (kind "in out parameter") (name "initialPosition") (declared-name "initialPosition") (range (start (line 16) (character 2)) (end (line 16) (character 46))) (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (authored (relationships (typing (reference "ISQ::length") (range none)))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::initialSpeed"))) (kind "in out parameter") (name "initialSpeed") (declared-name "initialSpeed") (range (start (line 17) (character 2)) (end (line 17) (character 42))) (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (authored (relationships (typing (reference "ISQ::speed") (range none)))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::position := initialPosition"))) (kind "action body decl") (name "position := initialPosition") (declared-name "position := initialPosition") (range (start (line 21) (character 2)) (end (line 21) (character 48))) (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::positions"))) (kind "in out parameter") (name "positions") (declared-name "positions") (range (start (line 19) (character 2)) (end (line 19) (character 51))) (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (authored (relationships (typing (reference "positions :> ISQ::length[*] := ( )") (range none)))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::powerProfile"))) (kind "in out parameter") (name "powerProfile") (declared-name "powerProfile") (range (start (line 14) (character 2)) (end (line 14) (character 45))) (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (authored (relationships (typing (reference "powerProfile :> ISQ::power[*]") (range none)))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::speed := initialSpeed"))) (kind "action body decl") (name "speed := initialSpeed") (declared-name "speed := initialSpeed") (range (start (line 22) (character 2)) (end (line 22) (character 42))) (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::vehicleMass"))) (kind "in out parameter") (name "vehicleMass") (declared-name "vehicleMass") (range (start (line 15) (character 2)) (end (line 15) (character 40))) (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (authored (relationships (typing (reference "ISQ::mass") (range none)))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (kind "action def") (name "StraightLineDynamics") (declared-name "StraightLineDynamics") (range (start (line 3) (character 4)) (end (line 3) (character 296))) (parent (node (document "d0") (qualified-name "For Loop Example"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::delta_t"))) (kind "in out parameter") (name "delta_t") (declared-name "delta_t") (range (start (line 6) (character 8)) (end (line 6) (character 36))) (parent (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (authored (relationships (typing (reference "ISQ::TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::mass"))) (kind "in out parameter") (name "mass") (declared-name "mass") (range (start (line 5) (character 8)) (end (line 5) (character 33))) (parent (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (authored (relationships (typing (reference "ISQ::MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::power"))) (kind "in out parameter") (name "power") (declared-name "power") (range (start (line 4) (character 8)) (end (line 4) (character 35))) (parent (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (authored (relationships (typing (reference "ISQ::PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::v_in"))) (kind "in out parameter") (name "v_in") (declared-name "v_in") (range (start (line 8) (character 8)) (end (line 8) (character 34))) (parent (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (authored (relationships (typing (reference "ISQ::SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::v_out"))) (kind "in out parameter") (name "v_out") (declared-name "v_out") (range (start (line 10) (character 8)) (end (line 10) (character 36))) (parent (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (authored (relationships (typing (reference "ISQ::SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::x_in"))) (kind "in out parameter") (name "x_in") (declared-name "x_in") (range (start (line 7) (character 8)) (end (line 7) (character 35))) (parent (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (authored (relationships (typing (reference "ISQ::LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::x_out"))) (kind "in out parameter") (name "x_out") (declared-name "x_out") (range (start (line 9) (character 8)) (end (line 9) (character 37))) (parent (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (authored (relationships (typing (reference "ISQ::LengthValue") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SequenceFunctions::*") (range (start (line 1) (character 16)) (end (line 1) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::deltaT"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::time") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::dynamics"))) (kind featureTyping) (ordinal 0)) (authored-target "StraightLineDynamics") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::initialPosition"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::length") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::initialSpeed"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::speed") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::positions"))) (kind featureTyping) (ordinal 0)) (authored-target "positions :> ISQ::length[*] := ( )") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::powerProfile"))) (kind featureTyping) (ordinal 0)) (authored-target "powerProfile :> ISQ::power[*]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::vehicleMass"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::delta_t"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::power"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::v_in"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::v_out"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::x_in"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::x_out"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::LengthValue") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::dynamics"))) (target (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::dynamics"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
