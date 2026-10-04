# META
~~~ini
description=SysML 8.3.12.2 validateConjugatedPortDefinitionOriginalPortDefinition requires the originalPortDefinition of the ownedPortConjugator to be the originalPortDefinition of the ConjugatedPortDefinition
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.12.2 validateConjugatedPortDefinitionOriginalPortDefinition
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.12.2:validateConjugatedPortDefinitionOriginalPortDefinition
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the conjugated port definition used below derives both its port conjugator and its
// originalPortDefinition from the same port definition.
//
// The violating side has no textual counterpart: both are derived from one declaration, so a
// source document cannot make them disagree.
package Ports {
    port def Signal;
    part def Holder {
        port inbound : ~Signal;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:43532133548f6f5a4f218cdc7685e3763954d9d6551f6fd6a54defe8a40c8974"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Holder::inbound"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Signal"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (path (named (kind package) (name "Ports")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Signal") (short-name absent) (provenance original-port-definition)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Holder::inbound"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Signal")))))
  )
  (relationships
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Holder::inbound"))) (target (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Holder::inbound"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Holder::inbound"))) (target (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Holder"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Holder::inbound"))) (target (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (path (named (kind package) (name "Ports")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (path (named (kind package) (name "Ports")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Signal"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Holder::inbound")))
      (featured-by (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Holder")))
      (type (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Signal")) (provenance authored))
      (type (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (path (named (kind package) (name "Ports")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (path (named (kind package) (name "Ports")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (supertype (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Signal")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (path (named (kind package) (name "Ports")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Signal")))
      (subtype (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Holder::inbound")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (path (named (kind package) (name "Ports")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Holder::inbound")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (range (start 8 24) (end 8 30)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Holder::inbound"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_conjugated_port_definition_original_port_definition.md") (qualified-name "Ports::Signal")))))
    )
  )
)
~~~
