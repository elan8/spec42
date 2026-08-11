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
    (element (id (node (document "d0") (qualified-name "For Loop Example"))) (kind "package") (name "For Loop Example") (declared-name "For Loop Example"))
    (element (id (node (document "d0") (qualified-name "For Loop Example::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "For Loop Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (kind "action def") (name "ComputeMotion") (declared-name "ComputeMotion") (parent (node (document "d0") (qualified-name "For Loop Example"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::deltaT"))) (kind "in out parameter") (name "deltaT") (declared-name "deltaT") (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (authored (relationships (typing (reference "ISQ::time")))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower"))) (kind "for loop") (name "vehiclePower") (declared-name "vehiclePower") (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::_assign#assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::_assign#assign2"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::dynamics"))) (kind "perform") (name "dynamics") (declared-name "dynamics") (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower"))) (authored (relationships (typing (reference "StraightLineDynamics")))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::initialPosition"))) (kind "in out parameter") (name "initialPosition") (declared-name "initialPosition") (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (authored (relationships (typing (reference "ISQ::length")))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::initialSpeed"))) (kind "in out parameter") (name "initialSpeed") (declared-name "initialSpeed") (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (authored (relationships (typing (reference "ISQ::speed")))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::position := initialPosition"))) (kind "action body decl") (name "position := initialPosition") (declared-name "position := initialPosition") (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::positions"))) (kind "in out parameter") (name "positions") (declared-name "positions") (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (authored (relationships (typing (reference "positions :> ISQ::length[*] := ( )")))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::powerProfile"))) (kind "in out parameter") (name "powerProfile") (declared-name "powerProfile") (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (authored (relationships (typing (reference "powerProfile :> ISQ::power[*]")))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::speed := initialSpeed"))) (kind "action body decl") (name "speed := initialSpeed") (declared-name "speed := initialSpeed") (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::vehicleMass"))) (kind "in out parameter") (name "vehicleMass") (declared-name "vehicleMass") (parent (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (authored (relationships (typing (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (kind "action def") (name "StraightLineDynamics") (declared-name "StraightLineDynamics") (parent (node (document "d0") (qualified-name "For Loop Example"))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::delta_t"))) (kind "in out parameter") (name "delta_t") (declared-name "delta_t") (parent (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (authored (relationships (typing (reference "ISQ::TimeValue")))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::mass"))) (kind "in out parameter") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (authored (relationships (typing (reference "ISQ::MassValue")))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::power"))) (kind "in out parameter") (name "power") (declared-name "power") (parent (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (authored (relationships (typing (reference "ISQ::PowerValue")))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::v_in"))) (kind "in out parameter") (name "v_in") (declared-name "v_in") (parent (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (authored (relationships (typing (reference "ISQ::SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::v_out"))) (kind "in out parameter") (name "v_out") (declared-name "v_out") (parent (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (authored (relationships (typing (reference "ISQ::SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::x_in"))) (kind "in out parameter") (name "x_in") (declared-name "x_in") (parent (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (authored (relationships (typing (reference "ISQ::LengthValue")))))
    (element (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::x_out"))) (kind "in out parameter") (name "x_out") (declared-name "x_out") (parent (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (authored (relationships (typing (reference "ISQ::LengthValue")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SequenceFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::deltaT"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::time") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::dynamics"))) (kind featureTyping) (ordinal 0)) (authored-target "StraightLineDynamics") (outcome (status resolved) (target (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::initialPosition"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::length") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::initialSpeed"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::speed") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::positions"))) (kind featureTyping) (ordinal 0)) (authored-target "positions :> ISQ::length[*] := ( )") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::powerProfile"))) (kind featureTyping) (ordinal 0)) (authored-target "powerProfile :> ISQ::power[*]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::vehicleMass"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::delta_t"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::power"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::v_in"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::v_out"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::x_in"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::x_out"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::LengthValue") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::dynamics"))) (target (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::dynamics"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 33)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "For Loop Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "SequenceFunctions::*")
        (range (start 1 16) (end 1 33))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
