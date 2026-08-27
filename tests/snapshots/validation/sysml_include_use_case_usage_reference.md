# META
~~~ini
description=SysML 8.3.25.2 validateIncludeUseCaseUsageReference requires the featureTarget of the referencedFeature of an IncludeUseCaseUsage ownedReferenceSubsetting to be a UseCaseUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.25.2 validateIncludeUseCaseUsageReference
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.25.2:validateIncludeUseCaseUsageReference
type=file
~~~
# SOURCE
~~~sysml
package UseCases {
    part def Component;
    part def Library {
        use case operate;
        part other : Component;
    }
    use case def Main {
        subject item : Component;

        // Conforming: the included feature is a use case usage.
        include Library::operate;

        // Invalid: the included feature is a part usage, not a use case usage.
        include Library::other;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_include_use_case_usage_reference.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "use_case_include_invalid_target")
        (source "semantic")
        (range (start 13 16) (end 13 30))
        (related-information
          (related
            (uri "memory://snapshot/sysml_include_use_case_usage_reference.md")
            (range (start 4 8) (end 4 31))
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
  (document "memory://snapshot/sysml_include_use_case_usage_reference.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "use_case_include_invalid_target")
        (source "semantic")
        (range (start 13 16) (end 13 30))
        (related-information
          (related
            (uri "memory://snapshot/sysml_include_use_case_usage_reference.md")
            (range (start 4 8) (end 4 31))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:c88d14aa4e5cbef60100bee9f15694b74b3e2ffd3aee26130c4e8e9e9c7f1542"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::operate"))) (kind use-case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::other"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main"))) (kind use-case-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (includeUseCase (reference "Library::operate")) (includeUseCase (reference "Library::other")))))
    (declaration (id (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main::item"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::other"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main"))) (kind includeUseCase) (ordinal 0))
      (authored-target "Library::operate")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::operate")))))
    (reference (id (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main"))) (kind includeUseCase) (ordinal 1))
      (authored-target "Library::other")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::other")))))
    (reference (id (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main::item"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::other"))) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::other"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind includeUseCase) (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main"))) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::operate"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main"))) (kind includeUseCase) (ordinal 0)))
    (relationship (kind includeUseCase) (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main"))) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::other"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main"))) (kind includeUseCase) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main::item"))) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main::item"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::operate"))) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::other"))) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main::item"))) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Component")))
      (subtype (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::other")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main::item")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::operate")))
      (featured-by (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::other")))
      (featured-by (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library")))
      (type (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main::item")))
      (featured-by (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main")))
      (type (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Component")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (range (start 4 21) (end 4 30)) (probe (position 4 21))
    (reference (id (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::other"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (range (start 10 16) (end 10 32)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main"))) (kind includeUseCase) (ordinal 0) (authored-target "Library::operate")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::operate")))))
    )
  )
  (query (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (range (start 13 16) (end 13 30)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main"))) (kind includeUseCase) (ordinal 1) (authored-target "Library::other")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Library::other")))))
    )
  )
  (query (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (range (start 7 23) (end 7 32)) (probe (position 7 23))
    (reference (id (source (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Main::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_include_use_case_usage_reference.md") (qualified-name "UseCases::Component")))))
    )
  )
)
~~~
