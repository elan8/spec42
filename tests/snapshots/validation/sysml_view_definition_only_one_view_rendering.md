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
type=file
~~~
# SOURCE
~~~sysml
package Views {
    rendering def Tree;
    rendering def Table;

    // Conforming: one rendering membership.
    view def Good {
        render rendering asTree : Tree;
    }

    // Invalid: two rendering memberships.
    view def Bad {
        render rendering asTree : Tree;
        render rendering asTable : Table;
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
        (range (start 12 8) (end 12 41))
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
        (code "view_multiple_renderings")
        (source "semantic")
        (range (start 12 8) (end 12 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e0061aabf312d4f35c2871971aacd17c8cab47253e85eac91b76ad3e491a0225"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTable"))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)) (authored (membership (kind feature) (visibility default) (role view-rendering)) (relationships (featureTyping (reference "Table")))))
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTree"))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)) (authored (membership (kind feature) (visibility default) (role view-rendering)) (relationships (featureTyping (reference "Tree")))))
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Good"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Good::asTree"))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)) (authored (membership (kind feature) (visibility default) (role view-rendering)) (relationships (featureTyping (reference "Tree")))))
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Table"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree"))) (kind rendering-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTable"))) (kind featureTyping) (ordinal 0))
      (authored-target "Table")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Table")))))
    (reference (id (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTree"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree")))))
    (reference (id (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Good::asTree"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTable"))) (target (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Table"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTable"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTree"))) (target (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTree"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Good::asTree"))) (target (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Good::asTree"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTable"))) (target (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTree"))) (target (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Good::asTree"))) (target (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTable")))
      (featured-by (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad")))
      (type (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Table")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Table")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Table")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTree")))
      (featured-by (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad")))
      (type (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Good::asTree")))
      (featured-by (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Good")))
      (type (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Table")))
      (subtype (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTable")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree")))
      (subtype (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTree")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Good::asTree")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (range (start 12 35) (end 12 40)) (probe (position 12 35))
    (reference (id (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTable"))) (kind featureTyping) (ordinal 0) (authored-target "Table")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Table")))))
    )
  )
  (query (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (range (start 11 34) (end 11 38)) (probe (position 11 34))
    (reference (id (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Bad::asTree"))) (kind featureTyping) (ordinal 0) (authored-target "Tree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree")))))
    )
  )
  (query (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (range (start 6 34) (end 6 38)) (probe (position 6 34))
    (reference (id (source (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Good::asTree"))) (kind featureTyping) (ordinal 0) (authored-target "Tree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_definition_only_one_view_rendering.md") (qualified-name "Views::Tree")))))
    )
  )
)
~~~
