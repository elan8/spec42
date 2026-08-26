# META
~~~ini
description=SysML 8.3.12.2 validateConjugatedPortDefinitionConjugatedPortDefinitionIsEmpty forbids a ConjugatedPortDefinition from having a conjugatedPortDefinition of its own
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.12.2 validateConjugatedPortDefinitionConjugatedPortDefinitionIsEmpty
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.12.2:validateConjugatedPortDefinitionConjugatedPortDefinitionIsEmpty
blocked_by=lowering-conjugated-port-definition
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the conjugate of a port definition is derived once from that port definition and
// is not itself conjugated again.
//
// The violating side has no textual counterpart: SysML concrete syntax never authors a
// ConjugatedPortDefinition directly, so a source document cannot give one a conjugate of its
// own.
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
  (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 9 8) (end 9 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5081caf7b7633e996f7ffd732e70bba5ff157f3f25f87f0ced3a5971c0ed2b8a") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Holder::inbound"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Signal"))) (kind port-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Holder::inbound"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Signal")))))
  )
  (relationships
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Holder::inbound"))) (target (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Holder::inbound"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Holder::inbound"))) (target (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Holder::inbound")))
      (featured-by (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Holder")))
      (type (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Signal")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Signal")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Signal")))
      (subtype (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Holder::inbound")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (range (start 9 24) (end 9 30)) (probe (position 9 24))
    (reference (id (source (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Holder::inbound"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_conjugated_port_definition_is_empty.md") (qualified-name "Ports::Signal")))))
    )
  )
)
~~~
