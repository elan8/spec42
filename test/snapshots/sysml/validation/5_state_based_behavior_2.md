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
  (document "memory://snapshot/5_state_based_behavior_2.md"
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
        (range (start 7 3) (end 7 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 8 3) (end 8 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 12 3) (end 12 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 20 35) (end 20 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 42 4) (end 42 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 42 11) (end 42 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 49 4) (end 49 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 55 4) (end 55 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 59 5) (end 59 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 60 5) (end 60 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 61 5) (end 61 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 63 4) (end 63 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 68 4) (end 68 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 68 11) (end 68 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 69 4) (end 69 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 76 4) (end 76 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 78 4) (end 78 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 83 4) (end 83 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 87 4) (end 87 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 94 4) (end 94 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 94 11) (end 94 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 97 4) (end 97 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 101 4) (end 101 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 108 4) (end 108 24))
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
        (range (start 112 30) (end 112 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 19) (end 113 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 115 3) (end 117 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 119 32) (end 119 58))
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
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:b1a801b7ba6e36db59716ec7a7128bd96dc2bb2cd703e6134f568f98b2d02621") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "3a-Function-based Behavior-1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Apply Parking Brake"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Controller States"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Off Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Over Temp"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Perform Self Test"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Return to Normal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Start Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle Off Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle On Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle Start Signal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::apply parking brake"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Apply Parking Brake"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::controller states"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Controller States"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::perform self test"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Perform Self Test"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::sense temperature"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sense Temperature"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle States"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::degraded"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::maintenance"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::starting"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleA"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind state) (ordinal 0))))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "VehicleA::vehicle states"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::Tmax"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TemperatureValue"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::brake pedal depressed"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::fuelCmdPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::maintenanceTime"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Time::DateTime"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleController"))))
    (declaration (id (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind state) (ordinal 0))))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "VehicleController::controller states"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "3a-Function-based Behavior-1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::apply parking brake"))) (kind featureTyping) (ordinal 0))
      (authored-target "Apply Parking Brake")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Apply Parking Brake")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::controller states"))) (kind featureTyping) (ordinal 0))
      (authored-target "Controller States")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Controller States")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::perform self test"))) (kind featureTyping) (ordinal 0))
      (authored-target "Perform Self Test")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Perform Self Test")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::sense temperature"))) (kind featureTyping) (ordinal 0))
      (authored-target "Sense Temperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle States")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleA")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind state) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "VehicleA::vehicle states")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 0))
      (authored-target "TemperatureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 0))
      (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController")))))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind state) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "VehicleController::controller states")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::apply parking brake"))) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Apply Parking Brake"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::apply parking brake"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::controller states"))) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Controller States"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::controller states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::perform self test"))) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Perform Self Test"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::perform self test"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::sense temperature"))) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::sense temperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 2 16) (end 2 22)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 3 16) (end 3 49)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "3a-Function-based Behavior-1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 33 17) (end 33 31)) (probe (position 33 17))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 36 32) (end 36 53)) (probe (position 36 32))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::apply parking brake"))) (kind featureTyping) (ordinal 0) (authored-target "Apply Parking Brake")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Apply Parking Brake")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 92 29) (end 92 48)) (probe (position 92 29))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::controller states"))) (kind featureTyping) (ordinal 0) (authored-target "Controller States")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Controller States")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 35 30) (end 35 49)) (probe (position 35 30))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::perform self test"))) (kind featureTyping) (ordinal 0) (authored-target "Perform Self Test")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Perform Self Test")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 37 30) (end 37 49)) (probe (position 37 30))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::sense temperature"))) (kind featureTyping) (ordinal 0) (authored-target "Sense Temperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 39 26) (end 39 42)) (probe (position 39 26))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle States")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 106 20) (end 106 28)) (probe (position 106 20))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleA")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 119 32) (end 119 58)) (probe (position 119 32))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind state) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "VehicleA::vehicle states")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 113 19) (end 113 35)) (probe (position 113 19))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 0) (authored-target "TemperatureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 111 38) (end 111 45)) (probe (position 111 38))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 112 30) (end 112 44)) (probe (position 112 30))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 0) (authored-target "Time::DateTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 121 27) (end 121 44)) (probe (position 121 27))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/5_state_based_behavior_2.md") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController")))))
  )
  (query (document "memory://snapshot/5_state_based_behavior_2.md") (range (start 122 36) (end 122 74)) (probe (position 122 36))
    (reference (id (source (node (document "memory://snapshot/5_state_based_behavior_2.md") (anonymous (kind state) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "VehicleController::controller states")
      (outcome (status unresolved)))
  )
)
~~~
