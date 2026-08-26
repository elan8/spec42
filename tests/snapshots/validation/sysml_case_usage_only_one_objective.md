# META
~~~ini
description=SysML 8.3.22.3 validateCaseUsageOnlyOneObjective allows a CaseUsage at most one featureMembership that is an ObjectiveMembership
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.22.3 validateCaseUsageOnlyOneObjective
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.22.3:validateCaseUsageOnlyOneObjective
blocked_by=semantic-duplicate-role-member
type=file
~~~
# SOURCE
~~~sysml
package Roles {
    part def Component;

    // Conforming: one objective membership.
    case Good {
        subject s : Component;
        objective first;
    }

    // Invalid: two objective memberships.
    case Bad {
        subject s : Component;
        objective first;
        objective second;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_case_usage_only_one_objective.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_role_member")
        (source "semantic")
        (range (start 13 8) (end 13 25))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_case_usage_only_one_objective.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:dbc5d53ce3c265a89476ee03fd644716184ac8a20d84ff5b2808342c4d9554e2") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad"))) (kind case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad::first"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad::s"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad::second"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good"))) (kind case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good::first"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good::s"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad::s"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good::s"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad::s"))) (target (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad::s"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good::s"))) (target (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good::s"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad::first"))) (target (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad::s"))) (target (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad::second"))) (target (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good::first"))) (target (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good::s"))) (target (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad::first")))
      (featured-by (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad::s")))
      (featured-by (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad")))
      (type (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad::second")))
      (featured-by (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Component")))
      (subtype (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad::s")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good::s")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good::first")))
      (featured-by (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good::s")))
      (featured-by (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good")))
      (type (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Component")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (range (start 11 20) (end 11 29)) (probe (position 11 20))
    (reference (id (source (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Bad::s"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (range (start 5 20) (end 5 29)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Good::s"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_usage_only_one_objective.md") (qualified-name "Roles::Component")))))
    )
  )
)
~~~
