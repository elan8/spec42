# META
~~~ini
description=SysML 8.3.26.10 validateViewRenderingMembershipOwningType requires the owningType of a ViewRenderingMembership to be a ViewDefinition or a ViewUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.26.10 validateViewRenderingMembershipOwningType
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.26.10:validateViewRenderingMembershipOwningType
type=file
~~~
# SOURCE
~~~sysml
package Views {
    rendering def Tree;

    // Conforming: the rendering membership is owned by a view definition.
    view def Good {
        render rendering asTree : Tree;
    }

    // Invalid: the rendering membership is owned by a part definition.
    part def Bad {
        render rendering asTree : Tree;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "view_rendering_invalid_owner")
        (source "semantic")
        (range (start 10 8) (end 10 39))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "view_rendering_invalid_owner")
        (source "semantic")
        (range (start 10 8) (end 10 39))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:eaa6a088affd691c3c98e0b7bfd80c4dd27bb016495e742c1e25d0be03986751"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Bad"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Bad::asTree"))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)) (authored (membership (kind feature) (visibility default) (role view-rendering)) (relationships (featureTyping (reference "Tree")))))
    (declaration (id (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Good"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Good::asTree"))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)) (authored (membership (kind feature) (visibility default) (role view-rendering)) (relationships (featureTyping (reference "Tree")))))
    (declaration (id (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree"))) (kind rendering-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Bad::asTree"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree")))))
    (reference (id (source (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Good::asTree"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Bad::asTree"))) (target (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Bad::asTree"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Good::asTree"))) (target (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Good::asTree"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Bad::asTree"))) (target (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Good::asTree"))) (target (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Bad::asTree")))
      (featured-by (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Bad")))
      (type (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Good::asTree")))
      (featured-by (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Good")))
      (type (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree")))
      (subtype (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Bad::asTree")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Good::asTree")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (range (start 10 34) (end 10 38)) (probe (position 10 34))
    (reference (id (source (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Bad::asTree"))) (kind featureTyping) (ordinal 0) (authored-target "Tree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree")))))
    )
  )
  (query (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (range (start 5 34) (end 5 38)) (probe (position 5 34))
    (reference (id (source (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Good::asTree"))) (kind featureTyping) (ordinal 0) (authored-target "Tree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree")))))
    )
  )
)
~~~
