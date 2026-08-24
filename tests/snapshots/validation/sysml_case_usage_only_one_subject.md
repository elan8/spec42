# META
~~~ini
description=SysML 8.3.22.3 validateCaseUsageOnlyOneSubject allows a CaseUsage at most one featureMembership that is a SubjectMembership
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.22.3 validateCaseUsageOnlyOneSubject
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.22.3:validateCaseUsageOnlyOneSubject
type=file
~~~
# SOURCE
~~~sysml
package Roles {
    part def Component;

    // Conforming: one subject membership.
    case Good {
        subject first : Component;
    }

    // Invalid: two subject memberships.
    case Bad {
        subject first : Component;
        subject second : Component;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_case_usage_only_one_subject.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_role_member")
        (source "semantic")
        (range (start 11 8) (end 11 35))
        (related-information
          (related
            (uri "memory://snapshot/sysml_case_usage_only_one_subject.md")
            (range (start 10 8) (end 10 34))
          )
        )
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_case_usage_only_one_subject.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_role_member")
        (source "semantic")
        (range (start 11 8) (end 11 35))
        (related-information
          (related
            (uri "memory://snapshot/sysml_case_usage_only_one_subject.md")
            (range (start 10 8) (end 10 34))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:989674672e8eda988ab790cd501f6ff867e08acb681097067d74e9b2cd775cc5") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad"))) (kind case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::first"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::second"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Good"))) (kind case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Good::first"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::first"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::second"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Good::first"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::first"))) (target (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::first"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::second"))) (target (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::second"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Good::first"))) (target (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Good::first"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::first"))) (target (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::second"))) (target (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Good::first"))) (target (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::first")))
      (featured-by (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad")))
      (type (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::second")))
      (featured-by (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad")))
      (type (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")))
      (subtype (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::first")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::second")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Good::first")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Good::first")))
      (featured-by (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Good")))
      (type (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (range (start 10 24) (end 10 33)) (probe (position 10 24))
    (reference (id (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::first"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (range (start 11 25) (end 11 34)) (probe (position 11 25))
    (reference (id (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Bad::second"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (range (start 5 24) (end 5 33)) (probe (position 5 24))
    (reference (id (source (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Good::first"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_case_usage_only_one_subject.md") (qualified-name "Roles::Component")))))
    )
  )
)
~~~
