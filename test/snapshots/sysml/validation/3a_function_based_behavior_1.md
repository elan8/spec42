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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 84 9) (end 84 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 84 20) (end 84 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 93 9) (end 93 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 99 9) (end 99 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 112 28) (end 112 36))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3b081ef8ba2e964be99eac92f6b641f30d9e18eb89acd1ed6e3d89b3a22fa8a3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Usages") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::engineTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction in))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::transmissionTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::driveShaftTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction in))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::EngineOff"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::EngineStart"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::engineTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmd") (direction in))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::fuelCmd"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmd") (direction in))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque1"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque2"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "ISQ::TorqueValue"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::driveshaftTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::transmissionTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction in))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Provide Power"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 0)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "fuelCmd")) (memberAccessOperand (reference "generate torque::fuelCmd"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 0)))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "generate torque::engineTorque")) (memberAccessOperand (reference "amplify torque::engineTorque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 1)))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "amplify torque::transmissionTorque")) (memberAccessOperand (reference "transfer torque::transmissionTorque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 2)))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "transfer torque::driveshaftTorque")) (memberAccessOperand (reference "distribute torque::driveShaftTorque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 1)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "wheelTorque1")) (memberAccessOperand (reference "distribute torque::wheelTorque1"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 2)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "wheelTorque2")) (memberAccessOperand (reference "distribute torque::wheelTorque2"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 0)))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "start")) (succession (reference "continue"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind merge) (ordinal 0)))))) (kind merge) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (mergeInput (reference "continue"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 1)))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "continue")) (succession (reference "engineStarted"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 2)))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "engineStarted")) (succession (reference "engineStopped"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 3)))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "engineStopped")) (succession (reference "continue"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 4)))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "engineStarted")) (succession (reference "generate torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 5)))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "engineStarted")) (succession (reference "amplify torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 6)))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "engineStarted")) (succession (reference "transfer torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 7)))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "engineStarted")) (succession (reference "distribute torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 8)))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "generate torque")) (succession (reference "engineStopped"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 9)))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "amplify torque")) (succession (reference "engineStopped"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 10)))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "transfer torque")) (succession (reference "engineStopped"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 11)))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "distribute torque")) (succession (reference "engineStopped"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Amplify Torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Distribute Torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (acceptPayloadType (reference "EngineStart"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (acceptPayloadType (reference "EngineOff"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmd") (direction in))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Generate Torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transfer Torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (kind aliasBinding) (ordinal 0))
      (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind featureTyping) (ordinal 0))
      (authored-target "Provide Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 0)))))) (kind succession) (ordinal 0))
      (authored-target "start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 1)))))) (kind succession) (ordinal 0))
      (authored-target "continue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 2)))))) (kind succession) (ordinal 0))
      (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 3)))))) (kind succession) (ordinal 0))
      (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 4)))))) (kind succession) (ordinal 0))
      (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 5)))))) (kind succession) (ordinal 0))
      (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 6)))))) (kind succession) (ordinal 0))
      (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 7)))))) (kind succession) (ordinal 0))
      (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 8)))))) (kind succession) (ordinal 0))
      (authored-target "generate torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 9)))))) (kind succession) (ordinal 0))
      (authored-target "amplify torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 10)))))) (kind succession) (ordinal 0))
      (authored-target "transfer torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 11)))))) (kind succession) (ordinal 0))
      (authored-target "distribute torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 0)))))) (kind succession) (ordinal 1))
      (authored-target "continue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 1)))))) (kind succession) (ordinal 1))
      (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 2)))))) (kind succession) (ordinal 1))
      (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 3)))))) (kind succession) (ordinal 1))
      (authored-target "continue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 4)))))) (kind succession) (ordinal 1))
      (authored-target "generate torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 5)))))) (kind succession) (ordinal 1))
      (authored-target "amplify torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 6)))))) (kind succession) (ordinal 1))
      (authored-target "transfer torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 7)))))) (kind succession) (ordinal 1))
      (authored-target "distribute torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 8)))))) (kind succession) (ordinal 1))
      (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 9)))))) (kind succession) (ordinal 1))
      (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 10)))))) (kind succession) (ordinal 1))
      (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 11)))))) (kind succession) (ordinal 1))
      (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 1)))))) (kind bindSource) (ordinal 0))
      (authored-target "wheelTorque1")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 2)))))) (kind bindSource) (ordinal 0))
      (authored-target "wheelTorque2")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 0)))))) (kind bindTarget) (ordinal 0))
      (authored-target "fuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "generate torque::fuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "generate torque::engineTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::engineTorque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "amplify torque::transmissionTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::transmissionTorque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 2)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "transfer torque::driveshaftTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::driveshaftTorque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "distribute torque::wheelTorque1")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 2)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "distribute torque::wheelTorque2")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "amplify torque::engineTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::engineTorque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "transfer torque::transmissionTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::transmissionTorque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 2)))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "distribute torque::driveShaftTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::driveShaftTorque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind merge) (ordinal 0)))))) (kind mergeInput) (ordinal 0))
      (authored-target "continue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Amplify Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Distribute Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (kind acceptPayloadType) (ordinal 0))
      (authored-target "EngineStart")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::EngineStart")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (kind acceptPayloadType) (ordinal 0))
      (authored-target "EngineOff")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::EngineOff")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Generate Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transfer Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::engineTorque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::transmissionTorque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::driveShaftTorque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::engineTorque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::fuelCmd"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque1"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque2"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::driveshaftTorque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::transmissionTorque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 2)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 2)))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 3)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 3)))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 4)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 4)))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 5)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 5)))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 6)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 6)))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 7)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 7)))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 8)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 8)))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 9)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 9)))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 10)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 10)))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 11)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 11)))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 1)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 1)))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 2)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 2)))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 4)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 4)))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 5)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 5)))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 6)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 6)))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 7)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 7)))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 8)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 8)))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 9)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 9)))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 10)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 10)))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 11)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 11)))))) (kind succession) (ordinal 1)))
    (relationship (kind bindSource) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 1)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 1)))))) (kind bindSource) (ordinal 0)))
    (relationship (kind bindSource) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 2)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 2)))))) (kind bindSource) (ordinal 0)))
    (relationship (kind bindTarget) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 0)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 0)))))) (kind bindTarget) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 0)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 0)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::engineTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 1)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::transmissionTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 2)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::driveshaftTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 2)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 1)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 2)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 2)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 0)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::engineTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 1)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::transmissionTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 2)))))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::driveShaftTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 2)))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind acceptPayloadType) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::EngineStart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (kind acceptPayloadType) (ordinal 0)))
    (relationship (kind acceptPayloadType) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::EngineOff"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (kind acceptPayloadType) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2"))) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 1 15) (end 1 29)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 2 15) (end 2 24)) (probe (position 2 15))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 26 49) (end 26 55)) (probe (position 26 49))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 26 81) (end 26 87)) (probe (position 26 81))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 28 56) (end 28 62)) (probe (position 28 56))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 28 82) (end 28 88)) (probe (position 28 82))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 28 108) (end 28 114)) (probe (position 28 108))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 25 72) (end 25 78)) (probe (position 25 72))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 25 45) (end 25 52)) (probe (position 25 45))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 30 43) (end 30 50)) (probe (position 30 43))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 30 70) (end 30 76)) (probe (position 30 70))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 30 96) (end 30 102)) (probe (position 30 96))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 5 19) (end 5 35)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (kind aliasBinding) (ordinal 0) (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 27 86) (end 27 92)) (probe (position 27 86))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 27 56) (end 27 62)) (probe (position 27 56))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 36 26) (end 36 41)) (probe (position 36 26))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind featureTyping) (ordinal 0) (authored-target "Provide Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 84 9) (end 84 14)) (probe (position 84 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 0)))))) (kind succession) (ordinal 0) (authored-target "start")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 99 9) (end 99 17)) (probe (position 99 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 1)))))) (kind succession) (ordinal 0) (authored-target "continue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 109 9) (end 109 22)) (probe (position 109 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 2)))))) (kind succession) (ordinal 0) (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 112 9) (end 112 22)) (probe (position 112 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 3)))))) (kind succession) (ordinal 0) (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 119 9) (end 119 22)) (probe (position 119 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 4)))))) (kind succession) (ordinal 0) (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 120 9) (end 120 22)) (probe (position 120 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 5)))))) (kind succession) (ordinal 0) (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 121 9) (end 121 22)) (probe (position 121 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 6)))))) (kind succession) (ordinal 0) (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 122 9) (end 122 22)) (probe (position 122 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 7)))))) (kind succession) (ordinal 0) (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 129 9) (end 129 26)) (probe (position 129 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 8)))))) (kind succession) (ordinal 0) (authored-target "generate torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 130 9) (end 130 25)) (probe (position 130 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 9)))))) (kind succession) (ordinal 0) (authored-target "amplify torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 131 9) (end 131 26)) (probe (position 131 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 10)))))) (kind succession) (ordinal 0) (authored-target "transfer torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 132 9) (end 132 28)) (probe (position 132 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 11)))))) (kind succession) (ordinal 0) (authored-target "distribute torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 84 20) (end 84 28)) (probe (position 84 20))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 0)))))) (kind succession) (ordinal 1) (authored-target "continue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 99 23) (end 99 36)) (probe (position 99 23))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 1)))))) (kind succession) (ordinal 1) (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 109 28) (end 109 41)) (probe (position 109 28))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 2)))))) (kind succession) (ordinal 1) (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 112 28) (end 112 36)) (probe (position 112 28))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 3)))))) (kind succession) (ordinal 1) (authored-target "continue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 119 28) (end 119 45)) (probe (position 119 28))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 4)))))) (kind succession) (ordinal 1) (authored-target "generate torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 120 28) (end 120 44)) (probe (position 120 28))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 5)))))) (kind succession) (ordinal 1) (authored-target "amplify torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 121 28) (end 121 45)) (probe (position 121 28))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 6)))))) (kind succession) (ordinal 1) (authored-target "transfer torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 122 28) (end 122 47)) (probe (position 122 28))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 7)))))) (kind succession) (ordinal 1) (authored-target "distribute torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 129 32) (end 129 45)) (probe (position 129 32))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 8)))))) (kind succession) (ordinal 1) (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 130 31) (end 130 44)) (probe (position 130 31))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 9)))))) (kind succession) (ordinal 1) (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 131 32) (end 131 45)) (probe (position 131 32))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 10)))))) (kind succession) (ordinal 1) (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 132 34) (end 132 47)) (probe (position 132 34))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind succession) (ordinal 11)))))) (kind succession) (ordinal 1) (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 79 8) (end 79 20)) (probe (position 79 8))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 1)))))) (kind bindSource) (ordinal 0) (authored-target "wheelTorque1")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 80 8) (end 80 20)) (probe (position 80 8))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 2)))))) (kind bindSource) (ordinal 0) (authored-target "wheelTorque2")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 43 36) (end 43 43)) (probe (position 43 36))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 0)))))) (kind bindTarget) (ordinal 0) (authored-target "fuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 43 8) (end 43 33)) (probe (position 43 8))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "generate torque::fuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 57 8) (end 57 38)) (probe (position 57 8))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "generate torque::engineTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::engineTorque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 69 8) (end 69 43)) (probe (position 69 8))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "amplify torque::transmissionTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::transmissionTorque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 74 8) (end 74 42)) (probe (position 74 8))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 2)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "transfer torque::driveshaftTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::driveshaftTorque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 79 23) (end 79 55)) (probe (position 79 23))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "distribute torque::wheelTorque1")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 80 23) (end 80 55)) (probe (position 80 23))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind bind) (ordinal 2)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "distribute torque::wheelTorque2")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 58 10) (end 58 39)) (probe (position 58 10))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 1) (authored-target "amplify torque::engineTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::engineTorque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 70 10) (end 70 46)) (probe (position 70 10))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 1) (authored-target "transfer torque::transmissionTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::transmissionTorque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 75 10) (end 75 46)) (probe (position 75 10))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind flow) (ordinal 2)))))) (kind memberAccessOperand) (ordinal 1) (authored-target "distribute torque::driveShaftTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::driveShaftTorque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 93 9) (end 93 17)) (probe (position 93 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (path (named (kind package) (name "3a-Function-based Behavior-1")) (named (kind package) (name "Usages")) (named (kind action) (name "provide power")) (anonymous (kind merge) (ordinal 0)))))) (kind mergeInput) (ordinal 0) (authored-target "continue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 67 28) (end 67 44)) (probe (position 67 28))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0) (authored-target "Amplify Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 77 31) (end 77 50)) (probe (position 77 31))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0) (authored-target "Distribute Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 101 44) (end 101 55)) (probe (position 101 44))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (kind acceptPayloadType) (ordinal 0) (authored-target "EngineStart")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::EngineStart")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 111 42) (end 111 51)) (probe (position 111 42))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (kind acceptPayloadType) (ordinal 0) (authored-target "EngineOff")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::EngineOff")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 37 15) (end 37 22)) (probe (position 37 15))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 50 29) (end 50 46)) (probe (position 50 29))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0) (authored-target "Generate Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 72 29) (end 72 46)) (probe (position 72 29))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0) (authored-target "Transfer Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 38 21) (end 38 27)) (probe (position 38 21))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_1.md") (range (start 39 21) (end 39 27)) (probe (position 39 21))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_1.md") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
  )
)
~~~
