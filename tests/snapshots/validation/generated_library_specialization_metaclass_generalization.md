# META
~~~ini
description=Generated library specializations keyed by a general metaclass apply to its SysML specializations through the nearest rule-carrying metaclass: SuccessionAsUsage takes checkSuccessionSpecialization from Succession and BindingConnectorAsUsage takes checkBindingConnectorSpecialization from BindingConnector, while a ForkNode's own checkForkNodeSpecialization shadows checkControlNodeSpecialization
specification=OMG SysML 2.0 and KerML 1.0 (formal/26-03)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.5.4:checkSuccessionSpecialization
rule_id=kerml-1.0:8.3.4.5.2:checkBindingConnectorSpecialization
rule_id=sysml-2.0:8.3.17.6:checkControlNodeSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
// The Pilot's ImplicitGeneralizationMap walks from an element's metaclass to the nearest one with
// a default supertype. A fork node's own anchor Actions::Action::forks subsets
// Actions::Action::controls in the library, so checkControlNodeSpecialization holds through it
// (the pinned XMI spells that anchor Action::Action::controls).
package MetaclassGeneralization {
    part def Holder {
        attribute a;
        attribute b;
        bind a = b;
    }
    action def Act {
        action start;
        action finish;
        succession first start then finish;
        fork f;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source (anonymous (owner "MetaclassGeneralization::Act") (kind SuccessionAsUsage) (ordinal 0))) (target "Occurrences::happensBeforeLinks") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source (anonymous (owner "MetaclassGeneralization::Holder") (kind BindingConnectorAsUsage) (ordinal 0))) (target "Links::selfLinks") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source "MetaclassGeneralization::Act::f") (target "Actions::Action::forks") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:8c95cd15053ef00c1fef132467b5aa00d563579917b946091c324534131bbf19") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "start")) (succession (reference "finish")))))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::f"))) (kind fork) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::finish"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::start"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind part-def) (name "Holder")) (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "a")) (bindTarget (reference "b")))))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder::a"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder::b"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::start")))))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "finish")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::finish")))))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind part-def) (name "Holder")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind part-def) (name "Holder")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder::b")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::start"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::finish"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind bindSource) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind part-def) (name "Holder")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind part-def) (name "Holder")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0)))
    (relationship (kind bindTarget) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind part-def) (name "Holder")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind part-def) (name "Holder")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::f"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::forks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::f"))) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::finish"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::finish"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::finish"))) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::start"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::start"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::start"))) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind part-def) (name "Holder")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::selfLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind part-def) (name "Holder")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder::a"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder::a"))) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder::b"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder::b"))) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Without")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::f")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ControlAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::controls"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ForkAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::forks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::controls")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::forks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ControlAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ForkAction")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::finish")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act")))
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
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::start")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act")))
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
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind part-def) (name "Holder")) (anonymous (kind bind) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::SelfLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::selfLinks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::SelfLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::selfLinks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder::a")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder::b")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (range (start 13 25) (end 13 30)) (probe (position 13 25))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::start")))))
    )
  )
  (query (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (range (start 13 36) (end 13 42)) (probe (position 13 36))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "finish")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Act::finish")))))
    )
  )
  (query (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (range (start 8 13) (end 8 14)) (probe (position 8 13))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind part-def) (name "Holder")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (range (start 8 17) (end 8 18)) (probe (position 8 17))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (path (named (kind package) (name "MetaclassGeneralization")) (named (kind part-def) (name "Holder")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_metaclass_generalization.md") (qualified-name "MetaclassGeneralization::Holder::b")))))
    )
  )
)
~~~
