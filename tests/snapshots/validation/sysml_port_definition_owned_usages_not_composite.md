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

    // Conforming: the non-port members of the port definition are referential.
    port def Good {
        attribute reading;
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
        (range (start 10 8) (end 10 31))
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
        (range (start 10 8) (end 10 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5540e3a20a362ce258e71607914063746b85a8bfd6898ce9ee69f6257cb8ee90") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad::owned"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Good"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Good::reading"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad::owned"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad::owned"))) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad::owned"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad::owned"))) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Good::reading")))
      (featured-by (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Good")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (range (start 10 21) (end 10 30)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Bad::owned"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_port_definition_owned_usages_not_composite.md") (qualified-name "Ports::Component")))))
    )
  )
)
~~~
