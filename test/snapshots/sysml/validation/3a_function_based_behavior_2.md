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
  (document "3a_function_based_behavior_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 4) (end 38 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 8) (end 41 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 10) (end 42 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 46 8) (end 46 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 47 10) (end 47 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 51 8) (end 51 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 52 10) (end 52 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 64 3) (end 64 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 66 3) (end 66 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 3) (end 67 57))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "756d8826a0903ce8ab5594a1ca717519bae20e219b188dfb08617446db913156") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2"))) (kind "package") (name "3a-Function-based Behavior-2") (declared-name "3a-Function-based Behavior-2"))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2"))) (authored (membership (kind Import) (visibility "public") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2"))) (authored (membership (kind Import) (visibility "public") (import (reference "Usages::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque"))) (kind "action def") (name "Amplify Torque") (declared-name "Amplify Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque"))) (kind "action def") (name "Distribute Torque") (declared-name "Distribute Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (kind "in out parameter") (name "driveShaftTorque") (declared-name "driveShaftTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::EngineOff"))) (kind "attribute def") (name "EngineOff") (declared-name "EngineOff") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::EngineStart"))) (kind "attribute def") (name "EngineStart") (declared-name "EngineStart") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))) (kind "attribute def") (name "FuelCmd") (declared-name "FuelCmd") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque"))) (kind "action def") (name "Generate Torque") (declared-name "Generate Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque"))) (authored (relationships (typing (reference "FuelCmd")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))) (kind "action def") (name "Provide Power") (declared-name "Provide Power") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))) (authored (relationships (typing (reference "FuelCmd")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (kind "alias") (name "Torque") (declared-name "Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque"))) (kind "action def") (name "Transfer Torque") (declared-name "Transfer Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (kind "in out parameter") (name "driveshaftTorque") (declared-name "driveshaftTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Provide Power")) (perform (reference "3a-Function-based Behavior-2::Usages::provide power::generate torque")) (perform (reference "3a-Function-based Behavior-2::Usages::provide power::amplify torque")) (perform (reference "3a-Function-based Behavior-2::Usages::provide power::transfer torque")) (perform (reference "3a-Function-based Behavior-2::Usages::provide power::distribute torque")) (perform (reference "3a-Function-based Behavior-2::Usages::provide power::engineStarted")) (perform (reference "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::_initial"))) (kind "initial") (name "_initial") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (relationships (flow (reference "3a-Function-based Behavior-2::Usages::provide power::start")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (kind "action") (name "amplify torque") (declared-name "amplify torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (membership (kind Feature)) (relationships (typing (reference "Amplify Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue"))) (kind "merge") (name "merge") (declared-name "merge") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (relationships (flow (reference "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (kind "action") (name "distribute torque") (declared-name "distribute torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (membership (kind Feature)) (relationships (typing (reference "Distribute Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (kind "action") (name "engineStarted") (declared-name "engineStarted") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (relationships (typing (reference "")) (flow (reference "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (kind "action") (name "engineStopped") (declared-name "engineStopped") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (relationships (typing (reference "")) (flow (reference "3a-Function-based Behavior-2::Usages::provide power::continue")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (relationships (typing (reference "FuelCmd")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (kind "action") (name "generate torque") (declared-name "generate torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (membership (kind Feature)) (relationships (typing (reference "Generate Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (kind "action") (name "transfer torque") (declared-name "transfer torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transfer Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (relationships (typing (reference "Torque")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Usages::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind featureTyping) (ordinal 0)) (authored-target "Provide Power") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 0)) (authored-target "generate torque::engineTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 1)) (authored-target "amplify torque::transmissionTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 2)) (authored-target "transfer torque::driveshaftTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 3)) (authored-target "engineStarted") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 4)) (authored-target "engineStarted") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 5)) (authored-target "engineStarted") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 6)) (authored-target "engineStarted") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 7)) (authored-target "generate torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 8)) (authored-target "amplify torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 9)) (authored-target "transfer torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 10)) (authored-target "distribute torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 0)) (authored-target "amplify torque::engineTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 1)) (authored-target "transfer torque::transmissionTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 2)) (authored-target "distribute torque::driveShaftTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 3)) (authored-target "generate torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 4)) (authored-target "amplify torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 5)) (authored-target "transfer torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 6)) (authored-target "distribute torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 7)) (authored-target "engineStopped") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 8)) (authored-target "engineStopped") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 9)) (authored-target "engineStopped") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 10)) (authored-target "engineStopped") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::generate torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 1)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::amplify torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 2)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::transfer torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 3)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::distribute torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 4)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::engineStarted") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 5)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::engineStopped") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::_initial"))) (kind flowSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::start") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Amplify Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue"))) (kind flowSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::engineStarted") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Distribute Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (kind flowSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::engineStopped") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (kind flowSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::continue") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Generate Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Transfer Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 3)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 4)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 5)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 2)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 8)) (expression (kind flow) (source "amplify torque") (target "engineStopped")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 10)) (expression (kind flow) (source "distribute torque") (target "engineStopped")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 4)) (expression (kind flow) (source "engineStarted") (target "amplify torque")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 6)) (expression (kind flow) (source "engineStarted") (target "distribute torque")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 3)) (expression (kind flow) (source "engineStarted") (target "generate torque")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 5)) (expression (kind flow) (source "engineStarted") (target "transfer torque")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 7)) (expression (kind flow) (source "generate torque") (target "engineStopped")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 9)) (expression (kind flow) (source "transfer torque") (target "engineStopped")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque::fuelCmd")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 15) (end 2 21)) (probe (position 2 15))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Usages::*")
        (range (start 2 15) (end 2 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages") (range (start 25 1) (end 25 1715)))
        )
      )
    )
    (query (range (start 1 15) (end 1 26)) (probe (position 1 15))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 1 15) (end 1 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions") (range (start 4 1) (end 4 707)))
        )
      )
    )
    (query (range (start 71 9) (end 71 22)) (probe (position 71 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowSource) (ordinal 3) (authored-target "engineStarted")
        (range (start 71 9) (end 71 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted") (range (start 66 3) (end 66 61)))
        )
      )
    )
    (query (range (start 72 9) (end 72 22)) (probe (position 72 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowSource) (ordinal 4) (authored-target "engineStarted")
        (range (start 72 9) (end 72 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted") (range (start 66 3) (end 66 61)))
        )
      )
    )
    (query (range (start 73 9) (end 73 22)) (probe (position 73 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowSource) (ordinal 5) (authored-target "engineStarted")
        (range (start 73 9) (end 73 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted") (range (start 66 3) (end 66 61)))
        )
      )
    )
    (query (range (start 74 9) (end 74 22)) (probe (position 74 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowSource) (ordinal 6) (authored-target "engineStarted")
        (range (start 74 9) (end 74 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted") (range (start 66 3) (end 66 61)))
        )
      )
    )
    (query (range (start 77 32) (end 77 45)) (probe (position 77 32))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowTarget) (ordinal 7) (authored-target "engineStopped")
        (range (start 77 32) (end 77 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped") (range (start 67 3) (end 67 57)))
        )
      )
    )
    (query (range (start 78 31) (end 78 44)) (probe (position 78 31))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowTarget) (ordinal 8) (authored-target "engineStopped")
        (range (start 78 31) (end 78 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped") (range (start 67 3) (end 67 57)))
        )
      )
    )
    (query (range (start 79 32) (end 79 45)) (probe (position 79 32))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowTarget) (ordinal 9) (authored-target "engineStopped")
        (range (start 79 32) (end 79 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped") (range (start 67 3) (end 67 57)))
        )
      )
    )
    (query (range (start 80 34) (end 80 47)) (probe (position 80 34))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowTarget) (ordinal 10) (authored-target "engineStopped")
        (range (start 80 34) (end 80 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped") (range (start 67 3) (end 67 57)))
        )
      )
    )
    (query (range (start 72 28) (end 72 44)) (probe (position 72 28))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowTarget) (ordinal 4) (authored-target "amplify torque")
        (range (start 72 28) (end 72 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque") (range (start 44 3) (end 44 45)))
        )
      )
    )
    (query (range (start 78 9) (end 78 25)) (probe (position 78 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowSource) (ordinal 8) (authored-target "amplify torque")
        (range (start 78 9) (end 78 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque") (range (start 44 3) (end 44 45)))
        )
      )
    )
    (query (range (start 71 28) (end 71 45)) (probe (position 71 28))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowTarget) (ordinal 3) (authored-target "generate torque")
        (range (start 71 28) (end 71 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque") (range (start 34 3) (end 34 183)))
        )
      )
    )
    (query (range (start 73 28) (end 73 45)) (probe (position 73 28))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowTarget) (ordinal 5) (authored-target "transfer torque")
        (range (start 73 28) (end 73 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque") (range (start 49 3) (end 49 47)))
        )
      )
    )
    (query (range (start 77 9) (end 77 26)) (probe (position 77 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowSource) (ordinal 7) (authored-target "generate torque")
        (range (start 77 9) (end 77 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque") (range (start 34 3) (end 34 183)))
        )
      )
    )
    (query (range (start 79 9) (end 79 26)) (probe (position 79 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowSource) (ordinal 9) (authored-target "transfer torque")
        (range (start 79 9) (end 79 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque") (range (start 49 3) (end 49 47)))
        )
      )
    )
    (query (range (start 74 28) (end 74 47)) (probe (position 74 28))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowTarget) (ordinal 6) (authored-target "distribute torque")
        (range (start 74 28) (end 74 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque") (range (start 54 3) (end 54 51)))
        )
      )
    )
    (query (range (start 80 9) (end 80 28)) (probe (position 80 9))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowSource) (ordinal 10) (authored-target "distribute torque")
        (range (start 80 9) (end 80 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque") (range (start 54 3) (end 54 51)))
        )
      )
    )
    (query (range (start 42 10) (end 42 39)) (probe (position 42 10))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowTarget) (ordinal 0) (authored-target "amplify torque::engineTorque")
        (range (start 42 10) (end 42 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 41 8) (end 41 38)) (probe (position 41 8))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowSource) (ordinal 0) (authored-target "generate torque::engineTorque")
        (range (start 41 8) (end 41 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 51 8) (end 51 42)) (probe (position 51 8))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowSource) (ordinal 2) (authored-target "transfer torque::driveshaftTorque")
        (range (start 51 8) (end 51 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 46 8) (end 46 43)) (probe (position 46 8))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowSource) (ordinal 1) (authored-target "amplify torque::transmissionTorque")
        (range (start 46 8) (end 46 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 47 10) (end 47 46)) (probe (position 47 10))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowTarget) (ordinal 1) (authored-target "transfer torque::transmissionTorque")
        (range (start 47 10) (end 47 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 52 10) (end 52 46)) (probe (position 52 10))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))
        (kind flowTarget) (ordinal 2) (authored-target "distribute torque::driveShaftTorque")
        (range (start 52 10) (end 52 46))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
