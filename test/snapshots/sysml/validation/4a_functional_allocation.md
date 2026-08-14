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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 15) (end 9 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 6) (end 20 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 16) (end 20 35))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 16) (end 37 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 45 7) (end 45 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 45 17) (end 45 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 46 8) (end 46 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 46 23) (end 46 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 49 12) (end 49 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 22) (end 50 28))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 31) (end 56 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 60 7) (end 60 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 60 22) (end 60 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 61 8) (end 61 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 61 29) (end 61 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 64 12) (end 64 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 28) (end 65 34))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 27) (end 71 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 75 7) (end 75 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 75 28) (end 75 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 76 8) (end 76 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 76 27) (end 76 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 79 12) (end 79 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 80 26) (end 80 32))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 25) (end 86 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 90 7) (end 90 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 90 26) (end 90 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 91 8) (end 91 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 91 23) (end 91 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 92 8) (end 92 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 92 23) (end 92 73))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 98 23) (end 98 29))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 103 23) (end 103 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a7b42e00c05051ab29e9a1a513a40cbb0f30070ae3a8068d88fe91f863b2368b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (qualified-name "4a-Functional Allocation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "2a-Parts Interconnection") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "3a-Function-based Behavior-1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "3a-Function-based Behavior-1::provide power") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle1_c1"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind port) (ordinal 0)))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelCmdPort"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind perform-action) (ordinal 0)))))) (kind perform-action) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t * This allocates the action '3a-Function-based Behavior-1'::'provide power' as an enacted \n\t\t * performance of 'vehicle_c1_functional_allocation'.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transmission"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driveshaft"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rearAxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind perform-parameter-binding) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "fuelCmdPort::fuelCmd")) (performParameterTarget (reference "fuelCmd"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 0)))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelCmdPort"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 1)))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "drivePwrPort"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 0)))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "clutchPort"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind perform-action) (ordinal 0)))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 1)))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shaftPort_a"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind port) (ordinal 0)))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shaftPort_b"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind perform-action) (ordinal 0)))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind port) (ordinal 1)))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shaftPort_c"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind port) (ordinal 0)))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shaftPort_d"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rearAxle"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind perform-parameter-binding) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "fuelCmdPort::fuelCmd")) (performParameterTarget (reference "fuelCmd"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind perform-parameter-binding) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "drivePwrPort::engineTorque")) (performParameterTarget (reference "engineTorque"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind perform-parameter-binding) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "clutchPort::engineTorque")) (performParameterTarget (reference "engineTorque"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind perform-parameter-binding) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "shaftPort_a::transmissionTorque")) (performParameterTarget (reference "transmissionTorque"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind perform-parameter-binding) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "shaftPort_b::transmissionTorque")) (performParameterTarget (reference "transmissionTorque"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind perform-parameter-binding) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "shaftPort_c::driveshaftTorque")) (performParameterTarget (reference "driveshaftTorque"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind perform-parameter-binding) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "shaftPort_d::driveshaftTorque")) (performParameterTarget (reference "driveshaftTorque"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind perform-parameter-binding) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "rearAxle::leftHalfAxle::axleToWheelPort::wheelTorque")) (performParameterTarget (reference "wheelTorque1"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 2)))))) (kind perform-parameter-binding) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "rearAxle::rightHalfAxle::axleToWheelPort::wheelTorque")) (performParameterTarget (reference "wheelTorque2"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "leftHalfAxle"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rightHalfAxle"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 0)))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "axleToWheelPort"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 0)))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "axleToWheelPort"))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "wheelTorque")))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "wheelTorque")))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind port) (ordinal 1)) (named (kind parameter) (name "driveshaftTorque")))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "driveshaftTorque")))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction in))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 1)) (named (kind parameter) (name "engineTorque")))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "engineTorque")))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction in))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "fuelCmd")))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmd") (direction in))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 1)) (named (kind parameter) (name "transmissionTorque")))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "transmissionTorque")))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Torque") (direction in))))
    (declaration (id (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "fuelCmd")))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmd") (direction in))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "2a-Parts Interconnection")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "3a-Function-based Behavior-1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "3a-Function-based Behavior-1::provide power")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle1_c1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind port) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelCmdPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)))))) (kind redefinition) (ordinal 0))
      (authored-target "transmission")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)))))) (kind redefinition) (ordinal 0))
      (authored-target "driveshaft")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)))))) (kind redefinition) (ordinal 0))
      (authored-target "rearAxleAssembly")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelCmdPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 1)))))) (kind redefinition) (ordinal 0))
      (authored-target "drivePwrPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "clutchPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 1)))))) (kind redefinition) (ordinal 0))
      (authored-target "shaftPort_a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind port) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "shaftPort_b")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind port) (ordinal 1)))))) (kind redefinition) (ordinal 0))
      (authored-target "shaftPort_c")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind port) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "shaftPort_d")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "rearAxle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "fuelCmdPort::fuelCmd")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind performParameterTarget) (ordinal 0))
      (authored-target "fuelCmd")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "leftHalfAxle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)))))) (kind redefinition) (ordinal 0))
      (authored-target "rightHalfAxle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "fuelCmdPort::fuelCmd")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "drivePwrPort::engineTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "clutchPort::engineTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "shaftPort_a::transmissionTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "shaftPort_b::transmissionTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "shaftPort_c::driveshaftTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "shaftPort_d::driveshaftTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "rearAxle::leftHalfAxle::axleToWheelPort::wheelTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 2)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "rearAxle::rightHalfAxle::axleToWheelPort::wheelTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind performParameterTarget) (ordinal 0))
      (authored-target "fuelCmd")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind performParameterTarget) (ordinal 0))
      (authored-target "engineTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind performParameterTarget) (ordinal 0))
      (authored-target "engineTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind performParameterTarget) (ordinal 0))
      (authored-target "transmissionTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind performParameterTarget) (ordinal 0))
      (authored-target "transmissionTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind performParameterTarget) (ordinal 0))
      (authored-target "driveshaftTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind performParameterTarget) (ordinal 0))
      (authored-target "driveshaftTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind performParameterTarget) (ordinal 0))
      (authored-target "wheelTorque1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 2)))))) (kind performParameterTarget) (ordinal 0))
      (authored-target "wheelTorque2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "axleToWheelPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "axleToWheelPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "wheelTorque")))))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "wheelTorque")))))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind port) (ordinal 1)) (named (kind parameter) (name "driveshaftTorque")))))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "driveshaftTorque")))))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 1)) (named (kind parameter) (name "engineTorque")))))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "engineTorque")))))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "fuelCmd")))))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmd")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 1)) (named (kind parameter) (name "transmissionTorque")))))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "transmissionTorque")))))) (kind featureTyping) (ordinal 0))
      (authored-target "Torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "fuelCmd")))))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmd")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 2)))))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 1 16) (end 1 45)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "2a-Parts Interconnection")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 2 16) (end 2 49)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "3a-Function-based Behavior-1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 3 16) (end 3 66)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0) (authored-target "3a-Function-based Behavior-1::provide power")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 5 43) (end 5 54)) (probe (position 5 43))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (kind subsetting) (ordinal 0) (authored-target "vehicle1_c1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 8 11) (end 8 22)) (probe (position 8 11))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind port) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "fuelCmdPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 35 11) (end 35 17)) (probe (position 35 11))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "engine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 54 11) (end 54 23)) (probe (position 54 11))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)))))) (kind redefinition) (ordinal 0) (authored-target "transmission")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 69 11) (end 69 21)) (probe (position 69 11))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)))))) (kind redefinition) (ordinal 0) (authored-target "driveshaft")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 84 11) (end 84 27)) (probe (position 84 11))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)))))) (kind redefinition) (ordinal 0) (authored-target "rearAxleAssembly")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 36 12) (end 36 23)) (probe (position 36 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "fuelCmdPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 49 12) (end 49 24)) (probe (position 49 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 1)))))) (kind redefinition) (ordinal 0) (authored-target "drivePwrPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 55 12) (end 55 22)) (probe (position 55 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "clutchPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 64 12) (end 64 23)) (probe (position 64 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 1)))))) (kind redefinition) (ordinal 0) (authored-target "shaftPort_a")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 70 12) (end 70 23)) (probe (position 70 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind port) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "shaftPort_b")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 79 12) (end 79 23)) (probe (position 79 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind port) (ordinal 1)))))) (kind redefinition) (ordinal 0) (authored-target "shaftPort_c")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 85 12) (end 85 23)) (probe (position 85 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind port) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "shaftPort_d")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 95 12) (end 95 20)) (probe (position 95 12))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "rearAxle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 20 16) (end 20 35)) (probe (position 20 16))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "fuelCmdPort::fuelCmd")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 20 6) (end 20 13)) (probe (position 20 6))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind performParameterTarget) (ordinal 0) (authored-target "fuelCmd")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 96 13) (end 96 25)) (probe (position 96 13))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "leftHalfAxle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 101 13) (end 101 26)) (probe (position 101 13))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)))))) (kind redefinition) (ordinal 0) (authored-target "rightHalfAxle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 45 17) (end 45 36)) (probe (position 45 17))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "fuelCmdPort::fuelCmd")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 46 23) (end 46 48)) (probe (position 46 23))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind expressionOperand) (ordinal 0) (authored-target "drivePwrPort::engineTorque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 60 22) (end 60 45)) (probe (position 60 22))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "clutchPort::engineTorque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 61 29) (end 61 59)) (probe (position 61 29))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind expressionOperand) (ordinal 0) (authored-target "shaftPort_a::transmissionTorque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 75 28) (end 75 58)) (probe (position 75 28))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "shaftPort_b::transmissionTorque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 76 27) (end 76 55)) (probe (position 76 27))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind expressionOperand) (ordinal 0) (authored-target "shaftPort_c::driveshaftTorque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 90 26) (end 90 54)) (probe (position 90 26))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "shaftPort_d::driveshaftTorque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 91 23) (end 91 72)) (probe (position 91 23))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind expressionOperand) (ordinal 0) (authored-target "rearAxle::leftHalfAxle::axleToWheelPort::wheelTorque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 92 23) (end 92 73)) (probe (position 92 23))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 2)))))) (kind expressionOperand) (ordinal 0) (authored-target "rearAxle::rightHalfAxle::axleToWheelPort::wheelTorque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 45 7) (end 45 14)) (probe (position 45 7))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind performParameterTarget) (ordinal 0) (authored-target "fuelCmd")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 46 8) (end 46 20)) (probe (position 46 8))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind performParameterTarget) (ordinal 0) (authored-target "engineTorque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 60 7) (end 60 19)) (probe (position 60 7))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind performParameterTarget) (ordinal 0) (authored-target "engineTorque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 61 8) (end 61 26)) (probe (position 61 8))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind performParameterTarget) (ordinal 0) (authored-target "transmissionTorque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 75 7) (end 75 25)) (probe (position 75 7))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind performParameterTarget) (ordinal 0) (authored-target "transmissionTorque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 76 8) (end 76 24)) (probe (position 76 8))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind performParameterTarget) (ordinal 0) (authored-target "driveshaftTorque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 90 7) (end 90 23)) (probe (position 90 7))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 0)))))) (kind performParameterTarget) (ordinal 0) (authored-target "driveshaftTorque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 91 8) (end 91 20)) (probe (position 91 8))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 1)))))) (kind performParameterTarget) (ordinal 0) (authored-target "wheelTorque1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 92 8) (end 92 20)) (probe (position 92 8))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind perform-parameter-binding) (ordinal 2)))))) (kind performParameterTarget) (ordinal 0) (authored-target "wheelTorque2")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 97 14) (end 97 29)) (probe (position 97 14))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "axleToWheelPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 102 14) (end 102 29)) (probe (position 102 14))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "axleToWheelPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 98 23) (end 98 29)) (probe (position 98 23))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "wheelTorque")))))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 103 23) (end 103 29)) (probe (position 103 23))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "wheelTorque")))))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 80 26) (end 80 32)) (probe (position 80 26))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind port) (ordinal 1)) (named (kind parameter) (name "driveshaftTorque")))))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 86 25) (end 86 31)) (probe (position 86 25))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 3)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "driveshaftTorque")))))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 50 22) (end 50 28)) (probe (position 50 22))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 1)) (named (kind parameter) (name "engineTorque")))))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 56 31) (end 56 37)) (probe (position 56 31))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "engineTorque")))))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 37 16) (end 37 23)) (probe (position 37 16))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 0)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "fuelCmd")))))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmd")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 65 28) (end 65 34)) (probe (position 65 28))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 1)) (anonymous (kind port) (ordinal 1)) (named (kind parameter) (name "transmissionTorque")))))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 71 27) (end 71 33)) (probe (position 71 27))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind part) (ordinal 2)) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "transmissionTorque")))))) (kind featureTyping) (ordinal 0) (authored-target "Torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/4a_functional_allocation.md") (range (start 9 15) (end 9 22)) (probe (position 9 15))
    (reference (id (source (node (document "memory://snapshot/4a_functional_allocation.md") (path (named (kind package) (name "4a-Functional Allocation")) (named (kind part) (name "vehicle1_c1_functional_allocation")) (anonymous (kind port) (ordinal 0)) (named (kind parameter) (name "fuelCmd")))))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmd")
      (outcome (status unresolved)))
  )
)
~~~
