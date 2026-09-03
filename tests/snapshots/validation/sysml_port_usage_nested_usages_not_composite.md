# META
~~~ini
description=SysML 8.3.12.6 validatePortUsageNestedUsagesNotComposite requires the nestedUsages of a PortUsage that are not themselves PortUsages to be non-composite
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.12.6 validatePortUsageNestedUsagesNotComposite
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.12.6:validatePortUsageNestedUsagesNotComposite
type=file
~~~
# SOURCE
~~~sysml
package Ports {
    part def Component;
    port def Signal;
    item def Message;
    part def Holder {
        // Conforming: the nested members of the port usage are referential.
        port good : Signal {
            attribute reading;
        }

        // Conforming: a directed item nested in a port usage is a flow feature.
        port directed : Signal {
            in item rx : Message;
            out item tx : Message;
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
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 6 8) (end 8 9))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 11 8) (end 14 9))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 17 8) (end 19 9))
      )
      (diagnostic
        (severity warning)
        (code "port_nested_usage_composite")
        (source "semantic")
        (range (start 18 12) (end 18 35))
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
        (range (start 6 8) (end 8 9))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 11 8) (end 14 9))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 17 8) (end 19 9))
      )
      (diagnostic
        (severity warning)
        (code "port_nested_usage_composite")
        (source "semantic")
        (range (start 18 12) (end 18 35))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9ecb70e46b115c0ca5aab1e924065ea622af814d39d51dfc61ac68deca6ea9ca"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad::owned"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::rx"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Message")))))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::tx"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Message")))))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good::reading"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Message"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal"))) (kind port-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")))))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad::owned"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")))))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::rx"))) (kind featureTyping) (ordinal 0))
      (authored-target "Message")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Message")))))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::tx"))) (kind featureTyping) (ordinal 0))
      (authored-target "Message")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Message")))))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad::owned"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad::owned"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::rx"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Message"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::rx"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::tx"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Message"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::tx"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad::owned"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::rx"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::tx"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good::reading"))) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Component")))
      (subtype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad::owned")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad")))
      (featured-by (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder")))
      (type (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad::owned")))
      (featured-by (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad")))
      (type (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed")))
      (featured-by (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder")))
      (type (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::rx")))
      (featured-by (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed")))
      (type (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Message")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Message")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Message")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::tx")))
      (featured-by (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed")))
      (type (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Message")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Message")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Message")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Message")))
      (subtype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::rx")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::tx")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")))
      (subtype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (range (start 17 19) (end 17 25)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")))))
    )
  )
  (query (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (range (start 18 25) (end 18 34)) (probe (position 18 25))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::bad::owned"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (range (start 11 24) (end 11 30)) (probe (position 11 24))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")))))
    )
  )
  (query (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (range (start 12 25) (end 12 32)) (probe (position 12 25))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::rx"))) (kind featureTyping) (ordinal 0) (authored-target "Message")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Message")))))
    )
  )
  (query (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (range (start 13 26) (end 13 33)) (probe (position 13 26))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::directed::tx"))) (kind featureTyping) (ordinal 0) (authored-target "Message")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Message")))))
    )
  )
  (query (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (range (start 6 20) (end 6 26)) (probe (position 6 20))
    (reference (id (source (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Holder::good"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_usage_nested_usages_not_composite.md") (qualified-name "Ports::Signal")))))
    )
  )
)
~~~
