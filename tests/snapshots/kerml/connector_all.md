# META
~~~ini
description=KerML connector with 'all' keyword (stdlib patterns from OccurrenceFunctions/TransitionPerformances)
type=file
~~~
# SOURCE
~~~kerml
package ConnectorAll {
    connector all during: HappensDuring from self to occ;
    connector all guardConstraint: TPCGuardConstraint[*] from transitionLink to guard;
    connector all x from a to b;
    connector all from a to b;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/connector_all.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 26) (end 1 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 1 45) (end 1 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 1 53) (end 1 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 35) (end 2 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 62) (end 2 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 80) (end 2 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 25) (end 3 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 30) (end 3 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 23) (end 4 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 28) (end 4 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5544e6186e9524f6031d7e6095efc59b0e47848e1809c9197683ca088ff33162") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_all.md") (path (named (kind package) (name "ConnectorAll")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind kerml-connector) (membership (kind feature) (visibility default)) (facts (modifiers all)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "a")) (connectorEnd (reference "b")))))
    (declaration (id (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::during"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (facts (modifiers all)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensDuring")) (connectorEnd (reference "self")) (connectorEnd (reference "occ")))))
    (declaration (id (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::guardConstraint"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (facts (modifiers all) (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TPCGuardConstraint")) (connectorEnd (reference "transitionLink")) (connectorEnd (reference "guard")))))
    (declaration (id (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::x"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (facts (modifiers all)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "a")) (connectorEnd (reference "b")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (path (named (kind package) (name "ConnectorAll")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (path (named (kind package) (name "ConnectorAll")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "b")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::during"))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensDuring")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::during"))) (kind connectorEnd) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::during"))) (kind connectorEnd) (ordinal 1))
      (authored-target "occ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::guardConstraint"))) (kind featureTyping) (ordinal 0))
      (authored-target "TPCGuardConstraint")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::guardConstraint"))) (kind connectorEnd) (ordinal 0))
      (authored-target "transitionLink")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::guardConstraint"))) (kind connectorEnd) (ordinal 1))
      (authored-target "guard")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::x"))) (kind connectorEnd) (ordinal 0))
      (authored-target "a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::x"))) (kind connectorEnd) (ordinal 1))
      (authored-target "b")
      (outcome (status unresolved)))
  )
  (relationships
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
  (query (document "memory://snapshot/connector_all.md") (range (start 4 23) (end 4 24)) (probe (position 4 23))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (path (named (kind package) (name "ConnectorAll")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "a")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connector_all.md") (range (start 4 28) (end 4 29)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (path (named (kind package) (name "ConnectorAll")) (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "b")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connector_all.md") (range (start 1 26) (end 1 39)) (probe (position 1 26))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::during"))) (kind featureTyping) (ordinal 0) (authored-target "HappensDuring")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connector_all.md") (range (start 1 45) (end 1 49)) (probe (position 1 45))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::during"))) (kind connectorEnd) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connector_all.md") (range (start 1 53) (end 1 56)) (probe (position 1 53))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::during"))) (kind connectorEnd) (ordinal 1) (authored-target "occ")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connector_all.md") (range (start 2 35) (end 2 53)) (probe (position 2 35))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::guardConstraint"))) (kind featureTyping) (ordinal 0) (authored-target "TPCGuardConstraint")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connector_all.md") (range (start 2 62) (end 2 76)) (probe (position 2 62))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::guardConstraint"))) (kind connectorEnd) (ordinal 0) (authored-target "transitionLink")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connector_all.md") (range (start 2 80) (end 2 85)) (probe (position 2 80))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::guardConstraint"))) (kind connectorEnd) (ordinal 1) (authored-target "guard")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connector_all.md") (range (start 3 25) (end 3 26)) (probe (position 3 25))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::x"))) (kind connectorEnd) (ordinal 0) (authored-target "a")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connector_all.md") (range (start 3 30) (end 3 31)) (probe (position 3 30))
    (reference (id (source (node (document "memory://snapshot/connector_all.md") (qualified-name "ConnectorAll::x"))) (kind connectorEnd) (ordinal 1) (authored-target "b")
      (outcome (status unresolved)))
    )
  )
)
~~~
