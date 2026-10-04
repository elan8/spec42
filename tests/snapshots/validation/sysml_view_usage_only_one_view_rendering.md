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
type=file
~~~
# SOURCE
~~~sysml
package Views {
    rendering def Tree;
    rendering def Table;

    // Conforming: one rendering membership.
    view good {
        render rendering asTree : Tree;
    }

    // Invalid: two rendering memberships.
    view bad {
        render rendering asTree : Tree;
        render rendering asTable : Table;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "view_expose_empty")
        (source "semantic")
        (range (start 5 4) (end 7 5))
      )
      (diagnostic
        (severity information)
        (code "view_expose_empty")
        (source "semantic")
        (range (start 10 4) (end 13 5))
      )
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
  (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "view_expose_empty")
        (source "semantic")
        (range (start 5 4) (end 7 5))
      )
      (diagnostic
        (severity information)
        (code "view_expose_empty")
        (source "semantic")
        (range (start 10 4) (end 13 5))
      )
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:052e67717dbe70f7126c1d50dc9c349a81ca0253df6973b7da73ca563ef2edb3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Table"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTable"))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)) (authored (membership (kind feature) (visibility default) (role view-rendering)) (relationships (featureTyping (reference "Table")))))
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTree"))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)) (authored (membership (kind feature) (visibility default) (role view-rendering)) (relationships (featureTyping (reference "Tree")))))
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::good"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::good::asTree"))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)) (authored (membership (kind feature) (visibility default) (role view-rendering)) (relationships (featureTyping (reference "Tree")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTable"))) (kind featureTyping) (ordinal 0))
      (authored-target "Table")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Table")))))
    (reference (id (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTree"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree")))))
    (reference (id (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::good::asTree"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTable"))) (target (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Table"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTable"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTree"))) (target (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTree"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::good::asTree"))) (target (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::good::asTree"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTable"))) (target (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTree"))) (target (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::good::asTree"))) (target (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Table")))
      (subtype (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTable")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree")))
      (subtype (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTree")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::good::asTree")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTable")))
      (featured-by (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad")))
      (type (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Table")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Table")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Table")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTree")))
      (featured-by (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad")))
      (type (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::good::asTree")))
      (featured-by (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::good")))
      (type (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (range (start 12 35) (end 12 40)) (probe (position 12 35))
    (reference (id (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTable"))) (kind featureTyping) (ordinal 0) (authored-target "Table")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Table")))))
    )
  )
  (query (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (range (start 11 34) (end 11 38)) (probe (position 11 34))
    (reference (id (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::bad::asTree"))) (kind featureTyping) (ordinal 0) (authored-target "Tree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree")))))
    )
  )
  (query (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (range (start 6 34) (end 6 38)) (probe (position 6 34))
    (reference (id (source (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::good::asTree"))) (kind featureTyping) (ordinal 0) (authored-target "Tree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_usage_only_one_view_rendering.md") (qualified-name "Views::Tree")))))
    )
  )
)
~~~
