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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 21) (end 6 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 13) (end 14 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 13) (end 15 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 13) (end 16 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 15) (end 17 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 14) (end 21 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 13) (end 22 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 14) (end 23 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 15) (end 24 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 14) (end 28 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 13) (end 29 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 14) (end 30 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 15) (end 31 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 20) (end 35 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 19) (end 36 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 22) (end 37 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 19) (end 38 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 19) (end 39 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 21) (end 40 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 21) (end 41 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 21) (end 42 38))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 32) (end 55 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 56 35) (end 56 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 57 32) (end 57 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:b8a004d4f36b6d521f47dcd9903ed6d85e2078c8499ee37938a622a8002aa61b") (contract-version "parser-owned-resolution-v1"))
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
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::position"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (expressionOperand (reference "initialPosition"))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::powerProfile"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::power"))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::speed"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (expressionOperand (reference "initialSpeed"))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AccelerationValue")) (expressionOperand (reference "p")) (expressionOperand (reference "m")) (expressionOperand (reference "v"))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::m"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::p"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (expressionOperand (reference "x0")) (expressionOperand (reference "v")) (expressionOperand (reference "dt"))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::dt"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::x0"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::a_out"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AccelerationValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::delta_t"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::mass"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::power"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::v_in"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::v_out"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::x_in"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::x_out"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue")) (expressionOperand (reference "v0")) (expressionOperand (reference "a")) (expressionOperand (reference "dt"))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::a"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AccelerationValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::dt"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::v0"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction in))))
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
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialPosition"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialSpeed"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::speed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::position"))) (kind expressionOperand) (ordinal 0))
      (authored-target "initialPosition")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialPosition")))))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::powerProfile"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::power")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::speed"))) (kind expressionOperand) (ordinal 0))
      (authored-target "initialSpeed")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialSpeed")))))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "p")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::p")))))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "m")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::m")))))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 2))
      (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::v")))))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "x0")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::x0")))))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::v")))))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 2))
      (authored-target "dt")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::dt")))))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::dt"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::x0"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::a_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::delta_t"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::power"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::v_in"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::v_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::x_in"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::x_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "v0")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::v0")))))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::a")))))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 2))
      (authored-target "dt")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::dt")))))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::dt"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::v0"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::position"))) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialPosition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::position"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::speed"))) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialSpeed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::speed"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::vehicle"))) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::p"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::m"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::v"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::x0"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::v"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::dt"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::v0"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::dt"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 2)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (value (kind non-constant)))
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
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 56 35) (end 56 46)) (probe (position 56 35))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialPosition"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::length")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 57 32) (end 57 42)) (probe (position 57 32))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialSpeed"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::speed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 61 33) (end 61 48)) (probe (position 61 33))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::position"))) (kind expressionOperand) (ordinal 0) (authored-target "initialPosition")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialPosition")))))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 55 32) (end 55 42)) (probe (position 55 32))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::powerProfile"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::power")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 62 30) (end 62 42)) (probe (position 62 30))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::speed"))) (kind expressionOperand) (ordinal 0) (authored-target "initialSpeed")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialSpeed")))))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 54 21) (end 54 28)) (probe (position 54 21))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle")))))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 17 15) (end 17 32)) (probe (position 17 15))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "AccelerationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 17 35) (end 17 36)) (probe (position 17 35))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "p")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::p")))))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 17 40) (end 17 41)) (probe (position 17 40))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "m")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::m")))))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 17 44) (end 17 45)) (probe (position 17 44))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 2) (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::v")))))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 15 13) (end 15 22)) (probe (position 15 13))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::m"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 14 13) (end 14 23)) (probe (position 14 13))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::p"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 16 13) (end 16 23)) (probe (position 16 13))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::v"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 31 15) (end 31 26)) (probe (position 31 15))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 31 29) (end 31 31)) (probe (position 31 29))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "x0")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::x0")))))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 31 34) (end 31 35)) (probe (position 31 34))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::v")))))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 31 38) (end 31 40)) (probe (position 31 38))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 2) (authored-target "dt")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::dt")))))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 30 14) (end 30 23)) (probe (position 30 14))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::dt"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 29 13) (end 29 23)) (probe (position 29 13))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::v"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 28 14) (end 28 25)) (probe (position 28 14))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::x0"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 42 21) (end 42 38)) (probe (position 42 21))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::a_out"))) (kind featureTyping) (ordinal 0) (authored-target "AccelerationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 37 22) (end 37 31)) (probe (position 37 22))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::delta_t"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 36 19) (end 36 28)) (probe (position 36 19))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 35 20) (end 35 30)) (probe (position 35 20))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::power"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 39 19) (end 39 29)) (probe (position 39 19))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::v_in"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 41 21) (end 41 31)) (probe (position 41 21))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::v_out"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 38 19) (end 38 30)) (probe (position 38 19))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::x_in"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 40 21) (end 40 32)) (probe (position 40 21))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::x_out"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 24 15) (end 24 25)) (probe (position 24 15))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 24 28) (end 24 30)) (probe (position 24 28))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "v0")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::v0")))))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 24 33) (end 24 34)) (probe (position 24 33))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::a")))))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 24 37) (end 24 39)) (probe (position 24 37))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 2) (authored-target "dt")
      (outcome (status resolved) (target (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::dt")))))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 22 13) (end 22 30)) (probe (position 22 13))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::a"))) (kind featureTyping) (ordinal 0) (authored-target "AccelerationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 23 14) (end 23 23)) (probe (position 23 14))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::dt"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 21 14) (end 21 24)) (probe (position 21 14))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::v0"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10d_dynamics_analysis.md") (range (start 6 21) (end 6 30)) (probe (position 6 21))
    (reference (id (source (node (document "memory://snapshot/10d_dynamics_analysis.md") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
)
~~~
