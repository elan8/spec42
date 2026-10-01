# META
~~~ini
description=KerML checkSuccessionSpecialization requires a Succession, including a SysML SuccessionAsUsage, to specialize Occurrences::happensBeforeLinks
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.5.4:checkSuccessionSpecialization
blocked_by=semantic-succession-as-usage-library-specialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package SuccessionSpecialization {
    action def Act {
        action start;
        action finish;
        succession first start then finish;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source (anonymous (owner "SuccessionSpecialization::Act") (kind SuccessionAsUsage) (ordinal 0))) (target "Occurrences::happensBeforeLinks") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_succession_as_usage_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:685d498b5c6bfb6cfd3910a7911409b4c54300245905252e0a05bff90edf2ca1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (path (named (kind package) (name "SuccessionSpecialization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "start")) (succession (reference "finish")))))
    (declaration (id (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::finish"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::start"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (path (named (kind package) (name "SuccessionSpecialization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::start")))))
    (reference (id (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (path (named (kind package) (name "SuccessionSpecialization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "finish")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::finish")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (path (named (kind package) (name "SuccessionSpecialization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::start"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (path (named (kind package) (name "SuccessionSpecialization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (path (named (kind package) (name "SuccessionSpecialization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::finish"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (path (named (kind package) (name "SuccessionSpecialization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (path (named (kind package) (name "SuccessionSpecialization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::finish"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::finish"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::finish"))) (target (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::start"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::start"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::start"))) (target (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (path (named (kind package) (name "SuccessionSpecialization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act")))
    )
    (declaration (id (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::finish")))
      (featured-by (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
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
    )
    (declaration (id (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::start")))
      (featured-by (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
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
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_succession_as_usage_specialization.md") (range (start 4 25) (end 4 30)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (path (named (kind package) (name "SuccessionSpecialization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::start")))))
    )
  )
  (query (document "memory://snapshot/generated_succession_as_usage_specialization.md") (range (start 4 36) (end 4 42)) (probe (position 4 36))
    (reference (id (source (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (path (named (kind package) (name "SuccessionSpecialization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "finish")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_succession_as_usage_specialization.md") (qualified-name "SuccessionSpecialization::Act::finish")))))
    )
  )
)
~~~
