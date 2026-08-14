# META
~~~ini
description=SysML Training 41 (Language Extension): User Keyword Example
type=file
~~~
# SOURCE
~~~sysml
package 'User Keyword Example' {
	private import ScalarValues::Real;
	private import 'Semantic Metadata Example'::*;
	private import RiskMetadata::LevelEnum;
	
	part def Device {
		part battery {
			attribute power : Real;
		}
	}
	
	#scenario def DeviceFailure {
		ref device : Device;
		attribute minPower : Real;
		
		#cause 'battery old' {
			:>> probability = 0.01;			
		}
		
		#causation connect 'battery old' to 'power low';
		
		#situation 'power low' {
			constraint { device.battery.power < minPower }			
		}
		
		#causation connect 'power low' to 'device shutoff';
		
		#failure 'device shutoff' {
			:>> severity = LevelEnum::high;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/41_user_keyword_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
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
        (range (start 3 16) (end 3 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 21) (end 7 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 23) (end 13 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "parser")
        (range (start 15 2) (end 19 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 19 2) (end 19 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 21) (end 19 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 38) (end 19 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "parser")
        (range (start 21 2) (end 25 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 25 2) (end 25 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 21) (end 25 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 36) (end 25 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "parser")
        (range (start 27 2) (end 30 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:b67a8c645e7b6c2fe182f63260be6d01a995d648542e390c9f164fa4ad04810c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Semantic Metadata Example") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "RiskMetadata::LevelEnum") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device::battery"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device::battery::power"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (named (kind extended-definition) (name "DeviceFailure")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "battery old")) (connectorEnd (reference "power low")))))
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (named (kind extended-definition) (name "DeviceFailure")) (anonymous (kind bare-connect) (ordinal 1))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "power low")) (connectorEnd (reference "device shutoff")))))
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure::device"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Device")))))
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure::minPower"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Semantic Metadata Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "RiskMetadata::LevelEnum")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device::battery::power"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (named (kind extended-definition) (name "DeviceFailure")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "battery old")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (named (kind extended-definition) (name "DeviceFailure")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connectorEnd) (ordinal 0))
      (authored-target "power low")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (named (kind extended-definition) (name "DeviceFailure")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "power low")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (named (kind extended-definition) (name "DeviceFailure")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connectorEnd) (ordinal 1))
      (authored-target "device shutoff")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure::device"))) (kind featureTyping) (ordinal 0))
      (authored-target "Device")
      (outcome (status resolved) (target (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device")))))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure::minPower"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure::device"))) (target (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure::device"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device")))
      (subtype (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure::device")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device::battery")))
      (featured-by (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device")))
    )
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device::battery::power")))
      (featured-by (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device::battery")))
    )
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (named (kind extended-definition) (name "DeviceFailure")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure")))
    )
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (named (kind extended-definition) (name "DeviceFailure")) (anonymous (kind bare-connect) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure")))
    )
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure::device")))
      (featured-by (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure")))
      (type (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device")) (provenance authored))
      (effective-type (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device")) (source direct))
      (supertype (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure::minPower")))
      (featured-by (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/41_user_keyword_example.md") (range (start 2 16) (end 2 46)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Semantic Metadata Example")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/41_user_keyword_example.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/41_user_keyword_example.md") (range (start 3 16) (end 3 39)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "RiskMetadata::LevelEnum")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/41_user_keyword_example.md") (range (start 7 21) (end 7 25)) (probe (position 7 21))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device::battery::power"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/41_user_keyword_example.md") (range (start 19 21) (end 19 34)) (probe (position 19 21))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (named (kind extended-definition) (name "DeviceFailure")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "battery old")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/41_user_keyword_example.md") (range (start 25 21) (end 25 32)) (probe (position 25 21))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (named (kind extended-definition) (name "DeviceFailure")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connectorEnd) (ordinal 0) (authored-target "power low")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/41_user_keyword_example.md") (range (start 19 38) (end 19 49)) (probe (position 19 38))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (named (kind extended-definition) (name "DeviceFailure")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "power low")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/41_user_keyword_example.md") (range (start 25 36) (end 25 52)) (probe (position 25 36))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (path (named (kind package) (name "User Keyword Example")) (named (kind extended-definition) (name "DeviceFailure")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connectorEnd) (ordinal 1) (authored-target "device shutoff")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/41_user_keyword_example.md") (range (start 12 15) (end 12 21)) (probe (position 12 15))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure::device"))) (kind featureTyping) (ordinal 0) (authored-target "Device")
      (outcome (status resolved) (target (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::Device")))))
    )
  )
  (query (document "memory://snapshot/41_user_keyword_example.md") (range (start 13 23) (end 13 27)) (probe (position 13 23))
    (reference (id (source (node (document "memory://snapshot/41_user_keyword_example.md") (qualified-name "User Keyword Example::DeviceFailure::minPower"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
)
~~~
