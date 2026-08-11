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
  (document "4a_functional_allocation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 43) (end 5 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 3) (end 9 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 35 2) (end 35 398))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 4) (end 37 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 4) (end 50 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 54 2) (end 54 330))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 4) (end 56 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 4) (end 65 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 2) (end 69 333))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 4) (end 71 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 80 4) (end 80 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 84 2) (end 84 604))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 4) (end 86 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 98 6) (end 98 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 103 6) (end 103 30))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b5b72b3d36fa8932b2cb521bb6848d20d072bbfc01b01d2541a25db1d1d6656e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation"))) (kind "package") (name "4a-Functional Allocation") (declared-name "4a-Functional Allocation"))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "4a-Functional Allocation"))) (authored (membership (kind Import) (visibility "private") (import (reference "2a-Parts Interconnection::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "4a-Functional Allocation"))) (authored (membership (kind Import) (visibility "private") (import (reference "3a-Function-based Behavior-1::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "4a-Functional Allocation"))) (authored (membership (kind Import) (visibility "private") (import (reference "3a-Function-based Behavior-1::provide power::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (kind "part") (name "vehicle1_c1_functional_allocation") (declared-name "vehicle1_c1_functional_allocation") (parent (node (document "d0") (qualified-name "4a-Functional Allocation"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle1_c1")) (perform (reference "4a-Functional Allocation::vehicle1_c1_functional_allocation::provide power")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (kind "part") (name "driveshaft") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "driveshaft")) (perform (reference "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::provide power::transfer torque")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::provide power.transfer torque"))) (kind "action") (name "provide power.transfer torque") (declared-name "provide power.transfer torque") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))) (kind "port") (name "shaftPort_b") (declared-name "shaftPort_b") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "shaftPort_b")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))) (kind "port") (name "shaftPort_c") (declared-name "shaftPort_c") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "shaftPort_c")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c::driveshaftTorque"))) (kind "in out parameter") (name "driveshaftTorque") (declared-name "driveshaftTorque") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (kind "part") (name "engine") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine")) (perform (reference "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::provide power::generate torque")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))) (kind "port") (name "drivePwrPort") (declared-name "drivePwrPort") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "drivePwrPort")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelCmdPort")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))) (authored (relationships (typing (reference "FuelCmd")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::provide power.generate torque"))) (kind "action") (name "provide power.generate torque") (declared-name "provide power.generate torque") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelCmdPort")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))) (authored (relationships (typing (reference "FuelCmd")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearAxleAssembly")) (perform (reference "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::provide power::distribute torque")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::provide power.distribute torque"))) (kind "action") (name "provide power.distribute torque") (declared-name "provide power.distribute torque") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (kind "part") (name "rearAxle") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearAxle")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind "part") (name "leftHalfAxle") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "leftHalfAxle")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))) (kind "port") (name "axleToWheelPort") (declared-name "axleToWheelPort") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "axleToWheelPort")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort::wheelTorque"))) (kind "in out parameter") (name "wheelTorque") (declared-name "wheelTorque") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind "part") (name "rightHalfAxle") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rightHalfAxle")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))) (kind "port") (name "axleToWheelPort") (declared-name "axleToWheelPort") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "axleToWheelPort")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort::wheelTorque"))) (kind "in out parameter") (name "wheelTorque") (declared-name "wheelTorque") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))) (kind "port") (name "shaftPort_d") (declared-name "shaftPort_d") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "shaftPort_d")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d::driveshaftTorque"))) (kind "in out parameter") (name "driveshaftTorque") (declared-name "driveshaftTorque") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (kind "part") (name "transmission") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transmission")) (perform (reference "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::provide power::amplify torque")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "clutchPort")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))) (authored (relationships (typing (reference "Torque")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::provide power.amplify torque"))) (kind "action") (name "provide power.amplify torque") (declared-name "provide power.amplify torque") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))) (kind "port") (name "shaftPort_a") (declared-name "shaftPort_a") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "shaftPort_a")))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))) (authored (relationships (typing (reference "Torque")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "2a-Parts Interconnection::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "3a-Function-based Behavior-1::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "3a-Function-based Behavior-1::provide power::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle1_c1") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (kind performSource) (ordinal 0)) (authored-target "4a-Functional Allocation::vehicle1_c1_functional_allocation::provide power") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (kind redefinition) (ordinal 0)) (authored-target "driveshaft") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (kind performSource) (ordinal 0)) (authored-target "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::provide power::transfer torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))) (kind redefinition) (ordinal 0)) (authored-target "shaftPort_b") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))) (kind redefinition) (ordinal 0)) (authored-target "shaftPort_c") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c::driveshaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (kind performSource) (ordinal 0)) (authored-target "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::provide power::generate torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))) (kind redefinition) (ordinal 0)) (authored-target "drivePwrPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))) (kind redefinition) (ordinal 0)) (authored-target "fuelCmdPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))) (kind redefinition) (ordinal 0)) (authored-target "fuelCmdPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (kind redefinition) (ordinal 0)) (authored-target "rearAxleAssembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (kind performSource) (ordinal 0)) (authored-target "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::provide power::distribute torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (kind redefinition) (ordinal 0)) (authored-target "rearAxle") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind redefinition) (ordinal 0)) (authored-target "leftHalfAxle") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))) (kind redefinition) (ordinal 0)) (authored-target "axleToWheelPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort::wheelTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind redefinition) (ordinal 0)) (authored-target "rightHalfAxle") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))) (kind redefinition) (ordinal 0)) (authored-target "axleToWheelPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort::wheelTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))) (kind redefinition) (ordinal 0)) (authored-target "shaftPort_d") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d::driveshaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (kind redefinition) (ordinal 0)) (authored-target "transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (kind performSource) (ordinal 0)) (authored-target "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::provide power::amplify torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))) (kind redefinition) (ordinal 0)) (authored-target "clutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))) (kind redefinition) (ordinal 0)) (authored-target "shaftPort_a") (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::provide power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (kind performSource) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 35 11) (end 35 17)) (probe (position 35 11))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))
        (kind redefinition) (ordinal 0) (authored-target "engine")
        (range (start 35 11) (end 35 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine") (range (start 35 2) (end 35 398)))
        )
      )
    )
    (query (range (start 95 12) (end 95 20)) (probe (position 95 12))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))
        (kind redefinition) (ordinal 0) (authored-target "rearAxle")
        (range (start 95 12) (end 95 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle") (range (start 95 3) (end 95 236)))
        )
      )
    )
    (query (range (start 55 12) (end 55 22)) (probe (position 55 12))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))
        (kind redefinition) (ordinal 0) (authored-target "clutchPort")
        (range (start 55 12) (end 55 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort") (range (start 55 3) (end 55 68)))
        )
      )
    )
    (query (range (start 69 11) (end 69 21)) (probe (position 69 11))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))
        (kind redefinition) (ordinal 0) (authored-target "driveshaft")
        (range (start 69 11) (end 69 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft") (range (start 69 2) (end 69 333)))
        )
      )
    )
    (query (range (start 5 43) (end 5 54)) (probe (position 5 43))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle1_c1")
        (range (start 5 43) (end 5 54))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 11) (end 8 22)) (probe (position 8 11))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))
        (kind redefinition) (ordinal 0) (authored-target "fuelCmdPort")
        (range (start 8 11) (end 8 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort") (range (start 8 2) (end 8 52)))
        )
      )
    )
    (query (range (start 36 12) (end 36 23)) (probe (position 36 12))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))
        (kind redefinition) (ordinal 0) (authored-target "fuelCmdPort")
        (range (start 36 12) (end 36 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort") (range (start 36 3) (end 36 55)))
        )
      )
    )
    (query (range (start 64 12) (end 64 23)) (probe (position 64 12))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))
        (kind redefinition) (ordinal 0) (authored-target "shaftPort_a")
        (range (start 64 12) (end 64 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a") (range (start 64 3) (end 64 66)))
        )
      )
    )
    (query (range (start 70 12) (end 70 23)) (probe (position 70 12))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))
        (kind redefinition) (ordinal 0) (authored-target "shaftPort_b")
        (range (start 70 12) (end 70 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b") (range (start 70 3) (end 70 65)))
        )
      )
    )
    (query (range (start 79 12) (end 79 23)) (probe (position 79 12))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))
        (kind redefinition) (ordinal 0) (authored-target "shaftPort_c")
        (range (start 79 12) (end 79 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c") (range (start 79 3) (end 79 64)))
        )
      )
    )
    (query (range (start 85 12) (end 85 23)) (probe (position 85 12))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))
        (kind redefinition) (ordinal 0) (authored-target "shaftPort_d")
        (range (start 85 12) (end 85 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d") (range (start 85 3) (end 85 63)))
        )
      )
    )
    (query (range (start 49 12) (end 49 24)) (probe (position 49 12))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))
        (kind redefinition) (ordinal 0) (authored-target "drivePwrPort")
        (range (start 49 12) (end 49 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort") (range (start 49 3) (end 49 61)))
        )
      )
    )
    (query (range (start 54 11) (end 54 23)) (probe (position 54 11))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))
        (kind redefinition) (ordinal 0) (authored-target "transmission")
        (range (start 54 11) (end 54 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission") (range (start 54 2) (end 54 330)))
        )
      )
    )
    (query (range (start 96 13) (end 96 25)) (probe (position 96 13))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))
        (kind redefinition) (ordinal 0) (authored-target "leftHalfAxle")
        (range (start 96 13) (end 96 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle") (range (start 96 4) (end 96 103)))
        )
      )
    )
    (query (range (start 101 13) (end 101 26)) (probe (position 101 13))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))
        (kind redefinition) (ordinal 0) (authored-target "rightHalfAxle")
        (range (start 101 13) (end 101 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle") (range (start 101 4) (end 101 104)))
        )
      )
    )
    (query (range (start 97 14) (end 97 29)) (probe (position 97 14))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))
        (kind redefinition) (ordinal 0) (authored-target "axleToWheelPort")
        (range (start 97 14) (end 97 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort") (range (start 97 5) (end 97 69)))
        )
      )
    )
    (query (range (start 102 14) (end 102 29)) (probe (position 102 14))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))
        (kind redefinition) (ordinal 0) (authored-target "axleToWheelPort")
        (range (start 102 14) (end 102 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort") (range (start 102 5) (end 102 69)))
        )
      )
    )
    (query (range (start 84 11) (end 84 27)) (probe (position 84 11))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))
        (kind redefinition) (ordinal 0) (authored-target "rearAxleAssembly")
        (range (start 84 11) (end 84 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly") (range (start 84 2) (end 84 604)))
        )
      )
    )
    (query (range (start 1 16) (end 1 42)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "2a-Parts Interconnection::*")
        (range (start 1 16) (end 1 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 46)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "3a-Function-based Behavior-1::*")
        (range (start 2 16) (end 2 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 63)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "4a-Functional Allocation::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "3a-Function-based Behavior-1::provide power::*")
        (range (start 3 16) (end 3 63))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
