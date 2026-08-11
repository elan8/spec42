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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e82c988c61c63239d64c376d5dd98c95539cbdd3d0f642cb01e06d74448d9532") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure"))) (kind "package") (name "MedicalDeviceFailure") (declared-name "MedicalDeviceFailure"))
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "MedicalDeviceFailure"))) (authored (membership (kind Import) (visibility "private") (import (reference "CauseAndEffect::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice"))) (kind "part") (name "medicalDevice") (declared-name "medicalDevice") (parent (node (document "d0") (qualified-name "MedicalDeviceFailure"))))
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::battery"))) (kind "part") (name "battery") (declared-name "battery") (parent (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice"))))
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::cannotBeCharged"))) (kind "occurrence") (name "cannotBeCharged") (declared-name "cannotBeCharged") (parent (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::battery"))))
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::depleted"))) (kind "occurrence") (name "depleted") (declared-name "depleted") (parent (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::battery"))))
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails"))) (kind "occurrence") (name "deviceFails") (declared-name "deviceFails") (parent (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice"))))
    (element (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::patient"))) (kind "ref") (name "patient") (declared-name "patient") (parent (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MedicalDeviceFailure::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "CauseAndEffect::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
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
  (document "d0"
    (query (range (start 1 16) (end 1 30)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "MedicalDeviceFailure::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "CauseAndEffect::*")
        (range (start 1 16) (end 1 30))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
