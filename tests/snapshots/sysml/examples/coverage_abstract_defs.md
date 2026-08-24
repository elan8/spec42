# META
~~~ini
description=Coverage: Abstract and variation SysML definitions in body and top-level context
type=file
~~~
# SOURCE
~~~sysml
abstract part def AbstractVehicle;
abstract attribute def Weight;
abstract item def AbstractWidget;
abstract port def AbstractPort;
abstract enum def AbstractPriority;
abstract individual def AbstractPerson;
abstract occurrence def AbstractEvent;

variation part def EngineChoices {
    variant part fourCyl;
    variant part sixCyl;
}

abstract part def Container {
    abstract attribute def InnerWeight;
    abstract enum def InnerColor;
    abstract item def InnerWidget;
    abstract part def InnerPart;
    abstract port def InnerPort;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/coverage_abstract_defs.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 4 0) (end 4 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 0) (end 4 35))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 9 12) (end 9 25))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 10 12) (end 10 24))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "parser")
        (range (start 15 4) (end 16 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:78b35de4d04c636bb29de5b8d361ee7022e2ce9eb901ed20f63f349bfb04f71d") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "AbstractEvent"))) (kind occurrence-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "AbstractPerson"))) (kind individual-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "AbstractPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "AbstractVehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "AbstractWidget"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "Container"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "Container::InnerPart"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "Container::InnerPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "Container::InnerWeight"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "Container::InnerWidget"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "EngineChoices"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "EngineChoices::fourCyl"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "EngineChoices::sixCyl"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "Weight"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "EngineChoices::fourCyl"))) (target (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "EngineChoices"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "EngineChoices::sixCyl"))) (target (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "EngineChoices"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "EngineChoices::fourCyl")))
      (featured-by (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "EngineChoices")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "EngineChoices::sixCyl")))
      (featured-by (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "EngineChoices")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
