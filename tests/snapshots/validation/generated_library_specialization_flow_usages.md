# META
~~~ini
description=A SysML flow usage specializes Transfers::transfers (KerML checkFlowSpecialization, as a Flow) and Flows::messages (checkFlowUsageSpecialization)
specification=OMG SysML 2.0 and KerML 1.0 (formal/26-03)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.9.2:checkFlowSpecialization
rule_id=sysml-2.0:8.3.16.3:checkFlowUsageSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package FlowSpecialization {
    part def Holder {
        part x { out item o; }
        part y { in item i; }
        flow x.o to y.i;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source (anonymous (owner "FlowSpecialization::Holder") (kind FlowConnectionUsage) (ordinal 0))) (target "Transfers::transfers") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source (anonymous (owner "FlowSpecialization::Holder") (kind FlowConnectionUsage) (ordinal 0))) (target "Flows::messages") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_library_specialization_flow_usages.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:f06acfddaff5e14df0bc7c4625d899a774cd2876cf80e9f0bcd91304fe91b4d8") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "x::o")) (flowTarget (reference "y::i")))))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::x"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::x::o"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::y"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::y::i"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "x::o")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::x::o")))))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "y::i")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::y::i")))))
  )
  (relationships
    (relationship (kind flowSource) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::x::o"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::y::i"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::x"))) (target (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::x"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::x"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::x::o"))) (target (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::x"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::x::o"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::y"))) (target (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::y"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::y"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::y::i"))) (target (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::y"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::y::i"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0)))))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::MessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::x")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::x::o")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::x")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::y")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::y::i")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::y")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_library_specialization_flow_usages.md") (range (start 4 13) (end 4 16)) (probe (position 4 13))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "x::o")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::x::o")))))
    )
  )
  (query (document "memory://snapshot/generated_library_specialization_flow_usages.md") (range (start 4 20) (end 4 23)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (path (named (kind package) (name "FlowSpecialization")) (named (kind part-def) (name "Holder")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "y::i")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_flow_usages.md") (qualified-name "FlowSpecialization::Holder::y::i")))))
    )
  )
)
~~~
