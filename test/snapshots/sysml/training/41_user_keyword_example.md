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
  (document "41_user_keyword_example.md"
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
        (range (start 2 16) (end 2 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 39))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 11 11) (end 11 418))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "8e134a3b55d7ad69ae40419928e2f634e648a5d66cdf3a414e58beda17f9d935") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "User Keyword Example"))) (kind "package") (name "User Keyword Example") (declared-name "User Keyword Example"))
    (element (id (node (document "d0") (qualified-name "User Keyword Example::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "User Keyword Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Semantic Metadata Example::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "User Keyword Example::Device"))) (kind "part def") (name "Device") (declared-name "Device") (parent (node (document "d0") (qualified-name "User Keyword Example"))))
    (element (id (node (document "d0") (qualified-name "User Keyword Example::Device::battery"))) (kind "part") (name "battery") (declared-name "battery") (parent (node (document "d0") (qualified-name "User Keyword Example::Device"))))
    (element (id (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (kind "attribute") (name "power") (declared-name "power") (parent (node (document "d0") (qualified-name "User Keyword Example::Device::battery"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "User Keyword Example::LevelEnum"))) (kind "import") (name "LevelEnum") (declared-name "LevelEnum") (parent (node (document "d0") (qualified-name "User Keyword Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "RiskMetadata::LevelEnum") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "User Keyword Example::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "User Keyword Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "User Keyword Example::_scenario"))) (kind "metadata keyword") (name "scenario") (declared-name "scenario") (parent (node (document "d0") (qualified-name "User Keyword Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "User Keyword Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Semantic Metadata Example::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "User Keyword Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "User Keyword Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "User Keyword Example::LevelEnum"))) (kind membershipImport) (ordinal 0)) (authored-target "RiskMetadata::LevelEnum") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "User Keyword Example::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (target (node (document "d0") (qualified-name "User Keyword Example::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (target (node (document "d0") (qualified-name "User Keyword Example::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (kind featureTyping) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 21) (end 7 25)) (probe (position 7 21))
      (reference
        (source (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 7 21) (end 7 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "User Keyword Example::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "User Keyword Example::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 39)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "User Keyword Example::LevelEnum"))
        (kind membershipImport) (ordinal 0) (authored-target "RiskMetadata::LevelEnum")
        (range (start 3 16) (end 3 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 43)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "User Keyword Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Semantic Metadata Example::*")
        (range (start 2 16) (end 2 43))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
