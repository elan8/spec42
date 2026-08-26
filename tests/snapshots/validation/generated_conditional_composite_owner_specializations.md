# META
~~~ini
description=Generated conditional library specialization follows the exact composite-plus-owning-type predicate
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.4:checkActionUsageOwnedActionSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package CompositeOwnerSpecializations {
    part def Vehicle {
        action maintain;
        ref action inspect;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "CompositeOwnerSpecializations::Vehicle::maintain") (target "Parts::Part::ownedActions") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_composite_owner_specializations.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:2eb61f75938c20f58698fb6008daf93d4601434f2312f9477ad14803436dda52") (contract-version "semantic-metadata-projection-v6") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle::inspect"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers reference)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle::maintain"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle::inspect"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle::inspect"))) (target (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle::maintain"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle::maintain"))) (target (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle::maintain"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedActions"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle::inspect")))
      (featured-by (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle::maintain")))
      (featured-by (node (document "memory://snapshot/generated_conditional_composite_owner_specializations.md") (qualified-name "CompositeOwnerSpecializations::Vehicle")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::involvingPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::ownedPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedActions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
