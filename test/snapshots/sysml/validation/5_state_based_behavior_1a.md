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
  (document "memory://snapshot/5_state_based_behavior_1a.md"
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
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 11 3) (end 11 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 12 3) (end 12 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 16 3) (end 16 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 31 35) (end 31 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 70 17) (end 70 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 77 4) (end 77 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 81 4) (end 81 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 100 4) (end 100 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 117 4) (end 117 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 129 17) (end 129 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 136 4) (end 136 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 140 4) (end 140 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 145 4) (end 145 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 153 4) (end 153 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 160 4) (end 160 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 169 17) (end 169 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 171 4) (end 171 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 175 4) (end 175 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 182 4) (end 182 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 191 4) (end 191 24))
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
        (range (start 199 30) (end 199 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 200 19) (end 200 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 202 3) (end 210 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 212 32) (end 212 58))
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
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:81cf55dbeef467827e92432b8fd1fe256fc65aefc2e6e98f1f1356f9f25558f6") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Apply Parking Brake"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::FuelCmd"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Off Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Over Temp"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Perform Self Test"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Provide Power"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Return to Normal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Start Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle Off Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle On Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle Start Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::apply parking brake"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Apply Parking Brake"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::controller states"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Controller States"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "initial"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::perform self test"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Perform Self Test"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::provide power"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Provide Power"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::sense temperature"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sense Temperature"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle States"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "initial"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "sense temperature"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "initial"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "perform self test"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "provide power"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exit-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (exitActionBinding (reference "apply parking brake"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleA"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind state) (ordinal 0))))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "VehicleA::vehicle states"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::Tmax"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TemperatureValue"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::brake pedal depressed"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::fuelCmdPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::maintenanceTime"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Time::DateTime"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleController"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind state) (ordinal 0))))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "VehicleController::controller states"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::apply parking brake"))) (kind featureTyping) (ordinal 0))
      (authored-target "Apply Parking Brake")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Apply Parking Brake")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::controller states"))) (kind featureTyping) (ordinal 0))
      (authored-target "Controller States")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "initial")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::perform self test"))) (kind featureTyping) (ordinal 0))
      (authored-target "Perform Self Test")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Perform Self Test")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::provide power"))) (kind featureTyping) (ordinal 0))
      (authored-target "Provide Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Provide Power")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::sense temperature"))) (kind featureTyping) (ordinal 0))
      (authored-target "Sense Temperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle States")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "initial")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "sense temperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::sense temperature")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "initial")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "perform self test")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::perform self test")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "provide power")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::provide power")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0))
      (authored-target "apply parking brake")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::apply parking brake")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleA")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind state) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "VehicleA::vehicle states")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 0))
      (authored-target "TemperatureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 0))
      (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind state) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "VehicleController::controller states")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::apply parking brake"))) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Apply Parking Brake"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::apply parking brake"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::controller states"))) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::controller states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::perform self test"))) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Perform Self Test"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::perform self test"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::provide power"))) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Provide Power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::provide power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::sense temperature"))) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::sense temperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind doActionBinding) (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind do-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::sense temperature"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0)))
    (relationship (kind entryActionBinding) (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::perform self test"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0)))
    (relationship (kind doActionBinding) (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind do-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::provide power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0)))
    (relationship (kind exitActionBinding) (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::apply parking brake"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController"))) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 2 16) (end 2 22)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 46 17) (end 46 31)) (probe (position 46 17))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 55 32) (end 55 53)) (probe (position 55 32))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::apply parking brake"))) (kind featureTyping) (ordinal 0) (authored-target "Apply Parking Brake")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Apply Parking Brake")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 167 29) (end 167 48)) (probe (position 167 29))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::controller states"))) (kind featureTyping) (ordinal 0) (authored-target "Controller States")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 169 17) (end 169 24)) (probe (position 169 17))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "initial")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 54 30) (end 54 49)) (probe (position 54 30))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::perform self test"))) (kind featureTyping) (ordinal 0) (authored-target "Perform Self Test")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Perform Self Test")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 53 26) (end 53 41)) (probe (position 53 26))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::provide power"))) (kind featureTyping) (ordinal 0) (authored-target "Provide Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Provide Power")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 56 30) (end 56 49)) (probe (position 56 30))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::sense temperature"))) (kind featureTyping) (ordinal 0) (authored-target "Sense Temperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 58 26) (end 58 42)) (probe (position 58 26))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle States")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 129 17) (end 129 24)) (probe (position 129 17))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "initial")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 130 7) (end 130 26)) (probe (position 130 7))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "sense temperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::sense temperature")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 70 17) (end 70 24)) (probe (position 70 17))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "initial")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 112 11) (end 112 30)) (probe (position 112 11))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "perform self test")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::perform self test")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 113 8) (end 113 23)) (probe (position 113 8))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "provide power")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::provide power")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 114 10) (end 114 31)) (probe (position 114 10))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0) (authored-target "apply parking brake")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::apply parking brake")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 189 20) (end 189 28)) (probe (position 189 20))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleA")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 212 32) (end 212 58)) (probe (position 212 32))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind state) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "VehicleA::vehicle states")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 200 19) (end 200 35)) (probe (position 200 19))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 0) (authored-target "TemperatureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 198 38) (end 198 45)) (probe (position 198 38))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 199 30) (end 199 44)) (probe (position 199 30))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 0) (authored-target "Time::DateTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 231 27) (end 231 44)) (probe (position 231 27))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1a.md") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1a.md") (range (start 232 36) (end 232 74)) (probe (position 232 36))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1a.md") (anonymous (kind state) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "VehicleController::controller states")
      (outcome (status unresolved)))
  )
)
~~~
