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
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 12 3) (end 12 51))
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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 26 2) (end 26 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 27 2) (end 27 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 29 2) (end 29 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 30 2) (end 30 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 31 2) (end 31 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 51 2) (end 51 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 52 2) (end 52 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 53 2) (end 53 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 55 2) (end 164 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 166 2) (end 186 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 190 4) (end 190 24))
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
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 201 3) (end 208 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 210 3) (end 216 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 230 4) (end 230 75))
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
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Off Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Over Temp"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Return to Normal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Start Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle Off Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle On Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::Vehicle Start Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleController"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleA"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::Tmax"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TemperatureValue"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::brake pedal depressed"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::fuelCmdPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::maintenanceTime"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Time::DateTime"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::vehicleController"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleController"))))
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
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleA")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 0))
      (authored-target "TemperatureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 0))
      (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleController")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)))
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
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 44 17) (end 44 31)) (probe (position 44 17))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 188 20) (end 188 28)) (probe (position 188 20))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleA")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Definitions::VehicleA")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 199 19) (end 199 35)) (probe (position 199 19))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 0) (authored-target "TemperatureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_1.md") (range (start 197 38) (end 197 45)) (probe (position 197 38))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_1.md") (qualified-name "5-State-based Behavior-1::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
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
)
~~~
