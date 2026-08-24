# META
~~~ini
description=SysML 8.3.17.5 checkAssignmentActionUsageAccessedFeatureRedefinition requires the nested target feature to redefine accessedFeature
specification=OMG SysML 2.0 (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.5:checkAssignmentActionUsageAccessedFeatureRedefinition
blocked_by=lowering-gap-redefinition-assignment-input-parameters
type=file
~~~
# SOURCE
~~~sysml
package Redefinition { action def Work; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "sysml-2.0:8.3.17.5:checkAssignmentActionUsageAccessedFeatureRedefinition") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_assignment_action_usage_accessed_feature_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:30ccc431b3e61856309b9e4b99eb4708b7e4f0a835943cd5225df9aef24f2978") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_accessed_feature_redefinition.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_accessed_feature_redefinition.md") (qualified-name "Redefinition::Work"))) (kind action-def) (membership (kind owning) (visibility default)))
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
