# META
~~~ini
description=SysML 8.3.12.5 validatePortDefinitionOwnedUsagesNotComposite requires the ownedUsages of a PortDefinition that are not PortUsages to be non-composite
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.12.5 validatePortDefinitionOwnedUsagesNotComposite
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.12.5:validatePortDefinitionOwnedUsagesNotComposite
type=file
~~~
# SOURCE
~~~sysml
package Ports {
    part def Component;
    item def Signal;

    // Conforming: the non-port members of the port definition are referential.
    port def Good {
        attribute reading;
    }

    // Conforming: directed items are flow features, not composite subparts
    // (Pilot `isReferenceDefault` treats every directed feature as a reference).
    port def Directed {
        in item rx : Signal;
        out item tx : Signal;
        inout item status : Signal;
    }

    // Invalid: a composite part usage member of a port definition.
    port def Bad {
        part owned : Component;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "port_owned_usage_composite")
        (source "semantic")
        (range (start 19 8) (end 19 31))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "port_owned_usage_composite")
        (source "semantic")
        (range (start 19 8) (end 19 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ee9aac67c049c718739699dcca124cab1678c756a4ac2500617382d9cb9e62ab"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad::owned"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::rx"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::status"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction inout)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::tx"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Good"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Good::reading"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal"))) (kind item-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad::owned"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::rx"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")))))
    (reference (id (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::status"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")))))
    (reference (id (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::tx"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad::owned"))) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad::owned"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::rx"))) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::rx"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::status"))) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::status"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::tx"))) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::tx"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad::owned"))) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::rx"))) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::status"))) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::tx"))) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Good::reading"))) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad::owned")))
      (featured-by (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad")))
      (type (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Component")))
      (subtype (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad::owned")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::rx")))
      (featured-by (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed")))
      (type (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::status")))
      (featured-by (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed")))
      (type (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::tx")))
      (featured-by (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed")))
      (type (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Good::reading")))
      (featured-by (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")))
      (subtype (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::rx")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::status")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::tx")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (range (start 19 21) (end 19 30)) (probe (position 19 21))
    (reference (id (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad::owned"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (range (start 12 21) (end 12 27)) (probe (position 12 21))
    (reference (id (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::rx"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")))))
    )
  )
  (query (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (range (start 14 28) (end 14 34)) (probe (position 14 28))
    (reference (id (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::status"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")))))
    )
  )
  (query (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (range (start 13 22) (end 13 28)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Directed::tx"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Signal")))))
    )
  )
)
~~~
