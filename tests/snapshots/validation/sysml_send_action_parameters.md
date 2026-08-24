# META
~~~ini
description=SysML 8.3.17.15 validateSendActionParameters requires a SendActionUsage to have at least three owned input parameters, for its payload, sender and receiver
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.15 validateSendActionParameters
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.15:validateSendActionParameters
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the send action below owns the payload, sender and receiver input parameters its
// concrete syntax implies.
//
// The violating side has no textual counterpart: SysML send syntax always authors all three, so a
// source document cannot produce a SendActionUsage with fewer than three input parameters.
package Actions {
    action def Act {
        action target;
        send 1 to target;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_send_action_parameters.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_send_action_parameters.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e0814c17d923de186543c1b9601b690dd414c2019df417db10cac9f8f0e910e8") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_send_action_parameters.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_send_action_parameters.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_send_action_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind action) (ordinal 0))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (sendTarget (reference "target")))))
    (declaration (id (node (document "memory://snapshot/sysml_send_action_parameters.md") (qualified-name "Actions::Act::target"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_send_action_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind action) (ordinal 0))))) (kind sendTarget) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_send_action_parameters.md") (qualified-name "Actions::Act::target")))))
  )
  (relationships
    (relationship (kind sendTarget) (source (node (document "memory://snapshot/sysml_send_action_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_send_action_parameters.md") (qualified-name "Actions::Act::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_send_action_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind action) (ordinal 0))))) (kind sendTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_send_action_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_send_action_parameters.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_send_action_parameters.md") (qualified-name "Actions::Act::target"))) (target (node (document "memory://snapshot/sysml_send_action_parameters.md") (qualified-name "Actions::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_send_action_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_send_action_parameters.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_send_action_parameters.md") (qualified-name "Actions::Act::target")))
      (featured-by (node (document "memory://snapshot/sysml_send_action_parameters.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_send_action_parameters.md") (range (start 8 18) (end 8 24)) (probe (position 8 18))
    (reference (id (source (node (document "memory://snapshot/sysml_send_action_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind action) (ordinal 0))))) (kind sendTarget) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_send_action_parameters.md") (qualified-name "Actions::Act::target")))))
    )
  )
)
~~~
