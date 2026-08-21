# META
~~~ini
description=SysML 8.3.17.14 validatePerformActionUsageReference requires the featureTarget of the referencedFeature of a PerformActionUsage ownedReferenceSubsetting to be an ActionUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.14 validatePerformActionUsageReference
type=file
skip_validation=the pinned parser rejects `perform <action usage>` -- the conforming side is reported as recovered_action_body_element -- and perform_target_invalid_kind is not raised for the non-action target that does parse
~~~
# SOURCE
~~~sysml
package Actions {
    part def Component;
    action def Library {
        action doIt;
        part comp : Component;
    }

    // Conforming: the performed feature is an action usage.
    action def Good {
        perform Library::doIt;
    }

    // Invalid: the performed feature is a part usage, not an action usage.
    action def Bad {
        perform Library::comp;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_perform_action_usage_reference.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "perform_target_invalid_kind")
        (source "semantic")
        (range (start 14 8) (end 14 30))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_perform_action_usage_reference.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 9 8) (end 10 4))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 9 8) (end 10 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:e61bf85f22395bfc50dfab9841fecfe1d395a79dc0c0c53bcd213897a02c0ddd") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Bad"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Good"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::doIt"))) (kind action) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")))
      (subtype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp")))
      (featured-by (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library")))
      (type (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::doIt")))
      (featured-by (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_perform_action_usage_reference.md") (range (start 4 20) (end 4 29)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")))))
    )
  )
)
~~~
