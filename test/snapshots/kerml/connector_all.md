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
  (document "connector_all.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 1 4) (end 1 209))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "8b1aa2652f6ff9ff643f8ba0e8e37117a2e8dd9df7bc25cbcbf4642536f81fe4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ConnectorAll"))) (kind "package") (name "ConnectorAll") (declared-name "ConnectorAll"))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
