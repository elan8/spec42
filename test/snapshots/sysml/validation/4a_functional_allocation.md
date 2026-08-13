# META
~~~ini
description=SysML Validation (04-Functional Allocation): 4a-Functional Allocation
type=file
~~~
# SOURCE
~~~sysml
package '4a-Functional Allocation' {
	private import '2a-Parts Interconnection'::*;
	private import '3a-Function-based Behavior-1'::*;
	private import '3a-Function-based Behavior-1'::'provide power'::*;
		
	part vehicle1_c1_functional_allocation :> vehicle1_c1 {
		// Note: The definitions of the port types in '2a-Parts Interconnection' do not include 
		// flow properties.
		port :>> fuelCmdPort {
			in fuelCmd: FuelCmd;
		}

		perform 'provide power' {
		doc
		/*
		 * This allocates the action '3a-Function-based Behavior-1'::'provide power' as an enacted 
		 * performance of 'vehicle_c1_functional_allocation'.
		 */
		
			// This assigns the fuelCmdPort to provide the input to 'provide power'.
			in fuelCmd = fuelCmdPort.fuelCmd;
		}
		
		//*
		// The above is semantically equivalent to:
		
		ref action 'provide power' (in fuelCmd = fuelCmdPort::fuelCmd) 
		   :> '3a-Function-based Behavior'::'provide power', performedActions;		
			
		// For a composite enacted performance within the vehicle, replace the above with:
		
		action 'provide power' (in fuelCmd = fuelCmdPort::fuelCmd) 
		   :> '3a-Function-based Behavior'::'provide power';
		*/
		
		part :>> engine {
			port :>> fuelCmdPort {
				in fuelCmd: FuelCmd;
			}
			
			perform 'provide power'.'generate torque' {
				/*
				 *  This allocates one of the sub-steps of 'provide power' to a sub-part of vehicle_c1. 
				 */

				in fuelCmd = fuelCmdPort.fuelCmd;
				out engineTorque = drivePwrPort.engineTorque;
			}
			
			port :>> drivePwrPort {
				out engineTorque: Torque;
			}
		}
		
		part :>> transmission {
			port :>> clutchPort {
				in attribute engineTorque: Torque;
			}
			
			perform 'provide power'.'amplify torque' {
				in engineTorque = clutchPort.engineTorque; 
				out transmissionTorque = shaftPort_a.transmissionTorque;
			}

			port :>> shaftPort_a {
				out transmissionTorque: Torque;
			}
		}
		
		part :>> driveshaft {
			port :>> shaftPort_b {
				in transmissionTorque: Torque;
			}

			perform 'provide power'.'transfer torque' {
				in transmissionTorque = shaftPort_b.transmissionTorque; 
				out driveshaftTorque = shaftPort_c.driveshaftTorque;
			}

			port :>> shaftPort_c {
				out driveshaftTorque: Torque;
			}			
		}
		
		part :>> rearAxleAssembly {
			port :>> shaftPort_d {
				in driveshaftTorque: Torque;
			}
				
			perform 'provide power'.'distribute torque' {
				in driveshaftTorque = shaftPort_d.driveshaftTorque; 
				out wheelTorque1 = rearAxle.leftHalfAxle.axleToWheelPort.wheelTorque; 
				out wheelTorque2 = rearAxle.rightHalfAxle.axleToWheelPort.wheelTorque;
			}
			
			part :>> rearAxle {
				part :>> leftHalfAxle {
					port :>> axleToWheelPort {
						out wheelTorque: Torque;
					}
				}
				part :>> rightHalfAxle {
					port :>> axleToWheelPort {
						out wheelTorque: Torque;
					}
				}
			}
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/4a_functional_allocation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 5 43) (end 5 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 8 2) (end 10 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 12 2) (end 21 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 35 11) (end 35 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 36 3) (end 38 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 40 3) (end 47 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 49 3) (end 51 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 54 11) (end 54 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 55 3) (end 57 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 59 3) (end 62 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 64 3) (end 66 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 69 11) (end 69 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 70 3) (end 72 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 74 3) (end 77 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 79 3) (end 81 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 84 11) (end 84 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 85 3) (end 87 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 89 3) (end 93 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 95 12) (end 95 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 96 13) (end 96 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 97 5) (end 99 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 101 13) (end 101 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 102 5) (end 104 6))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:a7b42e00c05051ab29e9a1a513a40cbb0f30070ae3a8068d88fe91f863b2368b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (qualified-name "4a-Functional Allocation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "2a-Parts Interconnection") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "3a-Function-based Behavior-1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "3a-Function-based Behavior-1::provide power") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle1_c1"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transmission"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 2))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driveshaft"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 3))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rearAxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rearAxle"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "leftHalfAxle"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rightHalfAxle"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "2a-Parts Interconnection")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "3a-Function-based Behavior-1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "3a-Function-based Behavior-1::provide power")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle1_c1")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "engine")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "transmission")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "driveshaft")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 3))))) (kind redefinition) (ordinal 0))
      (authored-target "rearAxleAssembly")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "rearAxle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "leftHalfAxle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "rightHalfAxle")
      (outcome (status unsupported)))
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
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 1 16) (end 1 45)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "2a-Parts Interconnection")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 2 16) (end 2 49)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "3a-Function-based Behavior-1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 3 16) (end 3 66)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "3a-Function-based Behavior-1::provide power")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 5 43) (end 5 54)) (probe (position 5 43))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (kind subsetting) (ordinal 0) (authored-target "vehicle1_c1")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 35 11) (end 35 17)) (probe (position 35 11))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "engine")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 54 11) (end 54 23)) (probe (position 54 11))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "transmission")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 69 11) (end 69 21)) (probe (position 69 11))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "driveshaft")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 84 11) (end 84 27)) (probe (position 84 11))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 3))))) (kind redefinition) (ordinal 0) (authored-target "rearAxleAssembly")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 95 12) (end 95 20)) (probe (position 95 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "rearAxle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 96 13) (end 96 25)) (probe (position 96 13))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "leftHalfAxle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 101 13) (end 101 26)) (probe (position 101 13))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "rightHalfAxle")
      (outcome (status unsupported)))
  )
)
~~~
