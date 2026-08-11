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
  (document "3a_function_based_behavior_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 43 8) (end 43 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 57 8) (end 57 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 10) (end 58 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 8) (end 69 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 70 10) (end 70 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 8) (end 74 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 75 10) (end 75 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 79 23) (end 79 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 80 23) (end 80 55))
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "51dbb6a32421c8d4a56ae8aeada330767a8fbd722c2c0f91d4c3fa7acc9ce63a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1"))) (kind "package") (name "3a-Function-based Behavior-1") (declared-name "3a-Function-based Behavior-1"))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1"))) (authored (membership (kind Import) (visibility "public") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1"))) (authored (membership (kind Import) (visibility "public") (import (reference "Usages::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque"))) (kind "action def") (name "Amplify Torque") (declared-name "Amplify Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque"))) (kind "action def") (name "Distribute Torque") (declared-name "Distribute Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::driveShaftTorque"))) (kind "in out parameter") (name "driveShaftTorque") (declared-name "driveShaftTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::EngineOff"))) (kind "attribute def") (name "EngineOff") (declared-name "EngineOff") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::EngineStart"))) (kind "attribute def") (name "EngineStart") (declared-name "EngineStart") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd"))) (kind "attribute def") (name "FuelCmd") (declared-name "FuelCmd") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque"))) (kind "action def") (name "Generate Torque") (declared-name "Generate Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque"))) (authored (relationships (typing (reference "FuelCmd")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power"))) (kind "action def") (name "Provide Power") (declared-name "Provide Power") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power"))) (authored (relationships (typing (reference "FuelCmd")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (kind "alias") (name "Torque") (declared-name "Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque"))) (kind "action def") (name "Transfer Torque") (declared-name "Transfer Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::driveshaftTorque"))) (kind "in out parameter") (name "driveshaftTorque") (declared-name "driveshaftTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Provide Power")) (perform (reference "3a-Function-based Behavior-1::Usages::provide power::generate torque")) (perform (reference "3a-Function-based Behavior-1::Usages::provide power::amplify torque")) (perform (reference "3a-Function-based Behavior-1::Usages::provide power::transfer torque")) (perform (reference "3a-Function-based Behavior-1::Usages::provide power::distribute torque")) (perform (reference "3a-Function-based Behavior-1::Usages::provide power::engineStarted")) (perform (reference "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (kind "action") (name "amplify torque") (declared-name "amplify torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (authored (membership (kind Feature)) (relationships (typing (reference "Amplify Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::continue"))) (kind "merge") (name "merge") (declared-name "merge") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (kind "action") (name "distribute torque") (declared-name "distribute torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (authored (membership (kind Feature)) (relationships (typing (reference "Distribute Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (kind "action") (name "engineStarted") (declared-name "engineStarted") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (kind "action") (name "engineStopped") (declared-name "engineStopped") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (authored (relationships (typing (reference "FuelCmd")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (kind "action") (name "generate torque") (declared-name "generate torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (authored (membership (kind Feature)) (relationships (typing (reference "Generate Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (kind "action") (name "transfer torque") (declared-name "transfer torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transfer Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (authored (relationships (typing (reference "Torque")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Usages::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind featureTyping) (ordinal 0)) (authored-target "Provide Power") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind bindSource) (ordinal 0)) (authored-target "generate torque::fuelCmd") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind bindSource) (ordinal 4)) (authored-target "wheelTorque1") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind bindSource) (ordinal 5)) (authored-target "wheelTorque2") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind bindTarget) (ordinal 0)) (authored-target "fuelCmd") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind bindTarget) (ordinal 4)) (authored-target "distribute torque::wheelTorque1") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind bindTarget) (ordinal 5)) (authored-target "distribute torque::wheelTorque2") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 1)) (authored-target "generate torque::engineTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 2)) (authored-target "amplify torque::transmissionTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 3)) (authored-target "transfer torque::driveshaftTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 6)) (authored-target "start") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 7)) (authored-target "continue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 8)) (authored-target "engineStarted") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 9)) (authored-target "engineStopped") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 10)) (authored-target "engineStarted") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 11)) (authored-target "engineStarted") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 12)) (authored-target "engineStarted") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 13)) (authored-target "engineStarted") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 14)) (authored-target "generate torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 15)) (authored-target "amplify torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 16)) (authored-target "transfer torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 17)) (authored-target "distribute torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 1)) (authored-target "amplify torque::engineTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 2)) (authored-target "transfer torque::transmissionTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 3)) (authored-target "distribute torque::driveShaftTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 6)) (authored-target "continue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 7)) (authored-target "engineStarted") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 8)) (authored-target "engineStopped") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 9)) (authored-target "continue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 10)) (authored-target "generate torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 11)) (authored-target "amplify torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 12)) (authored-target "transfer torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 13)) (authored-target "distribute torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 14)) (authored-target "engineStopped") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 15)) (authored-target "engineStopped") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 16)) (authored-target "engineStopped") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowTarget) (ordinal 17)) (authored-target "engineStopped") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind performSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-1::Usages::provide power::generate torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind performSource) (ordinal 1)) (authored-target "3a-Function-based Behavior-1::Usages::provide power::amplify torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind performSource) (ordinal 2)) (authored-target "3a-Function-based Behavior-1::Usages::provide power::transfer torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind performSource) (ordinal 3)) (authored-target "3a-Function-based Behavior-1::Usages::provide power::distribute torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind performSource) (ordinal 4)) (authored-target "3a-Function-based Behavior-1::Usages::provide power::engineStarted") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind performSource) (ordinal 5)) (authored-target "3a-Function-based Behavior-1::Usages::provide power::engineStopped") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Amplify Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Distribute Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Generate Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Transfer Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::driveShaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::driveshaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind performSource) (ordinal 3)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind performSource) (ordinal 4)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind performSource) (ordinal 5)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind performSource) (ordinal 2)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 15)) (expression (kind flow) (source "amplify torque") (target "engineStopped")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 17)) (expression (kind flow) (source "distribute torque") (target "engineStopped")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 11)) (expression (kind flow) (source "engineStarted") (target "amplify torque")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 13)) (expression (kind flow) (source "engineStarted") (target "distribute torque")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 8)) (expression (kind flow) (source "engineStarted") (target "engineStopped")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 10)) (expression (kind flow) (source "engineStarted") (target "generate torque")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 12)) (expression (kind flow) (source "engineStarted") (target "transfer torque")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 14)) (expression (kind flow) (source "generate torque") (target "engineStopped")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (kind flowSource) (ordinal 16)) (expression (kind flow) (source "transfer torque") (target "engineStopped")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 84 9) (end 84 14)) (probe (position 84 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 6) (authored-target "start")
        (range (start 84 9) (end 84 14))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 15) (end 2 21)) (probe (position 2 15))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Usages::*")
        (range (start 2 15) (end 2 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages") (range (start 34 1) (end 34 3098)))
        )
      )
    )
    (query (range (start 43 36) (end 43 43)) (probe (position 43 36))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind bindTarget) (ordinal 0) (authored-target "fuelCmd")
        (range (start 43 36) (end 43 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd") (range (start 37 3) (end 37 23)))
        )
      )
    )
    (query (range (start 84 20) (end 84 28)) (probe (position 84 20))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 6) (authored-target "continue")
        (range (start 84 20) (end 84 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 99 9) (end 99 17)) (probe (position 99 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 7) (authored-target "continue")
        (range (start 99 9) (end 99 17))
        (outcome (status unresolved))
      )
    )
    (query (range (start 112 28) (end 112 36)) (probe (position 112 28))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 9) (authored-target "continue")
        (range (start 112 28) (end 112 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 15) (end 1 26)) (probe (position 1 15))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 1 15) (end 1 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions") (range (start 4 1) (end 4 968)))
        )
      )
    )
    (query (range (start 79 8) (end 79 20)) (probe (position 79 8))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind bindSource) (ordinal 4) (authored-target "wheelTorque1")
        (range (start 79 8) (end 79 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1") (range (start 38 3) (end 38 28)))
        )
      )
    )
    (query (range (start 80 8) (end 80 20)) (probe (position 80 8))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind bindSource) (ordinal 5) (authored-target "wheelTorque2")
        (range (start 80 8) (end 80 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2") (range (start 39 3) (end 39 28)))
        )
      )
    )
    (query (range (start 99 23) (end 99 36)) (probe (position 99 23))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 7) (authored-target "engineStarted")
        (range (start 99 23) (end 99 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted") (range (start 101 3) (end 101 348)))
        )
      )
    )
    (query (range (start 109 9) (end 109 22)) (probe (position 109 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 8) (authored-target "engineStarted")
        (range (start 109 9) (end 109 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted") (range (start 101 3) (end 101 348)))
        )
      )
    )
    (query (range (start 109 28) (end 109 41)) (probe (position 109 28))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 8) (authored-target "engineStopped")
        (range (start 109 28) (end 109 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped") (range (start 111 3) (end 111 52)))
        )
      )
    )
    (query (range (start 112 9) (end 112 22)) (probe (position 112 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 9) (authored-target "engineStopped")
        (range (start 112 9) (end 112 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped") (range (start 111 3) (end 111 52)))
        )
      )
    )
    (query (range (start 119 9) (end 119 22)) (probe (position 119 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 10) (authored-target "engineStarted")
        (range (start 119 9) (end 119 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted") (range (start 101 3) (end 101 348)))
        )
      )
    )
    (query (range (start 120 9) (end 120 22)) (probe (position 120 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 11) (authored-target "engineStarted")
        (range (start 120 9) (end 120 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted") (range (start 101 3) (end 101 348)))
        )
      )
    )
    (query (range (start 121 9) (end 121 22)) (probe (position 121 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 12) (authored-target "engineStarted")
        (range (start 121 9) (end 121 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted") (range (start 101 3) (end 101 348)))
        )
      )
    )
    (query (range (start 122 9) (end 122 22)) (probe (position 122 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 13) (authored-target "engineStarted")
        (range (start 122 9) (end 122 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted") (range (start 101 3) (end 101 348)))
        )
      )
    )
    (query (range (start 129 32) (end 129 45)) (probe (position 129 32))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 14) (authored-target "engineStopped")
        (range (start 129 32) (end 129 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped") (range (start 111 3) (end 111 52)))
        )
      )
    )
    (query (range (start 130 31) (end 130 44)) (probe (position 130 31))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 15) (authored-target "engineStopped")
        (range (start 130 31) (end 130 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped") (range (start 111 3) (end 111 52)))
        )
      )
    )
    (query (range (start 131 32) (end 131 45)) (probe (position 131 32))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 16) (authored-target "engineStopped")
        (range (start 131 32) (end 131 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped") (range (start 111 3) (end 111 52)))
        )
      )
    )
    (query (range (start 132 34) (end 132 47)) (probe (position 132 34))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 17) (authored-target "engineStopped")
        (range (start 132 34) (end 132 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped") (range (start 111 3) (end 111 52)))
        )
      )
    )
    (query (range (start 120 28) (end 120 44)) (probe (position 120 28))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 11) (authored-target "amplify torque")
        (range (start 120 28) (end 120 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque") (range (start 67 3) (end 67 45)))
        )
      )
    )
    (query (range (start 130 9) (end 130 25)) (probe (position 130 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 15) (authored-target "amplify torque")
        (range (start 130 9) (end 130 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque") (range (start 67 3) (end 67 45)))
        )
      )
    )
    (query (range (start 119 28) (end 119 45)) (probe (position 119 28))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 10) (authored-target "generate torque")
        (range (start 119 28) (end 119 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque") (range (start 50 3) (end 50 163)))
        )
      )
    )
    (query (range (start 121 28) (end 121 45)) (probe (position 121 28))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 12) (authored-target "transfer torque")
        (range (start 121 28) (end 121 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque") (range (start 72 3) (end 72 47)))
        )
      )
    )
    (query (range (start 129 9) (end 129 26)) (probe (position 129 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 14) (authored-target "generate torque")
        (range (start 129 9) (end 129 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque") (range (start 50 3) (end 50 163)))
        )
      )
    )
    (query (range (start 131 9) (end 131 26)) (probe (position 131 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 16) (authored-target "transfer torque")
        (range (start 131 9) (end 131 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque") (range (start 72 3) (end 72 47)))
        )
      )
    )
    (query (range (start 122 28) (end 122 47)) (probe (position 122 28))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 13) (authored-target "distribute torque")
        (range (start 122 28) (end 122 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque") (range (start 77 3) (end 77 51)))
        )
      )
    )
    (query (range (start 132 9) (end 132 28)) (probe (position 132 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 17) (authored-target "distribute torque")
        (range (start 132 9) (end 132 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque") (range (start 77 3) (end 77 51)))
        )
      )
    )
    (query (range (start 43 8) (end 43 33)) (probe (position 43 8))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind bindSource) (ordinal 0) (authored-target "generate torque::fuelCmd")
        (range (start 43 8) (end 43 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 58 10) (end 58 39)) (probe (position 58 10))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 1) (authored-target "amplify torque::engineTorque")
        (range (start 58 10) (end 58 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 57 8) (end 57 38)) (probe (position 57 8))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 1) (authored-target "generate torque::engineTorque")
        (range (start 57 8) (end 57 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 79 23) (end 79 55)) (probe (position 79 23))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind bindTarget) (ordinal 4) (authored-target "distribute torque::wheelTorque1")
        (range (start 79 23) (end 79 55))
        (outcome (status unresolved))
      )
    )
    (query (range (start 80 23) (end 80 55)) (probe (position 80 23))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind bindTarget) (ordinal 5) (authored-target "distribute torque::wheelTorque2")
        (range (start 80 23) (end 80 55))
        (outcome (status unresolved))
      )
    )
    (query (range (start 74 8) (end 74 42)) (probe (position 74 8))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 3) (authored-target "transfer torque::driveshaftTorque")
        (range (start 74 8) (end 74 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 69 8) (end 69 43)) (probe (position 69 8))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowSource) (ordinal 2) (authored-target "amplify torque::transmissionTorque")
        (range (start 69 8) (end 69 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 70 10) (end 70 46)) (probe (position 70 10))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 2) (authored-target "transfer torque::transmissionTorque")
        (range (start 70 10) (end 70 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 75 10) (end 75 46)) (probe (position 75 10))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))
        (kind flowTarget) (ordinal 3) (authored-target "distribute torque::driveShaftTorque")
        (range (start 75 10) (end 75 46))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
