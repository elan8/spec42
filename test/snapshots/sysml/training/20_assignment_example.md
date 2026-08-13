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
  (document "memory://snapshot/20_assignment_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 19) (end 4 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 18) (end 5 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 21) (end 6 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 18) (end 7 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 18) (end 8 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 20) (end 9 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 20) (end 10 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 31) (end 14 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 30) (end 15 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 34) (end 16 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 31) (end 17 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 25) (end 18 34))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 19 2) (end 21 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 21 2) (end 21 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 22 2) (end 22 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 24 2) (end 37 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:b957cd70deb6bd168fa163de0ddf0ad5f384d7a9f75acd17822ae1ee819984ec") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SequenceFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::deltaT"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::time") (direction in))))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::initialPosition"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::length") (direction in))))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::initialSpeed"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::speed") (direction in))))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::powerProfile"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::power") (direction in))))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::vehicleMass"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::mass") (direction in))))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::delta_t"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::mass"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::power"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::PowerValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::v_in"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::SpeedValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::v_out"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::SpeedValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::x_in"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::LengthValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::x_out"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::LengthValue") (direction out))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SequenceFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::deltaT"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::time")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::initialPosition"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::initialSpeed"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::speed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::powerProfile"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::power")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::vehicleMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::delta_t"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::power"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::v_in"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::v_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::x_in"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::x_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::LengthValue")
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
  (query (document "memory://snapshot/20_assignment_example.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "SequenceFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/20_assignment_example.md") (range (start 18 25) (end 18 34)) (probe (position 18 25))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::deltaT"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::time")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/20_assignment_example.md") (range (start 16 34) (end 16 45)) (probe (position 16 34))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::initialPosition"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::length")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/20_assignment_example.md") (range (start 17 31) (end 17 41)) (probe (position 17 31))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::initialSpeed"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::speed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/20_assignment_example.md") (range (start 14 31) (end 14 41)) (probe (position 14 31))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::powerProfile"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::power")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/20_assignment_example.md") (range (start 15 30) (end 15 39)) (probe (position 15 30))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::ComputeMotion::vehicleMass"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/20_assignment_example.md") (range (start 6 21) (end 6 35)) (probe (position 6 21))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::delta_t"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/20_assignment_example.md") (range (start 5 18) (end 5 32)) (probe (position 5 18))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::mass"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/20_assignment_example.md") (range (start 4 19) (end 4 34)) (probe (position 4 19))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::power"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::PowerValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/20_assignment_example.md") (range (start 8 18) (end 8 33)) (probe (position 8 18))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::v_in"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/20_assignment_example.md") (range (start 10 20) (end 10 35)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::v_out"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/20_assignment_example.md") (range (start 7 18) (end 7 34)) (probe (position 7 18))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::x_in"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/20_assignment_example.md") (range (start 9 20) (end 9 36)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/20_assignment_example.md") (qualified-name "For Loop Example::StraightLineDynamics::x_out"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::LengthValue")
      (outcome (status unresolved)))
  )
)
~~~
