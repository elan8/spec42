# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_08-Range Restriction
type=file
~~~
# SOURCE
~~~sysml
package '15_08-Range Restriction' {
	private import ISQ::*;
	private import SI::*;
	private import '15_01-Constants'::'Mathematical Constants'::pi;
	
	part def HeadLightsTiltKnob {
		attribute headLightsTile : LightBeamTiltAngleValue[1];
	}
	
	attribute def LightBeamTiltAngleValue :> PlaneAngleValue {
		attribute angle: LightBeamTiltAngleValue :>> self {
			doc
			/*
			 * Tilt angle shall be limited to the range between 50 and 80 degrees (inclusive).
			 */
		}
		assert constraint { angle >= 50 ['°'] and angle <= 80 ['°'] }
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_08_range_restriction.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 1) (end 9 292))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 47) (end 10 51))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "05e2d9e7be9471b26f19a2a880272093da020c5ae1546c7b2adbfcb8113255f5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction"))) (kind "package") (name "15_08-Range Restriction") (declared-name "15_08-Range Restriction"))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_08-Range Restriction"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_08-Range Restriction"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob"))) (kind "part def") (name "HeadLightsTiltKnob") (declared-name "HeadLightsTiltKnob") (parent (node (document "d0") (qualified-name "15_08-Range Restriction"))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (kind "attribute") (name "headLightsTile") (declared-name "headLightsTile") (parent (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob"))) (authored (membership (kind Feature)) (relationships (typing (reference "LightBeamTiltAngleValue")) (typing (reference "LightBeamTiltAngleValue")))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (kind "attribute def") (name "LightBeamTiltAngleValue") (declared-name "LightBeamTiltAngleValue") (parent (node (document "d0") (qualified-name "15_08-Range Restriction"))) (authored (membership (kind Owning)) (relationships (typing (reference "PlaneAngleValue")))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (kind "attribute") (name "angle") (declared-name "angle") (parent (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LightBeamTiltAngleValue")) (redefinition (reference "self")))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::pi"))) (kind "import") (name "pi") (declared-name "pi") (parent (node (document "d0") (qualified-name "15_08-Range Restriction"))) (authored (membership (kind Import) (visibility "private") (import (reference "15_01-Constants::Mathematical Constants::pi") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (kind featureTyping) (ordinal 0)) (authored-target "LightBeamTiltAngleValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (kind featureTyping) (ordinal 1)) (authored-target "LightBeamTiltAngleValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (kind featureTyping) (ordinal 0)) (authored-target "PlaneAngleValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (kind featureTyping) (ordinal 0)) (authored-target "LightBeamTiltAngleValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (kind redefinition) (ordinal 0)) (authored-target "self") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::pi"))) (kind membershipImport) (ordinal 0)) (authored-target "15_01-Constants::Mathematical Constants::pi") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (target (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (target (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (target (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 16) (end 2 18)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "15_08-Range Restriction::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 2 16) (end 2 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 19)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "15_08-Range Restriction::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 1 16) (end 1 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 47) (end 10 51)) (probe (position 10 47))
      (reference
        (source (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))
        (kind redefinition) (ordinal 0) (authored-target "self")
        (range (start 10 47) (end 10 51))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 29) (end 6 52)) (probe (position 6 29))
      (reference
        (source (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))
        (kind featureTyping) (ordinal 1) (authored-target "LightBeamTiltAngleValue")
        (range (start 6 29) (end 6 52))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue") (range (start 9 1) (end 9 292)))
        )
      )
    )
    (query (range (start 3 16) (end 3 63)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "15_08-Range Restriction::pi"))
        (kind membershipImport) (ordinal 0) (authored-target "15_01-Constants::Mathematical Constants::pi")
        (range (start 3 16) (end 3 63))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
