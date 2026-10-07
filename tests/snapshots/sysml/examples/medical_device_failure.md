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
        (severity warning)
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 3) (end 15 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 8) (end 16 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 8) (end 17 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 8) (end 18 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 3) (end 21 12))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e7fcae56aad0159a9634836fd9ad7460442b6bdcc66108cd18338271005cb00c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "CauseAndEffect") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0))))) (kind connection) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "deviceFails")) (memberAccessOperand (reference "patient::therapyDelayed")))))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "multicausation")))))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "battery::depleted")))))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "battery::cannotBeCharged")))))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 2))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 2)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "deviceFails")))))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "causation")))))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "cause")))))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "cause")))))
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 2)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "effect")))))
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
      (outcome (status resolved) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient::therapyDelayed")))))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "battery::depleted")
      (outcome (status resolved) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::depleted")))))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (kind connectorEnd) (ordinal 0))
      (authored-target "battery::cannotBeCharged")
      (outcome (status resolved) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::cannotBeCharged")))))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 2))))) (kind connectorEnd) (ordinal 0))
      (authored-target "deviceFails")
      (outcome (status resolved) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails")))))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "multicausation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "causation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "cause")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "cause")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 2)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "effect")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient::therapyDelayed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::depleted"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::cannotBeCharged"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 2))))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 2))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (target (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 2))))) (target (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0))))) (provenance implied))
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
      (positional-ends (authored 3) (effective 3))
      (featured-by (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice")))
    )
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice")))
    )
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)))))
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
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0))))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 2))))) (form prefix-keyword) (definition unresolved))
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0))))) (kind connection) (end (name (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (feature-chain (root (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery"))) (terminal (resolved (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::depleted")))) (path (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery")) (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::depleted"))) "battery::depleted")) (end (name (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (feature-chain (root (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery"))) (terminal (resolved (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::cannotBeCharged")))) (path (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery")) (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::cannotBeCharged"))) "battery::cannotBeCharged")) (end (name (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 2))))) (feature (resolved (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails"))))))
  (connector (id (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connection) (end bare (feature (resolved (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails"))))) (end bare (feature-chain (root (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient"))) (terminal (resolved (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient::therapyDelayed")))) (path (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient")) (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient::therapyDelayed"))) "patient::therapyDelayed")))
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
      (outcome (status resolved) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::patient::therapyDelayed")))))
    )
  )
  (query (document "memory://snapshot/medical_device_failure.md") (range (start 16 18) (end 16 34)) (probe (position 16 18))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "battery::depleted")
      (outcome (status resolved) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::depleted")))))
    )
  )
  (query (document "memory://snapshot/medical_device_failure.md") (range (start 17 18) (end 17 41)) (probe (position 17 18))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (kind connectorEnd) (ordinal 0) (authored-target "battery::cannotBeCharged")
      (outcome (status resolved) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::cannotBeCharged")))))
    )
  )
  (query (document "memory://snapshot/medical_device_failure.md") (range (start 18 19) (end 18 30)) (probe (position 18 19))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 2))))) (kind connectorEnd) (ordinal 0) (authored-target "deviceFails")
      (outcome (status resolved) (target (node (document "memory://snapshot/medical_device_failure.md") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails")))))
    )
  )
  (query (document "memory://snapshot/medical_device_failure.md") (range (start 15 3) (end 15 17)) (probe (position 15 3))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "multicausation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/medical_device_failure.md") (range (start 21 3) (end 21 12)) (probe (position 21 3))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind bare-connect) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "causation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/medical_device_failure.md") (range (start 16 8) (end 16 13)) (probe (position 16 8))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "cause")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/medical_device_failure.md") (range (start 17 8) (end 17 13)) (probe (position 17 8))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "cause")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/medical_device_failure.md") (range (start 18 8) (end 18 14)) (probe (position 18 8))
    (reference (id (source (node (document "memory://snapshot/medical_device_failure.md") (path (named (kind package) (name "MedicalDeviceFailure")) (named (kind part) (name "medicalDevice")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 2)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "effect")
      (outcome (status unresolved)))
    )
  )
)
~~~
