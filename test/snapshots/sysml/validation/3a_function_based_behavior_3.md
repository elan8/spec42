# META
~~~ini
description=SysML Validation (03-Function-based Behavior): 3a-Function-based Behavior-3
type=file
~~~
# SOURCE
~~~sysml
package '3a-Function-based Behavior-5' {
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
	
		action 'provide power': 'Provide Power' {
			// PARAMETERS
			
			in fuelCmd: FuelCmd; 
			out wheelTorque1: Torque; 
			out wheelTorque2: Torque;
		
			loop {
				accept engineStart : EngineStart;
				then action {
					action 'generate torque': 'Generate Torque' {
						in fuelCmd = 'provide power'::fuelCmd;
						out engineTorque: Torque;
					}
					
					flow 'generate torque'.engineTorque 
					    to 'amplify torque'.engineTorque;
					
					action 'amplify torque': 'Amplify Torque' {
						in engineTorque: Torque;
						out transmissionTorque: Torque;
					}
					
					flow 'amplify torque'.transmissionTorque 
					    to 'transfer torque'.transmissionTorque;
					
					action 'transfer torque': 'Transfer Torque' {
						in transmissionTorque: Torque; 
						out driveshaftTorque: Torque;
					}
					
					flow 'transfer torque'.driveshaftTorque 
					    to 'distribute torque'.driveshaftTorque;
					
					action 'distribute torque': 'Distribute Torque' {
						in driveshaftTorque: Torque;
						out wheelTorque1: Torque;
						out wheelTorque2: Torque;
					}
				}
				then action accept engineOff : EngineOff;
			}	
		}
	
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "3a_function_based_behavior_3.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 36 4) (end 36 890))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/3a_function_based_behavior_3.md")
            (range (start 36 4) (end 36 890))
          )
          (related
            (uri "memory://snapshot/snapshot/3a_function_based_behavior_3.md")
            (range (start 67 4) (end 67 45))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 38 6) (end 38 44))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/3a_function_based_behavior_3.md")
            (range (start 36 4) (end 36 890))
          )
          (related
            (uri "memory://snapshot/snapshot/3a_function_based_behavior_3.md")
            (range (start 67 4) (end 67 45))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 67 4) (end 67 45))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/3a_function_based_behavior_3.md")
            (range (start 36 4) (end 36 890))
          )
          (related
            (uri "memory://snapshot/snapshot/3a_function_based_behavior_3.md")
            (range (start 67 4) (end 67 45))
          )
        )
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package '3a-Function-based Behavior-5' {
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

        action 'provide power': 'Provide Power' {
            // PARAMETERS

            in fuelCmd: FuelCmd;
            out wheelTorque1: Torque;
            out wheelTorque2: Torque;

            loop {
                accept engineStart : EngineStart;
                then action {
                    action 'generate torque': 'Generate Torque' {
                        in fuelCmd = 'provide power'::fuelCmd;
                        out engineTorque: Torque;
                    }

                    flow 'generate torque'.engineTorque
                    to 'amplify torque'.engineTorque;

                    action 'amplify torque': 'Amplify Torque' {
                        in engineTorque: Torque;
                        out transmissionTorque: Torque;
                    }

                    flow 'amplify torque'.transmissionTorque
                    to 'transfer torque'.transmissionTorque;

                    action 'transfer torque': 'Transfer Torque' {
                        in transmissionTorque: Torque;
                        out driveshaftTorque: Torque;
                    }

                    flow 'transfer torque'.driveshaftTorque
                    to 'distribute torque'.driveshaftTorque;

                    action 'distribute torque': 'Distribute Torque' {
                        in driveshaftTorque: Torque;
                        out wheelTorque1: Torque;
                        out wheelTorque2: Torque;
                    }
                }
                then action accept engineOff : EngineOff;
            }
        }

    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b4c53f3ad23e8791aa357e29ba47291f4dafe4714095ef4931384f904aaea3f8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5"))) (kind "package") (name "3a-Function-based Behavior-5") (declared-name "3a-Function-based Behavior-5"))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5"))) (authored (membership (kind Import) (visibility "public") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5"))) (authored (membership (kind Import) (visibility "public") (import (reference "Usages::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque"))) (kind "action def") (name "Amplify Torque") (declared-name "Amplify Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque"))) (kind "action def") (name "Distribute Torque") (declared-name "Distribute Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::driveShaftTorque"))) (kind "in out parameter") (name "driveShaftTorque") (declared-name "driveShaftTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::EngineOff"))) (kind "attribute def") (name "EngineOff") (declared-name "EngineOff") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::EngineStart"))) (kind "attribute def") (name "EngineStart") (declared-name "EngineStart") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd"))) (kind "attribute def") (name "FuelCmd") (declared-name "FuelCmd") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque"))) (kind "action def") (name "Generate Torque") (declared-name "Generate Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque"))) (authored (relationships (typing (reference "FuelCmd")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))) (kind "action def") (name "Provide Power") (declared-name "Provide Power") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))) (authored (relationships (typing (reference "FuelCmd")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (kind "alias") (name "Torque") (declared-name "Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque"))) (kind "action def") (name "Transfer Torque") (declared-name "Transfer Torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::driveshaftTorque"))) (kind "in out parameter") (name "driveshaftTorque") (declared-name "driveshaftTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Provide Power")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (kind "loop") (name "loop") (declared-name "loop") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (authored (relationships (perform (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::accept")) (perform (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::")) (perform (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::#action")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind "action") (name "") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (authored (relationships (typing (reference "")) (flow (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::#action")) (perform (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque")) (perform (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque")) (perform (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque")) (perform (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action"))) (kind "action") (name "") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (kind "action") (name "amplify torque") (declared-name "amplify torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (authored (membership (kind Feature)) (relationships (typing (reference "Amplify Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (kind "action") (name "distribute torque") (declared-name "distribute torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (authored (membership (kind Feature)) (relationships (typing (reference "Distribute Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque"))) (kind "in out parameter") (name "driveshaftTorque") (declared-name "driveshaftTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (kind "action") (name "generate torque") (declared-name "generate torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (authored (membership (kind Feature)) (relationships (typing (reference "Generate Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (kind "action") (name "transfer torque") (declared-name "transfer torque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transfer Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque"))) (kind "in out parameter") (name "driveshaftTorque") (declared-name "driveshaftTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::accept"))) (kind "action") (name "accept") (declared-name "accept") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (authored (relationships (typing (reference "FuelCmd")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (authored (relationships (typing (reference "Torque")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Usages::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (kind featureTyping) (ordinal 0)) (authored-target "Provide Power") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (kind performSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::accept") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::accept")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (kind performSource) (ordinal 1)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (kind performSource) (ordinal 2)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::#action") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::")) (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::#action") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 0)) (authored-target "generate torque::engineTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 1)) (authored-target "amplify torque::transmissionTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 2)) (authored-target "transfer torque::driveshaftTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowTarget) (ordinal 0)) (authored-target "amplify torque::engineTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowTarget) (ordinal 1)) (authored-target "transfer torque::transmissionTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowTarget) (ordinal 2)) (authored-target "distribute torque::driveshaftTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 1)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 2)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 3)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::")) (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Amplify Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Distribute Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Generate Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::")) (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Transfer Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::driveShaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::driveshaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (kind performSource) (ordinal 2)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::accept"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (kind performSource) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 3)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 2)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "amplify torque::transmissionTorque") (target "transfer torque::transmissionTorque")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "generate torque::engineTorque") (target "amplify torque::engineTorque")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 2)) (expression (kind flow) (source "transfer torque::driveshaftTorque") (target "distribute torque::driveshaftTorque")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::fuelCmd")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 15) (end 2 21)) (probe (position 2 15))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-5::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Usages::*")
        (range (start 2 15) (end 2 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages") (range (start 25 1) (end 25 1171)))
        )
      )
    )
    (query (range (start 1 15) (end 1 26)) (probe (position 1 15))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-5::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 1 15) (end 1 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions") (range (start 4 1) (end 4 707)))
        )
      )
    )
    (query (range (start 43 12) (end 43 41)) (probe (position 43 12))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))
        (kind flowTarget) (ordinal 0) (authored-target "amplify torque::engineTorque")
        (range (start 43 12) (end 43 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque") (range (start 46 6) (end 46 30)))
        )
      )
    )
    (query (range (start 42 10) (end 42 40)) (probe (position 42 10))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))
        (kind flowSource) (ordinal 0) (authored-target "generate torque::engineTorque")
        (range (start 42 10) (end 42 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque") (range (start 39 6) (end 39 31)))
        )
      )
    )
    (query (range (start 58 10) (end 58 44)) (probe (position 58 10))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))
        (kind flowSource) (ordinal 2) (authored-target "transfer torque::driveshaftTorque")
        (range (start 58 10) (end 58 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque") (range (start 55 6) (end 55 35)))
        )
      )
    )
    (query (range (start 50 10) (end 50 45)) (probe (position 50 10))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))
        (kind flowSource) (ordinal 1) (authored-target "amplify torque::transmissionTorque")
        (range (start 50 10) (end 50 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque") (range (start 47 6) (end 47 37)))
        )
      )
    )
    (query (range (start 51 12) (end 51 48)) (probe (position 51 12))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))
        (kind flowTarget) (ordinal 1) (authored-target "transfer torque::transmissionTorque")
        (range (start 51 12) (end 51 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque") (range (start 54 6) (end 54 36)))
        )
      )
    )
    (query (range (start 59 12) (end 59 48)) (probe (position 59 12))
      (reference
        (source (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))
        (kind flowTarget) (ordinal 2) (authored-target "distribute torque::driveshaftTorque")
        (range (start 59 12) (end 59 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque") (range (start 62 6) (end 62 34)))
        )
      )
    )
  )
)
~~~
