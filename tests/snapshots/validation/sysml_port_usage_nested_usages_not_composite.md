# META
~~~ini
description=SysML 8.3.12.6 validatePortUsageNestedUsagesNotComposite requires the nestedUsages of a PortUsage that are not themselves PortUsages to be non-composite
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.12.6 validatePortUsageNestedUsagesNotComposite
type=file
skip_validation=the pinned parser has no production for a part usage inside a port usage body, so the composite member is reported as unexpected_keyword_in_scope and never reaches semantics
~~~
# SOURCE
~~~sysml
package Ports {
    part def Component;
    port def Signal;
    part def Holder {
        // Conforming: the nested members of the port usage are referential.
        port good : Signal {
            attribute reading;
        }

        // Invalid: a composite part usage nested in a port usage.
        port bad : Signal {
            part owned : Component;
        }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "port_nested_usage_composite")
        (source "semantic")
        (range (start 11 12) (end 11 35))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 5 8) (end 7 9))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 10 8) (end 12 9))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 11 12) (end 12 8))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:633263db47c0a5fd62ce0e9618f1be2f3356b8c13d365fe1057d8f7347c25b0d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good::reading"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal"))) (kind port-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")))))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad")))
      (featured-by (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder")))
      (type (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good")))
      (featured-by (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder")))
      (type (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good::reading")))
      (featured-by (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")))
      (subtype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (range (start 10 19) (end 10 25)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")))))
    )
  )
  (query (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (range (start 5 20) (end 5 26)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")))))
    )
  )
)
~~~
