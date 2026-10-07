# META
~~~ini
description=SysML 8.3.18.2 validateExhibitStateUsageReference requires the featureTarget of the referencedFeature of an ExhibitStateUsage ownedReferenceSubsetting to be a StateUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.2 validateExhibitStateUsageReference
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.18.2:validateExhibitStateUsageReference
type=file
~~~
# SOURCE
~~~sysml
package States {
    part def Base {
        state operating;
        action inner;
    }
    part def Holder :> Base {
        // Conforming: the exhibited feature is a state usage.
        exhibit Base::operating;

        // Invalid: the exhibited feature is an action usage, not a state usage.
        exhibit Base::inner;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_exhibit_state_usage_reference.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "exhibit_target_invalid_kind")
        (source "semantic")
        (range (start 10 16) (end 10 27))
        (related-information
          (related
            (uri "memory://snapshot/sysml_exhibit_state_usage_reference.md")
            (range (start 3 8) (end 3 21))
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
  (document "memory://snapshot/sysml_exhibit_state_usage_reference.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "exhibit_target_invalid_kind")
        (source "semantic")
        (range (start 10 16) (end 10 27))
        (related-information
          (related
            (uri "memory://snapshot/sysml_exhibit_state_usage_reference.md")
            (range (start 3 8) (end 3 21))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:537f86eed1d7423e1c9e2cf3e350a617b00a45f2e29947ac4d41f7fe2207b88b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base::inner"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base::operating"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Holder"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 0))))) (kind exhibit-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "Base::operating")))))
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 1))))) (kind exhibit-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "Base::inner")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Holder"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "Base::operating")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base::operating")))))
    (reference (id (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 1))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "Base::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base::inner")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Holder"))) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Holder"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base::operating"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 1))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base::inner"))) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base::operating"))) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base")))
      (subtype (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Holder")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base::inner")))
      (featured-by (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base")))
      (subtype (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base::operating")))
      (featured-by (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base")))
      (subtype (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Holder")))
      (supertype (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Holder")))
      (supertype (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base::operating")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Holder")))
      (supertype (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base::inner")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (range (start 5 23) (end 5 27)) (probe (position 5 23))
    (reference (id (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Holder"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (range (start 7 16) (end 7 31)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "Base::operating")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base::operating")))))
    )
  )
  (query (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (range (start 10 16) (end 10 27)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (anonymous (kind exhibit-state) (ordinal 1))))) (kind referenceSubsetting) (ordinal 0) (authored-target "Base::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Base::inner")))))
    )
  )
)
~~~
