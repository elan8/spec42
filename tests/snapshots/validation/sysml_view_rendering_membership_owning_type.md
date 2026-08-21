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
blocked_by=parser-gap-79-membership-owner-forms
type=file
~~~
# SOURCE
~~~sysml
package Views {
    rendering def Tree;

    // Conforming: the rendering membership is owned by a view definition.
    view def Good {
        render asTree : Tree;
    }

    // Invalid: the rendering membership is owned by a part definition.
    part def Bad {
        render asTree : Tree;
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
        (code "view_rendering_invalid_target")
        (source "semantic")
        (range (start 9 4) (end 9 18))
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
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 5 8) (end 5 29))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 10 8) (end 11 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:c11a6ba7a831fa811abbe2f8f153d22ae796d9f466f2bf041c26afd7794331f3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Bad"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Good"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_view_rendering_membership_owning_type.md") (qualified-name "Views::Tree"))) (kind rendering-def) (membership (kind owning) (visibility default)))
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
