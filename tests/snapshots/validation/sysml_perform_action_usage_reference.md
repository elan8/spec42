# META
~~~ini
description=SysML 8.3.17.14 validatePerformActionUsageReference requires the featureTarget of the referencedFeature of a PerformActionUsage ownedReferenceSubsetting to be an ActionUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.14 validatePerformActionUsageReference
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.14:validatePerformActionUsageReference
type=file
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
        (range (start 14 16) (end 14 29))
        (related-information
          (related
            (uri "memory://snapshot/sysml_perform_action_usage_reference.md")
            (range (start 4 8) (end 4 30))
          )
        )
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
        (severity warning)
        (code "perform_target_invalid_kind")
        (source "semantic")
        (range (start 14 16) (end 14 29))
        (related-information
          (related
            (uri "memory://snapshot/sysml_perform_action_usage_reference.md")
            (range (start 4 8) (end 4 30))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e61bf85f22395bfc50dfab9841fecfe1d395a79dc0c0c53bcd213897a02c0ddd"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Bad"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "Library::comp")))))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Good"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "Library::doIt")))))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::doIt"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "Library::comp")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp")))))
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "Library::doIt")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::doIt")))))
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")))))
  )
  (relationships
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::doIt"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::doIt"))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")))
      (subtype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Good")))
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
  (query (document "memory://snapshot/sysml_perform_action_usage_reference.md") (range (start 14 16) (end 14 29)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "Library::comp")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp")))))
    )
  )
  (query (document "memory://snapshot/sysml_perform_action_usage_reference.md") (range (start 9 16) (end 9 29)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "Library::doIt")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::doIt")))))
    )
  )
  (query (document "memory://snapshot/sysml_perform_action_usage_reference.md") (range (start 4 20) (end 4 29)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")))))
    )
  )
)
~~~
