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
blocked_by=lowering-part-definition-members
type=file
~~~
# SOURCE
~~~sysml
package States {
    state def Machine {
        state operating;
    }
    part def Component {
        part inner;
    }
    part def Holder {
        // Conforming: the exhibited feature is a state usage.
        exhibit Machine::operating;

        // Invalid: the exhibited feature is a part usage, not a state usage.
        exhibit Component::inner;
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
        (range (start 12 8) (end 12 33))
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
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 1 4) (end 3 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 1 4) (end 3 5))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 5 8) (end 5 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 9 8) (end 9 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 12 8) (end 12 33))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:8e16bb5396457a78d73f640d5429aed396e61b327a48a124314b12bca753d17d") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Component::inner"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Machine::operating"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Component::inner"))) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Component"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Machine::operating"))) (target (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Machine"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Component::inner")))
      (featured-by (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Component")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Machine::operating")))
      (featured-by (node (document "memory://snapshot/sysml_exhibit_state_usage_reference.md") (qualified-name "States::Machine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
