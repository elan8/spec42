# META
~~~ini
description=SysML 8.3.26.6 checkRenderingUsageRedefinition requires a view rendering usage to redefine Views::View::viewRendering
specification=OMG SysML 2.0 (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.26.6:checkRenderingUsageRedefinition
blocked_by=lowering-gap-redefinition-view-rendering-membership
type=file
~~~
# SOURCE
~~~sysml
package Redefinition { view def View; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "sysml-2.0:8.3.26.6:checkRenderingUsageRedefinition") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_rendering_usage_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:c200e5ac9f85470907592e678bd49a63d1cb334af66406d5781b05309f298562") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_rendering_usage_redefinition.md") (qualified-name "Redefinition::View"))) (kind view-def) (membership (kind owning) (visibility default)))
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
