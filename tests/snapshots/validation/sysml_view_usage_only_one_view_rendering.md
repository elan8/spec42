# META
~~~ini
description=SysML 8.3.26.11 validateViewUsageOnlyOneViewRendering allows a ViewUsage at most one ViewRenderingMembership
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.26.11 validateViewUsageOnlyOneViewRendering
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.26.11:validateViewUsageOnlyOneViewRendering
blocked_by=lowering-view-rendering-membership
type=file
~~~
# SOURCE
~~~sysml
package Views {
    rendering def Tree;
    rendering def Table;

    // Conforming: one rendering membership.
    view good {
        render asTree : Tree;
    }

    // Invalid: two rendering memberships.
    view bad {
        render asTree : Tree;
        render asTable : Table;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "view_multiple_renderings")
        (source "semantic")
        (range (start 12 8) (end 12 31))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 6 8) (end 6 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 11 8) (end 11 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 12 8) (end 12 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:7e34beb6c8b77280361aec6b235f31518fc2e1a3eb430c2b31e36c87c1102eab") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Table"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::good"))) (kind view) (membership (kind feature) (visibility default)))
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
