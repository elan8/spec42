# META
~~~ini
description=SysML Validation (10-Analysis and Trades): 10d-Dynamics Analysis
type=file
~~~
# SOURCE
~~~sysml
package '10d-Dynamics Analysis' {
	private import ISQ::*;
	
	package VehicleModel {
	
		part def Vehicle {
			attribute mass :> ISQ::mass;
		}
	
	}
	
	package DynamicsModel {
	    
	    calc def Acceleration {
	    	in p : PowerValue;
	    	in m : MassValue;
	    	in v : SpeedValue;
	    	return : AccelerationValue = p / (m * v);
	    }
	    
	    calc def Velocity {
	    	in v0 : SpeedValue; 
	    	in a : AccelerationValue; 
	    	in dt : TimeValue;
	    	return : SpeedValue = v0 + a * dt;
	    }
	    
	    calc def Position {
	    	in x0 : LengthValue;
	    	in v : SpeedValue; 
	    	in dt : TimeValue;
	    	return : LengthValue = x0 + v * dt;
	    }
	    
	    action def StraightLineDynamics {
	        in power : PowerValue;
	        in mass : MassValue;
	        in delta_t : TimeValue;
	        in x_in : LengthValue;
	        in v_in : SpeedValue;
	        out x_out : LengthValue = Position(x_in, v_in, delta_t);
	        out v_out : SpeedValue = Velocity(v_in, a_out, delta_t);
	        out a_out : AccelerationValue = Acceleration(power, mass, v_in);
	    }
	}
	
	package AnalysisModel {
		private import VehicleModel::*;
		private import DynamicsModel::*;
		private import SampledFunctions::*;
		private import ScalarValues::Natural;
		private import SequenceFunctions::*;
		
		analysis def DynamicsAnalysis {
			subject vehicle : Vehicle;
			in attribute powerProfile :> ISQ::power[*];
			in attribute initialPosition :> ISQ::length;
			in attribute initialSpeed :> ISQ::speed;
			in attribute deltaT :> ISQ::time;
			return attribute accelerationProfile :> ISQ::acceleration[*] := ();
			
			private attribute position := initialPosition;
			private attribute speed := initialSpeed;
			
			for i in 1..powerProfile->size()-1 {
				perform action dynamics : StraightLineDynamics {
					in power = powerProfile#(i);
					in mass = vehicle.mass;
					in delta_t = deltaT;
					in x_in = position;
					in v_in = speed;
				}
				then assign position := dynamics.x_out;
				then assign speed := dynamics.v_out;
				then assign accelerationProfile := accelerationProfile->including(dynamics.a_out);
			}
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/10d_dynamics_analysis.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 6 21) (end 6 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 5) (end 18 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 20 5) (end 25 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 27 5) (end 32 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 35 9) (end 35 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 36 9) (end 36 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 37 9) (end 37 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 38 9) (end 38 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 39 9) (end 39 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 40 9) (end 40 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 41 9) (end 41 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 42 9) (end 42 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 49 17) (end 49 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 50 17) (end 50 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 51 17) (end 51 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 54 3) (end 54 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 55 32) (end 55 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 56 35) (end 56 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 57 32) (end 57 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 58 26) (end 58 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 59 3) (end 59 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 64 3) (end 75 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:b8a004d4f36b6d521f47dcd9903ed6d85e2078c8499ee37938a622a8002aa61b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "VehicleModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "DynamicsModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SampledFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Natural") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SequenceFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::deltaT"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::time"))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialPosition"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::length"))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialSpeed"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::speed"))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::position"))) (kind attribute) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::powerProfile"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::power"))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::speed"))) (kind attribute) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::VehicleModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VehicleModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::VehicleModel")))))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "DynamicsModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel")))))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SampledFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SequenceFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::deltaT"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::time")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialPosition"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::length")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialSpeed"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::speed")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::powerProfile"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::power")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
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
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 1 16) (end 1 22)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 47 17) (end 47 32)) (probe (position 47 17))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "VehicleModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::VehicleModel")))))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 48 17) (end 48 33)) (probe (position 48 17))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "DynamicsModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel")))))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 49 17) (end 49 36)) (probe (position 49 17))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "SampledFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 51 17) (end 51 37)) (probe (position 51 17))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0) (authored-target "SequenceFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 50 17) (end 50 38)) (probe (position 50 17))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 58 26) (end 58 35)) (probe (position 58 26))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::deltaT"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::time")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 56 35) (end 56 46)) (probe (position 56 35))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialPosition"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::length")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 57 32) (end 57 42)) (probe (position 57 32))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialSpeed"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::speed")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 55 32) (end 55 42)) (probe (position 55 32))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::powerProfile"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::power")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 6 21) (end 6 30)) (probe (position 6 21))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
)
~~~
