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
  (document "memory://snapshot/5_state_based_behavior_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 35) (end 12 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 13 3) (end 13 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 17 3) (end 17 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 45) (end 31 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 80 4) (end 95 6))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 83 8) (end 83 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 112 8) (end 112 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 139 4) (end 142 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 144 4) (end 148 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 144 4) (end 148 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 190 16) (end 190 23))
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
        (range (start 198 30) (end 198 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 199 19) (end 199 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 207 4) (end 207 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 210 32) (end 210 58))
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
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:bd4e70d8d29348added27f11a9dd4c31d219e92e5cf74856b03357d5ec1b1104") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "3a-Function-based Behavior-1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Apply Parking Brake"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Controller States"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Off Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Over Temp"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Perform Self Test"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Return to Normal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Sense Temperature"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Sense Temperature::temp"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TemperatureValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Start Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle Off Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle On Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle Start Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle States"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA::provide power"))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Provide Power"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleController"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::apply parking brake"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Apply Parking Brake"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Controller States"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "initial"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::initial"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "off"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "off")) (transitionTarget (reference "on")) (transitionTrigger (reference "Start Signal"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "on")) (transitionTarget (reference "off")) (transitionTrigger (reference "Off Signal"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::perform self test"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Perform Self Test"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::sense temperature"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sense Temperature"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle States"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "initial"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "sense temperature"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "degraded")) (transitionTarget (reference "normal")) (transitionTrigger (reference "Return to Normal"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::initial"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "normal"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "maintenance")) (transitionTarget (reference "normal")) (transitionTrigger (reference "Return to Normal"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-degraded"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "normal")) (transitionTarget (reference "degraded"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-maintenance"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "normal")) (transitionTarget (reference "maintenance"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "initial"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::initial"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "off"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "off")) (transitionTarget (reference "starting")) (transitionTrigger (reference "Vehicle Start Signal")) (memberAccessOperand (reference "vehicle1_c1::brake pedal depressed"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "on")) (transitionTarget (reference "off")) (transitionTrigger (reference "Vehicle Off Signal"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "perform self test"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "provide power"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exit-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (exitActionBinding (reference "apply parking brake"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "starting")) (transitionTarget (reference "on")) (transitionTrigger (reference "Vehicle On Signal"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::vehicle"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleA"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleA"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "VehicleA::provide power"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind state) (ordinal 0))))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "VehicleA::vehicle states"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::Tmax"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TemperatureValue"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::brake pedal depressed"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::fuelCmdPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmd") (direction in))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::maintenanceTime"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Time::DateTime"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::vehicleController"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleController"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind state) (ordinal 0))))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "VehicleController::controller states"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "3a-Function-based Behavior-1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Sense Temperature::temp"))) (kind featureTyping) (ordinal 0))
      (authored-target "TemperatureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA::provide power"))) (kind featureTyping) (ordinal 0))
      (authored-target "Provide Power")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::apply parking brake"))) (kind featureTyping) (ordinal 0))
      (authored-target "Apply Parking Brake")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Apply Parking Brake")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states"))) (kind featureTyping) (ordinal 0))
      (authored-target "Controller States")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Controller States")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "initial")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::initial")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::initial"))) (kind transitionTarget) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))) (kind transitionSource) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))) (kind transitionTarget) (ordinal 0))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Start Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Start Signal")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))) (kind transitionSource) (ordinal 0))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))) (kind transitionTarget) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Off Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Off Signal")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::perform self test"))) (kind featureTyping) (ordinal 0))
      (authored-target "Perform Self Test")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Perform Self Test")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::sense temperature"))) (kind featureTyping) (ordinal 0))
      (authored-target "Sense Temperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Sense Temperature")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle States")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle States")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "initial")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::initial")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "sense temperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::sense temperature")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))) (kind transitionSource) (ordinal 0))
      (authored-target "degraded")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))) (kind transitionTarget) (ordinal 0))
      (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Return to Normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Return to Normal")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::initial"))) (kind transitionTarget) (ordinal 0))
      (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))) (kind transitionSource) (ordinal 0))
      (authored-target "maintenance")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))) (kind transitionTarget) (ordinal 0))
      (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Return to Normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Return to Normal")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-degraded"))) (kind transitionSource) (ordinal 0))
      (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-degraded"))) (kind transitionTarget) (ordinal 0))
      (authored-target "degraded")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-maintenance"))) (kind transitionSource) (ordinal 0))
      (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-maintenance"))) (kind transitionTarget) (ordinal 0))
      (authored-target "maintenance")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "initial")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::initial")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::initial"))) (kind transitionTarget) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (kind transitionSource) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (kind transitionTarget) (ordinal 0))
      (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Vehicle Start Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle Start Signal")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle1_c1::brake pedal depressed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))) (kind transitionSource) (ordinal 0))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))) (kind transitionTarget) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Vehicle Off Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle Off Signal")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "perform self test")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::perform self test")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "provide power")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0))
      (authored-target "apply parking brake")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::apply parking brake")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))) (kind transitionSource) (ordinal 0))
      (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))) (kind transitionTarget) (ordinal 0))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Vehicle On Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle On Signal")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleA")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleA")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind perform-action) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "VehicleA::provide power")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA::provide power")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind state) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "VehicleA::vehicle states")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 0))
      (authored-target "TemperatureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmd")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 0))
      (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleController")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind state) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "VehicleController::controller states")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::apply parking brake"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Apply Parking Brake"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::apply parking brake"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Controller States"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind entryActionBinding) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::initial"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::initial"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::initial"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Start Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Off Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::perform self test"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Perform Self Test"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::perform self test"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::sense temperature"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Sense Temperature"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::sense temperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle States"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind entryActionBinding) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::initial"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0)))
    (relationship (kind doActionBinding) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind do-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::sense temperature"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Return to Normal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::initial"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::initial"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Return to Normal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-degraded"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-degraded"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-degraded"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-degraded"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-maintenance"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-maintenance"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-maintenance"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-maintenance"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind entryActionBinding) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::initial"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::initial"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::initial"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle Start Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle Off Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind entryActionBinding) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::perform self test"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0)))
    (relationship (kind exitActionBinding) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::apply parking brake"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle On Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::vehicle"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA::provide power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind perform-action) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::vehicleController"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleController"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 2 16) (end 2 22)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 3 16) (end 3 49)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "3a-Function-based Behavior-1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 31 45) (end 31 61)) (probe (position 31 45))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Sense Temperature::temp"))) (kind featureTyping) (ordinal 0) (authored-target "TemperatureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 12 35) (end 12 50)) (probe (position 12 35))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA::provide power"))) (kind featureTyping) (ordinal 0) (authored-target "Provide Power")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 44 17) (end 44 31)) (probe (position 44 17))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 52 32) (end 52 53)) (probe (position 52 32))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::apply parking brake"))) (kind featureTyping) (ordinal 0) (authored-target "Apply Parking Brake")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Apply Parking Brake")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 166 29) (end 166 48)) (probe (position 166 29))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states"))) (kind featureTyping) (ordinal 0) (authored-target "Controller States")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Controller States")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 168 17) (end 168 24)) (probe (position 168 17))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "initial")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::initial")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 170 28) (end 170 31)) (probe (position 170 28))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::initial"))) (kind transitionTarget) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 175 11) (end 175 14)) (probe (position 175 11))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))) (kind transitionSource) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 177 10) (end 177 12)) (probe (position 177 10))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))) (kind transitionTarget) (ordinal 0) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 176 12) (end 176 26)) (probe (position 176 12))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off-on"))) (kind transitionTrigger) (ordinal 0) (authored-target "Start Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Start Signal")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 182 11) (end 182 13)) (probe (position 182 11))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))) (kind transitionSource) (ordinal 0) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 184 10) (end 184 13)) (probe (position 184 10))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))) (kind transitionTarget) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::off")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 183 12) (end 183 24)) (probe (position 183 12))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::controller states::operational controller states::on-off"))) (kind transitionTrigger) (ordinal 0) (authored-target "Off Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Off Signal")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 51 30) (end 51 49)) (probe (position 51 30))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::perform self test"))) (kind featureTyping) (ordinal 0) (authored-target "Perform Self Test")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Perform Self Test")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 53 30) (end 53 49)) (probe (position 53 30))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::sense temperature"))) (kind featureTyping) (ordinal 0) (authored-target "Sense Temperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Sense Temperature")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 55 26) (end 55 42)) (probe (position 55 26))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle States")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle States")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 128 17) (end 128 24)) (probe (position 128 17))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "initial")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::initial")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 129 7) (end 129 26)) (probe (position 129 7))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "sense temperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::sense temperature")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 160 11) (end 160 19)) (probe (position 160 11))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))) (kind transitionSource) (ordinal 0) (authored-target "degraded")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 162 10) (end 162 16)) (probe (position 162 10))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))) (kind transitionTarget) (ordinal 0) (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 161 12) (end 161 30)) (probe (position 161 12))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded-normal"))) (kind transitionTrigger) (ordinal 0) (authored-target "Return to Normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Return to Normal")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 135 28) (end 135 34)) (probe (position 135 28))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::initial"))) (kind transitionTarget) (ordinal 0) (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 153 11) (end 153 22)) (probe (position 153 11))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))) (kind transitionSource) (ordinal 0) (authored-target "maintenance")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 155 10) (end 155 16)) (probe (position 155 10))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))) (kind transitionTarget) (ordinal 0) (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 154 12) (end 154 30)) (probe (position 154 12))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance-normal"))) (kind transitionTrigger) (ordinal 0) (authored-target "Return to Normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Return to Normal")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 145 11) (end 145 17)) (probe (position 145 11))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-degraded"))) (kind transitionSource) (ordinal 0) (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 148 10) (end 148 18)) (probe (position 148 10))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-degraded"))) (kind transitionTarget) (ordinal 0) (authored-target "degraded")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::degraded")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 140 11) (end 140 17)) (probe (position 140 11))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-maintenance"))) (kind transitionSource) (ordinal 0) (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 142 10) (end 142 21)) (probe (position 142 10))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::normal-maintenance"))) (kind transitionTarget) (ordinal 0) (authored-target "maintenance")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::health states::maintenance")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 69 17) (end 69 24)) (probe (position 69 17))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "initial")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::initial")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 76 28) (end 76 31)) (probe (position 76 28))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::initial"))) (kind transitionTarget) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 81 11) (end 81 14)) (probe (position 81 11))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (kind transitionSource) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 85 10) (end 85 18)) (probe (position 85 10))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (kind transitionTarget) (ordinal 0) (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 82 12) (end 82 34)) (probe (position 82 12))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (kind transitionTrigger) (ordinal 0) (authored-target "Vehicle Start Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle Start Signal")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 83 8) (end 83 43)) (probe (position 83 8))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off-starting"))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle1_c1::brake pedal depressed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 117 11) (end 117 13)) (probe (position 117 11))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))) (kind transitionSource) (ordinal 0) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 119 10) (end 119 13)) (probe (position 119 10))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))) (kind transitionTarget) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::off")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 118 12) (end 118 32)) (probe (position 118 12))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on-off"))) (kind transitionTrigger) (ordinal 0) (authored-target "Vehicle Off Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle Off Signal")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 111 11) (end 111 30)) (probe (position 111 11))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "perform self test")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::perform self test")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 112 8) (end 112 23)) (probe (position 112 8))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "provide power")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 113 10) (end 113 31)) (probe (position 113 10))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0) (authored-target "apply parking brake")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::apply parking brake")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 100 11) (end 100 19)) (probe (position 100 11))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))) (kind transitionSource) (ordinal 0) (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 102 10) (end 102 12)) (probe (position 102 10))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))) (kind transitionTarget) (ordinal 0) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::on")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 101 12) (end 101 31)) (probe (position 101 12))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::operational states::starting-on"))) (kind transitionTrigger) (ordinal 0) (authored-target "Vehicle On Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle On Signal")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 61 17) (end 61 25)) (probe (position 61 17))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle states::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleA")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 188 20) (end 188 28)) (probe (position 188 20))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleA")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 201 31) (end 201 56)) (probe (position 201 31))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind perform-action) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "VehicleA::provide power")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA::provide power")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 210 32) (end 210 58)) (probe (position 210 32))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind state) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "VehicleA::vehicle states")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 199 19) (end 199 35)) (probe (position 199 19))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 0) (authored-target "TemperatureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 197 38) (end 197 45)) (probe (position 197 38))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 190 16) (end 190 23)) (probe (position 190 16))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmd")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 198 30) (end 198 44)) (probe (position 198 30))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 0) (authored-target "Time::DateTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 229 27) (end 229 44)) (probe (position 229 27))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleController")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 230 36) (end 230 74)) (probe (position 230 36))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind state) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "VehicleController::controller states")
      (outcome (status unresolved)))
  )
)
~~~
