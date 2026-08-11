# META
~~~ini
description=SysML Validation (05-State-based Behavior): 5-State-based Behavior-2
type=file
~~~
# SOURCE
~~~sysml
package '5-State-based Behavior-2' {
	private import ScalarValues::*;
	private import ISQ::*;
	private import '3a-Function-based Behavior-1'::*;
	
	package Definitions {
		part def VehicleA {
			perform action 'provide power': 'Provide Power';
			exhibit state 'vehicle states': 'Vehicle States';
		}
		
		part def VehicleController {
			exhibit state 'controller states': 'Controller States';
		}

		state def 'Vehicle States';
		state def 'Controller States';	

		action def 'Perform Self Test';
		action def 'Apply Parking Brake';
		action def 'Sense Temperature' { out temp: TemperatureValue; }
		
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
		 
		action 'perform self test': 'Perform Self Test';
		action 'apply parking brake': 'Apply Parking Brake';
		action 'sense temperature': 'Sense Temperature';
		
		state 'vehicle states': 'Vehicle States' parallel {

			state 'operational states' {
				entry; then off;
				
				/*
				 * The following uses a shorthand for a transition whose source 
				 * is the immediately preceding state.
				 */
				state off;
				accept 'Vehicle Start Signal' 
					if vehicle1_c1.'brake pedal depressed'
					do send new 'Start Signal'() to vehicle1_c1.vehicleController
					then starting;
					
				state starting;
				accept 'Vehicle On Signal'
					then on;
					
				state on {
					entry 'perform self test';
					do 'provide power';
					exit 'apply parking brake';
				}
				accept 'Vehicle Off Signal'
					then off;
			}
			
			state 'health states' {
				entry; then normal;
				do 'sense temperature' { out temp; }
				
				/*
				 * The shorthand can be used for multiple transitions after
				 * a single state.
				 */
				state normal;
				accept at vehicle1_c1.maintenanceTime
					then maintenance;
				accept when 'sense temperature'.temp > vehicle1_c1.Tmax
					do send new 'Over Temp'() to vehicle1_c1.vehicleController 
					then degraded;
				
				state maintenance;
				accept 'Return to Normal'
					then normal;
				
				state degraded;
				accept 'Return to Normal'
					then normal;
			}
		}
		
		state 'controller states': 'Controller States' parallel {
			state 'operational controller states' {
				entry; then off;
				
				state off;
				accept 'Start Signal'
					then on;
				
				state on;
				accept 'Off Signal'
					then off;
			}
		}		

		part vehicle1_c1: VehicleA {
			port fuelCmdPort {
				in fuelCmd: FuelCmd;
			}
			
			attribute 'brake pedal depressed': Boolean;		
			attribute maintenanceTime: Time::DateTime;
			attribute Tmax: TemperatureValue;
			
			perform 'provide power' :>> VehicleA::'provide power' {
				in fuelCmd = fuelCmdPort.fuelCmd;
			}
				
			exhibit 'vehicle states' :>> VehicleA::'vehicle states';
				
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
  (document "5_state_based_behavior_2.md"
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
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 3) (end 7 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 35) (end 20 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 33 17) (end 33 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 2) (end 35 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 2) (end 36 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 2) (end 37 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 2) (end 39 1249))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 29) (end 69 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 92 2) (end 92 250))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 106 20) (end 106 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 108 4) (end 108 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 111 3) (end 111 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 111 38) (end 111 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 3) (end 112 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 30) (end 112 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 3) (end 113 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 19) (end 113 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 119 32) (end 119 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 121 27) (end 121 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 122 36) (end 122 74))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package '5-State-based Behavior-2' {
    private import ScalarValues::*;
    private import ISQ::*;
    private import '3a-Function-based Behavior-1'::*;

    package Definitions {
        part def VehicleA {
            perform action 'provide power': 'Provide Power';
            exhibit state 'vehicle states': 'Vehicle States';
        }

        part def VehicleController {
            exhibit state 'controller states': 'Controller States';
        }

        state def 'Vehicle States';
        state def 'Controller States';

        action def 'Perform Self Test';
        action def 'Apply Parking Brake';
        action def 'Sense Temperature' { out temp: TemperatureValue; }

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

        action 'perform self test': 'Perform Self Test';
        action 'apply parking brake': 'Apply Parking Brake';
        action 'sense temperature': 'Sense Temperature';

        state 'vehicle states': 'Vehicle States' parallel {

            state 'operational states' {
                entry; then off;

                /*
				 * The following uses a shorthand for a transition whose source 
				 * is the immediately preceding state.
				 */
                state off;
                accept 'Vehicle Start Signal'
                if vehicle1_c1.'brake pedal depressed'
                do send new 'Start Signal'() to vehicle1_c1.vehicleController
                then starting;

                state starting;
                accept 'Vehicle On Signal'
                then on;

                state on {
                    entry 'perform self test';
                    do 'provide power';
                    exit 'apply parking brake';
                }
                accept 'Vehicle Off Signal'
                then off;
            }

            state 'health states' {
                entry; then normal;
                do 'sense temperature' { out temp; }

                /*
				 * The shorthand can be used for multiple transitions after
				 * a single state.
				 */
                state normal;
                accept at vehicle1_c1.maintenanceTime
                then maintenance;
                accept when 'sense temperature'.temp > vehicle1_c1.Tmax
                do send new 'Over Temp'() to vehicle1_c1.vehicleController
                then degraded;

                state maintenance;
                accept 'Return to Normal'
                then normal;

                state degraded;
                accept 'Return to Normal'
                then normal;
            }
        }

        state 'controller states': 'Controller States' parallel {
            state 'operational controller states' {
                entry; then off;

                state off;
                accept 'Start Signal'
                then on;

                state on;
                accept 'Off Signal'
                then off;
            }
        }

        part vehicle1_c1: VehicleA {
            port fuelCmdPort {
                in fuelCmd: FuelCmd;
            }

            attribute 'brake pedal depressed': Boolean;
            attribute maintenanceTime: Time::DateTime;
            attribute Tmax: TemperatureValue;

            perform 'provide power' :>> VehicleA::'provide power' {
                in fuelCmd = fuelCmdPort.fuelCmd;
            }

            exhibit 'vehicle states' :>> VehicleA::'vehicle states';

            part vehicleController: VehicleController {
                exhibit 'controller states' :>> VehicleController::'controller states';
            }
        }
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "0a741149a3d2b006edec8295f92d3b3bedd45afc913420963db47b128f1dae75") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2"))) (kind "package") (name "5-State-based Behavior-2") (declared-name "5-State-based Behavior-2"))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "3a-Function-based Behavior-1::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Apply Parking Brake"))) (kind "action def") (name "Apply Parking Brake") (declared-name "Apply Parking Brake") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States"))) (kind "state def") (name "Controller States") (declared-name "Controller States") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Off Signal"))) (kind "attribute def") (name "Off Signal") (declared-name "Off Signal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Over Temp"))) (kind "attribute def") (name "Over Temp") (declared-name "Over Temp") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Perform Self Test"))) (kind "action def") (name "Perform Self Test") (declared-name "Perform Self Test") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Return to Normal"))) (kind "attribute def") (name "Return to Normal") (declared-name "Return to Normal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature"))) (kind "action def") (name "Sense Temperature") (declared-name "Sense Temperature") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature::temp"))) (kind "in out parameter") (name "temp") (declared-name "temp") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature"))) (authored (relationships (typing (reference "TemperatureValue")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Start Signal"))) (kind "attribute def") (name "Start Signal") (declared-name "Start Signal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle Off Signal"))) (kind "attribute def") (name "Vehicle Off Signal") (declared-name "Vehicle Off Signal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle On Signal"))) (kind "attribute def") (name "Vehicle On Signal") (declared-name "Vehicle On Signal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle Start Signal"))) (kind "attribute def") (name "Vehicle Start Signal") (declared-name "Vehicle Start Signal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))) (kind "state def") (name "Vehicle States") (declared-name "Vehicle States") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (kind "part def") (name "VehicleA") (declared-name "VehicleA") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))) (authored (membership (kind Owning)) (relationships (perform (reference "5-State-based Behavior-2::Definitions::VehicleA::provide power")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (authored (relationships (typing (reference "Provide Power")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::vehicle states"))) (kind "exhibit state") (name "vehicle states") (declared-name "vehicle states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (authored (relationships (typing (reference "Vehicle States")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController"))) (kind "part def") (name "VehicleController") (declared-name "VehicleController") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController::controller states"))) (kind "exhibit state") (name "controller states") (declared-name "controller states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController"))) (authored (relationships (typing (reference "Controller States")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::apply parking brake"))) (kind "action") (name "apply parking brake") (declared-name "apply parking brake") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Apply Parking Brake")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states"))) (kind "state") (name "controller states") (declared-name "controller states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Controller States")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind "state") (name "operational controller states") (declared-name "operational controller states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-2::Usages::controller states::operational controller states::on")) (transition (reference "5-State-based Behavior-2::Usages::controller states::operational controller states::off")) (initial-state (reference "5-State-based Behavior-2::Usages::controller states::operational controller states::off")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off"))) (kind "state") (name "off") (declared-name "off") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::on"))) (kind "state") (name "on") (declared-name "on") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_off"))) (kind "transition") (name "transition_operational controller states_to_off") (declared-name "transition_operational controller states_to_off") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_off"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_on"))) (kind "transition") (name "transition_operational controller states_to_on") (declared-name "transition_operational controller states_to_on") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::perform self test"))) (kind "action") (name "perform self test") (declared-name "perform self test") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Perform Self Test")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::sense temperature"))) (kind "action") (name "sense temperature") (declared-name "sense temperature") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Sense Temperature")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (kind "state") (name "vehicle states") (declared-name "vehicle states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle States")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind "state") (name "health states") (declared-name "health states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-2::Usages::vehicle states::health states::maintenance")) (transition (reference "5-State-based Behavior-2::Usages::vehicle states::health states::degraded")) (transition (reference "5-State-based Behavior-2::Usages::vehicle states::health states::normal")) (transition (reference "5-State-based Behavior-2::Usages::vehicle states::health states::normal")) (initial-state (reference "5-State-based Behavior-2::Usages::vehicle states::health states::normal")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_do"))) (kind "action") (name "do") (declared-name "do") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_do::temp"))) (kind "in out parameter") (name "temp") (declared-name "temp") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_do"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::degraded"))) (kind "state") (name "degraded") (declared-name "degraded") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::maintenance"))) (kind "state") (name "maintenance") (declared-name "maintenance") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal"))) (kind "state") (name "normal") (declared-name "normal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded"))) (kind "transition") (name "transition_health states_to_degraded") (declared-name "transition_health states_to_degraded") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_maintenance"))) (kind "transition") (name "transition_health states_to_maintenance") (declared-name "transition_health states_to_maintenance") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_maintenance::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_maintenance"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal"))) (kind "transition") (name "transition_health states_to_normal") (declared-name "transition_health states_to_normal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal#transition"))) (kind "transition") (name "transition_health states_to_normal") (declared-name "transition_health states_to_normal") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal#transition::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal#transition"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind "state") (name "operational states") (declared-name "operational states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-2::Usages::vehicle states::operational states::starting")) (transition (reference "5-State-based Behavior-2::Usages::vehicle states::operational states::on")) (transition (reference "5-State-based Behavior-2::Usages::vehicle states::operational states::off")) (initial-state (reference "5-State-based Behavior-2::Usages::vehicle states::operational states::off")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off"))) (kind "state") (name "off") (declared-name "off") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on"))) (kind "state") (name "on") (declared-name "on") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on::_do"))) (kind "action") (name "do") (declared-name "do") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on::_exit"))) (kind "action") (name "exit") (declared-name "exit") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::starting"))) (kind "state") (name "starting") (declared-name "starting") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_off"))) (kind "transition") (name "transition_operational states_to_off") (declared-name "transition_operational states_to_off") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_off"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_on"))) (kind "transition") (name "transition_operational states_to_on") (declared-name "transition_operational states_to_on") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting"))) (kind "transition") (name "transition_operational states_to_starting") (declared-name "transition_operational states_to_starting") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting::guard"))) (kind "transition guard") (name "guard") (declared-name "guard") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleA")) (perform (reference "5-State-based Behavior-2::Usages::vehicle1_c1::provide power")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::Tmax"))) (kind "attribute") (name "Tmax") (declared-name "Tmax") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "TemperatureValue")) (typing (reference "TemperatureValue")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::brake pedal depressed"))) (kind "attribute") (name "brake pedal depressed") (declared-name "brake pedal depressed") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean")) (typing (reference "Boolean")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::fuelCmdPort"))) (authored (relationships (typing (reference "FuelCmd")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::maintenanceTime"))) (kind "attribute") (name "maintenanceTime") (declared-name "maintenanceTime") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "DateTime")) (typing (reference "Time::DateTime")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicle states"))) (kind "state") (name "vehicle states") (declared-name "vehicle states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "VehicleA::vehicle states")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))) (kind "part") (name "vehicleController") (declared-name "vehicleController") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleController")))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController::controller states"))) (kind "state") (name "controller states") (declared-name "controller states") (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "VehicleController::controller states")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "3a-Function-based Behavior-1::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature::temp"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (kind performSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Definitions::VehicleA::provide power") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::provide power"))) (kind featureTyping) (ordinal 0)) (authored-target "Provide Power") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::vehicle states"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle States") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController::controller states"))) (kind featureTyping) (ordinal 0)) (authored-target "Controller States") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::apply parking brake"))) (kind featureTyping) (ordinal 0)) (authored-target "Apply Parking Brake") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states"))) (kind featureTyping) (ordinal 0)) (authored-target "Controller States") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Usages::controller states::operational controller states::on") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::on")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind transitionSource) (ordinal 1)) (authored-target "5-State-based Behavior-2::Usages::controller states::operational controller states::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind initialStateSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Usages::controller states::operational controller states::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::perform self test"))) (kind featureTyping) (ordinal 0)) (authored-target "Perform Self Test") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::sense temperature"))) (kind featureTyping) (ordinal 0)) (authored-target "Sense Temperature") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle States") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::health states::maintenance") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::maintenance")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 1)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::health states::degraded") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::degraded")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 2)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::health states::normal") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 3)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::health states::normal") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind initialStateSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::health states::normal") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_do::temp"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::operational states::starting") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::starting")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 1)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::operational states::on") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 2)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::operational states::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind initialStateSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::operational states::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleA") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (kind performSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Usages::vehicle1_c1::provide power") (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 1)) (authored-target "TemperatureValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 1)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 0)) (authored-target "DateTime") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 1)) (authored-target "Time::DateTime") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicle states"))) (kind redefinition) (ordinal 0)) (authored-target "VehicleA::vehicle states") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleController") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController::controller states"))) (kind redefinition) (ordinal 0)) (authored-target "VehicleController::controller states") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::provide power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::vehicle states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::vehicle states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController::controller states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController::controller states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::degraded"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::maintenance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 2)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 3)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 2)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::starting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::provide power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (kind performSource) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting::guard")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 16) (end 2 19)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-2::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 2 16) (end 2 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 111 38) (end 111 45)) (probe (position 111 38))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::brake pedal depressed"))
        (kind featureTyping) (ordinal 1) (authored-target "Boolean")
        (range (start 111 38) (end 111 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 106 20) (end 106 28)) (probe (position 106 20))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))
        (kind featureTyping) (ordinal 0) (authored-target "VehicleA")
        (range (start 106 20) (end 106 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 17) (end 33 28)) (probe (position 33 17))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-2::Usages::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 33 17) (end 33 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 112 30) (end 112 44)) (probe (position 112 30))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::maintenanceTime"))
        (kind featureTyping) (ordinal 1) (authored-target "Time::DateTime")
        (range (start 112 30) (end 112 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 113 19) (end 113 35)) (probe (position 113 19))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::Tmax"))
        (kind featureTyping) (ordinal 1) (authored-target "TemperatureValue")
        (range (start 113 19) (end 113 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 121 27) (end 121 44)) (probe (position 121 27))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))
        (kind featureTyping) (ordinal 0) (authored-target "VehicleController")
        (range (start 121 27) (end 121 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 119 32) (end 119 58)) (probe (position 119 32))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicle states"))
        (kind redefinition) (ordinal 0) (authored-target "VehicleA::vehicle states")
        (range (start 119 32) (end 119 58))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 46)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-2::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "3a-Function-based Behavior-1::*")
        (range (start 3 16) (end 3 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 122 36) (end 122 74)) (probe (position 122 36))
      (reference
        (source (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController::controller states"))
        (kind redefinition) (ordinal 0) (authored-target "VehicleController::controller states")
        (range (start 122 36) (end 122 74))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
