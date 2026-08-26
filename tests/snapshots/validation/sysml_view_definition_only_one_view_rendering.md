# META
~~~ini
description=SysML 8.3.26.7 validateViewDefinitionOnlyOneViewRendering allows a ViewDefinition at most one ViewRenderingMembership
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.26.7 validateViewDefinitionOnlyOneViewRendering
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.26.7:validateViewDefinitionOnlyOneViewRendering
blocked_by=lowering-view-rendering-membership
type=file
~~~
# SOURCE
~~~sysml
package Views {
    rendering def Tree;
    rendering def Table;

    // Conforming: one rendering membership.
    view def Good {
        render asTree : Tree;
    }

    // Invalid: two rendering memberships.
    view def Bad {
        render asTree : Tree;
        render asTable : Table;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md"
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
  (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md"
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
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:ed93b90fad421e3b56c1864ebf9fee8d64e32398f22f799d437520fd45f34ab7") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Good"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Table"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree"))) (kind rendering-def) (membership (kind owning) (visibility default)))
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
