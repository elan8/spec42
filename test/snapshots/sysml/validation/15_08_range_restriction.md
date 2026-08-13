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
  (document "memory://snapshot/15_08_range_restriction.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 9 42) (end 9 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 47) (end 10 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 16 31) (end 16 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 16 54) (end 16 63))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:0a23013531728ed48d4775509e224cbd04a0426ce5db9318477f567c43e3d0c1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "15_01-Constants::Mathematical Constants::pi") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LightBeamTiltAngleValue"))))
    (declaration (id (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PlaneAngleValue"))))
    (declaration (id (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind constraint) (ordinal 0))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "angle")) (expressionOperand (reference "angle"))))
    (declaration (id (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LightBeamTiltAngleValue")) (redefinition (reference "self"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "15_01-Constants::Mathematical Constants::pi")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (kind featureTyping) (ordinal 0))
      (authored-target "LightBeamTiltAngleValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue")))))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (kind specialization) (ordinal 0))
      (authored-target "PlaneAngleValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "angle")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle")))))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "angle")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle")))))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (kind featureTyping) (ordinal 0))
      (authored-target "LightBeamTiltAngleValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue")))))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (target (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind constraint) (ordinal 0))))) (target (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind constraint) (ordinal 0))))) (target (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (target (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/15_08_range_restriction.md") (range (start 1 16) (end 1 22)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_08_range_restriction.md") (range (start 2 16) (end 2 21)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_08_range_restriction.md") (range (start 3 16) (end 3 63)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "15_01-Constants::Mathematical Constants::pi")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_08_range_restriction.md") (range (start 6 29) (end 6 52)) (probe (position 6 29))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (kind featureTyping) (ordinal 0) (authored-target "LightBeamTiltAngleValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue")))))
  )
  (query (document "memory://snapshot/15_08_range_restriction.md") (range (start 9 42) (end 9 57)) (probe (position 9 42))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (kind specialization) (ordinal 0) (authored-target "PlaneAngleValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_08_range_restriction.md") (range (start 16 22) (end 16 27)) (probe (position 16 22))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "angle")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle")))))
  )
  (query (document "memory://snapshot/15_08_range_restriction.md") (range (start 16 45) (end 16 50)) (probe (position 16 45))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "angle")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle")))))
  )
  (query (document "memory://snapshot/15_08_range_restriction.md") (range (start 10 19) (end 10 42)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (kind featureTyping) (ordinal 0) (authored-target "LightBeamTiltAngleValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue")))))
  )
  (query (document "memory://snapshot/15_08_range_restriction.md") (range (start 10 47) (end 10 51)) (probe (position 10 47))
    (reference (id (source (node (document "memory://snapshot/15_08_range_restriction.md") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
  )
)
~~~
