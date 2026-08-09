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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,CloseParen,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,Plus,Ident,Star,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,Plus,Ident,Star,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwOut,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwOut,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAnalysis,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwIn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwIn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwIn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwReturn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,OpenSquare,Star,CloseSquare,ColonEq,OpenParen,CloseParen,Semicolon,
KwPrivate,KwAttribute,Ident,ColonEq,Ident,Semicolon,
KwPrivate,KwAttribute,Ident,ColonEq,Ident,Semicolon,
KwFor,Ident,KwIn,DecimalValue,DotDot,Ident,Arrow,Ident,OpenParen,CloseParen,Minus,DecimalValue,OpenCurly,
KwPerform,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Hash,OpenParen,Ident,CloseParen,Semicolon,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwThen,KwAssign,Ident,ColonEq,Ident,Dot,Ident,Semicolon,
KwThen,KwAssign,Ident,ColonEq,Ident,Dot,Ident,Semicolon,
KwThen,KwAssign,Ident,ColonEq,Ident,Arrow,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''10d-Dynamics Analysis''
    (import_decl private 'ISQ::*')
    (package_def 'VehicleModel'
      (part_def 'Vehicle'
        (attribute_usage 'mass' :> 'ISQ::mass')))
    (package_def 'DynamicsModel'
      (calc_def 'Acceleration'
        (default_ref_usage in 'p' : 'PowerValue')
        (default_ref_usage in 'm' : 'MassValue')
        (default_ref_usage in 'v' : 'SpeedValue')
        (return_member))
      (calc_def 'Velocity'
        (default_ref_usage in 'v0' : 'SpeedValue')
        (default_ref_usage in 'a' : 'AccelerationValue')
        (default_ref_usage in 'dt' : 'TimeValue')
        (return_member))
      (calc_def 'Position'
        (default_ref_usage in 'x0' : 'LengthValue')
        (default_ref_usage in 'v' : 'SpeedValue')
        (default_ref_usage in 'dt' : 'TimeValue')
        (return_member))
      (action_def 'StraightLineDynamics'
        (default_ref_usage in 'power' : 'PowerValue')
        (default_ref_usage in 'mass' : 'MassValue')
        (default_ref_usage in 'delta_t' : 'TimeValue')
        (default_ref_usage in 'x_in' : 'LengthValue')
        (default_ref_usage in 'v_in' : 'SpeedValue')
        (default_ref_usage out 'x_out' : 'LengthValue' value)
        (default_ref_usage out 'v_out' : 'SpeedValue' value)
        (default_ref_usage out 'a_out' : 'AccelerationValue' value)))
    (package_def 'AnalysisModel'
      (import_decl private 'VehicleModel::*')
      (import_decl private 'DynamicsModel::*')
      (import_decl private 'SampledFunctions::*')
      (import_decl private 'ScalarValues::Natural')
      (import_decl private 'SequenceFunctions::*')
      (analysis_case_def 'DynamicsAnalysis'
        (sysml_decl 'vehicle' : 'Vehicle')
        (attribute_usage in 'powerProfile' :> 'ISQ::power' multiplicity)
        (attribute_usage in 'initialPosition' :> 'ISQ::length')
        (attribute_usage in 'initialSpeed' :> 'ISQ::speed')
        (attribute_usage in 'deltaT' :> 'ISQ::time')
        (return_member)
        (attribute_usage private 'position' value)
        (attribute_usage private 'speed' value)
        (for_loop_node)))))
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
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'ISQ::length'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'ISQ::time'
semantic.unresolved_name 'ISQ::acceleration'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'ISQ::length'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'ISQ::time'
semantic.unresolved_name 'ISQ::acceleration'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis"))) (name "10d-Dynamics Analysis") (declared-name "10d-Dynamics Analysis")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::*"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel"))) (name "AnalysisModel") (declared-name "AnalysisModel")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*#import"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*#import2"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::*#import3"))) (name "*") (declared-name "*"))
            (element (kind "analysis def") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis"))) (name "DynamicsAnalysis") (declared-name "DynamicsAnalysis")
              (contains
                (element (kind "analysis result") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::accelerationProfile"))) (name "accelerationProfile") (declared-name "accelerationProfile") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::deltaT"))) (name "deltaT") (declared-name "deltaT") (declared (properties (direction "in") (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialPosition"))) (name "initialPosition") (declared-name "initialPosition") (declared (properties (direction "in") (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialSpeed"))) (name "initialSpeed") (declared-name "initialSpeed") (declared (properties (direction "in") (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::position"))) (name "position") (declared-name "position") (declared (properties (ordered false) (unique true)) (feature-value (kind initial) (expression (kind "featureReference") (reference "initialPosition")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::powerProfile"))) (name "powerProfile") (declared-name "powerProfile") (declared (properties (direction "in") (ordered false) (unique true)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::speed"))) (name "speed") (declared-name "speed") (declared (properties (ordered false) (unique true)) (feature-value (kind initial) (expression (kind "featureReference") (reference "initialSpeed")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis")))))
                (element (kind "subject") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis")))))
              )
            )
            (element (kind "import") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::Natural"))) (name "Natural") (declared-name "Natural"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel"))) (name "DynamicsModel") (declared-name "DynamicsModel")
          (contains
            (element (kind "calc def") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration"))) (name "Acceleration") (declared-name "Acceleration")
              (contains
                (element (kind "return parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::m"))) (name "m") (declared-name "m") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::p"))) (name "p") (declared-name "p") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration::v"))) (name "v") (declared-name "v") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration")))))
              )
            )
            (element (kind "calc def") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position"))) (name "Position") (declared-name "Position")
              (contains
                (element (kind "return parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::dt"))) (name "dt") (declared-name "dt") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::v"))) (name "v") (declared-name "v") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position::x0"))) (name "x0") (declared-name "x0") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position")))))
              )
            )
            (element (kind "action def") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics"))) (name "StraightLineDynamics") (declared-name "StraightLineDynamics")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::a_out"))) (name "a_out") (declared-name "a_out") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::delta_t"))) (name "delta_t") (declared-name "delta_t") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::mass"))) (name "mass") (declared-name "mass") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::power"))) (name "power") (declared-name "power") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::v_in"))) (name "v_in") (declared-name "v_in") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::v_out"))) (name "v_out") (declared-name "v_out") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::x_in"))) (name "x_in") (declared-name "x_in") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics::x_out"))) (name "x_out") (declared-name "x_out") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics")))))
              )
            )
            (element (kind "calc def") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity"))) (name "Velocity") (declared-name "Velocity")
              (contains
                (element (kind "return parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::a"))) (name "a") (declared-name "a") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::dt"))) (name "dt") (declared-name "dt") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity::v0"))) (name "v0") (declared-name "v0") (effective (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel"))) (name "VehicleModel") (declared-name "VehicleModel")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (subject (status resolved) (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis"))) (to (node (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialSpeed"))) (to (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::speed"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::vehicle"))) (to (node (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis"))) (status missing-prerequisite) (target "AnalysisCases::AnalysisCase"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::deltaT"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialPosition"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::initialSpeed"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::position"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::powerProfile"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::AnalysisModel::DynamicsAnalysis::speed"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Acceleration"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Position"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::DynamicsModel::Velocity"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10d-Dynamics Analysis::VehicleModel::Vehicle::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/10d_dynamics_analysis.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 19))
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
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 53 2) (end 53 828))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 59 3) (end 59 70))
      )
    )
  )
)
~~~
