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

    // Conforming: the performed feature is an action usage, accessible through Library.
    action def Good :> Library {
        perform Library::doIt;
    }

    // Invalid: the performed feature is a part usage, not an action usage.
    action def Bad :> Library {
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:0de1c273b51cac66f515cecdf3c6b7e15bd2ecb1ceefd3a240fbfb85a6c4fa02"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Bad"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Library")))))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (effective-identification (name "comp") (short-name absent) (provenance performed-action)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "Library::comp")))))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Good"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Library")))))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (effective-identification (name "doIt") (short-name absent) (provenance performed-action)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "Library::doIt")))))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::doIt"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Bad"))) (kind specialization) (ordinal 0))
      (authored-target "Library")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library")))))
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "Library::comp")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp")))))
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Good"))) (kind specialization) (ordinal 0))
      (authored-target "Library")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library")))))
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "Library::doIt")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::doIt")))))
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Bad"))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Bad"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Good"))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Good"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::doIt"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Bad"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Good"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::doIt"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Bad")))
      (supertype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Bad")))
      (effective-type (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")) (source inherited) (from (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp"))))
      (supertype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")))
      (subtype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Good")))
      (supertype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Good")))
      (supertype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::doIt")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library")))
      (subtype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Bad")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Good")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp")))
      (featured-by (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library")))
      (type (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Component")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::doIt")))
      (featured-by (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library")))
      (subtype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Good")) (anonymous (kind perform-action) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_perform_action_usage_reference.md") (range (start 13 22) (end 13 29)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Bad"))) (kind specialization) (ordinal 0) (authored-target "Library")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library")))))
    )
  )
  (query (document "memory://snapshot/sysml_perform_action_usage_reference.md") (range (start 14 16) (end 14 29)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Bad")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "Library::comp")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library::comp")))))
    )
  )
  (query (document "memory://snapshot/sysml_perform_action_usage_reference.md") (range (start 8 23) (end 8 30)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Good"))) (kind specialization) (ordinal 0) (authored-target "Library")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_perform_action_usage_reference.md") (qualified-name "Actions::Library")))))
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
