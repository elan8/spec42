# META
~~~ini
description=SysML Example (Cause and Effect): MedicalDeviceFailure
type=file
~~~
# SOURCE
~~~sysml
package MedicalDeviceFailure {
	private import CauseAndEffect::*;
	
	part medicalDevice {
		part battery {
			event occurrence depleted;
			event occurrence cannotBeCharged;
		}
		
		event occurrence deviceFails;
		
		ref patient {
			event occurrence therapyDelayed;
		}
		
		#multicausation connection {
			end #cause ::> battery.depleted;
			end #cause ::> battery.cannotBeCharged;
			end #effect ::> deviceFails;
		}
		
		#causation connect deviceFails to patient.therapyDelayed;
	}	
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "medical_device_failure.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 30))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package MedicalDeviceFailure {
    private import CauseAndEffect::*;

    part medicalDevice {
        part battery {
            event occurrence depleted;
            event occurrence cannotBeCharged;
        }

        event occurrence deviceFails;

        ref patient {
            event occurrence therapyDelayed;
        }

        #multicausation connection {
            end #cause ::> battery.depleted;
            end #cause ::> battery.cannotBeCharged;
            end #effect ::> deviceFails;
        }

        #causation connect deviceFails to patient.therapyDelayed;
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e82c988c61c63239d64c376d5dd98c95539cbdd3d0f642cb01e06d74448d9532") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure"))) (kind "package") (name "MedicalDeviceFailure") (declared-name "MedicalDeviceFailure") (range (start (line 0) (character 0)) (end (line 0) (character 491))))
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 34))) (parent (node (document "d0") (qualified-name "MedicalDeviceFailure"))) (authored (membership (kind Import) (visibility "private") (import (reference "CauseAndEffect::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 30))))))
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice"))) (kind "part") (name "medicalDevice") (declared-name "medicalDevice") (range (start (line 3) (character 1)) (end (line 3) (character 418))) (parent (node (document "d0") (qualified-name "MedicalDeviceFailure"))))
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::battery"))) (kind "part") (name "battery") (declared-name "battery") (range (start (line 4) (character 2)) (end (line 4) (character 87))) (parent (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice"))))
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::cannotBeCharged"))) (kind "occurrence") (name "cannotBeCharged") (declared-name "cannotBeCharged") (range (start (line 6) (character 20)) (end (line 6) (character 36))) (parent (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::battery"))))
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::depleted"))) (kind "occurrence") (name "depleted") (declared-name "depleted") (range (start (line 5) (character 20)) (end (line 5) (character 29))) (parent (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::battery"))))
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails"))) (kind "occurrence") (name "deviceFails") (declared-name "deviceFails") (range (start (line 9) (character 19)) (end (line 9) (character 31))) (parent (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice"))))
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::patient"))) (kind "ref") (name "patient") (declared-name "patient") (range (start (line 11) (character 2)) (end (line 11) (character 55))) (parent (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MedicalDeviceFailure::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "CauseAndEffect::*") (range (start (line 1) (character 16)) (end (line 1) (character 30))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
