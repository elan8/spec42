# META
~~~ini
description=SysML 8.3.17.10 validateIfActionUsageParameters requires an IfActionUsage to have at least two owned input parameters
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.10 validateIfActionUsageParameters
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.10:validateIfActionUsageParameters
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the if action below owns the condition and then-branch input parameters its
// concrete syntax implies.
//
// The violating side has no textual counterpart: SysML if syntax always authors a condition and
// at least one branch, so a source document cannot produce an IfActionUsage with fewer than two
// owned input parameters.
package Actions {
    action def Act {
        action a1;
        action a2;
        if true { perform a1; } else { perform a2; }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_if_action_usage_parameters.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_if_action_usage_parameters.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:ad05883502189b64d4b95d36ffc748911677bfb0516a191b1bdba458c69ad700"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0))))) (kind if) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (kind kerml-literal-boolean) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "a1")))))
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 1))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "a2")))))
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a1"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a2"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a1")))))
    (reference (id (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 1))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "a2")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a2")))))
  )
  (relationships
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 1))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a1"))) (target (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a2"))) (target (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a1")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a2")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a1")))
      (featured-by (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act")))
      (subtype (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a2")))
      (featured-by (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act")))
      (subtype (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 1)))) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0))))) (outcome resolved) (literal (value (kind boolean) (boolean true))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_if_action_usage_parameters.md") (range (start 10 26) (end 10 28)) (probe (position 10 26))
    (reference (id (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a1")))))
    )
  )
  (query (document "memory://snapshot/sysml_if_action_usage_parameters.md") (range (start 10 47) (end 10 49)) (probe (position 10 47))
    (reference (id (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (anonymous (kind perform-action) (ordinal 1))))) (kind referenceSubsetting) (ordinal 0) (authored-target "a2")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a2")))))
    )
  )
)
~~~
