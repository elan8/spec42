# META
~~~ini
description=SysML 8.3.12.6 validatePortUsageIsReference requires a PortUsage whose owningType is neither a PortDefinition nor a PortUsage to be referential
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.12.6 validatePortUsageIsReference
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.12.6:validatePortUsageIsReference
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the port usage below is owned by a part definition and is referential, which is
// what the port keyword produces there.
//
// The violating side has no textual counterpart: SysML concrete syntax has no spelling that
// makes a port usage composite outside a port definition or port usage, so the rule is
// observable only as the accepted side pinned here.
package Ports {
    port def Signal;
    part def Holder {
        port inbound : Signal;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_port_usage_is_reference.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_port_usage_is_reference.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:47f3c1fb7e53235de2a05553b8cd385f64ebce6493ffd0dbcaad4c848e8d668f"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Holder::inbound"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Signal"))) (kind port-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Holder::inbound"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Signal")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Holder::inbound"))) (target (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Holder::inbound"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Holder::inbound"))) (target (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Holder::inbound")))
      (featured-by (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Holder")))
      (type (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Signal")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Signal")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Signal")))
      (subtype (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Holder::inbound")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_port_usage_is_reference.md") (range (start 9 23) (end 9 29)) (probe (position 9 23))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Holder::inbound"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_is_reference.md") (qualified-name "Ports::Signal")))))
    )
  )
)
~~~
