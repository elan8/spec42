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
  (document "10d_dynamics_analysis.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 19))
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
        (range (start 14 6) (end 14 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 6) (end 15 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 6) (end 16 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 6) (end 17 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 6) (end 21 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 6) (end 22 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 6) (end 23 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 6) (end 24 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 6) (end 28 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 6) (end 29 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 6) (end 30 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 6) (end 31 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 9) (end 35 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 9) (end 36 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 9) (end 37 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 9) (end 38 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 9) (end 39 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 9) (end 40 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 9) (end 41 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 9) (end 42 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 47 17) (end 47 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 48 17) (end 48 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 49 17) (end 49 33))
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
        (range (start 51 17) (end 51 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 3) (end 54 29))
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
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 59 3) (end 59 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 3) (end 59 70))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 61 3) (end 61 49))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 62 3) (end 62 43))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "21a69e5c6d0c911d70a8d03eac4e45695deccfe3ce208fb37a993b590a28755d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis"))) (kind "package") (name "10d-Dynamics Analysis") (declared-name "10d-Dynamics Analysis") (range (start (line 0) (character 0)) (end (line 0) (character 2131))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 23))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 19))))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel"))) (kind "package") (name "AnalysisModel") (declared-name "AnalysisModel") (range (start (line 46) (character 1)) (end (line 46) (character 1045))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis"))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 47) (character 2)) (end (line 47) (character 33))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 47) (character 17)) (end (line 47) (character 29))))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 48) (character 2)) (end (line 48) (character 34))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "DynamicsModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 48) (character 17)) (end (line 48) (character 30))))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 49) (character 2)) (end (line 49) (character 37))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SampledFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 49) (character 17)) (end (line 49) (character 33))))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 51) (character 2)) (end (line 51) (character 38))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 51) (character 17)) (end (line 51) (character 34))))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis"))) (kind "analysis def") (name "DynamicsAnalysis") (declared-name "DynamicsAnalysis") (range (start (line 53) (character 2)) (end (line 53) (character 828))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::accelerationProfile"))) (kind "analysis result") (name "accelerationProfile") (declared-name "accelerationProfile") (range (start (line 59) (character 3)) (end (line 59) (character 70))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis"))) (authored (relationships (typing (reference "ISQ::acceleration") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::deltaT"))) (kind "attribute") (name "deltaT") (declared-name "deltaT") (range (start (line 58) (character 3)) (end (line 58) (character 36))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::time") (range (start (line 58) (character 26)) (end (line 58) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialPosition"))) (kind "attribute") (name "initialPosition") (declared-name "initialPosition") (range (start (line 56) (character 3)) (end (line 56) (character 47))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::length") (range (start (line 56) (character 35)) (end (line 56) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialSpeed"))) (kind "attribute") (name "initialSpeed") (declared-name "initialSpeed") (range (start (line 57) (character 3)) (end (line 57) (character 43))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::speed") (range (start (line 57) (character 32)) (end (line 57) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::position"))) (kind "attribute") (name "position") (declared-name "position") (range (start (line 61) (character 3)) (end (line 61) (character 49))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis"))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::powerProfile"))) (kind "attribute") (name "powerProfile") (declared-name "powerProfile") (range (start (line 55) (character 3)) (end (line 55) (character 46))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::power") (range (start (line 55) (character 32)) (end (line 55) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::speed"))) (kind "attribute") (name "speed") (declared-name "speed") (range (start (line 62) (character 3)) (end (line 62) (character 43))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis"))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 54) (character 3)) (end (line 54) (character 29))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (range (start (line 50) (character 2)) (end (line 50) (character 39))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 50) (character 17)) (end (line 50) (character 38))))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel"))) (kind "package") (name "DynamicsModel") (declared-name "DynamicsModel") (range (start (line 11) (character 1)) (end (line 11) (character 929))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis"))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration"))) (kind "calc def") (name "Acceleration") (declared-name "Acceleration") (range (start (line 13) (character 5)) (end (line 13) (character 157))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel"))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::"))) (kind "return parameter") (name "") (range (start (line 17) (character 6)) (end (line 17) (character 47))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration"))) (authored (relationships (typing (reference "AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::m"))) (kind "in out parameter") (name "m") (declared-name "m") (range (start (line 15) (character 6)) (end (line 15) (character 23))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::p"))) (kind "in out parameter") (name "p") (declared-name "p") (range (start (line 14) (character 6)) (end (line 14) (character 24))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration"))) (authored (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::v"))) (kind "in out parameter") (name "v") (declared-name "v") (range (start (line 16) (character 6)) (end (line 16) (character 24))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position"))) (kind "calc def") (name "Position") (declared-name "Position") (range (start (line 27) (character 5)) (end (line 27) (character 151))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel"))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::"))) (kind "return parameter") (name "") (range (start (line 31) (character 6)) (end (line 31) (character 41))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (range (start (line 30) (character 6)) (end (line 30) (character 24))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::v"))) (kind "in out parameter") (name "v") (declared-name "v") (range (start (line 29) (character 6)) (end (line 29) (character 24))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::x0"))) (kind "in out parameter") (name "x0") (declared-name "x0") (range (start (line 28) (character 6)) (end (line 28) (character 26))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics"))) (kind "action def") (name "StraightLineDynamics") (declared-name "StraightLineDynamics") (range (start (line 34) (character 5)) (end (line 34) (character 409))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel"))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::a_out"))) (kind "in out parameter") (name "a_out") (declared-name "a_out") (range (start (line 42) (character 9)) (end (line 42) (character 73))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::delta_t"))) (kind "in out parameter") (name "delta_t") (declared-name "delta_t") (range (start (line 37) (character 9)) (end (line 37) (character 32))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::mass"))) (kind "in out parameter") (name "mass") (declared-name "mass") (range (start (line 36) (character 9)) (end (line 36) (character 29))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::power"))) (kind "in out parameter") (name "power") (declared-name "power") (range (start (line 35) (character 9)) (end (line 35) (character 31))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::v_in"))) (kind "in out parameter") (name "v_in") (declared-name "v_in") (range (start (line 39) (character 9)) (end (line 39) (character 30))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::v_out"))) (kind "in out parameter") (name "v_out") (declared-name "v_out") (range (start (line 41) (character 9)) (end (line 41) (character 65))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::x_in"))) (kind "in out parameter") (name "x_in") (declared-name "x_in") (range (start (line 38) (character 9)) (end (line 38) (character 31))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::x_out"))) (kind "in out parameter") (name "x_out") (declared-name "x_out") (range (start (line 40) (character 9)) (end (line 40) (character 65))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity"))) (kind "calc def") (name "Velocity") (declared-name "Velocity") (range (start (line 20) (character 5)) (end (line 20) (character 157))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel"))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::"))) (kind "return parameter") (name "") (range (start (line 24) (character 6)) (end (line 24) (character 40))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::a"))) (kind "in out parameter") (name "a") (declared-name "a") (range (start (line 22) (character 6)) (end (line 22) (character 31))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity"))) (authored (relationships (typing (reference "AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (range (start (line 23) (character 6)) (end (line 23) (character 24))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::v0"))) (kind "in out parameter") (name "v0") (declared-name "v0") (range (start (line 21) (character 6)) (end (line 21) (character 25))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel"))) (kind "package") (name "VehicleModel") (declared-name "VehicleModel") (range (start (line 3) (character 1)) (end (line 3) (character 87))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis"))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 5) (character 2)) (end (line 5) (character 56))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 6) (character 3)) (end (line 6) (character 31))) (parent (node (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 6) (character 21)) (end (line 6) (character 30)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 1) (character 16)) (end (line 1) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleModel::*") (range (start (line 47) (character 17)) (end (line 47) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "DynamicsModel::*") (range (start (line 48) (character 17)) (end (line 48) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SampledFunctions::*") (range (start (line 49) (character 17)) (end (line 49) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "SequenceFunctions::*") (range (start (line 51) (character 17)) (end (line 51) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::accelerationProfile"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::acceleration") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::deltaT"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::time") (range (start (line 58) (character 26)) (end (line 58) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialPosition"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::length") (range (start (line 56) (character 35)) (end (line 56) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialSpeed"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::speed") (range (start (line 57) (character 32)) (end (line 57) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::powerProfile"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::power") (range (start (line 55) (character 32)) (end (line 55) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (range (start (line 50) (character 17)) (end (line 50) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::p"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::x0"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::a_out"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::delta_t"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::power"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::v_in"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::v_out"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::x_in"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::x_out"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::a"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::v0"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 6) (character 21)) (end (line 6) (character 30))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (node (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::accelerationProfile")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::position")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::speed")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::a_out")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::v_out")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::x_out")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 19)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "10d-Dynamics Analysis::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 1 16) (end 1 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 21) (end 6 30)) (probe (position 6 21))
      (reference
        (source (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 6 21) (end 6 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 58 26) (end 58 35)) (probe (position 58 26))
      (reference
        (source (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::deltaT"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::time")
        (range (start 58 26) (end 58 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 55 32) (end 55 42)) (probe (position 55 32))
      (reference
        (source (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::powerProfile"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::power")
        (range (start 55 32) (end 55 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 57 32) (end 57 42)) (probe (position 57 32))
      (reference
        (source (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialSpeed"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::speed")
        (range (start 57 32) (end 57 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 56 35) (end 56 46)) (probe (position 56 35))
      (reference
        (source (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialPosition"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::length")
        (range (start 56 35) (end 56 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 47 17) (end 47 29)) (probe (position 47 17))
      (reference
        (source (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "VehicleModel::*")
        (range (start 47 17) (end 47 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 48 17) (end 48 30)) (probe (position 48 17))
      (reference
        (source (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "DynamicsModel::*")
        (range (start 48 17) (end 48 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 49 17) (end 49 33)) (probe (position 49 17))
      (reference
        (source (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "SampledFunctions::*")
        (range (start 49 17) (end 49 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 51 17) (end 51 34)) (probe (position 51 17))
      (reference
        (source (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "SequenceFunctions::*")
        (range (start 51 17) (end 51 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 50 17) (end 50 38)) (probe (position 50 17))
      (reference
        (source (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::Natural"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
        (range (start 50 17) (end 50 38))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
