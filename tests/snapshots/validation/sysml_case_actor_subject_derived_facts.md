# META
~~~ini
description=Case definitions and usages derive their actor and subject parameters
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.22.2:deriveCaseDefinitionActorParameter
rule_id=sysml-2.0:8.3.22.2:deriveCaseDefinitionSubjectParameter
rule_id=sysml-2.0:8.3.22.3:deriveCaseUsageActorParameter
rule_id=sysml-2.0:8.3.22.3:deriveCaseUsageSubjectParameter
libraries=none
~~~
# SOURCE
~~~sysml
package Cases {
    part def Person;
    case def Inquiry {
        subject driver : Person;
        actor operator : Person;
    }
    case inquiry {
        subject driver : Person;
        actor operator : Person;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.22.2:deriveCaseDefinitionActorParameter") (source "Cases::Inquiry") (target "Cases::Inquiry::operator") (outcome resolved))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.22.2:deriveCaseDefinitionSubjectParameter") (source "Cases::Inquiry") (target "Cases::Inquiry::driver") (outcome resolved))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.22.3:deriveCaseUsageActorParameter") (source "Cases::inquiry") (target "Cases::inquiry::operator") (outcome resolved))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.22.3:deriveCaseUsageSubjectParameter") (source "Cases::inquiry") (target "Cases::inquiry::driver") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8947d3a01d263885ae7876adc5cf97984acd6726967d9fedd37b34ec1dd1b2b1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry"))) (kind case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::driver"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::operator"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry"))) (kind case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::driver"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::operator"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")))))
    (reference (id (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::operator"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")))))
    (reference (id (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")))))
    (reference (id (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::operator"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::driver"))) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::operator"))) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::operator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::driver"))) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::operator"))) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::operator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::driver"))) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::operator"))) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::driver"))) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::operator"))) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::driver")))
      (featured-by (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry")))
      (type (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::operator")))
      (featured-by (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry")))
      (type (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")))
      (subtype (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::driver")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::operator")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::driver")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::operator")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::driver")))
      (featured-by (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry")))
      (type (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::operator")))
      (featured-by (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry")))
      (type (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (range (start 3 25) (end 3 31)) (probe (position 3 25))
    (reference (id (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")))))
    )
  )
  (query (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (range (start 4 25) (end 4 31)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Inquiry::operator"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")))))
    )
  )
  (query (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (range (start 7 25) (end 7 31)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")))))
    )
  )
  (query (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (range (start 8 25) (end 8 31)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::inquiry::operator"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_actor_subject_derived_facts.md") (qualified-name "Cases::Person")))))
    )
  )
)
~~~
