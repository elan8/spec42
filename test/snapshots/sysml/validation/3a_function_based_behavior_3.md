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
  (document "memory://snapshot/3a_function_based_behavior_3.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 19) (end 5 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 16 2) (end 16 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 17 2) (end 17 90))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 18 2) (end 18 95))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 19 2) (end 19 117))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 21 2) (end 21 105))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 27 2) (end 69 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:99fbe701c20e1659c4bb8ceb24e41a7f8035e87d918364db2980dad33650d58d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_3.md") (qualified-name "3a-Function-based Behavior-5"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_3.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_3.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Usages") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_3.md") (qualified-name "3a-Function-based Behavior-5::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_3.md") (qualified-name "3a-Function-based Behavior-5::Definitions::EngineOff"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_3.md") (qualified-name "3a-Function-based Behavior-5::Definitions::EngineStart"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_3.md") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_3.md") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "ISQ::TorqueValue"))))
    (declaration (id (node (document "memory://snapshot/3a_function_based_behavior_3.md") (qualified-name "3a-Function-based Behavior-5::Usages"))) (kind package) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_3.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_3.md") (qualified-name "3a-Function-based Behavior-5::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_3.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_3.md") (qualified-name "3a-Function-based Behavior-5::Usages")))))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_3.md") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (kind aliasBinding) (ordinal 0))
      (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/3a_function_based_behavior_3.md") (range (start 1 15) (end 1 29)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_3.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_3.md") (qualified-name "3a-Function-based Behavior-5::Definitions")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_3.md") (range (start 2 15) (end 2 24)) (probe (position 2 15))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_3.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/3a_function_based_behavior_3.md") (qualified-name "3a-Function-based Behavior-5::Usages")))))
  )
  (query (document "memory://snapshot/3a_function_based_behavior_3.md") (range (start 5 19) (end 5 35)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/3a_function_based_behavior_3.md") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (kind aliasBinding) (ordinal 0) (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
  )
)
~~~
