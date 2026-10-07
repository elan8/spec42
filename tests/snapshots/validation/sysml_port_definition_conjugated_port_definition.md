# META
~~~ini
description=SysML 8.3.12.5 validatePortDefinitionConjugatedPortDefinition requires a PortDefinition that is not itself a ConjugatedPortDefinition to have exactly one ownedMember that is a ConjugatedPortDefinition
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.12.5 validatePortDefinitionConjugatedPortDefinition
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.12.5:validatePortDefinitionConjugatedPortDefinition
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the port def declaration below implies its own conjugated port definition, which
// is what the ~ operator names elsewhere in a model.
//
// The violating side has no textual counterpart: the implied ConjugatedPortDefinition is derived
// from the PortDefinition declaration itself, so a source document cannot author a port
// definition without one.
//
// Note: the implied ConjugatedPortDefinition is not published as an owned member in SMG, so this
// fixture pins only that the accepted side reports nothing.
package Ports {
    port def Signal;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_port_definition_conjugated_port_definition.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_port_definition_conjugated_port_definition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8bd3632bcf616efb0fdec6bddad90c2d1a23a35018dde51b76de3ef4997238c9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_conjugated_port_definition.md") (qualified-name "Ports"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_conjugated_port_definition.md") (qualified-name "Ports::Signal"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_conjugated_port_definition.md") (path (named (kind package) (name "Ports")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Signal") (short-name absent) (provenance original-port-definition)))
  )
  (references
  )
  (relationships
    (relationship (kind conjugation) (source (node (document "memory://snapshot/sysml_port_definition_conjugated_port_definition.md") (path (named (kind package) (name "Ports")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_port_definition_conjugated_port_definition.md") (qualified-name "Ports::Signal"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
