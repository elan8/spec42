# META
~~~ini
description=Generated accept-action specialization distinguishes standalone and subaction AcceptActionUsage facts
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.2:checkAcceptActionUsageSpecialization
rule_id=sysml-2.0:8.3.17.2:checkAcceptActionUsageSubactionSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package AcceptActionSpecializations {
    item def Message;

    action standalone accept payload : Message;

    action def Parent {
        action child accept payload : Message;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source "AcceptActionSpecializations::standalone") (target "Actions::acceptActions") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source "AcceptActionSpecializations::Parent::child") (target "Actions::acceptActions") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source "AcceptActionSpecializations::Parent::child") (target "Actions::Action::acceptSubactions") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_accept_action_specializations.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:97c5cfe4a958c23f32313593c20bab7a1cf7000e9c9275f28a7054f5e5303a7f") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Message"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Parent"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Parent::child"))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (acceptPayloadType (reference "Message")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::standalone"))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (acceptPayloadType (reference "Message")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Parent::child"))) (kind acceptPayloadType) (ordinal 0))
      (authored-target "Message")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Message")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::standalone"))) (kind acceptPayloadType) (ordinal 0))
      (authored-target "Message")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Message")))))
  )
  (relationships
    (relationship (kind acceptPayloadType) (source (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Parent::child"))) (target (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Message"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Parent::child"))) (kind acceptPayloadType) (ordinal 0)))
    (relationship (kind acceptPayloadType) (source (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::standalone"))) (target (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Message"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::standalone"))) (kind acceptPayloadType) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Message"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Parent"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Parent::child"))) (target (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Parent"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Parent::child"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Parent::child"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::acceptActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::standalone"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::acceptActions"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Message")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Parent")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Parent::child")))
      (featured-by (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Parent")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::acceptSubactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::acceptActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::acceptSubactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::acceptActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::standalone")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::acceptActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::acceptActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (range (start 6 38) (end 6 45)) (probe (position 6 38))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Parent::child"))) (kind acceptPayloadType) (ordinal 0) (authored-target "Message")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Message")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (range (start 3 39) (end 3 46)) (probe (position 3 39))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::standalone"))) (kind acceptPayloadType) (ordinal 0) (authored-target "Message")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_accept_action_specializations.md") (qualified-name "AcceptActionSpecializations::Message")))))
    )
  )
)
~~~
