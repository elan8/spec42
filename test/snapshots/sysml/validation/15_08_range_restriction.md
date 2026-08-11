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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "05e2d9e7be9471b26f19a2a880272093da020c5ae1546c7b2adbfcb8113255f5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction"))) (kind "package") (name "15_08-Range Restriction") (declared-name "15_08-Range Restriction") (range (start (line 0) (character 0)) (end (line 0) (character 537))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 23))) (parent (node (document "d0") (qualified-name "15_08-Range Restriction"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 19))))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 22))) (parent (node (document "d0") (qualified-name "15_08-Range Restriction"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 18))))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob"))) (kind "part def") (name "HeadLightsTiltKnob") (declared-name "HeadLightsTiltKnob") (range (start (line 5) (character 1)) (end (line 5) (character 90))) (parent (node (document "d0") (qualified-name "15_08-Range Restriction"))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (kind "attribute") (name "headLightsTile") (declared-name "headLightsTile") (range (start (line 6) (character 2)) (end (line 6) (character 56))) (parent (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob"))) (authored (membership (kind Feature)) (relationships (typing (reference "LightBeamTiltAngleValue") (range none)) (typing (reference "LightBeamTiltAngleValue") (range (start (line 6) (character 29)) (end (line 6) (character 52)))))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (kind "attribute def") (name "LightBeamTiltAngleValue") (declared-name "LightBeamTiltAngleValue") (range (start (line 9) (character 1)) (end (line 9) (character 292))) (parent (node (document "d0") (qualified-name "15_08-Range Restriction"))) (authored (membership (kind Owning)) (relationships (typing (reference "PlaneAngleValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (kind "attribute") (name "angle") (declared-name "angle") (range (start (line 10) (character 2)) (end (line 10) (character 163))) (parent (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LightBeamTiltAngleValue") (range none)) (redefinition (reference "self") (range (start (line 10) (character 47)) (end (line 10) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle::_documentation"))) (kind "documentation") (name "") (range (start (line 10) (character 2)) (end (line 10) (character 163))) (parent (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))))
    (element (id (node (document "d0") (qualified-name "15_08-Range Restriction::pi"))) (kind "import") (name "pi") (declared-name "pi") (range (start (line 3) (character 1)) (end (line 3) (character 64))) (parent (node (document "d0") (qualified-name "15_08-Range Restriction"))) (authored (membership (kind Import) (visibility "private") (import (reference "15_01-Constants::Mathematical Constants::pi") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 63))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 1) (character 16)) (end (line 1) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 2) (character 16)) (end (line 2) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (kind featureTyping) (ordinal 0)) (authored-target "LightBeamTiltAngleValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (kind featureTyping) (ordinal 1)) (authored-target "LightBeamTiltAngleValue") (range (start (line 6) (character 29)) (end (line 6) (character 52))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (kind featureTyping) (ordinal 0)) (authored-target "PlaneAngleValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (kind featureTyping) (ordinal 0)) (authored-target "LightBeamTiltAngleValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (kind redefinition) (ordinal 0)) (authored-target "self") (range (start (line 10) (character 47)) (end (line 10) (character 51))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_08-Range Restriction::pi"))) (kind membershipImport) (ordinal 0)) (authored-target "15_01-Constants::Mathematical Constants::pi") (range (start (line 3) (character 16)) (end (line 3) (character 63))) (outcome (status unresolved)))
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
