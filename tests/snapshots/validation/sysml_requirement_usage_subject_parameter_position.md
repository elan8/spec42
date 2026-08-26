# META
~~~ini
description=SysML 8.3.21.9 validateRequirementUsageSubjectParameterPosition requires the subjectParameter of a RequirementUsage to be its first input
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.21.9 validateRequirementUsageSubjectParameterPosition
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.21.9:validateRequirementUsageSubjectParameterPosition
blocked_by=semantic-subject-member-order
type=file
~~~
# SOURCE
~~~sysml
package Roles {
    part def Component;

    // Conforming: the subject parameter comes first.
    requirement Good {
        subject first : Component;
        in ref part other : Component;
    }

    // Invalid: another input precedes the subject parameter.
    requirement Bad {
        in ref part other : Component;
        subject later : Component;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "subject_member_not_first")
        (source "semantic")
        (range (start 12 8) (end 12 34))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:66280e19e650228a42951c148a73839fc3b6ce708b1b13f2bfbcb493183cdb24") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::later"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::other"))) (kind ref) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::first"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::other"))) (kind ref) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::later"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::other"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::first"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::other"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::later"))) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::later"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::other"))) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::other"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::first"))) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::first"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::other"))) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::other"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::later"))) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::other"))) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::first"))) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::other"))) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::later")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad")))
      (type (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::other")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad")))
      (type (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")))
      (subtype (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::later")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::other")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::first")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::other")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::first")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good")))
      (type (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::other")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good")))
      (type (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (range (start 12 24) (end 12 33)) (probe (position 12 24))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::later"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (range (start 11 28) (end 11 37)) (probe (position 11 28))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Bad::other"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (range (start 5 24) (end 5 33)) (probe (position 5 24))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::first"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (range (start 6 28) (end 6 37)) (probe (position 6 28))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Good::other"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_usage_subject_parameter_position.md") (qualified-name "Roles::Component")))))
    )
  )
)
~~~
