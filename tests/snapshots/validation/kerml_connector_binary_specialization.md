# META
~~~ini
description=KerML 8.3.4.5.3 validateConnectorBinarySpecialization forbids a Connector with more than two connectorEnds from specializing Links::BinaryLink
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.5.3 validateConnectorBinarySpecialization
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.5.3:validateConnectorBinarySpecialization
blocked_by=parser-gap-69-connector-end-body
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package Connectors {
    classifier Thing;
    classifier Holder {
        feature a : Thing;
        feature b : Thing;
        feature c : Thing;

        // Conforming: a two-ended connector may specialize BinaryLink.
        connector pair specializes Links::BinaryLink { end feature e1 :>> a; end feature e2 :>> b; }

        // Invalid: a three-ended connector must not specialize BinaryLink.
        connector tern specializes Links::BinaryLink { end feature e1 :>> a; end feature e2 :>> b; end feature e3 :>> c; }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_connector_binary_specialization.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "binary_connector_end_count")
        (source "semantic")
        (range (start 11 34) (end 11 51))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_connector_binary_specialization.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 8 8) (end 11 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 11 8) (end 12 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery,unsupported-syntax) (has-evaluation true) (source-digest "blake3:f548ccbf31623958e01c79bf454ae33b11148e08b9ca163c4901020285f865ac") (contract-version "lossless-publication-completeness-v3") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::c"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::a"))) (target (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::b"))) (target (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::c"))) (target (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::a"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::a"))) (target (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::b"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::b"))) (target (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::c"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::c"))) (target (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::a")))
      (featured-by (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder")))
      (type (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::b")))
      (featured-by (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder")))
      (type (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::c")))
      (featured-by (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder")))
      (type (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")))
      (subtype (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::a")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::b")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::c")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_connector_binary_specialization.md") (range (start 3 20) (end 3 25)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::a"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_binary_specialization.md") (range (start 4 20) (end 4 25)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::b"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_binary_specialization.md") (range (start 5 20) (end 5 25)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Holder::c"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_binary_specialization.md") (qualified-name "Connectors::Thing")))))
    )
  )
)
~~~
