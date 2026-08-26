# META
~~~ini
description=SysML 8.3.17.2 validateAcceptActionUsageParameters requires an AcceptActionUsage to have at least two input parameters, for its payload and its receiver
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.2 validateAcceptActionUsageParameters
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.2:validateAcceptActionUsageParameters
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the accept action below owns the payload and receiver input parameters its
// concrete syntax implies.
//
// The violating side has no textual counterpart: SysML accept syntax always authors both, so a
// source document cannot produce an AcceptActionUsage with fewer than two input parameters.
package Actions {
    action def Act {
        accept when true;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_accept_action_usage_parameters.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_accept_action_usage_parameters.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:917ff83da8bd8210ece5c1b649632edc73bc8e1cb57bd3a5ab3673fa55ebd464") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_usage_parameters.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_usage_parameters.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_accept_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_accept_action_usage_parameters.md") (qualified-name "Actions::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_accept_action_usage_parameters.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
