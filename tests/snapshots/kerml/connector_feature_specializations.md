# META
~~~ini
description=KerML connector FeatureDeclaration specializations (`:`, `:>`, `:>>`) are lowered in authored order as typing, subsetting and redefinition
type=file
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureOwnedSubsetting
coverage_role=secondary
~~~
# SOURCE
~~~kerml
package ConnSpec {
    assoc A;
    classifier Holder {
        feature a;
        feature b;
        connector base : A from a to b;
    }
    classifier Sub specializes Holder {
        connector sub : A :> base from a to b;
    }
    classifier Redef specializes Holder {
        connector :>> base from a to b;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind feature_typing)
    (source "ConnSpec::Sub::sub")
    (target "ConnSpec::A")
    (provenance authored)
    (outcome resolved))
  (relationship
    (kind subsetting)
    (source "ConnSpec::Sub::sub")
    (target "ConnSpec::Holder::base")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/connector_feature_specializations.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "association_related_types_insufficient")
        (source "semantic")
        (range (start 1 4) (end 1 12))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d982f4fe79cc9efb43a7abdb40df7ea7456543178efc938f5a4d7c9dd38072db"))
  (declarations
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")) (connectorEnd (reference "a")) (connectorEnd (reference "b")))))
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Redef"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Holder")))))
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind kerml-connector) (membership (kind feature) (visibility default)) (effective-identification (name "base") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "base")) (connectorEnd (reference "a")) (connectorEnd (reference "b")))))
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Holder")))))
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")) (subsetting (reference "base")) (connectorEnd (reference "a")) (connectorEnd (reference "b")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")))))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (kind connectorEnd) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (kind connectorEnd) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b")))))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Redef"))) (kind specialization) (ordinal 0))
      (authored-target "Holder")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder")))))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base")))))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b")))))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub"))) (kind specialization) (ordinal 0))
      (authored-target "Holder")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder")))))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")))))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (kind subsetting) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base")))))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (kind connectorEnd) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (kind connectorEnd) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Redef"))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Redef"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub"))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (kind subsetting) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a"))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b"))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Redef"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")))
      (subtype (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base")) (scopes any))
      (subtype (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder")))
      (subtype (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Redef")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a")))
      (featured-by (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b")))
      (featured-by (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base")))
      (featured-by (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder")))
      (type (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")) (source direct))
      (supertype (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")) (scopes any))
      (subtype (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Redef")))
      (supertype (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Redef")))
      (effective-type (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")) (source inherited) (from (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))))
      (supertype (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")) (scopes any))
      (supertype (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub")))
      (supertype (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub")))
      (featured-by (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub")))
      (type (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")) (source direct))
      (effective-type (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")) (source inherited) (from (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))))
      (supertype (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")) (scopes any))
      (supertype (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base")) (scopes any feature))
    )
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (kind connection) (type (resolved (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")))) (end bare (feature (resolved (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a"))))) (end bare (feature (resolved (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b"))))))
  (connector (id (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connection) (end bare (feature (resolved (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a"))))) (end bare (feature (resolved (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b"))))))
  (connector (id (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (kind connection) (type (resolved (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")))) (end bare (feature (resolved (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a"))))) (end bare (feature (resolved (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b"))))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/connector_feature_specializations.md") (range (start 5 25) (end 5 26)) (probe (position 5 25))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")))))
    )
  )
  (query (document "memory://snapshot/connector_feature_specializations.md") (range (start 5 32) (end 5 33)) (probe (position 5 32))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (kind connectorEnd) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/connector_feature_specializations.md") (range (start 5 37) (end 5 38)) (probe (position 5 37))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base"))) (kind connectorEnd) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b")))))
    )
  )
  (query (document "memory://snapshot/connector_feature_specializations.md") (range (start 10 33) (end 10 39)) (probe (position 10 33))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Redef"))) (kind specialization) (ordinal 0) (authored-target "Holder")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder")))))
    )
  )
  (query (document "memory://snapshot/connector_feature_specializations.md") (range (start 11 22) (end 11 26)) (probe (position 11 22))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base")))))
    )
  )
  (query (document "memory://snapshot/connector_feature_specializations.md") (range (start 11 32) (end 11 33)) (probe (position 11 32))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/connector_feature_specializations.md") (range (start 11 37) (end 11 38)) (probe (position 11 37))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (path (named (kind package) (name "ConnSpec")) (named (kind kerml-classifier) (name "Redef")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b")))))
    )
  )
  (query (document "memory://snapshot/connector_feature_specializations.md") (range (start 7 31) (end 7 37)) (probe (position 7 31))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub"))) (kind specialization) (ordinal 0) (authored-target "Holder")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder")))))
    )
  )
  (query (document "memory://snapshot/connector_feature_specializations.md") (range (start 8 24) (end 8 25)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::A")))))
    )
  )
  (query (document "memory://snapshot/connector_feature_specializations.md") (range (start 8 29) (end 8 33)) (probe (position 8 29))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (kind subsetting) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::base")))))
    )
  )
  (query (document "memory://snapshot/connector_feature_specializations.md") (range (start 8 39) (end 8 40)) (probe (position 8 39))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (kind connectorEnd) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/connector_feature_specializations.md") (range (start 8 44) (end 8 45)) (probe (position 8 44))
    (reference (id (source (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Sub::sub"))) (kind connectorEnd) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_feature_specializations.md") (qualified-name "ConnSpec::Holder::b")))))
    )
  )
)
~~~
