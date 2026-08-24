# META
~~~ini
description=Usage mayTimeVary retains its required effective occurrence, library-specialization, and portion fact boundary
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.6.4:deriveUsageMayTimeVary
blocked_by=lowering-gap-definition-usage-effective-occurrence-time-variation-facts
libraries=none
~~~
# SOURCE
~~~sysml
package Model { part vehicle; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageMayTimeVary") (source "Model::vehicle") (outcome false)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_may_time_vary.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 0 16) (end 0 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:620d7ded4019e63eaa4c3c23977ed00db4e63f9812b54ed774826fd66f6e6fa2") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_may_time_vary.md") (qualified-name "Model::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
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
