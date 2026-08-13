# META
~~~ini
description=SysML Validation (03-Function-based Behavior): 3a-Function-based Behavior-1
type=file
~~~
# SOURCE
~~~sysml
package '3a-Function-based Behavior-1' {
	public import Definitions::*;
	public import Usages::*;

	package Definitions {
		alias Torque for ISQ::TorqueValue {
			/*
			 * The 'TorqueValue' type is aliased as 'Torque'.
			 */
		}
		
		attribute def FuelCmd;
		
		/*
		 * There is no special construct for modeling "signals". Data to be
		 * transmitted asynchronously can simply be modeled using attribute defs.
		 */
		
		attribute def EngineStart;
		attribute def EngineOff;
		
		/*
		 * Black box definitions for actions include their inputs and outputs.
		 */
		
		action def 'Generate Torque' { in fuelCmd: FuelCmd; out engineTorque: Torque; }
		action def 'Amplify Torque' { in engineTorque: Torque; out transmissionTorque: Torque; }
		action def 'Transfer Torque' { in transmissionTorque: Torque; out driveshaftTorque: Torque; }
		action def 'Distribute Torque' { in driveShaftTorque: Torque; out wheelTorque1: Torque; out wheelTorque2: Torque; }
		
		action def 'Provide Power' { in fuelCmd: FuelCmd; out wheelTorque1: Torque; out wheelTorque2: Torque; }
	
	}
	
	package Usages {
	
		action 'provide power': 'Provide Power'{
			in fuelCmd: FuelCmd;
			out wheelTorque1: Torque; 
			out wheelTorque2: Torque;
		
			// ITEM FLOW PART
			
			bind 'generate torque'.fuelCmd = fuelCmd {
				/*
				 * This is a binding connector, just as was used to
				 * model delegation between ports.
				 */
			}
			
			action 'generate torque': 'Generate Torque' {
				/*
				 * An action usage inherits parameters from its definition.
				 * They act as its "pins".
				 */
			}
			
			flow 'generate torque'.engineTorque 
			    to 'amplify torque'.engineTorque {
				/*
				 * A flow is a connection between two actions that streams items from
				 * an output parameter of one action to an input parameter of the other.
				 * Note that streaming is a property of the connection, not the
				 * actions or their parameters.
				 */
			}
			
			action 'amplify torque': 'Amplify Torque';
			
			flow 'amplify torque'.transmissionTorque 
			    to 'transfer torque'.transmissionTorque;
			
			action 'transfer torque': 'Transfer Torque';
			
			flow 'transfer torque'.driveshaftTorque 
			    to 'distribute torque'.driveShaftTorque;
			
			action 'distribute torque': 'Distribute Torque';
			
			bind wheelTorque1 = 'distribute torque'.wheelTorque1;
			bind wheelTorque2 = 'distribute torque'.wheelTorque2;
			
			// CONTROL FLOW PART

			first start then continue {
				/*
				 * A first is an assertion that one thing must occur
				 * before another, acting like a "control flow". 'start' is
				 * the start snapshot of the action, which acts like an
				 * "initial node".
				 */
			}
			
			merge continue {
				/*
				 * A merge node is necessary to prevent a loop of successions
				 * from being unsatisfiable.
				 */
			}
			first continue then engineStarted;
			
			action engineStarted accept engineStart: EngineStart {
				/*
				 * An accept action accepts an incoming transfer of some item
				 * from outside an action, in this case the "signal" 'EngineStart'.
				 * Note that 'engineStarted' is the name of the action, while
				 * 'engineStart' is the name of the received signal attribute.
				 */
			}			
			first engineStarted then engineStopped;
					
			action engineStopped accept engineOff: EngineOff;	
			first engineStopped then continue;
			
			/*
			 * These successions act to "enable" the torque-related actions.
			 * Each action on the right can only be performed following the
			 * completion of a performance of 'engineStarted'.
			 */
			first engineStarted then 'generate torque';
			first engineStarted then 'amplify torque';
			first engineStarted then 'transfer torque';
			first engineStarted then 'distribute torque';
			
			/*
			 * These successions act to "disable" the torque-related actions.
			 * The performance of the actions on the left cannot continue
			 * once there is a performance of 'engineStopped'.
			 */
			first 'generate torque' then engineStopped;
			first 'amplify torque' then engineStopped;		
			first 'transfer torque' then engineStopped;		
			first 'distribute torque' then engineStopped;		
		}
	
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/3a_function_based_behavior_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 19) (end 5 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 25 33) (end 25 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 25 54) (end 25 79))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 26 32) (end 26 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 26 57) (end 26 88))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 27 33) (end 27 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 27 64) (end 27 93))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 28 35) (end 28 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 28 64) (end 28 89))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 28 90) (end 28 115))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 30 31) (end 30 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 30 52) (end 30 77))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 30 78) (end 30 103))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 37 3) (end 37 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 38 3) (end 38 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 39 3) (end 39 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 43 3) (end 48 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 57 3) (end 65 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 69 3) (end 70 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 74 3) (end 75 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 79 3) (end 79 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 80 3) (end 80 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 84 3) (end 91 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 93 3) (end 98 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 99 3) (end 99 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 109 3) (end 109 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 112 3) (end 112 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 119 3) (end 119 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 120 3) (end 120 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 121 3) (end 121 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 122 3) (end 122 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 129 3) (end 129 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 130 3) (end 130 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 131 3) (end 131 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 132 3) (end 132 48))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:3b081ef8ba2e964be99eac92f6b641f30d9e18eb89acd1ed6e3d89b3a22fa8a3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Usages") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::EngineOff"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::EngineStart"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "ISQ::TorqueValue"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Provide Power"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Amplify Torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Distribute Torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Generate Torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transfer Torque"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (kind aliasBinding) (ordinal 0))
      (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind featureTyping) (ordinal 0))
      (authored-target "Provide Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Amplify Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Distribute Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Generate Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transfer Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 1 15) (end 1 29)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 2 15) (end 2 24)) (probe (position 2 15))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 5 19) (end 5 35)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (kind aliasBinding) (ordinal 0) (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 36 26) (end 36 41)) (probe (position 36 26))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind featureTyping) (ordinal 0) (authored-target "Provide Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 67 28) (end 67 44)) (probe (position 67 28))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0) (authored-target "Amplify Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 77 31) (end 77 50)) (probe (position 77 31))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0) (authored-target "Distribute Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 50 29) (end 50 46)) (probe (position 50 29))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0) (authored-target "Generate Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 72 29) (end 72 46)) (probe (position 72 29))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0) (authored-target "Transfer Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque")))))
  )
)
~~~
