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
blocked_by=parser-gap-76-action-body-members
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
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 7 8) (end 8 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:917ff83da8bd8210ece5c1b649632edc73bc8e1cb57bd3a5ab3673fa55ebd464") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_usage_parameters.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_usage_parameters.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
  )
  (references
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
)
~~~
