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
  (document "memory://snapshot/medical_device_failure.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 33))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 3 1) (end 22 2))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 4 2) (end 7 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 15 2) (end 15 17))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "parser")
        (range (start 16 3) (end 17 3))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 16 3) (end 17 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 21 2) (end 21 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 36) (end 21 58))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery,unsupported-syntax) (has-evaluation false) (source-digest "blake3:51366626473ebc97f3725151bcf295affa7c19f7c78f889c7f47421f9356db44") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "CauseAndEffect") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0))))) (kind connection) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "deviceFails")) (memberAccessOperand (reference "patient::therapyDelayed")))))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::cannotBeCharged"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::depleted"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient"))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient::therapyDelayed"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "CauseAndEffect")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "deviceFails")
      (outcome (status resolved) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails")))))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "patient::therapyDelayed")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery"))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::cannotBeCharged"))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::depleted"))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails"))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient"))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient::therapyDelayed"))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice")))
    )
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice")))
    )
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery")))
      (featured-by (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice")))
    )
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::cannotBeCharged")))
      (featured-by (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery")))
    )
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::depleted")))
      (featured-by (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery")))
    )
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails")))
      (featured-by (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice")))
    )
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient")))
      (featured-by (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice")))
    )
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient::therapyDelayed")))
      (featured-by (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/medical_device_failure.md") (range (start 1 16) (end 1 33)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "CauseAndEffect")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/medical_device_failure.md") (range (start 21 21) (end 21 32)) (probe (position 21 21))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "deviceFails")
      (outcome (status resolved) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails")))))
    )
  )
  (query (document "memory://snapshot/medical_device_failure.md") (range (start 21 36) (end 21 58)) (probe (position 21 36))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "patient::therapyDelayed")
      (outcome (status unresolved)))
    )
  )
)
~~~
