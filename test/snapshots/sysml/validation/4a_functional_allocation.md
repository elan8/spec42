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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 43) (end 5 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 11) (end 8 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 9 3) (end 9 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 20 3) (end 20 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 35 11) (end 35 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 12) (end 36 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 37 4) (end 37 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 45 4) (end 45 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 46 4) (end 46 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 49 12) (end 49 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 50 4) (end 50 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 54 11) (end 54 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 12) (end 55 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 56 4) (end 56 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 60 4) (end 60 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 61 4) (end 61 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 64 12) (end 64 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 65 4) (end 65 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 11) (end 69 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 70 12) (end 70 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 71 4) (end 71 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 75 4) (end 75 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 76 4) (end 76 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 79 12) (end 79 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 80 4) (end 80 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 84 11) (end 84 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 85 12) (end 85 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 86 4) (end 86 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 90 4) (end 90 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 91 4) (end 91 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 92 4) (end 92 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 95 12) (end 95 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 96 13) (end 96 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 97 14) (end 97 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 98 6) (end 98 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 101 13) (end 101 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 102 14) (end 102 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 103 6) (end 103 30))
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
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelCmdPort"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transmission"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 2))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driveshaft"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 3))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rearAxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelCmdPort"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 1))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "drivePwrPort"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "clutchPort"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 1))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shaftPort_a"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shaftPort_b"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 1))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shaftPort_c"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shaftPort_d"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rearAxle"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "leftHalfAxle"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rightHalfAxle"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "axleToWheelPort"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "axleToWheelPort"))))
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
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelCmdPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "transmission")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "driveshaft")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 3))))) (kind redefinition) (ordinal 0))
      (authored-target "rearAxleAssembly")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelCmdPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "drivePwrPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "clutchPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "shaftPort_a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "shaftPort_b")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "shaftPort_c")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "shaftPort_d")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "rearAxle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "leftHalfAxle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "rightHalfAxle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "axleToWheelPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "axleToWheelPort")
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
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 8 11) (end 8 22)) (probe (position 8 11))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "fuelCmdPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 35 11) (end 35 17)) (probe (position 35 11))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "engine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 54 11) (end 54 23)) (probe (position 54 11))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "transmission")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 69 11) (end 69 21)) (probe (position 69 11))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "driveshaft")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 84 11) (end 84 27)) (probe (position 84 11))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 3))))) (kind redefinition) (ordinal 0) (authored-target "rearAxleAssembly")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 36 12) (end 36 23)) (probe (position 36 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "fuelCmdPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 49 12) (end 49 24)) (probe (position 49 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "drivePwrPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 55 12) (end 55 22)) (probe (position 55 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "clutchPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 64 12) (end 64 23)) (probe (position 64 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "shaftPort_a")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 70 12) (end 70 23)) (probe (position 70 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "shaftPort_b")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 79 12) (end 79 23)) (probe (position 79 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "shaftPort_c")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 85 12) (end 85 23)) (probe (position 85 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "shaftPort_d")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 95 12) (end 95 20)) (probe (position 95 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "rearAxle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 96 13) (end 96 25)) (probe (position 96 13))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "leftHalfAxle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 101 13) (end 101 26)) (probe (position 101 13))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "rightHalfAxle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 97 14) (end 97 29)) (probe (position 97 14))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "axleToWheelPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 102 14) (end 102 29)) (probe (position 102 14))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "axleToWheelPort")
      (outcome (status unresolved)))
  )
)
~~~
