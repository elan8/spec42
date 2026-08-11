# META
~~~ini
description=SysML Validation (05-State-based Behavior): 5-State-based Behavior-1
type=file
~~~
# SOURCE
~~~sysml
package '5-State-based Behavior-1' {
	private import ScalarValues::*;
	private import ISQ::*;
	private import '3a-Function-based Behavior-1'::*;
	
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
		
		/*
		 * These actions are used enabled in the state usage 
		 * 'vehicle states', in addition to 'provide power'.
		 */
		 
		action 'perform self test': 'Perform Self Test';
		action 'apply parking brake': 'Apply Parking Brake';
		action 'sense temperature': 'Sense Temperature';
		
		state 'vehicle states': 'Vehicle States' parallel {
			/*
			 * This is a usage of the state definition 'Vehicle States'.
			 * Note that it depends specifically on on the part 'vehicle1_c1'.
			 */
		
			ref vehicle : VehicleA;

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
					 * The notation "new 'Start Signal'()" constructs a specific instance of the
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
  (document "5_state_based_behavior_1.md"
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
        (range (start 12 3) (end 12 51))
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
        (range (start 44 17) (end 44 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 2) (end 51 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 2) (end 52 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 2) (end 53 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 2) (end 55 2782))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 61 17) (end 61 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 129 29) (end 129 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 166 2) (end 166 383))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 188 20) (end 188 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 190 4) (end 190 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 197 3) (end 197 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 197 38) (end 197 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 198 3) (end 198 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 198 30) (end 198 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 199 3) (end 199 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 199 19) (end 199 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 210 32) (end 210 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 229 27) (end 229 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 230 36) (end 230 74))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package '5-State-based Behavior-1' {
    private import ScalarValues::*;
    private import ISQ::*;
    private import '3a-Function-based Behavior-1'::*;

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

        /*
		 * These actions are used enabled in the state usage 
		 * 'vehicle states', in addition to 'provide power'.
		 */

        action 'perform self test': 'Perform Self Test';
        action 'apply parking brake': 'Apply Parking Brake';
        action 'sense temperature': 'Sense Temperature';

        state 'vehicle states': 'Vehicle States' parallel {
            /*
			 * This is a usage of the state definition 'Vehicle States'.
			 * Note that it depends specifically on on the part 'vehicle1_c1'.
			 */

            ref vehicle : VehicleA;

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
					 * The notation "new 'Start Signal'()" constructs a specific instance of the
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "0c58abddc5ffa27d5d50620b22507ac2578bf537d36506c9ea8e421f5a5f4d60") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1"))) (kind "package") (name "5-State-based Behavior-1") (declared-name "5-State-based Behavior-1") (range (start (line 0) (character 0)) (end (line 0) (character 6080))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 23))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 19))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 50))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "3a-Function-based Behavior-1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 46))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 5) (character 1)) (end (line 5) (character 1048))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Apply Parking Brake"))) (kind "action def") (name "Apply Parking Brake") (declared-name "Apply Parking Brake") (range (start (line 30) (character 2)) (end (line 30) (character 35))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Controller States"))) (kind "state def") (name "Controller States") (declared-name "Controller States") (range (start (line 27) (character 2)) (end (line 27) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Off Signal"))) (kind "attribute def") (name "Off Signal") (declared-name "Off Signal") (range (start (line 38) (character 2)) (end (line 38) (character 29))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Over Temp"))) (kind "attribute def") (name "Over Temp") (declared-name "Over Temp") (range (start (line 39) (character 2)) (end (line 39) (character 28))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Perform Self Test"))) (kind "action def") (name "Perform Self Test") (declared-name "Perform Self Test") (range (start (line 29) (character 2)) (end (line 29) (character 33))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Return to Normal"))) (kind "attribute def") (name "Return to Normal") (declared-name "Return to Normal") (range (start (line 40) (character 2)) (end (line 40) (character 35))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Sense Temperature"))) (kind "action def") (name "Sense Temperature") (declared-name "Sense Temperature") (range (start (line 31) (character 2)) (end (line 31) (character 64))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Sense Temperature::temp"))) (kind "in out parameter") (name "temp") (declared-name "temp") (range (start (line 31) (character 35)) (end (line 31) (character 62))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Sense Temperature"))) (authored (relationships (typing (reference "TemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Start Signal"))) (kind "attribute def") (name "Start Signal") (declared-name "Start Signal") (range (start (line 37) (character 2)) (end (line 37) (character 31))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle Off Signal"))) (kind "attribute def") (name "Vehicle Off Signal") (declared-name "Vehicle Off Signal") (range (start (line 35) (character 2)) (end (line 35) (character 37))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle On Signal"))) (kind "attribute def") (name "Vehicle On Signal") (declared-name "Vehicle On Signal") (range (start (line 34) (character 2)) (end (line 34) (character 36))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle Start Signal"))) (kind "attribute def") (name "Vehicle Start Signal") (declared-name "Vehicle Start Signal") (range (start (line 33) (character 2)) (end (line 33) (character 39))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle States"))) (kind "state def") (name "Vehicle States") (declared-name "Vehicle States") (range (start (line 26) (character 2)) (end (line 26) (character 29))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA"))) (kind "part def") (name "VehicleA") (declared-name "VehicleA") (range (start (line 6) (character 2)) (end (line 6) (character 315))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))) (authored (membership (kind Owning)) (relationships (perform (reference "5-State-based Behavior-1::Definitions::VehicleA::provide power") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (range (start (line 12) (character 3)) (end (line 12) (character 51))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA"))) (authored (relationships (typing (reference "Provide Power") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA::vehicle states"))) (kind "exhibit state") (name "vehicle states") (declared-name "vehicle states") (range (start (line 13) (character 3)) (end (line 13) (character 52))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA"))) (authored (relationships (typing (reference "Vehicle States") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleController"))) (kind "part def") (name "VehicleController") (declared-name "VehicleController") (range (start (line 16) (character 2)) (end (line 16) (character 93))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleController::controller states"))) (kind "exhibit state") (name "controller states") (declared-name "controller states") (range (start (line 17) (character 3)) (end (line 17) (character 58))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleController"))) (authored (relationships (typing (reference "Controller States") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 43) (character 1)) (end (line 43) (character 4878))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 44) (character 2)) (end (line 44) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 44) (character 17)) (end (line 44) (character 28))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::apply parking brake"))) (kind "action") (name "apply parking brake") (declared-name "apply parking brake") (range (start (line 52) (character 2)) (end (line 52) (character 54))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Apply Parking Brake") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states"))) (kind "state") (name "controller states") (declared-name "controller states") (range (start (line 166) (character 2)) (end (line 166) (character 383))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Controller States") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states"))) (kind "state") (name "operational controller states") (declared-name "operational controller states") (range (start (line 167) (character 3)) (end (line 167) (character 319))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1::Usages::controller states::operational controller states::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 168) (character 4)) (end (line 168) (character 25))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::initial"))) (kind "transition") (name "initial") (declared-name "initial") (range (start (line 170) (character 4)) (end (line 170) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off"))) (kind "state") (name "off") (declared-name "off") (range (start (line 172) (character 4)) (end (line 172) (character 14))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1::Usages::controller states::operational controller states::on") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))) (kind "transition") (name "off-on") (declared-name "off-on") (range (start (line 174) (character 4)) (end (line 174) (character 79))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 174) (character 4)) (end (line 174) (character 79))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on"))) (kind "state") (name "on") (declared-name "on") (range (start (line 179) (character 4)) (end (line 179) (character 13))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1::Usages::controller states::operational controller states::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))) (kind "transition") (name "on-off") (declared-name "on-off") (range (start (line 181) (character 4)) (end (line 181) (character 77))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 181) (character 4)) (end (line 181) (character 77))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::perform self test"))) (kind "action") (name "perform self test") (declared-name "perform self test") (range (start (line 51) (character 2)) (end (line 51) (character 50))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Perform Self Test") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::sense temperature"))) (kind "action") (name "sense temperature") (declared-name "sense temperature") (range (start (line 53) (character 2)) (end (line 53) (character 50))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Sense Temperature") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states"))) (kind "state") (name "vehicle states") (declared-name "vehicle states") (range (start (line 55) (character 2)) (end (line 55) (character 2782))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle States") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))) (kind "state") (name "health states") (declared-name "health states") (range (start (line 122) (character 3)) (end (line 122) (character 994))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1::Usages::vehicle states::health states::normal") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::_do"))) (kind "action") (name "do") (declared-name "do") (range (start (line 129) (character 4)) (end (line 129) (character 132))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::_do::temp"))) (kind "in out parameter") (name "temp") (declared-name "temp") (range (start (line 129) (character 29)) (end (line 129) (character 38))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::_do"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 128) (character 4)) (end (line 128) (character 25))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded"))) (kind "state") (name "degraded") (declared-name "degraded") (range (start (line 157) (character 4)) (end (line 157) (character 19))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1::Usages::vehicle states::health states::normal") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))) (kind "transition") (name "degraded-normal") (declared-name "degraded-normal") (range (start (line 159) (character 4)) (end (line 159) (character 101))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 159) (character 4)) (end (line 159) (character 101))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::initial"))) (kind "transition") (name "initial") (declared-name "initial") (range (start (line 135) (character 4)) (end (line 135) (character 35))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance"))) (kind "state") (name "maintenance") (declared-name "maintenance") (range (start (line 150) (character 4)) (end (line 150) (character 22))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1::Usages::vehicle states::health states::normal") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))) (kind "transition") (name "maintenance-normal") (declared-name "maintenance-normal") (range (start (line 152) (character 4)) (end (line 152) (character 107))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 152) (character 4)) (end (line 152) (character 107))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (kind "state") (name "normal") (declared-name "normal") (range (start (line 137) (character 4)) (end (line 137) (character 17))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance") (range none)) (transition (reference "5-State-based Behavior-1::Usages::vehicle states::health states::degraded") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-degraded"))) (kind "transition") (name "normal-degraded") (declared-name "normal-degraded") (range (start (line 144) (character 4)) (end (line 144) (character 196))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-degraded::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (range (start (line 144) (character 4)) (end (line 144) (character 196))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-degraded"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-degraded::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 144) (character 4)) (end (line 144) (character 196))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-degraded"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-maintenance"))) (kind "transition") (name "normal-maintenance") (declared-name "normal-maintenance") (range (start (line 139) (character 4)) (end (line 139) (character 119))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-maintenance::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 139) (character 4)) (end (line 139) (character 119))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-maintenance"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states"))) (kind "state") (name "operational states") (declared-name "operational states") (range (start (line 63) (character 3)) (end (line 63) (character 1547))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1::Usages::vehicle states::operational states::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::_documentation"))) (kind "documentation") (name "") (range (start (line 63) (character 3)) (end (line 63) (character 1547))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 69) (character 4)) (end (line 69) (character 118))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::_entry::_documentation"))) (kind "documentation") (name "") (range (start (line 69) (character 4)) (end (line 69) (character 118))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::_entry"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::initial"))) (kind "transition") (name "initial") (declared-name "initial") (range (start (line 76) (character 4)) (end (line 76) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off"))) (kind "state") (name "off") (declared-name "off") (range (start (line 78) (character 4)) (end (line 78) (character 14))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1::Usages::vehicle states::operational states::starting") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (kind "transition") (name "off-starting") (declared-name "off-starting") (range (start (line 80) (character 4)) (end (line 80) (character 676))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (range (start (line 80) (character 4)) (end (line 80) (character 676))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting::guard"))) (kind "transition guard") (name "guard") (declared-name "guard") (range (start (line 83) (character 8)) (end (line 83) (character 43))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 80) (character 4)) (end (line 80) (character 676))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on"))) (kind "state") (name "on") (declared-name "on") (range (start (line 104) (character 4)) (end (line 104) (character 347))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1::Usages::vehicle states::operational states::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))) (kind "transition") (name "on-off") (declared-name "on-off") (range (start (line 116) (character 4)) (end (line 116) (character 85))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 116) (character 4)) (end (line 116) (character 85))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on::_do"))) (kind "action") (name "do") (declared-name "do") (range (start (line 112) (character 5)) (end (line 112) (character 24))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 111) (character 5)) (end (line 111) (character 31))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on::_exit"))) (kind "action") (name "exit") (declared-name "exit") (range (start (line 113) (character 5)) (end (line 113) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting"))) (kind "state") (name "starting") (declared-name "starting") (range (start (line 97) (character 4)) (end (line 97) (character 19))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1::Usages::vehicle states::operational states::on") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))) (kind "transition") (name "starting-on") (declared-name "starting-on") (range (start (line 99) (character 4)) (end (line 99) (character 94))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 99) (character 4)) (end (line 99) (character 94))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::vehicle"))) (kind "ref") (name "vehicle") (declared-name "vehicle") (range (start (line 61) (character 3)) (end (line 61) (character 26))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleA") (range (start (line 61) (character 17)) (end (line 61) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (range (start (line 188) (character 2)) (end (line 188) (character 1362))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleA") (range (start (line 188) (character 20)) (end (line 188) (character 28)))) (perform (reference "5-State-based Behavior-1::Usages::vehicle1_c1::provide power") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::Tmax"))) (kind "attribute") (name "Tmax") (declared-name "Tmax") (range (start (line 199) (character 3)) (end (line 199) (character 36))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "TemperatureValue") (range none)) (typing (reference "TemperatureValue") (range (start (line 199) (character 19)) (end (line 199) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::brake pedal depressed"))) (kind "attribute") (name "brake pedal depressed") (declared-name "brake pedal depressed") (range (start (line 197) (character 3)) (end (line 197) (character 46))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)) (typing (reference "Boolean") (range (start (line 197) (character 38)) (end (line 197) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (range (start (line 189) (character 3)) (end (line 189) (character 51))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (range (start (line 190) (character 4)) (end (line 190) (character 24))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::fuelCmdPort"))) (authored (relationships (typing (reference "FuelCmd") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::maintenanceTime"))) (kind "attribute") (name "maintenanceTime") (declared-name "maintenanceTime") (range (start (line 198) (character 3)) (end (line 198) (character 45))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "DateTime") (range none)) (typing (reference "Time::DateTime") (range (start (line 198) (character 30)) (end (line 198) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (range (start (line 201) (character 3)) (end (line 201) (character 278))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::vehicle states"))) (kind "state") (name "vehicle states") (declared-name "vehicle states") (range (start (line 210) (character 3)) (end (line 210) (character 257))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "VehicleA::vehicle states") (range (start (line 210) (character 32)) (end (line 210) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::vehicleController"))) (kind "part") (name "vehicleController") (declared-name "vehicleController") (range (start (line 229) (character 3)) (end (line 229) (character 127))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleController") (range (start (line 229) (character 27)) (end (line 229) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::vehicleController::controller states"))) (kind "state") (name "controller states") (declared-name "controller states") (range (start (line 230) (character 4)) (end (line 230) (character 75))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::vehicleController"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "VehicleController::controller states") (range (start (line 230) (character 36)) (end (line 230) (character 74)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 2) (character 16)) (end (line 2) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "3a-Function-based Behavior-1::*") (range (start (line 3) (character 16)) (end (line 3) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Sense Temperature::temp"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA"))) (kind performSource) (ordinal 0)) (authored-target "5-State-based Behavior-1::Definitions::VehicleA::provide power") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA::provide power"))) (kind featureTyping) (ordinal 0)) (authored-target "Provide Power") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA::vehicle states"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle States") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle States")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleController::controller states"))) (kind featureTyping) (ordinal 0)) (authored-target "Controller States") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Controller States")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 44) (character 17)) (end (line 44) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::apply parking brake"))) (kind featureTyping) (ordinal 0)) (authored-target "Apply Parking Brake") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states"))) (kind featureTyping) (ordinal 0)) (authored-target "Controller States") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1::Usages::controller states::operational controller states::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1::Usages::controller states::operational controller states::on") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1::Usages::controller states::operational controller states::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::perform self test"))) (kind featureTyping) (ordinal 0)) (authored-target "Perform Self Test") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::sense temperature"))) (kind featureTyping) (ordinal 0)) (authored-target "Sense Temperature") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle States") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1::Usages::vehicle states::health states::normal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::_do::temp"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1::Usages::vehicle states::health states::normal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1::Usages::vehicle states::health states::normal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (kind transitionSource) (ordinal 1)) (authored-target "5-State-based Behavior-1::Usages::vehicle states::health states::degraded") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1::Usages::vehicle states::operational states::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1::Usages::vehicle states::operational states::starting") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1::Usages::vehicle states::operational states::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1::Usages::vehicle states::operational states::on") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleA") (range (start (line 61) (character 17)) (end (line 61) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleA") (range (start (line 188) (character 20)) (end (line 188) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (kind performSource) (ordinal 0)) (authored-target "5-State-based Behavior-1::Usages::vehicle1_c1::provide power") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 1)) (authored-target "TemperatureValue") (range (start (line 199) (character 19)) (end (line 199) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 1)) (authored-target "Boolean") (range (start (line 197) (character 38)) (end (line 197) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 0)) (authored-target "DateTime") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 1)) (authored-target "Time::DateTime") (range (start (line 198) (character 30)) (end (line 198) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::vehicle states"))) (kind redefinition) (ordinal 0)) (authored-target "VehicleA::vehicle states") (range (start (line 210) (character 32)) (end (line 210) (character 58))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleController") (range (start (line 229) (character 27)) (end (line 229) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::vehicleController::controller states"))) (kind redefinition) (ordinal 0)) (authored-target "VehicleController::controller states") (range (start (line 230) (character 36)) (end (line 230) (character 74))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA::provide power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA::vehicle states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle States"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA::vehicle states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleController::controller states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::Controller States"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Definitions::VehicleController::controller states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::provide power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (kind performSource) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting::guard")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
