# META
~~~ini
description=SysML Validation (03-Function-based Behavior): 3a-Function-based Behavior-2
type=file
~~~
# SOURCE
~~~sysml
package '3a-Function-based Behavior-2' {
	public import Definitions::*;
	public import Usages::*;

	package Definitions {
		alias Torque for ISQ::TorqueValue;
		
		// ATTRIBUTE DEFINITIONS
		
		attribute def FuelCmd;
		
		attribute def EngineStart;
		attribute def EngineOff;
		
		// ACTION DEFINITIONS
		
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
			
			action 'generate torque': 'Generate Torque'{
				/*
				 * The binding connector shorthand can be used on action parameters.
				 */
				in fuelCmd = 'provide power'::fuelCmd;
			}
			
			flow 'generate torque'.engineTorque 
			    to 'amplify torque'.engineTorque;
			
			action 'amplify torque': 'Amplify Torque';
			
			flow 'amplify torque'.transmissionTorque 
			    to 'transfer torque'.transmissionTorque;
			
			action 'transfer torque': 'Transfer Torque';
			
			flow 'transfer torque'.driveshaftTorque 
			    to 'distribute torque'.driveShaftTorque;
			
			action 'distribute torque': 'Distribute Torque';
			
			// CONTROL FLOW PART

			/*
			 * The following uses a shorthand for a sequence of successions.
			 * The source of the first first is given by "first start",
			 * and the target of each succeeding first is indicated by
			 * using the "then" keyword.
			 */
			first start;
			then merge continue;	
			then action engineStarted accept engineStart: EngineStart;			
			then action engineStopped accept engineOff: EngineOff;	
			then continue;
			
			/* Enable torque generation. */
			first engineStarted then 'generate torque';
			first engineStarted then 'amplify torque';
			first engineStarted then 'transfer torque';
			first engineStarted then 'distribute torque';
			
			/* Disable torque generation. */
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
  (document "memory://snapshot/3a_function_based_behavior_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 19) (end 5 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 41 3) (end 42 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 46 3) (end 47 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 51 3) (end 52 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 64 9) (end 64 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 65 14) (end 65 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 68 8) (end 68 16))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:789b5126e10a26a79b27a711efd51c754c2cc6f8119fdfae0d346afa0964cc51") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Usages") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction in))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction in))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::EngineOff"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::EngineStart"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmd") (direction in))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmd") (direction in))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "ISQ::TorqueValue"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction in))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Provide Power"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "start"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind merge) (ordinal 0))))) (kind merge) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (mergeInput (reference "continue"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "continue"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "engineStarted")) (succession (reference "generate torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 2))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "engineStarted")) (succession (reference "amplify torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 3))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "engineStarted")) (succession (reference "transfer torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 4))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "engineStarted")) (succession (reference "distribute torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 5))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "generate torque")) (succession (reference "engineStopped"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 6))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "amplify torque")) (succession (reference "engineStopped"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 7))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "transfer torque")) (succession (reference "engineStopped"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 8))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "distribute torque")) (succession (reference "engineStopped"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Amplify Torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Distribute Torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmd") (direction in))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Generate Torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque::fuelCmd"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transfer Torque"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (kind aliasBinding) (ordinal 0))
      (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind featureTyping) (ordinal 0))
      (authored-target "Provide Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0))
      (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 0))
      (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 4))))) (kind succession) (ordinal 0))
      (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 5))))) (kind succession) (ordinal 0))
      (authored-target "generate torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 6))))) (kind succession) (ordinal 0))
      (authored-target "amplify torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 7))))) (kind succession) (ordinal 0))
      (authored-target "transfer torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 8))))) (kind succession) (ordinal 0))
      (authored-target "distribute torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "generate torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1))
      (authored-target "amplify torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 1))
      (authored-target "transfer torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 4))))) (kind succession) (ordinal 1))
      (authored-target "distribute torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 5))))) (kind succession) (ordinal 1))
      (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 6))))) (kind succession) (ordinal 1))
      (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 7))))) (kind succession) (ordinal 1))
      (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 8))))) (kind succession) (ordinal 1))
      (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind merge) (ordinal 0))))) (kind mergeInput) (ordinal 0))
      (authored-target "continue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "continue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Amplify Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Distribute Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Generate Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transfer Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 3))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 4))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 4))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 5))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 5))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 6))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 6))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 7))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 7))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 8))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 8))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 3))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 4))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 4))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 5))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 5))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 6))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 6))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 7))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 7))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 8))))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 8))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 1 15) (end 1 29)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 2 15) (end 2 24)) (probe (position 2 15))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 17 49) (end 17 55)) (probe (position 17 49))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 17 81) (end 17 87)) (probe (position 17 81))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 19 56) (end 19 62)) (probe (position 19 56))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 19 82) (end 19 88)) (probe (position 19 82))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 19 108) (end 19 114)) (probe (position 19 108))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 16 72) (end 16 78)) (probe (position 16 72))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 16 45) (end 16 52)) (probe (position 16 45))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 21 43) (end 21 50)) (probe (position 21 43))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 21 70) (end 21 76)) (probe (position 21 70))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 21 96) (end 21 102)) (probe (position 21 96))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 5 19) (end 5 35)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (kind aliasBinding) (ordinal 0) (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 18 86) (end 18 92)) (probe (position 18 86))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 18 56) (end 18 62)) (probe (position 18 56))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 27 26) (end 27 41)) (probe (position 27 26))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind featureTyping) (ordinal 0) (authored-target "Provide Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 64 9) (end 64 14)) (probe (position 64 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "start")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 71 9) (end 71 22)) (probe (position 71 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 72 9) (end 72 22)) (probe (position 72 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0) (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 73 9) (end 73 22)) (probe (position 73 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 0) (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 74 9) (end 74 22)) (probe (position 74 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 4))))) (kind succession) (ordinal 0) (authored-target "engineStarted")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 77 9) (end 77 26)) (probe (position 77 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 5))))) (kind succession) (ordinal 0) (authored-target "generate torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 78 9) (end 78 25)) (probe (position 78 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 6))))) (kind succession) (ordinal 0) (authored-target "amplify torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 79 9) (end 79 26)) (probe (position 79 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 7))))) (kind succession) (ordinal 0) (authored-target "transfer torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 80 9) (end 80 28)) (probe (position 80 9))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 8))))) (kind succession) (ordinal 0) (authored-target "distribute torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 71 28) (end 71 45)) (probe (position 71 28))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "generate torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 72 28) (end 72 44)) (probe (position 72 28))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1) (authored-target "amplify torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 73 28) (end 73 45)) (probe (position 73 28))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 1) (authored-target "transfer torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 74 28) (end 74 47)) (probe (position 74 28))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 4))))) (kind succession) (ordinal 1) (authored-target "distribute torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 77 32) (end 77 45)) (probe (position 77 32))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 5))))) (kind succession) (ordinal 1) (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 78 31) (end 78 44)) (probe (position 78 31))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 6))))) (kind succession) (ordinal 1) (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 79 32) (end 79 45)) (probe (position 79 32))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 7))))) (kind succession) (ordinal 1) (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 80 34) (end 80 47)) (probe (position 80 34))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind succession) (ordinal 8))))) (kind succession) (ordinal 1) (authored-target "engineStopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 65 14) (end 65 22)) (probe (position 65 14))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind merge) (ordinal 0))))) (kind mergeInput) (ordinal 0) (authored-target "continue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 68 8) (end 68 16)) (probe (position 68 8))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "continue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 44 28) (end 44 44)) (probe (position 44 28))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0) (authored-target "Amplify Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 54 31) (end 54 50)) (probe (position 54 31))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0) (authored-target "Distribute Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 28 15) (end 28 22)) (probe (position 28 15))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 34 29) (end 34 46)) (probe (position 34 29))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0) (authored-target "Generate Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 49 29) (end 49 46)) (probe (position 49 29))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0) (authored-target "Transfer Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 29 21) (end 29 27)) (probe (position 29 21))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_2.md") (range (start 30 21) (end 30 27)) (probe (position 30 21))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_2.md") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
)
~~~
