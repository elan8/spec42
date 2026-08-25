# META
~~~ini
description=Generated Connector association specialization consumes the canonical typed AssociationStructure relation
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.5.3:checkConnectorObjectSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package ConnectorAssociationSpecializations {
    classifier Thing;
    assoc struct LinkObject;

    classifier Holder {
        feature a : Thing;
        feature b : Thing;
        connector pair : LinkObject {
            end feature source :>> a;
            end feature target :>> b;
        }
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "ConnectorAssociationSpecializations::Holder::pair") (target "Objects::linkObjects") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_connector_association_specializations.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:130908310e076b4f3193f29651211727a553651da3d0268d87a5895957725375") (contract-version "lossless-publication-completeness-v3") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LinkObject")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "a")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "b")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::LinkObject"))) (kind kerml-association-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair"))) (kind featureTyping) (ordinal 0))
      (authored-target "LinkObject")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::LinkObject")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::source"))) (kind redefinition) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::target"))) (kind redefinition) (ordinal 0))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::b")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::a"))) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::b"))) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair"))) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::LinkObject"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::source"))) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::source"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::target"))) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::target"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::a"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::a"))) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::b"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::b"))) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair"))) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::source"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::source"))) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::source"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::target"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::target"))) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::target"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::LinkObject"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::a")))
      (featured-by (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder")))
      (type (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::source")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::b")))
      (featured-by (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder")))
      (type (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::target")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair")))
      (featured-by (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder")))
      (type (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::LinkObject")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::LinkObject")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::source")))
      (featured-by (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair")))
      (effective-type (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")) (source inherited) (from (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::a"))))
      (supertype (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::a")) (scopes any feature))
      (supertype (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::target")))
      (featured-by (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair")))
      (effective-type (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")) (source inherited) (from (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::b"))))
      (supertype (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::b")) (scopes any feature))
      (supertype (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::LinkObject")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")))
      (subtype (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::a")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::b")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (range (start 5 20) (end 5 25)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::a"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (range (start 6 20) (end 6 25)) (probe (position 6 20))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::b"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Thing")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (range (start 7 25) (end 7 35)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair"))) (kind featureTyping) (ordinal 0) (authored-target "LinkObject")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::LinkObject")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (range (start 8 35) (end 8 36)) (probe (position 8 35))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::source"))) (kind redefinition) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (range (start 9 35) (end 9 36)) (probe (position 9 35))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::pair::target"))) (kind redefinition) (ordinal 0) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_connector_association_specializations.md") (qualified-name "ConnectorAssociationSpecializations::Holder::b")))))
    )
  )
)
~~~
