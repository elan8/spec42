# META
~~~ini
description=SysML Validation (05-State-based Behavior): 5-State-based Behavior-1a
type=file
~~~
# SOURCE
~~~sysml
package '5-State-based Behavior-1a' {
	private import ScalarValues::*;
	private import ISQ::*;
	
	package Definitions {
		part def VehicleA {
			/*
			 * The following declare that 'VehicleA' performs a
			 * 'provide power' action and exhibits some 'vehicle states',
			 * without giving details about these behaviors.
			 */
			perform action 'provide power': 'Provide Power';
			exhibit state 'vehicle states': 'Vehicle States';
		}
		
		part def VehicleController {
			exhibit state 'controller states': 'Controller States';
		}

		/*
		 * Black box specifications for state definitions may also have
		 * input and output parameters, like activities, though none
		 * are used here.
		 */

		state def 'Vehicle States';
		state def 'Controller States';	

		action def 'Provide Power';
		action def 'Perform Self Test';
		action def 'Apply Parking Brake';
		action def 'Sense Temperature' { out temp: TemperatureValue; }
		
		attribute def FuelCmd;
		
		attribute def 'Vehicle Start Signal';
		attribute def 'Vehicle On Signal';
		attribute def 'Vehicle Off Signal';
		
		attribute def 'Start Signal';
		attribute def 'Off Signal';
		attribute def 'Over Temp';
		attribute def 'Return to Normal';
	}
	
	package Usages {
		private import Definitions::*;
		
		/*
		 * These actions are used enabled in the state usage 
		 * 'vehicle states', in addition to 'provide power'.
		 */
		 
		action 'provide power': 'Provide Power';
		action 'perform self test': 'Perform Self Test';
		action 'apply parking brake': 'Apply Parking Brake';
		action 'sense temperature': 'Sense Temperature';
		
		state 'vehicle states': 'Vehicle States' parallel {
			/*
			 * This is a usage of the state definition 'Vehicle States'.
			 * Note that it depends specifically on on the part 'vehicle1_c1'.
			 */		 

			state 'operational states' {
			doc
			/*
			 * The state definition for this usage is implicit.
			 */
			
				entry action initial {
				doc
				/*
				 * This empty entry action acts like a start pseudo state.
				 */
				}
				
				transition initial then off;
			    
				state off;
				
				transition 'off-starting'
					first off
					accept 'Vehicle Start Signal' 
					if vehicle1_c1.'brake pedal depressed'
					do send new 'Start Signal'() to vehicle1_c1.vehicleController
					then starting {
					/*
					 * The transition definition for a transition usage is always implicit.
					 * "accept" marks the trigger, "if" the guard and "do" the effect.
					 * 
					 * The notation "'new Start Signal'()" constructs a specific instance of the
					 * 'Start Signal' attribute def to be sent to the 'vehicleController'. If the
					 * attribute def had properties, their values would be given as arguments
					 * inside the parentheses.
					 */						
				}
					
				state starting;
				
				transition 'starting-on'
					first starting
					accept 'Vehicle On Signal'
					then on;
				
				state on {
					/*
					 * A state may have a "entry" action that is performed on entry into
					 * the state, a "do" action that is performed while in the state
					 * and an "exit" action that is performed on exit from the state.
					 */
				
					entry 'perform self test';
					do 'provide power';
					exit 'apply parking brake';
				}
				
				transition 'on-off'
					first on
					accept 'Vehicle Off Signal'
					then off;
			}
			
			state 'health states' {
				/*
				 * 'health states' is concurrent with 'operational states', because the
				 * containing state usage is "parallel".
				 */
			
				entry action initial;
				do 'sense temperature' { out temp; 
					/*
					 * State-behavior actions may have input and output parameters.
					 */
				}
				
				transition initial then normal;
				
				state normal;
				
				transition 'normal-maintenance'
					first normal
					accept at vehicle1_c1.maintenanceTime
					then maintenance;
				
				transition 'normal-degraded'
					first normal
					accept when 'sense temperature'.temp > vehicle1_c1.Tmax
					do send new 'Over Temp'() to vehicle1_c1.vehicleController 
					then degraded;
				
				state maintenance;
				
				transition 'maintenance-normal'
					first maintenance
					accept 'Return to Normal'
					then normal;
				
				state degraded;
				
				transition 'degraded-normal'
					first degraded
					accept 'Return to Normal'
					then normal;
			}
		}
		
		state 'controller states': 'Controller States' parallel {
			state 'operational controller states' {
				entry action initial; 
				
				transition initial then off;
				
				state off;
				
				transition 'off-on'
					first off
					accept 'Start Signal'
					then on;
				
				state on;
				
				transition 'on-off'
					first on
					accept 'Off Signal'
					then off;
			}
		}		

		part vehicle1_c1: VehicleA {
			port fuelCmdPort {
				in fuelCmd: FuelCmd;
			}
			
			/*
			 * These attribute properties are used in the specification for
			 * 'vehicle states'.
			 */
			attribute 'brake pedal depressed': Boolean;		
			attribute maintenanceTime: Time::DateTime;
			attribute Tmax: TemperatureValue;
			
			perform 'provide power' :>> VehicleA::'provide power' {
			doc
			/*
			 * In the context of the 'vehicle1_c1' part, the 'provide power' action
			 * that is enabled in 'vehicle states' gets its input from the 'fuelCmdPort'.
			 */
			
				in fuelCmd = fuelCmdPort.fuelCmd;
			}
			
			exhibit 'vehicle states' :>> VehicleA::'vehicle states' {
				/*
				 * This allocates the state usage 'vehicle states' as the detailed
				 * state-based behavior for 'vehicle1_c1' that fills in the generic
				 * declaration in 'VehicleA'.
				 */
			}
				
			//*
			// The above is semantically equivalent to:
			
			ref state 'vehicle states' :> Usages::'vehicle states', exhibitedStates
				:>> VehicleA::'vehicle states';		
				
			// For a composite state performance within the vehicle, replace the above with:
			
			state 'vehicle states' :>> Usages::'vehicle states', VehicleA::'vehicle states';
			*/

			part vehicleController: VehicleController {
				exhibit 'controller states' :>> VehicleController::'controller states';
			}			
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "5_state_based_behavior_1a.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
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
        (range (start 31 35) (end 31 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 46 17) (end 46 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 2) (end 53 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 2) (end 54 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 2) (end 55 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 2) (end 56 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 58 2) (end 58 2753))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 130 29) (end 130 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 167 2) (end 167 383))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 189 20) (end 189 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 191 4) (end 191 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 198 3) (end 198 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 198 38) (end 198 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 199 3) (end 199 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 199 30) (end 199 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 200 3) (end 200 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 200 19) (end 200 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 212 32) (end 212 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 231 27) (end 231 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 232 36) (end 232 74))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "584182264a56d09f82960f6b2a229b07fa03bc1a63138f0234ac40c0d8b450c6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a"))) (kind "package") (name "5-State-based Behavior-1a") (declared-name "5-State-based Behavior-1a"))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Apply Parking Brake"))) (kind "action def") (name "Apply Parking Brake") (declared-name "Apply Parking Brake") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States"))) (kind "state def") (name "Controller States") (declared-name "Controller States") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::FuelCmd"))) (kind "attribute def") (name "FuelCmd") (declared-name "FuelCmd") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Off Signal"))) (kind "attribute def") (name "Off Signal") (declared-name "Off Signal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Over Temp"))) (kind "attribute def") (name "Over Temp") (declared-name "Over Temp") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Perform Self Test"))) (kind "action def") (name "Perform Self Test") (declared-name "Perform Self Test") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Provide Power"))) (kind "action def") (name "Provide Power") (declared-name "Provide Power") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Return to Normal"))) (kind "attribute def") (name "Return to Normal") (declared-name "Return to Normal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature"))) (kind "action def") (name "Sense Temperature") (declared-name "Sense Temperature") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature::temp"))) (kind "in out parameter") (name "temp") (declared-name "temp") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature"))) (authored (relationships (typing (reference "TemperatureValue")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Start Signal"))) (kind "attribute def") (name "Start Signal") (declared-name "Start Signal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle Off Signal"))) (kind "attribute def") (name "Vehicle Off Signal") (declared-name "Vehicle Off Signal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle On Signal"))) (kind "attribute def") (name "Vehicle On Signal") (declared-name "Vehicle On Signal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle Start Signal"))) (kind "attribute def") (name "Vehicle Start Signal") (declared-name "Vehicle Start Signal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))) (kind "state def") (name "Vehicle States") (declared-name "Vehicle States") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (kind "part def") (name "VehicleA") (declared-name "VehicleA") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))) (authored (membership (kind Owning)) (relationships (perform (reference "5-State-based Behavior-1a::Definitions::VehicleA::provide power")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (authored (relationships (typing (reference "Provide Power")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::vehicle states"))) (kind "exhibit state") (name "vehicle states") (declared-name "vehicle states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (authored (relationships (typing (reference "Vehicle States")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController"))) (kind "part def") (name "VehicleController") (declared-name "VehicleController") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController::controller states"))) (kind "exhibit state") (name "controller states") (declared-name "controller states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController"))) (authored (relationships (typing (reference "Controller States")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::apply parking brake"))) (kind "action") (name "apply parking brake") (declared-name "apply parking brake") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Apply Parking Brake")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states"))) (kind "state") (name "controller states") (declared-name "controller states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Controller States")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (kind "state") (name "operational controller states") (declared-name "operational controller states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::controller states::operational controller states::off")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::initial"))) (kind "transition") (name "initial") (declared-name "initial") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (kind "state") (name "off") (declared-name "off") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::controller states::operational controller states::on")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off-on"))) (kind "transition") (name "off-on") (declared-name "off-on") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off-on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off-on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on"))) (kind "state") (name "on") (declared-name "on") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::controller states::operational controller states::off")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on-off"))) (kind "transition") (name "on-off") (declared-name "on-off") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on-off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on-off"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::perform self test"))) (kind "action") (name "perform self test") (declared-name "perform self test") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Perform Self Test")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Provide Power")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::sense temperature"))) (kind "action") (name "sense temperature") (declared-name "sense temperature") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Sense Temperature")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (kind "state") (name "vehicle states") (declared-name "vehicle states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle States")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (kind "state") (name "health states") (declared-name "health states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::health states::normal")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::_do"))) (kind "action") (name "do") (declared-name "do") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::_do::temp"))) (kind "in out parameter") (name "temp") (declared-name "temp") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::_do"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded"))) (kind "state") (name "degraded") (declared-name "degraded") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::health states::normal")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded-normal"))) (kind "transition") (name "degraded-normal") (declared-name "degraded-normal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded-normal::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded-normal"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::initial"))) (kind "transition") (name "initial") (declared-name "initial") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance"))) (kind "state") (name "maintenance") (declared-name "maintenance") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::health states::normal")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance-normal"))) (kind "transition") (name "maintenance-normal") (declared-name "maintenance-normal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance-normal::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance-normal"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (kind "state") (name "normal") (declared-name "normal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance")) (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-degraded"))) (kind "transition") (name "normal-degraded") (declared-name "normal-degraded") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-degraded::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-degraded"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-degraded::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-degraded"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-maintenance"))) (kind "transition") (name "normal-maintenance") (declared-name "normal-maintenance") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-maintenance::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-maintenance"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (kind "state") (name "operational states") (declared-name "operational states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::operational states::off")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::_entry::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::_entry"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::initial"))) (kind "transition") (name "initial") (declared-name "initial") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (kind "state") (name "off") (declared-name "off") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting"))) (kind "transition") (name "off-starting") (declared-name "off-starting") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting::guard"))) (kind "transition guard") (name "guard") (declared-name "guard") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))) (kind "state") (name "on") (declared-name "on") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::operational states::off")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on-off"))) (kind "transition") (name "on-off") (declared-name "on-off") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on-off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on-off"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on::_do"))) (kind "action") (name "do") (declared-name "do") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on::_exit"))) (kind "action") (name "exit") (declared-name "exit") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting"))) (kind "state") (name "starting") (declared-name "starting") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::operational states::on")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting-on"))) (kind "transition") (name "starting-on") (declared-name "starting-on") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting-on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting-on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleA")) (perform (reference "5-State-based Behavior-1a::Usages::vehicle1_c1::provide power")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::Tmax"))) (kind "attribute") (name "Tmax") (declared-name "Tmax") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "TemperatureValue")) (typing (reference "TemperatureValue")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::brake pedal depressed"))) (kind "attribute") (name "brake pedal depressed") (declared-name "brake pedal depressed") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean")) (typing (reference "Boolean")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::fuelCmdPort"))) (authored (relationships (typing (reference "FuelCmd")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::maintenanceTime"))) (kind "attribute") (name "maintenanceTime") (declared-name "maintenanceTime") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "DateTime")) (typing (reference "Time::DateTime")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicle states"))) (kind "state") (name "vehicle states") (declared-name "vehicle states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "VehicleA::vehicle states")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController"))) (kind "part") (name "vehicleController") (declared-name "vehicleController") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleController")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController::controller states"))) (kind "state") (name "controller states") (declared-name "controller states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "VehicleController::controller states")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature::temp"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (kind performSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Definitions::VehicleA::provide power") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power"))) (kind featureTyping) (ordinal 0)) (authored-target "Provide Power") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Provide Power")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::vehicle states"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle States") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController::controller states"))) (kind featureTyping) (ordinal 0)) (authored-target "Controller States") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::apply parking brake"))) (kind featureTyping) (ordinal 0)) (authored-target "Apply Parking Brake") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states"))) (kind featureTyping) (ordinal 0)) (authored-target "Controller States") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::controller states::operational controller states::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::controller states::operational controller states::on") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::controller states::operational controller states::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::perform self test"))) (kind featureTyping) (ordinal 0)) (authored-target "Perform Self Test") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::provide power"))) (kind featureTyping) (ordinal 0)) (authored-target "Provide Power") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::sense temperature"))) (kind featureTyping) (ordinal 0)) (authored-target "Sense Temperature") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle States") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::health states::normal") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::_do::temp"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::health states::normal") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::health states::normal") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (kind transitionSource) (ordinal 1)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::operational states::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::operational states::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::operational states::on") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleA") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (kind performSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle1_c1::provide power") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 1)) (authored-target "TemperatureValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 1)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 0)) (authored-target "DateTime") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 1)) (authored-target "Time::DateTime") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicle states"))) (kind redefinition) (ordinal 0)) (authored-target "VehicleA::vehicle states") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleController") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController::controller states"))) (kind redefinition) (ordinal 0)) (authored-target "VehicleController::controller states") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Provide Power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::vehicle states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::vehicle states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController::controller states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController::controller states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::provide power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (kind performSource) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting::guard")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 16) (end 2 19)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-1a::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 2 16) (end 2 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 198 38) (end 198 45)) (probe (position 198 38))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::brake pedal depressed"))
        (kind featureTyping) (ordinal 1) (authored-target "Boolean")
        (range (start 198 38) (end 198 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 189 20) (end 189 28)) (probe (position 189 20))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))
        (kind featureTyping) (ordinal 0) (authored-target "VehicleA")
        (range (start 189 20) (end 189 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 46 17) (end 46 28)) (probe (position 46 17))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 46 17) (end 46 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-1a::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 199 30) (end 199 44)) (probe (position 199 30))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::maintenanceTime"))
        (kind featureTyping) (ordinal 1) (authored-target "Time::DateTime")
        (range (start 199 30) (end 199 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 200 19) (end 200 35)) (probe (position 200 19))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::Tmax"))
        (kind featureTyping) (ordinal 1) (authored-target "TemperatureValue")
        (range (start 200 19) (end 200 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 231 27) (end 231 44)) (probe (position 231 27))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController"))
        (kind featureTyping) (ordinal 0) (authored-target "VehicleController")
        (range (start 231 27) (end 231 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 212 32) (end 212 58)) (probe (position 212 32))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicle states"))
        (kind redefinition) (ordinal 0) (authored-target "VehicleA::vehicle states")
        (range (start 212 32) (end 212 58))
        (outcome (status unresolved))
      )
    )
    (query (range (start 232 36) (end 232 74)) (probe (position 232 36))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController::controller states"))
        (kind redefinition) (ordinal 0) (authored-target "VehicleController::controller states")
        (range (start 232 36) (end 232 74))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
